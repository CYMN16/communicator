use crate::theme::{self, Palette, Theme};
use egui::{
    Align, Align2, Color32, Context, CornerRadius, FontId, Id, Layout, Margin, Pos2, Rect,
    Response, Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Localisation
// ---------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
enum Language {
    English,
    Turkish,
}

/// Free function (rather than a method) so it can be called from inside UI
/// closures that already hold a mutable borrow of the app.
/// The user-visible strings, one table per language, in the same key order.
///
/// A table rather than a `match` so that adding a language stays a data change
/// — and so the whole lot can move to a file the day a translator needs it.
static STRINGS_EN: [(&str, &str); 85] = [
    ("menu", "Menu"),
    ("close", "Close"),
    ("done", "Done"),
    ("edit_mode", "Edit"),
    ("sentence_builder", "SENTENCE"),
    ("clear", "Clear"),
    ("undo", "Undo"),
    ("undo_edit", "Undo edit"),
    ("settings", "Settings"),
    ("back", "‹ Back"),
    ("add_category", "+ Category"),
    ("add_word", "+ Word"),
    ("delete_category", "Delete this category"),
    ("category_name", "Name"),
    ("category_color", "Colour"),
    ("icon", "Icon"),
    ("vocab_terms", "Words"),
    ("new_category", "New Category"),
    ("new_word", "New Word"),
    ("categories", "Categories"),
    ("editor", "Editor"),
    ("add_new_category", "+ Add category"),
    ("add_term", "+ Add word"),
    ("language", "Language"),
    (
        "select_category",
        "Pick a category on the left to edit its name, colour and words.",
    ),
    ("planes_of_existence", "PLANES"),
    ("appearance", "Appearance"),
    ("theme", "Theme"),
    ("light", "Light"),
    ("dark", "Dark"),
    ("high_contrast", "High contrast"),
    ("interface_size", "Interface size"),
    ("planes", "Planes"),
    ("add_plane", "+ Add plane"),
    ("belongs_to", "Plane"),
    ("presets", "Presets"),
    ("custom", "Custom"),
    ("dock_hint", "Tap words to build a sentence here"),
    ("empty_plane", "This plane is empty"),
    (
        "empty_plane_hint",
        "Turn on Edit to place your first category here.",
    ),
    ("edit_banner", "Edit mode — drag to move, tap to rename"),
    ("home", "Home"),
    ("hospital", "Hospital"),
    ("social", "Social"),
    ("remove", "Tap to remove"),
    ("speak", "Speak"),
    ("stop_speaking", "Stop speaking"),
    ("speech", "Speech"),
    ("speech_enabled", "Speak sentences out loud"),
    ("speak_on_tap", "Also speak each word as it is tapped"),
    ("speech_rate", "Speed"),
    ("test", "Test"),
    ("speech_sample", "I want to talk with you"),
    (
        "no_voice",
        "No speech engine was found on this device. On Linux, install speech-dispatcher or espeak-ng; the browser version speaks without any setup.",
    ),
    ("safety", "Safety"),
    ("lock_editing", "Unlock editing with"),
    ("lock_hold", "Press and hold"),
    ("lock_pin", "PIN code"),
    (
        "lock_hold_hint",
        "Hold the Edit button for a moment to start editing. One tap leaves it.",
    ),
    (
        "lock_pin_hint",
        "Editing asks for the four-digit code. One tap leaves it.",
    ),
    ("pin_code", "Code"),
    (
        "pin_incomplete",
        "Needs four digits — holding still works until then.",
    ),
    ("unlock_hold", "Hold to unlock editing"),
    ("unlock_pin", "Enter the code to unlock editing"),
    ("enter_pin", "Enter code"),
    ("wrong_pin", "Not that code"),
    ("undo_hint", "Undo the last change to the board"),
    ("board_data", "Board data"),
    (
        "board_data_hint",
        "Copy your board somewhere safe — a note, an email to yourself, a file. Paste it back here to restore it on any device.",
    ),
    ("copy_board", "Copy board"),
    ("load_board", "Load board"),
    ("board_copied", "Copied to the clipboard."),
    ("board_loaded", "Board loaded."),
    ("board_invalid", "That is not a Communicator board"),
    (
        "board_too_new",
        "That board was saved by a newer version of the app.",
    ),
    ("board_no_planes", "That board has no planes."),
    (
        "board_paste_hint",
        "Paste a saved board here, then press Load board",
    ),
    ("cancel", "Cancel"),
    ("confirm_delete", "Delete"),
    ("confirm_replace", "Replace"),
    ("delete_q", "Delete"),
    ("words_lost", "words will be deleted with it."),
    ("categories_moved", "categories will move to another plane."),
    ("replace_board_q", "Replace the whole board?"),
    ("undo_available", "You can undo this with the Undo button."),
];

static STRINGS_TR: [(&str, &str); 85] = [
    ("menu", "Menü"),
    ("close", "Kapat"),
    ("done", "Bitti"),
    ("edit_mode", "Düzenle"),
    ("sentence_builder", "CÜMLE"),
    ("clear", "Temizle"),
    ("undo", "Geri al"),
    ("undo_edit", "Geri al"),
    ("settings", "Ayarlar"),
    ("back", "‹ Geri"),
    ("add_category", "+ Kategori"),
    ("add_word", "+ Kelime"),
    ("delete_category", "Bu kategoriyi sil"),
    ("category_name", "Ad"),
    ("category_color", "Renk"),
    ("icon", "Simge"),
    ("vocab_terms", "Kelimeler"),
    ("new_category", "Yeni Kategori"),
    ("new_word", "Yeni Kelime"),
    ("categories", "Kategoriler"),
    ("editor", "Düzenleyici"),
    ("add_new_category", "+ Kategori ekle"),
    ("add_term", "+ Kelime ekle"),
    ("language", "Dil"),
    (
        "select_category",
        "Adını, rengini ve kelimelerini düzenlemek için soldan bir kategori seçin.",
    ),
    ("planes_of_existence", "DÜZLEMLER"),
    ("appearance", "Görünüm"),
    ("theme", "Tema"),
    ("light", "Açık"),
    ("dark", "Koyu"),
    ("high_contrast", "Yüksek kontrast"),
    ("interface_size", "Arayüz boyutu"),
    ("planes", "Düzlemler"),
    ("add_plane", "+ Düzlem ekle"),
    ("belongs_to", "Düzlem"),
    ("presets", "Hazır renkler"),
    ("custom", "Özel"),
    ("dock_hint", "Cümle kurmak için kelimelere dokunun"),
    ("empty_plane", "Bu düzlem boş"),
    (
        "empty_plane_hint",
        "İlk kategoriyi eklemek için Düzenle modunu açın.",
    ),
    (
        "edit_banner",
        "Düzenleme modu — taşımak için sürükleyin, yeniden adlandırmak için dokunun",
    ),
    ("home", "Ana Sayfa"),
    ("hospital", "Hastane"),
    ("social", "Sosyal"),
    ("remove", "Kaldırmak için dokunun"),
    ("speak", "Seslendir"),
    ("stop_speaking", "Konuşmayı durdur"),
    ("speech", "Konuşma"),
    ("speech_enabled", "Cümleleri sesli oku"),
    ("speak_on_tap", "Her kelimeyi dokunulduğunda da oku"),
    ("speech_rate", "Hız"),
    ("test", "Dene"),
    ("speech_sample", "Seninle konuşmak istiyorum"),
    (
        "no_voice",
        "Bu cihazda konuşma motoru bulunamadı. Linux'ta speech-dispatcher veya espeak-ng kurun; tarayıcı sürümü ek kurulum gerektirmez.",
    ),
    ("safety", "Güvenlik"),
    ("lock_editing", "Düzenlemeyi açma yöntemi"),
    ("lock_hold", "Basılı tut"),
    ("lock_pin", "PIN kodu"),
    (
        "lock_hold_hint",
        "Düzenlemeye başlamak için Düzenle düğmesini basılı tutun. Çıkmak için tek dokunuş yeter.",
    ),
    (
        "lock_pin_hint",
        "Düzenleme dört haneli kodu ister. Çıkmak için tek dokunuş yeter.",
    ),
    ("pin_code", "Kod"),
    (
        "pin_incomplete",
        "Dört hane gerekli — o zamana kadar basılı tutma çalışır.",
    ),
    ("unlock_hold", "Düzenlemeyi açmak için basılı tutun"),
    ("unlock_pin", "Düzenlemeyi açmak için kodu girin"),
    ("enter_pin", "Kodu girin"),
    ("wrong_pin", "Kod yanlış"),
    ("undo_hint", "Panodaki son değişikliği geri al"),
    ("board_data", "Pano verisi"),
    (
        "board_data_hint",
        "Panonuzu güvenli bir yere kopyalayın — bir not, kendinize bir e-posta, bir dosya. Geri yüklemek için buraya yapıştırın.",
    ),
    ("copy_board", "Panoyu kopyala"),
    ("load_board", "Panoyu yükle"),
    ("board_copied", "Panoya kopyalandı."),
    ("board_loaded", "Pano yüklendi."),
    ("board_invalid", "Bu bir Communicator panosu değil"),
    (
        "board_too_new",
        "Bu pano uygulamanın daha yeni bir sürümüyle kaydedilmiş.",
    ),
    ("board_no_planes", "Bu panoda hiç düzlem yok."),
    (
        "board_paste_hint",
        "Kayıtlı bir panoyu buraya yapıştırın, sonra Panoyu yükle'ye basın",
    ),
    ("cancel", "Vazgeç"),
    ("confirm_delete", "Sil"),
    ("confirm_replace", "Değiştir"),
    ("delete_q", "Silinsin mi:"),
    ("words_lost", "kelime de silinecek."),
    ("categories_moved", "kategori başka bir düzleme taşınacak."),
    ("replace_board_q", "Panonun tamamı değiştirilsin mi?"),
    (
        "undo_available",
        "Bunu Geri al düğmesiyle geri alabilirsiniz.",
    ),
];

/// Look up `key` for `lang`, falling back to the key itself so a missing
/// translation is visible rather than blank.
fn t(lang: Language, key: &'static str) -> &'static str {
    let table: &[(&str, &str)] = match lang {
        Language::English => &STRINGS_EN,
        Language::Turkish => &STRINGS_TR,
    };
    table
        .iter()
        .find(|(k, _)| *k == key)
        .map_or(key, |(_, value)| *value)
}

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone)]
struct Plane {
    name: String,
    icon: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct Term {
    text: String,
    #[serde(default)]
    icon: String,
    pos: [f32; 2],
}

#[derive(Deserialize, Serialize, Clone)]
struct Category {
    name: String,
    #[serde(default)]
    icon: String,
    color: [u8; 3],
    pos: [f32; 2],
    terms: Vec<Term>,
    #[serde(default = "default_plane_name")]
    plane: String,
}

/// One word sitting in the sentence dock.
#[derive(Clone, Default)]
struct DockChip {
    text: String,
    icon: String,
    color: [u8; 3],
}

/// Bumped whenever the saved shape changes in a way `serde(default)` cannot
/// paper over. Written into every exported board so old files stay readable.
const SCHEMA_VERSION: u32 = 1;

/// The portable form of a board: what "Copy board" produces and "Load board"
/// accepts. Deliberately just the vocabulary — not the theme or the pan offset.
#[derive(Deserialize, Serialize, Clone)]
struct BoardFile {
    schema_version: u32,
    planes: Vec<Plane>,
    categories: Vec<Category>,
}

/// A restore point for the undo stack. Editing is destructive and the person
/// doing it is often in a hurry, next to someone waiting to speak.
#[derive(Clone)]
struct Snapshot {
    planes: Vec<Plane>,
    categories: Vec<Category>,
    active_plane: String,
}

/// Something irreversible waiting for a second, deliberate confirmation.
#[derive(Clone)]
enum Confirm {
    DeleteCategory(usize),
    DeletePlane(usize),
    LoadBoard(BoardFile),
}

/// How editing is protected from the person using the board.
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
enum EditLock {
    /// Press and hold the Edit button for a moment.
    #[default]
    Hold,
    /// Enter a four-digit code.
    Pin,
}

/// Everything about the voice, in one persisted lump.
#[derive(Deserialize, Serialize, Clone)]
struct SpeechSettings {
    enabled: bool,
    /// Speak each word as it is added, as well as the finished sentence.
    speak_on_tap: bool,
    rate: f32,
}

impl Default for SpeechSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            speak_on_tap: false,
            rate: 1.0,
        }
    }
}

/// How long the Edit button must be held before editing unlocks.
const EDIT_HOLD_SECONDS: f32 = 0.7;

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone)]
pub struct CommunicatorApp {
    // --- Persistent ---
    #[serde(default = "default_schema_version")]
    schema_version: u32,
    #[serde(default = "default_planes")]
    planes: Vec<Plane>,
    categories: Vec<Category>,
    #[serde(default = "default_language")]
    language: Language,
    #[serde(default)]
    theme: Theme,
    #[serde(default = "default_scale")]
    ui_scale: f32,
    #[serde(default = "default_plane_name")]
    active_plane: String,
    #[serde(default)]
    pan_offset: [f32; 2],
    #[serde(default)]
    speech: SpeechSettings,
    #[serde(default)]
    edit_lock: EditLock,
    /// Four digits, or empty when no code has been set.
    #[serde(default)]
    pin: String,

    // --- Volatile ---
    #[serde(skip)]
    voice: std::sync::Arc<crate::speech::Voice>,
    #[serde(skip)]
    undo_stack: Vec<Snapshot>,
    #[serde(skip)]
    confirm: Option<Confirm>,
    /// The board-transfer text area: holds an export, or a paste to import.
    #[serde(skip)]
    transfer_buffer: String,
    #[serde(skip)]
    transfer_status: Option<(bool, String)>,
    /// 0.0 – 1.0 progress of the press-and-hold that unlocks editing.
    #[serde(skip)]
    edit_hold: f32,
    #[serde(skip)]
    pin_prompt_open: bool,
    #[serde(skip)]
    pin_entry: String,
    #[serde(skip)]
    pin_error: bool,
    #[serde(skip)]
    sentence_dock: Vec<DockChip>,
    #[serde(skip)]
    ghost_menu_open: bool,
    #[serde(skip)]
    settings_open: bool,
    #[serde(skip)]
    active_category: Option<usize>,
    /// Kept alive for the duration of the zoom-out animation.
    #[serde(skip)]
    closing_category: Option<usize>,
    /// Screen position the semantic zoom flies in and out of.
    #[serde(skip)]
    zoom_origin: [f32; 2],
    #[serde(skip)]
    editing_category_idx: Option<usize>,
    #[serde(skip)]
    is_edit_mode: bool,
    #[serde(skip)]
    inline_edit_idx: Option<usize>,
    #[serde(skip)]
    hex_buffer: String,
    #[serde(skip)]
    hex_owner: Option<usize>,
    #[serde(skip)]
    applied_theme: Option<Theme>,
}

fn default_schema_version() -> u32 {
    SCHEMA_VERSION
}

fn default_language() -> Language {
    Language::English
}

fn default_scale() -> f32 {
    1.0
}

fn default_plane_name() -> String {
    "Home".to_owned()
}

fn default_planes() -> Vec<Plane> {
    vec![
        Plane {
            name: "Home".to_owned(),
            icon: "🏠".to_owned(),
        },
        Plane {
            name: "Hospital".to_owned(),
            icon: "🏥".to_owned(),
        },
        Plane {
            name: "Social".to_owned(),
            icon: "👋".to_owned(),
        },
    ]
}

impl Default for CommunicatorApp {
    fn default() -> Self {
        let word = |text: &str, icon: &str, col: usize, row: usize| Term {
            text: text.to_owned(),
            icon: icon.to_owned(),
            pos: [48.0 + col as f32 * 216.0, 72.0 + row as f32 * 168.0],
        };
        let categories = vec![
            Category {
                name: "People".to_owned(),
                icon: "👥".to_owned(),
                color: [0x2F, 0x6F, 0xD0],
                pos: [48.0, 72.0],
                plane: "Home".to_owned(),
                terms: vec![
                    word("I", "🙋", 0, 0),
                    word("You", "👉", 1, 0),
                    word("They", "👪", 2, 0),
                ],
            },
            Category {
                name: "Actions".to_owned(),
                icon: "🏃".to_owned(),
                color: [0x2E, 0x9E, 0x5B],
                pos: [264.0, 72.0],
                plane: "Home".to_owned(),
                terms: vec![
                    word("Want", "🙌", 0, 0),
                    word("Go", "➡", 1, 0),
                    word("Stop", "✋", 2, 0),
                ],
            },
            Category {
                name: "Feelings".to_owned(),
                icon: "❤".to_owned(),
                color: [0x7B, 0x54, 0xC8],
                pos: [480.0, 72.0],
                plane: "Home".to_owned(),
                terms: vec![
                    word("Happy", "😀", 0, 0),
                    word("Tired", "😴", 1, 0),
                    word("Sad", "😢", 2, 0),
                ],
            },
            Category {
                name: "Needs".to_owned(),
                icon: "⚠".to_owned(),
                color: [0xE0, 0x9F, 0x11],
                pos: [48.0, 240.0],
                plane: "Home".to_owned(),
                terms: vec![
                    word("Water", "💧", 0, 0),
                    word("Toilet", "🚻", 1, 0),
                    word("Pain", "😖", 2, 0),
                ],
            },
        ];

        Self {
            schema_version: SCHEMA_VERSION,
            planes: default_planes(),
            categories,
            language: default_language(),
            theme: Theme::default(),
            ui_scale: default_scale(),
            active_plane: default_plane_name(),
            pan_offset: [0.0, 0.0],
            speech: SpeechSettings::default(),
            edit_lock: EditLock::default(),
            pin: String::new(),
            voice: std::sync::Arc::default(),
            undo_stack: vec![],
            confirm: None,
            transfer_buffer: String::new(),
            transfer_status: None,
            edit_hold: 0.0,
            pin_prompt_open: false,
            pin_entry: String::new(),
            pin_error: false,
            sentence_dock: vec![],
            ghost_menu_open: false,
            settings_open: false,
            active_category: None,
            closing_category: None,
            zoom_origin: [0.0, 0.0],
            editing_category_idx: None,
            is_edit_mode: false,
            inline_edit_idx: None,
            hex_buffer: String::new(),
            hex_owner: None,
            applied_theme: None,
        }
    }
}

impl CommunicatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: Self = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default();

        // Probing for a speech engine touches the filesystem, so it happens
        // once here rather than on every frame.
        app.voice = std::sync::Arc::new(crate::speech::Voice::new());
        app.schema_version = SCHEMA_VERSION;

        theme::apply(&cc.egui_ctx, app.theme);
        cc.egui_ctx.set_zoom_factor(app.ui_scale);
        app
    }

    fn plane_icon(&self, name: &str) -> &str {
        self.planes
            .iter()
            .find(|p| p.name == name)
            .map_or("", |p| p.icon.as_str())
    }

    /// Leave the currently opened category, playing the zoom-out animation.
    fn close_category(&mut self) {
        if self.active_category.is_some() {
            self.closing_category = self.active_category;
            self.active_category = None;
            self.inline_edit_idx = None;
        }
    }

    // --- Undo ---------------------------------------------------------------

    /// Take a restore point. Call this *before* anything structural.
    fn push_undo(&mut self) {
        self.undo_stack.push(Snapshot {
            planes: self.planes.clone(),
            categories: self.categories.clone(),
            active_plane: self.active_plane.clone(),
        });
        if self.undo_stack.len() > 25 {
            self.undo_stack.remove(0);
        }
    }

    fn undo(&mut self) {
        let Some(snapshot) = self.undo_stack.pop() else {
            return;
        };
        self.planes = snapshot.planes;
        self.categories = snapshot.categories;
        self.active_plane = snapshot.active_plane;
        // Indices in the restored board may not mean what they used to.
        self.active_category = None;
        self.closing_category = None;
        self.inline_edit_idx = None;
        self.editing_category_idx = None;
        self.hex_owner = None;
    }

    // --- Destructive operations ---------------------------------------------

    /// Remove a category and everything in it.
    fn delete_category(&mut self, idx: usize) {
        if idx >= self.categories.len() {
            return;
        }
        self.push_undo();
        self.categories.remove(idx);
        self.editing_category_idx = None;
        self.hex_owner = None;
        self.inline_edit_idx = None;
        if self.active_category == Some(idx) {
            self.active_category = None;
            self.closing_category = None;
        }
    }

    /// Remove a plane, re-homing its categories rather than deleting them.
    fn delete_plane(&mut self, idx: usize) {
        if self.planes.len() < 2 {
            return; // There has to be somewhere to stand.
        }
        let Some(plane) = self.planes.get(idx).cloned() else {
            return;
        };
        self.push_undo();
        let fallback = self
            .planes
            .iter()
            .find(|p| p.name != plane.name)
            .map_or_else(default_plane_name, |p| p.name.clone());
        for category in &mut self.categories {
            if category.plane == plane.name {
                category.plane.clone_from(&fallback);
            }
        }
        if self.active_plane == plane.name {
            self.active_plane = fallback;
            self.close_category();
        }
        self.planes.remove(idx);
    }

    /// How many categories would be re-homed if this plane went away.
    fn categories_on_plane(&self, name: &str) -> usize {
        self.categories.iter().filter(|c| c.plane == name).count()
    }

    // --- Speech -------------------------------------------------------------

    fn lang_tag(&self) -> &'static str {
        match self.language {
            Language::English => "en-US",
            Language::Turkish => "tr-TR",
        }
    }

    fn can_speak(&self) -> bool {
        self.speech.enabled && self.voice.available()
    }

    fn say(&self, text: &str) {
        if self.can_speak() {
            self.voice.speak(text, self.lang_tag(), self.speech.rate);
        }
    }

    /// The dock, read out as one utterance.
    fn sentence(&self) -> String {
        self.sentence_dock
            .iter()
            .map(|chip| chip.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    // --- Board transfer -----------------------------------------------------

    fn board_file(&self) -> BoardFile {
        BoardFile {
            schema_version: SCHEMA_VERSION,
            planes: self.planes.clone(),
            categories: self.categories.clone(),
        }
    }

    fn apply_board(&mut self, board: BoardFile) {
        self.push_undo();
        self.planes = board.planes;
        self.categories = board.categories;
        self.active_plane = self
            .planes
            .first()
            .map_or_else(default_plane_name, |plane| plane.name.clone());
        self.active_category = None;
        self.closing_category = None;
        self.inline_edit_idx = None;
        self.editing_category_idx = None;
        self.hex_owner = None;
        self.pan_offset = [0.0, 0.0];
    }

    // --- Edit lock ----------------------------------------------------------

    /// A PIN that was never set cannot be the lock, so fall back to hold.
    fn effective_lock(&self) -> EditLock {
        if self.edit_lock == EditLock::Pin && self.pin.chars().count() == 4 {
            EditLock::Pin
        } else {
            EditLock::Hold
        }
    }

    fn leave_edit_mode(&mut self) {
        self.is_edit_mode = false;
        self.inline_edit_idx = None;
        self.edit_hold = 0.0;
    }
}

/// Snap a canvas coordinate to the layout grid.
fn snap_to_grid(value: f32) -> f32 {
    (value / theme::GRID).round() * theme::GRID
}

/// Where the camera is allowed to sit on one axis.
///
/// Content smaller than the viewport is centred and pinned; larger content can
/// be panned but never dragged entirely off screen.
fn clamp_pan_axis(value: f32, content_min: f32, content_max: f32, viewport: f32) -> f32 {
    let padding = 160.0;
    let content = content_max - content_min;
    if content + 2.0 * padding <= viewport {
        return (viewport - content) * 0.5 - content_min;
    }
    let a = padding - content_min;
    let b = viewport - padding - content_max;
    value.clamp(a.min(b), a.max(b))
}

// ---------------------------------------------------------------------------
// Small painting helpers
// ---------------------------------------------------------------------------

struct NodeAnim {
    hover: f32,
    press: f32,
    ping: f32,
}

impl NodeAnim {
    const STILL: Self = Self {
        hover: 0.0,
        press: 0.0,
        ping: 0.0,
    };
}

struct NodeView<'a> {
    rect: Rect,
    label: &'a str,
    icon: &'a str,
    fill: Color32,
    /// 0.0 – 1.0, used by the semantic-zoom transition.
    alpha: f32,
    ring: Option<Color32>,
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn scale_alpha(c: Color32, factor: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        c.r(),
        c.g(),
        c.b(),
        (f32::from(c.a()) * factor.clamp(0.0, 1.0)) as u8,
    )
}

/// Read the hover / press / click-ripple animations for a node.
fn node_anim(ctx: &Context, id: Id, response: &Response) -> NodeAnim {
    let hover = ctx.animate_bool_with_time(id.with("hov"), response.hovered(), 0.12);
    let press =
        ctx.animate_bool_with_time(id.with("prs"), response.is_pointer_button_down_on(), 0.07);
    if response.clicked() {
        ctx.animate_value_with_time(id.with("png"), 1.0, 0.0);
    }
    let ping = ctx.animate_value_with_time(id.with("png"), 0.0, 0.45);
    NodeAnim { hover, press, ping }
}

/// The one and only way a category / term tile is drawn.
fn paint_node(painter: &egui::Painter, pal: &Palette, view: &NodeView<'_>, anim: &NodeAnim) {
    let a = view.alpha.clamp(0.0, 1.0);
    if a <= 0.01 {
        return;
    }
    // Gentle press: the tile sinks a little instead of flashing.
    let lift = 1.0 + 0.015 * anim.hover - 0.03 * anim.press;
    let rect = Rect::from_center_size(view.rect.center(), view.rect.size() * lift);
    let radius = CornerRadius::same(theme::RADIUS_NODE);
    let fill = scale_alpha(view.fill, a);
    let fg = scale_alpha(pal.on(view.fill), a);

    // Click ripple — silent confirmation that the tap registered.
    if anim.ping > 0.01 {
        let grow = 26.0 * (1.0 - anim.ping);
        painter.rect_stroke(
            rect.expand(grow),
            CornerRadius::same(theme::RADIUS_NODE + 6),
            Stroke::new(3.0_f32, scale_alpha(view.fill, 0.55 * anim.ping * a)),
            StrokeKind::Outside,
        );
    }

    let mut shadow = pal.elevation(1 + (anim.hover * 1.9) as u8);
    shadow.color = scale_alpha(shadow.color, a);
    painter.add(shadow.as_shape(rect, radius));

    let outline = pal.node_outline(view.fill);
    painter.rect(
        rect,
        radius,
        fill,
        Stroke::new(outline.width, scale_alpha(outline.color, a)),
        StrokeKind::Inside,
    );

    // Hover glow: a soft inner highlight, never a hard border.
    if anim.hover > 0.01 {
        painter.rect_stroke(
            rect.shrink(2.0),
            CornerRadius::same(theme::RADIUS_NODE - 2),
            Stroke::new(
                2.0_f32,
                scale_alpha(pal.on(view.fill), 0.28 * anim.hover * a),
            ),
            StrokeKind::Inside,
        );
    }

    if let Some(ring) = view.ring {
        painter.rect_stroke(
            rect.expand(3.0),
            CornerRadius::same(theme::RADIUS_NODE + 3),
            Stroke::new(3.0_f32, scale_alpha(ring, a)),
            StrokeKind::Outside,
        );
    }

    let has_icon = !view.icon.is_empty();
    if has_icon {
        painter.text(
            rect.center() - vec2(0.0, rect.height() * 0.17),
            Align2::CENTER_CENTER,
            view.icon,
            FontId::proportional(36.0 * lift),
            fg,
        );
    }
    let font = FontId::proportional(if has_icon { 21.0 } else { 26.0 } * lift);
    let galley = painter.layout(view.label.to_owned(), font, fg, rect.width() - 24.0);
    let baseline = if has_icon {
        rect.center().y + rect.height() * 0.21
    } else {
        rect.center().y
    };
    painter.galley(
        pos2(
            rect.center().x - galley.size().x * 0.5,
            baseline - galley.size().y * 0.5,
        ),
        galley,
        fg,
    );
}

/// A pill-shaped button used across the chrome.
fn pill_button(ui: &mut Ui, label: &str, fill: Color32, fg: Color32) -> Response {
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(fg).size(18.0))
            .fill(fill)
            .stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(theme::RADIUS_PILL))
            .min_size(vec2(0.0, 44.0)),
    )
}

/// A non-interactive pill, used for the breadcrumb trail.
fn breadcrumb_chip(ui: &mut Ui, icon: &str, label: &str, fill: Color32, fg: Color32) -> Response {
    let text = if icon.is_empty() {
        label.to_owned()
    } else {
        format!("{icon}  {label}")
    };
    let galley = ui
        .painter()
        .layout_no_wrap(text, FontId::proportional(18.0), fg);
    let (rect, response) = ui.allocate_exact_size(galley.size() + vec2(32.0, 18.0), Sense::click());
    let name = label.to_owned();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &name));
    ui.painter()
        .rect_filled(rect, CornerRadius::same(theme::RADIUS_PILL), fill);
    ui.painter()
        .galley(rect.center() - galley.size() * 0.5, galley, fg);
    response
}

/// Section header used inside the settings and menu cards.
fn section_label(ui: &mut Ui, pal: &Palette, text: &str) {
    ui.label(
        egui::RichText::new(text.to_uppercase())
            .size(14.0)
            .color(pal.ink_muted)
            .strong(),
    );
}

// ---------------------------------------------------------------------------
// eframe::App
// ---------------------------------------------------------------------------

impl eframe::App for CommunicatorApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        let c = self.theme.palette().canvas;
        [
            f32::from(c.r()) / 255.0,
            f32::from(c.g()) / 255.0,
            f32::from(c.b()) / 255.0,
            1.0,
        ]
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if self.applied_theme != Some(self.theme) {
            theme::apply(ctx, self.theme);
            self.applied_theme = Some(self.theme);
        }
        if (ctx.zoom_factor() - self.ui_scale).abs() > 0.005 {
            ctx.set_zoom_factor(self.ui_scale);
        }
        if !self.planes.iter().any(|p| p.name == self.active_plane) {
            self.active_plane = self
                .planes
                .first()
                .map_or_else(default_plane_name, |p| p.name.clone());
        }

        let pal = self.theme.palette();
        let lang = self.language;

        self.top_bar(ctx, &pal, lang);
        self.dock(ctx, &pal, lang);
        self.canvas(ctx, &pal, lang);
        self.ghost_menu(ctx, &pal, lang);
        self.settings_window(ctx, &pal, lang);
        self.pin_prompt(ctx, &pal, lang);
        self.confirm_dialog(ctx, &pal, lang);
    }
}

// ---------------------------------------------------------------------------
// Chrome: top bar, dock, ghost menu
// ---------------------------------------------------------------------------

impl CommunicatorApp {
    fn top_bar(&mut self, ctx: &Context, pal: &Palette, lang: Language) {
        let frame = egui::Frame::new()
            .fill(pal.surface)
            .inner_margin(Margin::symmetric(theme::GUTTER as i8, 12));

        let open_category = self
            .active_category
            .and_then(|idx| self.categories.get(idx))
            .map(|c| (c.name.clone(), c.icon.clone(), theme::to_color(c.color)));

        let plane_icon = self.plane_icon(&self.active_plane).to_owned();
        let plane_name = self.active_plane.clone();

        let response = egui::TopBottomPanel::top("chrome")
            .frame(frame)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // --- Breadcrumb: where am I? ---
                    let plane_chip =
                        breadcrumb_chip(ui, &plane_icon, &plane_name, pal.surface_sunken, pal.ink);
                    if plane_chip.clicked() {
                        self.close_category();
                    }
                    if let Some((name, icon, color)) = &open_category {
                        ui.label(egui::RichText::new("›").size(22.0).color(pal.ink_muted));
                        breadcrumb_chip(ui, icon, name, *color, pal.on(*color));
                    }

                    // --- Controls, pinned right ---
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let menu_fill = if self.ghost_menu_open {
                            pal.accent
                        } else {
                            pal.surface_sunken
                        };
                        let menu_fg = if self.ghost_menu_open {
                            pal.on(pal.accent)
                        } else {
                            pal.ink
                        };
                        if pill_button(ui, &format!("☰  {}", t(lang, "menu")), menu_fill, menu_fg)
                            .clicked()
                        {
                            self.ghost_menu_open = !self.ghost_menu_open;
                        }

                        self.edit_button(ui, pal, lang);

                        // Undo lives next to the thing that causes damage.
                        if self.is_edit_mode && !self.undo_stack.is_empty() {
                            let label = format!("↺  {}", t(lang, "undo_edit"));
                            if pill_button(ui, &label, pal.surface_sunken, pal.ink)
                                .on_hover_text(t(lang, "undo_hint"))
                                .clicked()
                            {
                                self.undo();
                            }
                        }
                    });
                });
            });

        // Hairline instead of a hard border.
        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            Id::new("chrome_hairline"),
        ))
        .hline(
            response.response.rect.x_range(),
            response.response.rect.max.y,
            Stroke::new(1.0_f32, pal.outline),
        );
    }

    /// Edit is a mode the *speaker* must not enter by accident, so turning it
    /// on takes a deliberate gesture while turning it off takes one tap.
    fn edit_button(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        let fill = if self.is_edit_mode {
            pal.accent
        } else {
            pal.surface_sunken
        };
        let fg = if self.is_edit_mode {
            pal.on(pal.accent)
        } else {
            pal.ink
        };
        let label = format!("✏  {}", t(lang, "edit_mode"));
        let response = pill_button(ui, &label, fill, fg);

        if self.is_edit_mode {
            if response.clicked() {
                self.leave_edit_mode();
            }
            return;
        }

        match self.effective_lock() {
            EditLock::Pin => {
                if response.on_hover_text(t(lang, "unlock_pin")).clicked() {
                    self.pin_prompt_open = true;
                    self.pin_entry.clear();
                    self.pin_error = false;
                }
            }
            EditLock::Hold => {
                let held = response.is_pointer_button_down_on();
                let dt = ui.input(|i| i.stable_dt).min(0.1);
                if held {
                    self.edit_hold = (self.edit_hold + dt / EDIT_HOLD_SECONDS).min(1.0);
                    ui.ctx().request_repaint();
                    if self.edit_hold >= 1.0 {
                        self.is_edit_mode = true;
                        self.edit_hold = 0.0;
                    }
                } else {
                    self.edit_hold = (self.edit_hold - dt * 4.0).max(0.0);
                }

                // A filling bar under the label, so the gesture explains itself.
                if self.edit_hold > 0.0 {
                    let rect = response.rect;
                    let bar = Rect::from_min_size(
                        pos2(rect.min.x + 8.0, rect.max.y - 7.0),
                        vec2((rect.width() - 16.0) * self.edit_hold, 4.0),
                    );
                    ui.painter()
                        .rect_filled(bar, CornerRadius::same(2), pal.accent);
                }
                response.on_hover_text(t(lang, "unlock_hold"));
            }
        }
    }

    fn dock(&mut self, ctx: &Context, pal: &Palette, lang: Language) {
        let frame = egui::Frame::new()
            .fill(pal.surface)
            .inner_margin(Margin::symmetric(theme::GUTTER as i8, 16));

        let response = egui::TopBottomPanel::bottom("dock")
            .min_height(150.0)
            .frame(frame)
            .show(ctx, |ui| {
                self.dock_controls(ui, pal, lang);
                ui.add_space(4.0);
                self.dock_chips(ui, pal, lang);
            });

        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            Id::new("dock_hairline"),
        ))
        .hline(
            response.response.rect.x_range(),
            response.response.rect.min.y,
            Stroke::new(1.0_f32, pal.outline),
        );
    }

    /// Speak, stop, undo, clear.
    fn dock_controls(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        ui.horizontal(|ui| {
            section_label(ui, pal, t(lang, "sentence_builder"));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let has_words = !self.sentence_dock.is_empty();

                // Speaking is the point of the dock, so it is the one
                // control that is coloured, and the largest.
                if self.can_speak() {
                    let speak = ui.add_enabled(
                        has_words,
                        egui::Button::new(
                            egui::RichText::new(format!("▶  {}", t(lang, "speak")))
                                .size(20.0)
                                .color(pal.on(pal.accent)),
                        )
                        .fill(pal.accent)
                        .stroke(Stroke::NONE)
                        .min_size(vec2(150.0, 52.0))
                        .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                    );
                    if speak.clicked() {
                        let sentence = self.sentence();
                        self.say(&sentence);
                    }
                    if ui
                        .add_enabled(
                            has_words,
                            egui::Button::new("⏹")
                                .min_size(vec2(52.0, 52.0))
                                .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                        )
                        .on_hover_text(t(lang, "stop_speaking"))
                        .clicked()
                    {
                        self.voice.stop();
                    }
                }

                if ui
                    .add_enabled(
                        has_words,
                        egui::Button::new(t(lang, "clear"))
                            .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                    )
                    .clicked()
                {
                    self.sentence_dock.clear();
                }
                if ui
                    .add_enabled(
                        has_words,
                        egui::Button::new(format!("⏴ {}", t(lang, "undo")))
                            .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                    )
                    .clicked()
                {
                    self.sentence_dock.pop();
                }
            });
        });
    }

    /// The words themselves, or an invitation to add some.
    fn dock_chips(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        if self.sentence_dock.is_empty() {
            let (rect, _) =
                ui.allocate_exact_size(vec2(ui.available_width(), 68.0), Sense::hover());
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(theme::RADIUS_CARD),
                Stroke::new(1.5_f32, theme::fade(pal.ink_muted, 0.35)),
                StrokeKind::Inside,
            );
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                t(lang, "dock_hint"),
                FontId::proportional(19.0),
                pal.ink_muted,
            );
        } else {
            let mut remove = None;
            ui.horizontal_wrapped(|ui| {
                for (idx, chip) in self.sentence_dock.iter().enumerate() {
                    if dock_chip(ui, pal, chip, idx)
                        .on_hover_text(t(lang, "remove"))
                        .clicked()
                    {
                        remove = Some(idx);
                    }
                }
            });
            if let Some(idx) = remove {
                self.sentence_dock.remove(idx);
            }
        }
    }

    fn ghost_menu(&mut self, ctx: &Context, pal: &Palette, lang: Language) {
        if !self.ghost_menu_open {
            return;
        }
        // The menu is a floating card: drag it anywhere that suits the user's
        // reach zone. It only exists while it is explicitly open.
        egui::Window::new("ghost_menu")
            .id(Id::new("ghost_menu"))
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .frame(theme::card_frame(pal))
            .default_pos(pos2(120.0, 140.0))
            .show(ctx, |ui| {
                ui.set_min_width(280.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("☰").size(18.0).color(pal.ink_muted));
                    section_label(ui, pal, t(lang, "menu"));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("×").on_hover_text(t(lang, "close")).clicked() {
                            self.ghost_menu_open = false;
                        }
                    });
                });
                ui.add_space(4.0);
                section_label(ui, pal, t(lang, "planes_of_existence"));

                let mut switch_to = None;
                for plane in &self.planes {
                    let active = plane.name == self.active_plane;
                    let fill = if active {
                        pal.accent
                    } else {
                        pal.surface_sunken
                    };
                    let fg = if active { pal.on(pal.accent) } else { pal.ink };
                    let label = format!("{}  {}", plane.icon, plane.name);
                    if ui
                        .add_sized(
                            [ui.available_width(), 52.0],
                            egui::Button::new(egui::RichText::new(label).size(19.0).color(fg))
                                .fill(fill)
                                .stroke(Stroke::NONE)
                                .corner_radius(CornerRadius::same(theme::RADIUS_CARD)),
                        )
                        .clicked()
                    {
                        switch_to = Some(plane.name.clone());
                    }
                }
                if let Some(name) = switch_to {
                    self.active_plane = name;
                    self.close_category();
                    self.pan_offset = [0.0, 0.0];
                }

                ui.add_space(4.0);
                ui.separator();
                if ui
                    .add_sized(
                        [ui.available_width(), 48.0],
                        egui::Button::new(format!("⚙  {}", t(lang, "settings")))
                            .corner_radius(CornerRadius::same(theme::RADIUS_CARD)),
                    )
                    .clicked()
                {
                    self.settings_open = true;
                    self.ghost_menu_open = false;
                }
            });
    }
}

/// A word waiting in the sentence dock, coloured by its parent category.
fn dock_chip(ui: &mut Ui, pal: &Palette, chip: &DockChip, idx: usize) -> Response {
    let fill = theme::to_color(chip.color);
    let fg = pal.on(fill);
    let text = if chip.icon.is_empty() {
        chip.text.clone()
    } else {
        format!("{}  {}", chip.icon, chip.text)
    };
    let galley = ui
        .painter()
        .layout_no_wrap(text, FontId::proportional(23.0), fg);
    let size = vec2(galley.size().x + 40.0, (galley.size().y + 30.0).max(60.0));
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let label = chip.text.clone();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &label));

    let hover =
        ui.ctx()
            .animate_bool_with_time(Id::new(("dock_chip", idx)), response.hovered(), 0.12);
    let rect = Rect::from_center_size(rect.center(), rect.size() * (1.0 + 0.02 * hover));
    let radius = CornerRadius::same(theme::RADIUS_PILL);

    ui.painter().add(pal.elevation(1).as_shape(rect, radius));
    ui.painter().rect(
        rect,
        radius,
        fill,
        pal.node_outline(fill),
        StrokeKind::Inside,
    );
    ui.painter()
        .galley(rect.center() - galley.size() * 0.5, galley, fg);

    if hover > 0.01 {
        // A removal affordance that only appears when the word is targeted.
        let badge = Rect::from_center_size(rect.right_top() + vec2(-14.0, 14.0), Vec2::splat(22.0));
        ui.painter()
            .circle_filled(badge.center(), 11.0 * hover, scale_alpha(pal.danger, hover));
        ui.painter().text(
            badge.center(),
            Align2::CENTER_CENTER,
            "×",
            FontId::proportional(13.0 * hover),
            scale_alpha(Color32::WHITE, hover),
        );
    }
    response
}

// ---------------------------------------------------------------------------
// Dialogs
// ---------------------------------------------------------------------------

impl CommunicatorApp {
    /// A second, deliberate step in front of anything that destroys work.
    fn confirm_dialog(&mut self, ctx: &Context, pal: &Palette, lang: Language) {
        let Some(pending) = self.confirm.clone() else {
            return;
        };
        let verb = match &pending {
            Confirm::LoadBoard(_) => t(lang, "confirm_replace"),
            _ => t(lang, "confirm_delete"),
        };
        let (title, body) = match &pending {
            Confirm::DeleteCategory(idx) => {
                let Some(category) = self.categories.get(*idx) else {
                    self.confirm = None;
                    return;
                };
                (
                    format!("{} {}", t(lang, "delete_q"), category.name),
                    format!("{} {}", category.terms.len(), t(lang, "words_lost")),
                )
            }
            Confirm::DeletePlane(idx) => {
                let Some(plane) = self.planes.get(*idx) else {
                    self.confirm = None;
                    return;
                };
                let moved = self.categories_on_plane(&plane.name);
                (
                    format!("{} {}", t(lang, "delete_q"), plane.name),
                    format!("{moved} {}", t(lang, "categories_moved")),
                )
            }
            Confirm::LoadBoard(board) => (
                t(lang, "replace_board_q").to_owned(),
                format!(
                    "{} {} · {} {}",
                    board.planes.len(),
                    t(lang, "planes").to_lowercase(),
                    board.categories.len(),
                    t(lang, "categories").to_lowercase()
                ),
            ),
        };

        let mut decision: Option<bool> = None;
        let modal = egui::Modal::new(Id::new("confirm_dialog"))
            .frame(theme::card_frame(pal))
            .show(ctx, |ui| {
                ui.set_max_width(460.0);
                ui.label(egui::RichText::new(title).size(22.0).strong());
                ui.add_space(4.0);
                ui.label(egui::RichText::new(body).color(pal.ink_muted));
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(t(lang, "undo_available"))
                        .size(15.0)
                        .color(pal.ink_muted)
                        .italics(),
                );
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            [150.0, 52.0],
                            egui::Button::new(t(lang, "cancel"))
                                .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                        )
                        .clicked()
                    {
                        decision = Some(false);
                    }
                    if ui
                        .add_sized(
                            [150.0, 52.0],
                            egui::Button::new(egui::RichText::new(verb).color(pal.on(pal.danger)))
                                .fill(pal.danger)
                                .stroke(Stroke::NONE)
                                .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                        )
                        .clicked()
                    {
                        decision = Some(true);
                    }
                });
            });

        if modal.should_close() {
            decision = Some(false);
        }
        match decision {
            None => {}
            Some(false) => self.confirm = None,
            Some(true) => {
                match pending {
                    Confirm::DeleteCategory(idx) => self.delete_category(idx),
                    Confirm::DeletePlane(idx) => self.delete_plane(idx),
                    Confirm::LoadBoard(board) => {
                        self.apply_board(board);
                        self.transfer_status = Some((true, t(lang, "board_loaded").to_owned()));
                    }
                }
                self.confirm = None;
            }
        }
    }

    /// The keypad that unlocks editing when a PIN is set.
    fn pin_prompt(&mut self, ctx: &Context, pal: &Palette, lang: Language) {
        if !self.pin_prompt_open {
            return;
        }
        let mut close = false;
        let modal = egui::Modal::new(Id::new("pin_prompt"))
            .frame(theme::card_frame(pal))
            .show(ctx, |ui| {
                ui.set_max_width(320.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(t(lang, "enter_pin"))
                            .size(20.0)
                            .strong(),
                    );
                    ui.add_space(8.0);

                    // Filled dots, so the code is never shown on a screen that
                    // other people in the room can see.
                    let entered = self.pin_entry.chars().count();
                    let dots = (0..4)
                        .map(|i| if i < entered { "●" } else { "○" })
                        .collect::<Vec<_>>()
                        .join("  ");
                    let colour = if self.pin_error { pal.danger } else { pal.ink };
                    ui.label(egui::RichText::new(dots).size(28.0).color(colour));
                    ui.add_space(12.0);

                    let mut typed: Option<char> = None;
                    let key = |ui: &mut Ui, label: &str| {
                        ui.add_sized(
                            [72.0, 60.0],
                            egui::Button::new(egui::RichText::new(label).size(24.0))
                                .corner_radius(CornerRadius::same(theme::RADIUS_CARD)),
                        )
                        .clicked()
                    };
                    for row in [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']] {
                        ui.horizontal(|ui| {
                            for digit in row {
                                if key(ui, &digit.to_string()) {
                                    typed = Some(digit);
                                }
                            }
                        });
                    }
                    ui.horizontal(|ui| {
                        if key(ui, "⬅") {
                            self.pin_entry.pop();
                            self.pin_error = false;
                        }
                        if key(ui, "0") {
                            typed = Some('0');
                        }
                        if key(ui, "×") {
                            close = true;
                        }
                    });

                    if let Some(digit) = typed {
                        self.pin_error = false;
                        if self.pin_entry.chars().count() < 4 {
                            self.pin_entry.push(digit);
                        }
                    }
                    if self.pin_entry.chars().count() == 4 {
                        if self.pin_entry == self.pin {
                            self.is_edit_mode = true;
                            close = true;
                        } else {
                            self.pin_error = true;
                            self.pin_entry.clear();
                        }
                    }
                    if self.pin_error {
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new(t(lang, "wrong_pin")).color(pal.danger));
                    }
                });
            });

        if close || modal.should_close() {
            self.pin_prompt_open = false;
            self.pin_entry.clear();
        }
    }
}

// ---------------------------------------------------------------------------
// The spatial canvas
// ---------------------------------------------------------------------------

impl CommunicatorApp {
    fn canvas(&mut self, ctx: &Context, pal: &Palette, lang: Language) {
        let frame = egui::Frame::new().fill(pal.canvas);
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let canvas_rect = ui.max_rect();
            let response = ui.interact(
                canvas_rect,
                ui.id().with("canvas_background"),
                Sense::click_and_drag(),
            );

            if response.clicked() {
                self.inline_edit_idx = None;
            }
            if response.dragged() {
                self.pan_offset[0] += response.drag_delta().x;
                self.pan_offset[1] += response.drag_delta().y;
            }

            // How far into the "inside a category" state are we?
            let zoom_t = ease_out_cubic(ctx.animate_bool_with_time(
                Id::new("semantic_zoom"),
                self.active_category.is_some(),
                0.24,
            ));
            let shown = self.active_category.or(self.closing_category);
            if zoom_t <= 0.001 {
                self.closing_category = None;
            }

            let tint = shown
                .and_then(|idx| self.categories.get(idx))
                .map(|c| theme::to_color(c.color));
            theme::paint_canvas_backdrop(
                ui.painter(),
                canvas_rect,
                Vec2::from(self.pan_offset),
                pal,
                tint.filter(|_| zoom_t > 0.5),
            );

            self.clamp_pan(canvas_rect, shown);

            if zoom_t < 0.999 {
                self.draw_plane_nodes(ui, pal, canvas_rect, zoom_t);
            }
            if zoom_t > 0.001
                && let Some(idx) = shown
            {
                self.draw_term_nodes(ui, pal, canvas_rect, idx, zoom_t);
            }

            self.canvas_overlays(ui, pal, lang, canvas_rect, zoom_t, shown.is_some());
        });
    }

    /// Keep the content roughly on screen — freedom to roam, but never lost.
    fn clamp_pan(&mut self, canvas_rect: Rect, shown: Option<usize>) {
        let mut min = pos2(f32::MAX, f32::MAX);
        let mut max = pos2(f32::MIN, f32::MIN);
        let mut count = 0_usize;

        let mut include = |p: [f32; 2]| {
            min.x = min.x.min(p[0]);
            min.y = min.y.min(p[1]);
            max.x = max.x.max(p[0] + theme::NODE_SIZE.x);
            max.y = max.y.max(p[1] + theme::NODE_SIZE.y);
        };

        if let Some(idx) = shown {
            if let Some(category) = self.categories.get(idx) {
                for term in &category.terms {
                    include(term.pos);
                    count += 1;
                }
            }
        } else {
            for category in self
                .categories
                .iter()
                .filter(|c| c.plane == self.active_plane)
            {
                include(category.pos);
                count += 1;
            }
        }

        if count == 0 {
            self.pan_offset = [0.0, 0.0];
            return;
        }

        self.pan_offset[0] = clamp_pan_axis(self.pan_offset[0], min.x, max.x, canvas_rect.width());
        self.pan_offset[1] = clamp_pan_axis(self.pan_offset[1], min.y, max.y, canvas_rect.height());
    }

    /// Root level: the categories that live on the active plane.
    fn draw_plane_nodes(&mut self, ui: &mut Ui, pal: &Palette, canvas_rect: Rect, zoom_t: f32) {
        let interactive = zoom_t <= 0.001;
        let origin = Pos2::from(self.zoom_origin);
        let pan = Vec2::from(self.pan_offset);
        let plane = self.active_plane.clone();
        let edit_mode = self.is_edit_mode;
        let inline_edit = self.inline_edit_idx;

        let mut open_request: Option<(usize, Pos2)> = None;
        let mut edit_request: Option<usize> = None;
        let mut delete_request: Option<usize> = None;

        for (idx, category) in self.categories.iter_mut().enumerate() {
            if category.plane != plane {
                continue;
            }
            let base = Rect::from_min_size(
                canvas_rect.min + Vec2::from(category.pos) + pan,
                theme::NODE_SIZE,
            );
            // Fly the whole plane past the camera as we zoom into one node.
            let spread = 1.0 + 2.4 * zoom_t;
            let rect = Rect::from_center_size(
                origin + (base.center() - origin) * spread,
                base.size() * spread,
            );
            let fill = theme::to_color(category.color);
            let id = Id::new(("category_node", idx));

            let (response, anim) = if interactive {
                let response = ui.interact(rect, id, Sense::click_and_drag());
                let label = format!("{} {}", category.icon, category.name);
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &label)
                });
                let anim = node_anim(ui.ctx(), id, &response);
                (Some(response), anim)
            } else {
                (None, NodeAnim::STILL)
            };

            let editing = edit_mode && inline_edit == Some(idx);
            paint_node(
                ui.painter(),
                pal,
                &NodeView {
                    rect,
                    label: if editing { "" } else { &category.name },
                    icon: &category.icon,
                    fill,
                    alpha: 1.0 - zoom_t,
                    ring: editing.then_some(pal.accent),
                },
                &anim,
            );

            if editing {
                // Rename in place, right where the user is already looking.
                let mut edit_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(Rect::from_center_size(
                            rect.center() + vec2(0.0, rect.height() * 0.22),
                            vec2(rect.width() - 24.0, 40.0),
                        ))
                        .layout(Layout::top_down(Align::Center)),
                );
                edit_ui.add(
                    egui::TextEdit::singleline(&mut category.name)
                        .font(FontId::proportional(20.0))
                        .text_color(pal.on(fill))
                        .horizontal_align(Align::Center)
                        .frame(false),
                );
                if delete_badge(ui, pal, rect, id.with("del")) {
                    delete_request = Some(idx);
                }
            }

            let Some(response) = response else { continue };
            if response.dragged() {
                category.pos[0] += response.drag_delta().x;
                category.pos[1] += response.drag_delta().y;
            }
            if response.drag_stopped() {
                // Grid snapping keeps the board tidy no matter how shaky the drag was.
                category.pos[0] = snap_to_grid(category.pos[0]);
                category.pos[1] = snap_to_grid(category.pos[1]);
            }
            if response.clicked() {
                if edit_mode {
                    edit_request = Some(idx);
                } else {
                    open_request = Some((idx, rect.center()));
                }
            }
        }

        if let Some(idx) = delete_request {
            self.confirm = Some(Confirm::DeleteCategory(idx));
        }
        if let Some(idx) = edit_request {
            self.inline_edit_idx = Some(idx);
        }
        if let Some((idx, center)) = open_request {
            self.active_category = Some(idx);
            self.closing_category = None;
            self.zoom_origin = [center.x, center.y];
            self.inline_edit_idx = None;
            self.pan_offset = [0.0, 0.0];
        }
    }

    /// Inside a category: its words, expanded out of the parent tile.
    fn draw_term_nodes(
        &mut self,
        ui: &mut Ui,
        pal: &Palette,
        canvas_rect: Rect,
        cat_idx: usize,
        zoom_t: f32,
    ) {
        let interactive = zoom_t >= 0.999 && self.active_category == Some(cat_idx);
        let origin = Pos2::from(self.zoom_origin);
        let pan = Vec2::from(self.pan_offset);
        let edit_mode = self.is_edit_mode;
        let inline_edit = self.inline_edit_idx;

        let Some(category) = self.categories.get_mut(cat_idx) else {
            return;
        };
        let fill = theme::to_color(category.color);

        let mut picked: Option<DockChip> = None;
        let mut edit_request: Option<usize> = None;
        let mut delete_request: Option<usize> = None;

        for (idx, term) in category.terms.iter_mut().enumerate() {
            let base = Rect::from_min_size(
                canvas_rect.min + Vec2::from(term.pos) + pan,
                theme::NODE_SIZE,
            );
            // Children grow out of the parent's position: the mental map survives.
            let scale = 0.35 + 0.65 * zoom_t;
            let rect = Rect::from_center_size(
                origin + (base.center() - origin) * zoom_t,
                base.size() * scale,
            );
            let id = Id::new(("term_node", cat_idx, idx));

            let (response, anim) = if interactive {
                let response = ui.interact(rect, id, Sense::click_and_drag());
                let label = format!("{} {}", term.icon, term.text);
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &label)
                });
                let anim = node_anim(ui.ctx(), id, &response);
                (Some(response), anim)
            } else {
                (None, NodeAnim::STILL)
            };

            let editing = edit_mode && inline_edit == Some(idx);
            paint_node(
                ui.painter(),
                pal,
                &NodeView {
                    rect,
                    label: if editing { "" } else { &term.text },
                    icon: &term.icon,
                    fill,
                    alpha: zoom_t,
                    ring: editing.then_some(pal.accent),
                },
                &anim,
            );

            if editing {
                let mut edit_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(Rect::from_center_size(
                            rect.center() + vec2(0.0, rect.height() * 0.22),
                            vec2(rect.width() - 24.0, 40.0),
                        ))
                        .layout(Layout::top_down(Align::Center)),
                );
                edit_ui.add(
                    egui::TextEdit::singleline(&mut term.text)
                        .font(FontId::proportional(20.0))
                        .text_color(pal.on(fill))
                        .horizontal_align(Align::Center)
                        .frame(false),
                );
                if delete_badge(ui, pal, rect, id.with("del")) {
                    delete_request = Some(idx);
                }
            }

            let Some(response) = response else { continue };
            if response.dragged() {
                term.pos[0] += response.drag_delta().x;
                term.pos[1] += response.drag_delta().y;
            }
            if response.drag_stopped() {
                term.pos[0] = snap_to_grid(term.pos[0]);
                term.pos[1] = snap_to_grid(term.pos[1]);
            }
            if response.clicked() {
                if edit_mode {
                    edit_request = Some(idx);
                } else {
                    picked = Some(DockChip {
                        text: term.text.clone(),
                        icon: term.icon.clone(),
                        color: category.color,
                    });
                }
            }
        }

        self.apply_term_requests(cat_idx, delete_request, edit_request, picked);
    }

    /// Apply what the term loop asked for, once its borrows have ended.
    fn apply_term_requests(
        &mut self,
        cat_idx: usize,
        delete_request: Option<usize>,
        edit_request: Option<usize>,
        picked: Option<DockChip>,
    ) {
        if let Some(idx) = delete_request {
            // One word is small enough to delete outright — undo covers it.
            self.push_undo();
            if let Some(category) = self.categories.get_mut(cat_idx) {
                category.terms.remove(idx);
                self.inline_edit_idx = None;
            } else {
                self.undo_stack.pop();
            }
        }
        if let Some(idx) = edit_request {
            self.inline_edit_idx = Some(idx);
        }
        if let Some(chip) = picked {
            if self.speech.speak_on_tap {
                self.say(&chip.text);
            }
            self.sentence_dock.push(chip);
        }
    }

    /// Back button, add button, edit-mode banner and empty states.
    fn canvas_overlays(
        &mut self,
        ui: &mut Ui,
        pal: &Palette,
        lang: Language,
        canvas_rect: Rect,
        zoom_t: f32,
        inside_category: bool,
    ) {
        self.edit_mode_frame(ui, pal, lang, canvas_rect);
        self.back_button(ui, pal, lang, canvas_rect, zoom_t);
        self.empty_state(ui, pal, lang, canvas_rect, zoom_t, inside_category);
        self.add_node_button(ui, pal, lang, canvas_rect, inside_category);
    }

    /// Edit mode is a *mode*, so it gets an unmistakable frame.
    fn edit_mode_frame(&self, ui: &Ui, pal: &Palette, lang: Language, canvas_rect: Rect) {
        if self.is_edit_mode {
            ui.painter().rect_stroke(
                canvas_rect.shrink(3.0),
                CornerRadius::same(theme::RADIUS_CARD),
                Stroke::new(2.0_f32, theme::fade(pal.accent, 0.55)),
                StrokeKind::Inside,
            );
            ui.painter().text(
                pos2(canvas_rect.center().x, canvas_rect.max.y - 18.0),
                Align2::CENTER_CENTER,
                t(lang, "edit_banner"),
                FontId::proportional(15.0),
                theme::fade(pal.accent, 0.9),
            );
        }
    }

    /// Back: always in the same spot, so it can be found without looking.
    fn back_button(
        &mut self,
        ui: &mut Ui,
        pal: &Palette,
        lang: Language,
        canvas_rect: Rect,
        zoom_t: f32,
    ) {
        if zoom_t > 0.01 {
            let rect = Rect::from_min_size(canvas_rect.min + vec2(24.0, 24.0), vec2(132.0, 56.0));
            let mut back_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .layout(Layout::centered_and_justified(egui::Direction::TopDown)),
            );
            let button = egui::Button::new(
                egui::RichText::new(t(lang, "back"))
                    .size(20.0)
                    .color(pal.ink),
            )
            .fill(pal.surface)
            .stroke(Stroke::new(1.0_f32, pal.outline))
            .corner_radius(CornerRadius::same(theme::RADIUS_PILL));
            ui.painter().add(
                pal.elevation(2)
                    .as_shape(rect, CornerRadius::same(theme::RADIUS_PILL)),
            );
            if back_ui.add(button).clicked() {
                self.close_category();
            }
        }
    }

    /// Empty states tell the user what to do next instead of showing a void.
    fn empty_state(
        &self,
        ui: &Ui,
        pal: &Palette,
        lang: Language,
        canvas_rect: Rect,
        zoom_t: f32,
        inside_category: bool,
    ) {
        let settled = if inside_category {
            zoom_t > 0.99
        } else {
            zoom_t < 0.01
        };
        let empty = if let Some(idx) = self.active_category {
            self.categories.get(idx).is_some_and(|c| c.terms.is_empty())
        } else {
            !self.categories.iter().any(|c| c.plane == self.active_plane)
        };
        if empty && settled {
            let center = canvas_rect.center();
            ui.painter().text(
                center - vec2(0.0, 16.0),
                Align2::CENTER_CENTER,
                t(lang, "empty_plane"),
                FontId::proportional(24.0),
                pal.ink_muted,
            );
            ui.painter().text(
                center + vec2(0.0, 20.0),
                Align2::CENTER_CENTER,
                t(lang, "empty_plane_hint"),
                FontId::proportional(18.0),
                theme::fade(pal.ink_muted, 0.8),
            );
        }
    }

    /// Add-node button, bottom right, only while editing.
    fn add_node_button(
        &mut self,
        ui: &mut Ui,
        pal: &Palette,
        lang: Language,
        canvas_rect: Rect,
        inside_category: bool,
    ) {
        if self.is_edit_mode {
            let rect = Rect::from_min_size(
                pos2(canvas_rect.max.x - 200.0, canvas_rect.max.y - 84.0),
                vec2(176.0, 56.0),
            );
            let mut fab_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .layout(Layout::centered_and_justified(egui::Direction::TopDown)),
            );
            let label = if inside_category {
                t(lang, "add_word")
            } else {
                t(lang, "add_category")
            };
            ui.painter().add(
                pal.elevation(3)
                    .as_shape(rect, CornerRadius::same(theme::RADIUS_PILL)),
            );
            let button = egui::Button::new(
                egui::RichText::new(label)
                    .size(20.0)
                    .color(pal.on(pal.accent)),
            )
            .fill(pal.accent)
            .stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(theme::RADIUS_PILL));

            if fab_ui.add(button).clicked() {
                let drop = [
                    -self.pan_offset[0] + canvas_rect.width() * 0.5 - theme::NODE_SIZE.x * 0.5,
                    -self.pan_offset[1] + canvas_rect.height() * 0.5 - theme::NODE_SIZE.y * 0.5,
                ];
                let drop = [snap_to_grid(drop[0]), snap_to_grid(drop[1])];
                self.push_undo();
                if let Some(cat_idx) = self.active_category {
                    if let Some(category) = self.categories.get_mut(cat_idx) {
                        category.terms.push(Term {
                            text: t(lang, "new_word").to_owned(),
                            icon: String::new(),
                            pos: drop,
                        });
                        self.inline_edit_idx = Some(category.terms.len() - 1);
                    }
                } else {
                    self.categories.push(Category {
                        name: t(lang, "new_category").to_owned(),
                        icon: String::new(),
                        color: [0x50, 0x5E, 0x72],
                        pos: drop,
                        terms: vec![],
                        plane: self.active_plane.clone(),
                    });
                    self.inline_edit_idx = Some(self.categories.len() - 1);
                }
            }
        }
    }
}

/// The little red ✕ shown on a node while it is being edited.
fn delete_badge(ui: &Ui, pal: &Palette, node_rect: Rect, id: Id) -> bool {
    let rect = Rect::from_center_size(node_rect.right_top() + vec2(-4.0, 4.0), Vec2::splat(34.0));
    let response = ui.interact(rect, id, Sense::click());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Delete"));
    let hover = ui
        .ctx()
        .animate_bool_with_time(id.with("h"), response.hovered(), 0.1);
    ui.painter()
        .circle_filled(rect.center(), 15.0 + 2.0 * hover, pal.danger);
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        "×",
        FontId::proportional(17.0),
        Color32::WHITE,
    );
    response.clicked()
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

impl CommunicatorApp {
    fn settings_window(&mut self, ctx: &Context, pal: &Palette, lang: Language) {
        if !self.settings_open {
            return;
        }
        let mut open = true;
        let mut close_requested = false;
        let screen = ctx.content_rect();
        let width = (screen.width() - 120.0).clamp(420.0, 880.0);
        let height = (screen.height() - 140.0).clamp(320.0, 720.0);
        egui::Window::new(egui::RichText::new(t(lang, "settings")).size(22.0).strong())
            .open(&mut open)
            .frame(theme::card_frame(pal))
            .default_size(vec2(width, height))
            .max_size(vec2(width, height))
            .default_pos(screen.center() - vec2(width, height) * 0.5)
            .constrain(true)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("settings_scroll")
                    .auto_shrink([false, false])
                    .max_height(height - 150.0)
                    .show(ui, |ui| {
                        self.appearance_section(ui, pal, lang);
                        ui.add_space(8.0);
                        self.speech_section(ui, pal, lang);
                        ui.add_space(8.0);
                        self.safety_section(ui, pal, lang);
                        ui.add_space(8.0);
                        self.board_data_section(ui, pal, lang);
                        ui.add_space(8.0);
                        self.planes_section(ui, pal, lang);
                        ui.add_space(8.0);
                        self.vocabulary_section(ui, pal, lang);
                    });
                ui.add_space(8.0);
                // A full-width target to leave: the title bar "×" is far too
                // small for someone steering with a headstick or a single finger.
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add_sized(
                            [180.0, 52.0],
                            egui::Button::new(
                                egui::RichText::new(t(lang, "done"))
                                    .size(19.0)
                                    .color(pal.on(pal.accent)),
                            )
                            .fill(pal.accent)
                            .stroke(Stroke::NONE)
                            .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                        )
                        .clicked()
                    {
                        close_requested = true;
                    }
                });
            });
        self.settings_open = open && !close_requested;
    }

    fn appearance_section(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        card(ui, pal, |ui| {
            section_label(ui, pal, t(lang, "appearance"));
            ui.horizontal_wrapped(|ui| {
                ui.label(t(lang, "theme"));
                for theme_option in Theme::ALL {
                    let label = match theme_option {
                        Theme::Light => t(lang, "light"),
                        Theme::Dark => t(lang, "dark"),
                        Theme::HighContrast => t(lang, "high_contrast"),
                    };
                    let selected = self.theme == theme_option;
                    if ui
                        .add(
                            egui::Button::selectable(selected, label)
                                .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                        )
                        .clicked()
                    {
                        self.theme = theme_option;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label(t(lang, "interface_size"));
                ui.add(
                    egui::Slider::new(&mut self.ui_scale, 0.8..=1.8)
                        .step_by(0.05)
                        .show_value(false),
                );
                ui.label(format!("{:.0}%", self.ui_scale * 100.0));
            });
            ui.horizontal(|ui| {
                ui.label(t(lang, "language"));
                ui.radio_value(&mut self.language, Language::English, "English");
                ui.radio_value(&mut self.language, Language::Turkish, "Türkçe");
            });
        });
    }

    fn speech_section(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        card(ui, pal, |ui| {
            section_label(ui, pal, t(lang, "speech"));
            if self.voice.available() {
                ui.checkbox(&mut self.speech.enabled, t(lang, "speech_enabled"));
                ui.add_enabled_ui(self.speech.enabled, |ui| {
                    ui.checkbox(&mut self.speech.speak_on_tap, t(lang, "speak_on_tap"));
                    ui.horizontal(|ui| {
                        ui.label(t(lang, "speech_rate"));
                        ui.add(
                            egui::Slider::new(&mut self.speech.rate, 0.6..=1.6)
                                .step_by(0.05)
                                .show_value(false),
                        );
                        ui.label(format!("{:.0}%", self.speech.rate * 100.0));
                        if ui.button(format!("▶ {}", t(lang, "test"))).clicked() {
                            self.say(t(lang, "speech_sample"));
                        }
                    });
                });
            } else {
                // Better to say so plainly than to show a button that does nothing.
                ui.label(
                    egui::RichText::new(t(lang, "no_voice"))
                        .color(pal.ink_muted)
                        .italics(),
                );
            }
        });
    }

    fn safety_section(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        card(ui, pal, |ui| {
            section_label(ui, pal, t(lang, "safety"));
            ui.horizontal_wrapped(|ui| {
                ui.label(t(lang, "lock_editing"));
                for (option, label) in [
                    (EditLock::Hold, t(lang, "lock_hold")),
                    (EditLock::Pin, t(lang, "lock_pin")),
                ] {
                    if ui
                        .add(
                            egui::Button::selectable(self.edit_lock == option, label)
                                .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
                        )
                        .clicked()
                    {
                        self.edit_lock = option;
                    }
                }
            });
            if self.edit_lock == EditLock::Pin {
                ui.horizontal(|ui| {
                    ui.label(t(lang, "pin_code"));
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.pin)
                                .desired_width(90.0)
                                .char_limit(4)
                                .hint_text("1234"),
                        )
                        .changed()
                    {
                        self.pin.retain(|c| c.is_ascii_digit());
                    }
                    if self.pin.chars().count() != 4 {
                        ui.label(
                            egui::RichText::new(t(lang, "pin_incomplete"))
                                .color(pal.ink_muted)
                                .italics(),
                        );
                    }
                });
            }
            ui.label(
                egui::RichText::new(match self.effective_lock() {
                    EditLock::Hold => t(lang, "lock_hold_hint"),
                    EditLock::Pin => t(lang, "lock_pin_hint"),
                })
                .size(15.0)
                .color(pal.ink_muted),
            );
        });
    }

    /// Backup and restore. A board that only lives in browser storage is one
    /// cleared cache away from erasing hours of a carer's work.
    fn board_data_section(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        card(ui, pal, |ui| {
            section_label(ui, pal, t(lang, "board_data"));
            ui.label(
                egui::RichText::new(t(lang, "board_data_hint"))
                    .size(15.0)
                    .color(pal.ink_muted),
            );
            ui.horizontal(|ui| {
                if ui.button(format!("📋 {}", t(lang, "copy_board"))).clicked() {
                    match serde_json::to_string_pretty(&self.board_file()) {
                        Ok(json) => {
                            ui.ctx().copy_text(json.clone());
                            self.transfer_buffer = json;
                            self.transfer_status = Some((true, t(lang, "board_copied").to_owned()));
                        }
                        Err(err) => {
                            self.transfer_status = Some((false, err.to_string()));
                        }
                    }
                }
                if ui.button(format!("📥 {}", t(lang, "load_board"))).clicked() {
                    match serde_json::from_str::<BoardFile>(&self.transfer_buffer) {
                        Ok(board) if board.schema_version > SCHEMA_VERSION => {
                            self.transfer_status =
                                Some((false, t(lang, "board_too_new").to_owned()));
                        }
                        Ok(board) if board.planes.is_empty() => {
                            self.transfer_status =
                                Some((false, t(lang, "board_no_planes").to_owned()));
                        }
                        Ok(board) => {
                            self.transfer_status = None;
                            self.confirm = Some(Confirm::LoadBoard(board));
                        }
                        Err(err) => {
                            self.transfer_status =
                                Some((false, format!("{}: {err}", t(lang, "board_invalid"))));
                        }
                    }
                }
            });
            // A whole board is long: keep it in a fixed window so the rest of
            // settings does not jump around when it is filled.
            egui::ScrollArea::vertical()
                .id_salt("board_transfer")
                .max_height(120.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.transfer_buffer)
                            .desired_rows(5)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                            .hint_text(t(lang, "board_paste_hint")),
                    );
                });
            if let Some((ok, message)) = &self.transfer_status {
                let colour = if *ok { pal.accent } else { pal.danger };
                ui.label(egui::RichText::new(message).color(colour));
            }
        });
    }

    fn planes_section(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        card(ui, pal, |ui| {
            section_label(ui, pal, t(lang, "planes"));
            let mut remove = None;
            let can_remove = self.planes.len() > 1;
            for (idx, plane) in self.planes.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut plane.icon)
                            .desired_width(56.0)
                            .char_limit(8)
                            .hint_text("🏠"),
                    );
                    ui.add(egui::TextEdit::singleline(&mut plane.name).desired_width(220.0));
                    if ui.add_enabled(can_remove, egui::Button::new("×")).clicked() {
                        remove = Some(idx);
                    }
                });
            }
            if let Some(idx) = remove {
                self.confirm = Some(Confirm::DeletePlane(idx));
            }
            if ui.button(t(lang, "add_plane")).clicked() {
                self.push_undo();
                self.planes.push(Plane {
                    name: format!("Plane {}", self.planes.len() + 1),
                    icon: "★".to_owned(),
                });
            }
        });
    }

    /// The master list of categories inside settings.
    fn category_list(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        section_label(ui, pal, t(lang, "categories"));
        egui::ScrollArea::vertical()
            .id_salt("category_list")
            .show(ui, |ui| {
                for (idx, category) in self.categories.iter().enumerate() {
                    let selected = self.editing_category_idx == Some(idx);
                    let label =
                        format!("{} {}  ·  {}", category.icon, category.name, category.plane);
                    ui.horizontal(|ui| {
                        // The colour is the fastest way to recognise a category,
                        // so it leads the row.
                        let (rect, _) = ui.allocate_exact_size(vec2(10.0, 36.0), Sense::hover());
                        ui.painter().rect_filled(
                            rect,
                            CornerRadius::same(5),
                            theme::to_color(category.color),
                        );
                        let button = egui::Button::selectable(
                            selected,
                            egui::RichText::new(label).size(18.0),
                        )
                        .corner_radius(CornerRadius::same(12));
                        if ui.add_sized([ui.available_width(), 44.0], button).clicked() {
                            self.editing_category_idx = Some(idx);
                        }
                    });
                }
                if ui
                    .add_sized(
                        [ui.available_width(), 44.0],
                        egui::Button::new(t(lang, "add_new_category")),
                    )
                    .clicked()
                {
                    self.push_undo();
                    self.categories.push(Category {
                        name: t(lang, "new_category").to_owned(),
                        icon: String::new(),
                        color: [0x50, 0x5E, 0x72],
                        pos: [48.0, 72.0],
                        terms: vec![],
                        plane: self.active_plane.clone(),
                    });
                    self.editing_category_idx = Some(self.categories.len() - 1);
                }
            });
    }

    fn vocabulary_section(&mut self, ui: &mut Ui, pal: &Palette, lang: Language) {
        card(ui, pal, |ui| {
            ui.horizontal_top(|ui| {
                let list_width = (ui.available_width() * 0.36).max(200.0);

                ui.allocate_ui_with_layout(
                    vec2(list_width, 440.0),
                    Layout::top_down(Align::Min),
                    |ui| self.category_list(ui, pal, lang),
                );

                ui.separator();

                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), 440.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        section_label(ui, pal, t(lang, "editor"));
                        let Some(idx) = self.editing_category_idx else {
                            ui.label(egui::RichText::new(t(lang, "select_category")).italics());
                            return;
                        };
                        if self.hex_owner != Some(idx)
                            && let Some(category) = self.categories.get(idx)
                        {
                            self.hex_buffer = theme::to_hex(category.color);
                            self.hex_owner = Some(idx);
                        }

                        // Split the borrows so the editor can touch several fields.
                        let Self {
                            categories,
                            planes,
                            hex_buffer,
                            ..
                        } = self;
                        let Some(category) = categories.get_mut(idx) else {
                            return;
                        };
                        let delete = egui::ScrollArea::vertical()
                            .id_salt("category_editor")
                            .show(ui, |ui| {
                                category_editor(ui, pal, lang, category, planes, hex_buffer)
                            })
                            .inner;

                        if delete {
                            self.confirm = Some(Confirm::DeleteCategory(idx));
                        }
                    },
                );
            });
        });
    }
}

/// A sunken panel that groups related settings.
fn card(ui: &mut Ui, pal: &Palette, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(pal.surface_sunken)
        .stroke(Stroke::new(1.0_f32, pal.outline))
        .corner_radius(CornerRadius::same(theme::RADIUS_CARD))
        .inner_margin(Margin::same(18))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui);
        });
}

/// Returns `true` when the user asked to delete this category.
fn category_editor(
    ui: &mut Ui,
    pal: &Palette,
    lang: Language,
    category: &mut Category,
    planes: &[Plane],
    hex_buffer: &mut String,
) -> bool {
    identity_row(ui, lang, category, planes);
    ui.add_space(4.0);
    ui.label(t(lang, "category_color"));
    colour_picker(ui, pal, lang, category, hex_buffer);
    ui.add_space(8.0);
    section_label(ui, pal, t(lang, "vocab_terms"));
    term_rows(ui, lang, category);

    ui.add_space(16.0);
    ui.separator();
    ui.add(
        egui::Button::new(
            egui::RichText::new(t(lang, "delete_category")).color(pal.on(pal.danger)),
        )
        .fill(pal.danger)
        .corner_radius(CornerRadius::same(theme::RADIUS_PILL)),
    )
    .clicked()
}

/// Icon, name and which plane this category lives on.
fn identity_row(ui: &mut Ui, lang: Language, category: &mut Category, planes: &[Plane]) {
    ui.horizontal(|ui| {
        ui.label(t(lang, "icon"));
        ui.add(
            egui::TextEdit::singleline(&mut category.icon)
                .desired_width(56.0)
                .char_limit(8)
                .hint_text("😊"),
        );
        ui.label(t(lang, "category_name"));
        ui.add(egui::TextEdit::singleline(&mut category.name).desired_width(220.0));
    });

    ui.horizontal(|ui| {
        ui.label(t(lang, "belongs_to"));
        egui::ComboBox::from_id_salt("plane_picker")
            .selected_text(category.plane.clone())
            .show_ui(ui, |ui| {
                for plane in planes {
                    let label = format!("{} {}", plane.icon, plane.name);
                    if ui
                        .selectable_label(category.plane == plane.name, label)
                        .clicked()
                    {
                        category.plane.clone_from(&plane.name);
                    }
                }
            });
    });
}

/// Preset swatches plus a hex field, so carers can match an existing board.
fn colour_picker(
    ui: &mut Ui,
    pal: &Palette,
    lang: Language,
    category: &mut Category,
    hex_buffer: &mut String,
) {
    ui.horizontal_wrapped(|ui| {
        for (name, rgb) in theme::CATEGORY_PRESETS {
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(40.0), Sense::click());
            let color = theme::to_color(rgb);
            ui.painter()
                .rect_filled(rect, CornerRadius::same(12), color);
            if category.color == rgb {
                ui.painter().rect_stroke(
                    rect.expand(3.0),
                    CornerRadius::same(14),
                    Stroke::new(3.0_f32, pal.accent),
                    StrokeKind::Outside,
                );
            }
            if response.on_hover_text(name).clicked() {
                category.color = rgb;
                *hex_buffer = theme::to_hex(rgb);
            }
        }
    });
    ui.horizontal(|ui| {
        ui.label(t(lang, "custom"));
        if ui
            .add(egui::TextEdit::singleline(hex_buffer).desired_width(110.0))
            .changed()
            && let Some(rgb) = theme::from_hex(hex_buffer)
        {
            category.color = rgb;
        }
        let mut rgb = [
            f32::from(category.color[0]) / 255.0,
            f32::from(category.color[1]) / 255.0,
            f32::from(category.color[2]) / 255.0,
        ];
        if ui.color_edit_button_rgb(&mut rgb).changed() {
            category.color = [
                (rgb[0] * 255.0) as u8,
                (rgb[1] * 255.0) as u8,
                (rgb[2] * 255.0) as u8,
            ];
            *hex_buffer = theme::to_hex(category.color);
        }
    });
}

/// The word list of one category.
fn term_rows(ui: &mut Ui, lang: Language, category: &mut Category) {
    let mut remove_term = None;
    for (term_idx, term) in category.terms.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut term.icon)
                    .desired_width(56.0)
                    .char_limit(8)
                    .hint_text("😊"),
            );
            ui.add(egui::TextEdit::singleline(&mut term.text).desired_width(220.0));
            if ui.button("×").clicked() {
                remove_term = Some(term_idx);
            }
        });
    }
    if let Some(idx) = remove_term {
        category.terms.remove(idx);
    }
    if ui.button(t(lang, "add_term")).clicked() {
        let row = category.terms.len();
        category.terms.push(Term {
            text: t(lang, "new_word").to_owned(),
            icon: String::new(),
            pos: [
                48.0 + (row % 4) as f32 * 216.0,
                72.0 + (row / 4) as f32 * 168.0,
            ],
        });
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapping_lands_on_the_grid() {
        assert!((snap_to_grid(0.0) - 0.0).abs() < f32::EPSILON);
        assert!(
            (snap_to_grid(11.0) - 0.0).abs() < f32::EPSILON,
            "rounds down"
        );
        assert!(
            (snap_to_grid(13.0) - theme::GRID).abs() < f32::EPSILON,
            "rounds up"
        );
        assert!((snap_to_grid(-13.0) + theme::GRID).abs() < f32::EPSILON);
        // Already-snapped positions must not drift when dragged and released.
        let snapped = snap_to_grid(5.0 * theme::GRID);
        assert!((snap_to_grid(snapped) - snapped).abs() < f32::EPSILON);
    }

    #[test]
    fn content_smaller_than_the_viewport_is_centred() {
        // 200pt of content in a 1000pt viewport: equal margins either side.
        let pan = clamp_pan_axis(9999.0, 0.0, 200.0, 1000.0);
        assert!((pan - 400.0).abs() < 0.01, "got {pan}");
        // And the camera is pinned there whatever the user tries.
        assert!((clamp_pan_axis(-9999.0, 0.0, 200.0, 1000.0) - 400.0).abs() < 0.01);
    }

    #[test]
    fn content_larger_than_the_viewport_can_be_panned_but_not_lost() {
        // 3000pt of content in a 1000pt viewport, 160pt of slack each way.
        let padding = 160.0;
        let far_left = clamp_pan_axis(-9999.0, 0.0, 3000.0, 1000.0);
        let far_right = clamp_pan_axis(9999.0, 0.0, 3000.0, 1000.0);
        assert!((far_right - padding).abs() < 0.01, "got {far_right}");
        assert!(
            (far_left - (1000.0 - padding - 3000.0)).abs() < 0.01,
            "got {far_left}"
        );
        // Something is always on screen at both extremes.
        assert!(far_left + 3000.0 > 0.0);
        assert!(far_right < 1000.0);
    }

    #[test]
    fn zoom_easing_is_bounded() {
        assert!((ease_out_cubic(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((ease_out_cubic(1.0) - 1.0).abs() < f32::EPSILON);
        assert!(ease_out_cubic(0.5) > 0.5, "should ease out, not in");
    }

    #[test]
    fn a_board_survives_export_and_import() {
        let app = CommunicatorApp::default();
        let json = serde_json::to_string(&app.board_file()).expect("serialise");

        let mut restored = CommunicatorApp::default();
        restored.planes.clear();
        restored.categories.clear();
        restored.apply_board(serde_json::from_str(&json).expect("deserialise"));

        assert_eq!(restored.planes.len(), app.planes.len());
        assert_eq!(restored.categories.len(), app.categories.len());
        assert_eq!(
            restored.categories.first().map(|c| c.terms.len()),
            app.categories.first().map(|c| c.terms.len())
        );
        assert_eq!(restored.active_plane, app.active_plane);
    }

    #[test]
    fn rubbish_is_not_mistaken_for_a_board() {
        assert!(serde_json::from_str::<BoardFile>("not json at all").is_err());
        assert!(serde_json::from_str::<BoardFile>(r#"{"hello": 1}"#).is_err());
    }

    #[test]
    fn undo_restores_a_deleted_category() {
        let mut app = CommunicatorApp::default();
        let before = app.categories.len();
        let name = app.categories.first().map(|c| c.name.clone());

        app.delete_category(0);
        assert_eq!(app.categories.len(), before - 1);

        app.undo();
        assert_eq!(app.categories.len(), before);
        assert_eq!(app.categories.first().map(|c| c.name.clone()), name);
    }

    #[test]
    fn deleting_a_plane_rehomes_its_categories_instead_of_dropping_them() {
        let mut app = CommunicatorApp::default();
        let categories = app.categories.len();
        let home = app.planes.first().map(|p| p.name.clone()).expect("a plane");
        assert!(app.categories_on_plane(&home) > 0);

        app.delete_plane(0);
        assert_eq!(app.categories.len(), categories, "no words may be lost");
        assert!(app.categories.iter().all(|c| c.plane != home));
        assert!(app.planes.iter().all(|p| p.name != home));
    }

    #[test]
    fn the_last_plane_cannot_be_deleted() {
        let mut app = CommunicatorApp::default();
        app.planes.truncate(1);
        app.delete_plane(0);
        assert_eq!(app.planes.len(), 1, "there must be somewhere to stand");
    }

    #[test]
    fn a_pin_only_locks_when_it_is_complete() {
        let mut app = CommunicatorApp {
            edit_lock: EditLock::Pin,
            ..Default::default()
        };
        assert_eq!(app.effective_lock(), EditLock::Hold, "no code set yet");
        app.pin = "12".to_owned();
        assert_eq!(app.effective_lock(), EditLock::Hold, "half a code");
        app.pin = "1234".to_owned();
        assert_eq!(app.effective_lock(), EditLock::Pin);
    }

    #[test]
    fn the_sentence_reads_as_words_in_order() {
        let mut app = CommunicatorApp::default();
        for word in ["I", "want", "water"] {
            app.sentence_dock.push(DockChip {
                text: word.to_owned(),
                icon: String::new(),
                color: [0, 0, 0],
            });
        }
        assert_eq!(app.sentence(), "I want water");
    }
}
