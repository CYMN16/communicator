use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, PartialEq)]
enum Language {
    English,
    Turkish,
}

#[derive(Deserialize, Serialize, Clone)]
struct Term {
    text: String,
    pos: [f32; 2],
}

#[derive(Deserialize, Serialize, Clone)]
struct Category {
    name: String,
    color: [u8; 3], 
    pos: [f32; 2],
    terms: Vec<Term>, // Changed from Vec<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CommunicatorApp {
// Persistent Data
    categories: Vec<Category>,

    #[serde(default = "default_language")]
    language: Language,

    // Camera/Canvas State
    #[serde(default)]
    pan_offset: [f32; 2],

    // Volatile Data (Reset on launch)
    #[serde(skip)]
    active_plane: String,
    #[serde(skip)]
    sentence_dock: Vec<(String, [u8; 3])>,
    #[serde(skip)]
    ghost_menu_open: bool,
    #[serde(skip)]
    settings_open: bool,
    #[serde(skip)]
    active_category: Option<usize>,
    #[serde(skip)]
    editing_category_idx: Option<usize>,
    // Interaction State
    #[serde(skip)]
    dragging_category_idx: Option<usize>,
    // Interaction State
    #[serde(skip)]
    dragging_term_idx: Option<usize>,
    #[serde(skip)]
    is_edit_mode: bool,
    #[serde(skip)]
    inline_edit_idx: Option<usize>,
}
fn default_language() -> Language {
    Language::English
}

impl Default for CommunicatorApp {
    fn default() -> Self {
        Self {
            categories: vec![
                Category {
                    name: "People".to_string(),
                    color: [41, 128, 185],
                    terms: vec![
                        Term { text: "I".to_string(), pos: [24.0, 108.0] },
                        Term { text: "You".to_string(), pos: [208.0, 108.0] },
                        Term { text: "They".to_string(), pos: [392.0, 108.0] },
                    ],
                    pos: [0., 0.]
                },
                Category {
                    name: "Actions".to_string(),
                    color: [39, 174, 96],
                    terms: vec![
                        Term { text: "Want".to_string(), pos: [24.0, 108.0] },
                        Term { text: "Go".to_string(), pos: [208.0, 108.0] },
                        Term { text: "Stop".to_string(), pos: [392.0, 108.0] },
                    ],
                    pos: [20., 0.]
                },
            ],
            language: default_language(),
            pan_offset: [0. ,0.],
            active_plane: "Home".to_string(),
            sentence_dock: vec![],
            ghost_menu_open: false,
            settings_open: false,
            active_category: None,
            editing_category_idx: None,
            dragging_category_idx: None,
            dragging_term_idx: None,
            is_edit_mode: false,
            inline_edit_idx: None,
        }
    }
}

impl CommunicatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply our Soft-Tech aesthetic constraints
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(24.0, 24.0);
        style.spacing.window_margin = egui::Margin::same(24);
        style.spacing.button_padding = egui::vec2(24.0, 16.0);
        style.visuals.window_corner_radius = egui::CornerRadius::same(16);
        style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(12);
        cc.egui_ctx.set_style(style);

        // Load previous state if it exists
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    fn t(&self, key: &str) -> String {
        match self.language {
            Language::English => match key {
                "menu" => "Menu",
                "edit_mode" => "Edit Mode",
                "sentence_builder" => "Sentence Builder",
                "clear" => "Clear",
                "settings" => "Settings",
                "back" => "< Back",
                "add_category" => "+ Add Category",
                "add_word" => "+ Add Word",
                "delete_category" => "Delete Entire Category",
                "category_name" => "Category Name:",
                "category_color" => "Category Color:",
                "vocab_terms" => "Vocabulary Terms",
                "new_category" => "New Category",
                "new_word" => "New Word",
                "categories" => "Categories",
                "editor" => "Editor",
                "add_new_category" => "+ Add New Category",
                "delete_term" => "Delete Term",
                "add_term" => "+ Add Term",
                "language" => "Language / Dil:",
                "select_category" => "Select a category on the left to edit its details and vocabulary.",
                "planes_of_existence" => "Planes of Existence",
                "home" => "Home",
                "hospital" => "Hospital",
                _ => key, // Fallback to the key itself if missing
            },
            Language::Turkish => match key {
                "menu" => "Menü",
                "edit_mode" => "Düzenleme Modu",
                "sentence_builder" => "Cümle Kurucu",
                "clear" => "Temizle",
                "settings" => "Ayarlar",
                "back" => "< Geri",
                "add_category" => "+ Kategori Ekle",
                "add_word" => "+ Kelime Ekle",
                "delete_category" => "Tüm Kategoriyi Sil",
                "category_name" => "Kategori Adı:",
                "category_color" => "Kategori Rengi:",
                "vocab_terms" => "Sözlük Kelimeleri",
                "new_category" => "Yeni Kategori",
                "new_word" => "Yeni Kelime",
                "categories" => "Kategoriler",
                "editor" => "Düzenleyici",
                "add_new_category" => "+ Yeni Kategori Ekle",
                "delete_term" => "Kelimeyi Sil",
                "add_term" => "+ Kelime Ekle",
                "language" => "Dil / Language:",
                "select_category" => "Düzenlemek için soldan bir kategori seçin.",
                "planes_of_existence" => "Varlık Düzlemleri",
                "home" => "Ana Sayfa",
                "hospital" => "Hastane",
                _ => key,
            },
        }.to_string()
    }
}

impl eframe::App for CommunicatorApp {
    // 2. The Persistence Hook
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Cache all translated strings at the start to avoid borrow checker issues
        let menu_label = self.t("menu");
        let edit_mode_label = self.t("edit_mode");
        let sentence_builder_label = self.t("sentence_builder");
        let clear_label = self.t("clear");
        let planes_label = self.t("planes_of_existence");
        let home_label = self.t("home");
        let hospital_label = self.t("hospital");
        let settings_label = self.t("settings");
        let language_label = self.t("language");
        let categories_label = self.t("categories");
        let add_new_category_label = self.t("add_new_category");
        let new_category_label = self.t("new_category");
        let editor_label = self.t("editor");
        let category_name_label = self.t("category_name");
        let category_color_label = self.t("category_color");
        let vocab_terms_label = self.t("vocab_terms");
        let add_term_label = self.t("add_term");
        let delete_category_label = self.t("delete_category");
        let select_category_label = self.t("select_category");
        let new_word_label = self.t("new_word");
        let add_category_label = self.t("add_category");
        let add_word_label = self.t("add_word");
        
        // Top Navigation & Ghost Menu Toggle
        egui::TopBottomPanel::top("top_plane_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(&menu_label).clicked() {
                        self.ghost_menu_open = !self.ghost_menu_open;
                    }
                    ui.add_space(16.0);
                    // Add the edit mode toggle here
                    if ui.toggle_value(&mut self.is_edit_mode, &edit_mode_label).changed() {
                        // Clear any active edits if they toggle the mode off
                        self.inline_edit_idx = None; 
                    }
                });
            });
            ui.add_space(8.0);
        });
        // The Bottom Dock
        egui::TopBottomPanel::bottom("dock_panel")
            .min_height(120.0)
            .show(ctx, |ui| {
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.heading(&sentence_builder_label);
                    if ui.button(&clear_label).clicked() {
                        self.sentence_dock.clear();
                    }
                });
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    for (term, color) in &self.sentence_dock {
                        let egui_color = egui::Color32::from_rgb(color[0], color[1], color[2]);
                        let dock_button = egui::Button::new(
                            egui::RichText::new(term).color(egui::Color32::WHITE).size(20.0)
                        )
                        .fill(egui_color)
                        .min_size(egui::vec2(80.0, 60.0));
                        ui.add(dock_button);
                    }
                });
                ui.add_space(12.0);
        });

        // Ghost Menu
        if self.ghost_menu_open {
            egui::Window::new(&menu_label)
                .vscroll(true)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label(&planes_label);
                    ui.separator();
                    if ui.button(&home_label).clicked() { self.active_plane = home_label.clone(); }
                    if ui.button(&hospital_label).clicked() { self.active_plane = hospital_label.clone(); }
                    ui.separator();
                    if ui.button(&settings_label).clicked() {
                        self.settings_open = true;
                        self.ghost_menu_open = false; // Close ghost menu when opening settings
                    }
                });
        }

        // 3. Customization Interface (Settings)
        
        
        // Track if a category needs deletion outside the UI closure to satisfy the borrow checker
        let mut category_to_delete: Option<usize> = None;

        if self.settings_open {
            egui::Window::new(&settings_label)
                .open(&mut self.settings_open)
                .default_width(700.0)
                .default_height(500.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(&language_label);
                        ui.radio_value(&mut self.language, Language::English, "English");
                        ui.radio_value(&mut self.language, Language::Turkish, "Türkçe");
                    });
                    ui.separator();

                    ui.columns(2, |columns| {
                        // Left Column: Master List of Categories
                        columns[0].heading(&categories_label);
                        columns[0].add_space(8.0);
                        
                        egui::ScrollArea::vertical().id_salt("category_list_scroll").show(&mut columns[0], |ui| {
                            for (idx, category) in self.categories.iter().enumerate() {
                                let is_selected = self.editing_category_idx == Some(idx);
                                
                                let button = egui::Button::selectable(
                                    is_selected, 
                                    egui::RichText::new(&category.name).size(20.0)
                                );
                                
                                if ui.add_sized([ui.available_width(), 40.0], button).clicked() {
                                    self.editing_category_idx = Some(idx);
                                }
                                ui.add_space(4.0);
                            }
                            
                            ui.separator();
                            if ui.button(egui::RichText::new(&add_new_category_label).size(18.0)).clicked() {
                                self.categories.push(Category {
                                    name: new_category_label.clone(),
                                    color: [100, 100, 100],
                                    pos: [100.0, 100.0], // Ensure new categories have default coordinates
                                    terms: vec![],
                                });
                                self.editing_category_idx = Some(self.categories.len() - 1);
                            }
                        });

                        // Right Column: Detail Editor
                        columns[1].heading(&editor_label);
                        columns[1].add_space(8.0);

                        if let Some(idx) = self.editing_category_idx {
                            if idx < self.categories.len() {
                                let category = &mut self.categories[idx];
                                egui::ScrollArea::vertical().id_salt("category_edit_scroll").show(&mut columns[1], |ui| {
                                    ui.group(|ui| {
                                        ui.label(&category_name_label);
                                        ui.text_edit_singleline(&mut category.name);
                                        
                                        ui.add_space(8.0);
                                        ui.label(&category_color_label);
                                        let mut rgb = [
                                            category.color[0] as f32 / 255.0,
                                            category.color[1] as f32 / 255.0,
                                            category.color[2] as f32 / 255.0,
                                        ];
                                        if ui.color_edit_button_rgb(&mut rgb).changed() {
                                            category.color = [
                                                (rgb[0] * 255.0) as u8,
                                                (rgb[1] * 255.0) as u8,
                                                (rgb[2] * 255.0) as u8,
                                            ];
                                        }
                                    });

                                    ui.add_space(16.0);
                                    ui.heading(&vocab_terms_label);
                                    ui.add_space(8.0);

                                    let mut term_to_remove = None;
                                    for (term_idx, term) in category.terms.iter_mut().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.text_edit_singleline(&mut term.text);
                                            if ui.button("✕").clicked() {
                                                term_to_remove = Some(term_idx);
                                            }
                                        });
                                        ui.add_space(4.0);
                                    }

                                    if let Some(r_idx) = term_to_remove {
                                        category.terms.remove(r_idx);
                                    }

                                    ui.add_space(8.0);
                                    if ui.button(&add_term_label).clicked() {
                                        category.terms.push(Term { 
                                            text: new_word_label.clone(),
                                            pos: [100.0, 100.0] 
                                        });
                                    }
                                    
                                    ui.add_space(24.0);
                                    ui.separator();
                                    
                                    // Trigger the deletion state
                                    if ui.button(egui::RichText::new(&delete_category_label).color(egui::Color32::RED)).clicked() {
                                        category_to_delete = Some(idx);
                                    }
                                });
                            }
                        } else {
                            columns[1].label(egui::RichText::new(&select_category_label).italics());
                        }
                    });
                });
                
            // Execute the deletion safely outside the UI rendering closure
            if let Some(idx) = category_to_delete {
                 self.categories.remove(idx);
                 // Clear the editor view so it doesn't try to render a deleted category
                 self.editing_category_idx = None;
                 
                 // If the user deleted the category they were currently viewing on the canvas, return to the root view
                 if self.active_category == Some(idx) {
                     self.active_category = None;
                 }
            }
        }
        // Main Interaction Grid (Spatial Canvas)
        egui::CentralPanel::default().show(ctx, |ui| {
            let canvas_rect = ui.available_rect_before_wrap();
            let mut canvas_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(canvas_rect)
                    .layout(egui::Layout::default())
            );
            
            // UPGRADED: Capture both clicks and drags on the background
            let response = canvas_ui.interact(canvas_rect, canvas_ui.id().with("background"), egui::Sense::click_and_drag());
            
            // DESELECT LOGIC: If the user clicks empty canvas, exit inline edit mode
            if response.clicked() {
                self.inline_edit_idx = None;
            }

            if response.dragged() && self.dragging_category_idx.is_none() && self.dragging_term_idx.is_none() {
                self.pan_offset[0] += response.drag_delta().x;
                self.pan_offset[1] += response.drag_delta().y;
            }

            // --- NEW: Calculate Bounding Box and Apply Panning Limits ---
            let padding = 200.0; // The "limited freedom" buffer zone
            let node_w = 160.0;
            let node_h = 120.0;
            
            let mut min_x = f32::MAX;
            let mut min_y = f32::MAX;
            let mut max_x = f32::MIN;
            let mut max_y = f32::MIN;

            let has_items = if let Some(cat_idx) = self.active_category {
                // We are looking at terms
                if let Some(category) = self.categories.get(cat_idx) {
                    for term in &category.terms {
                        min_x = min_x.min(term.pos[0]);
                        min_y = min_y.min(term.pos[1]);
                        max_x = max_x.max(term.pos[0] + node_w);
                        max_y = max_y.max(term.pos[1] + node_h);
                    }
                    !category.terms.is_empty()
                } else {
                    false
                }
            } else {
                // We are looking at root categories
                for cat in &self.categories {
                    min_x = min_x.min(cat.pos[0]);
                    min_y = min_y.min(cat.pos[1]);
                    max_x = max_x.max(cat.pos[0] + node_w);
                    max_y = max_y.max(cat.pos[1] + node_h);
                }
                !self.categories.is_empty()
            };

            // Only clamp if there is content on the screen
            if has_items {
                let canvas_w = canvas_rect.width();
                let canvas_h = canvas_rect.height();

                let allowed_max_x = -min_x + padding;
                let allowed_max_y = -min_y + padding;
                
                let allowed_min_x = canvas_w - max_x - padding;
                let allowed_min_y = canvas_h - max_y - padding;

                // Clamp X
                if allowed_min_x > allowed_max_x {
                    // Content is smaller than the screen; constrain movement within the padding
                    self.pan_offset[0] = self.pan_offset[0].clamp(allowed_max_x, allowed_min_x);
                } else {
                    // Content is larger than the screen; constrain to edges
                    self.pan_offset[0] = self.pan_offset[0].clamp(allowed_min_x, allowed_max_x);
                }

                // Clamp Y
                if allowed_min_y > allowed_max_y {
                    self.pan_offset[1] = self.pan_offset[1].clamp(allowed_max_y, allowed_min_y);
                } else {
                    self.pan_offset[1] = self.pan_offset[1].clamp(allowed_min_y, allowed_max_y);
                }
            } else {
                // If the canvas is completely empty, lock the camera to the center
                self.pan_offset = [0.0, 0.0];
            }
            // 2. Render Categories at explicit coordinates
            if self.active_category.is_none() {
                let mut cat_to_delete = None; // Track deletion outside the loop
                // Removed unused add_category_label
                for (idx, category) in self.categories.iter_mut().enumerate() {
                    let screen_pos = egui::pos2(
                        canvas_rect.min.x + category.pos[0] + self.pan_offset[0],
                        canvas_rect.min.y + category.pos[1] + self.pan_offset[1],
                    );
                    let node_rect = egui::Rect::from_min_size(screen_pos, egui::vec2(160.0, 120.0));
                    let egui_color = egui::Color32::from_rgb(category.color[0], category.color[1], category.color[2]);
                    // Manual background paint
                    canvas_ui.painter().rect_filled(node_rect, egui::CornerRadius::same(12), egui_color);
                    let mut child_ui = canvas_ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(node_rect)
                            .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                    );
                    let text_color = if egui_color.r() as u32 + egui_color.g() as u32 + egui_color.b() as u32 > 382 {
                        egui::Color32::BLACK
                    } else { 
                        egui::Color32::WHITE 
                    };
                    if self.is_edit_mode && self.inline_edit_idx == Some(idx) {
                        // DELETION BUTTON: Absolute positioning in the top right corner
                        let x_rect = egui::Rect::from_min_size(node_rect.right_top() + egui::vec2(-28.0, 4.0), egui::vec2(24.0, 24.0));
                        let mut x_ui = canvas_ui.new_child(
                            egui::UiBuilder::new()
                                .max_rect(x_rect)
                                .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                        );
                        if x_ui.add(egui::Button::new(egui::RichText::new("X").color(egui::Color32::WHITE).strong()).fill(egui::Color32::RED)).clicked() {
                            cat_to_delete = Some(idx);
                        }
                        // Editor Layout
                        child_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.add_space(24.0); 
                            ui.add(
                                egui::TextEdit::singleline(&mut category.name)
                                    .font(egui::FontId::proportional(20.0))
                                    .text_color(text_color)
                                    .horizontal_align(egui::Align::Center)
                                    .frame(false)
                            );
                            ui.add_space(8.0);
                            let mut rgb = [
                                category.color[0] as f32 / 255.0,
                                category.color[1] as f32 / 255.0,
                                category.color[2] as f32 / 255.0,
                            ];
                            if ui.color_edit_button_rgb(&mut rgb).changed() {
                                category.color = [(rgb[0] * 255.0) as u8, (rgb[1] * 255.0) as u8, (rgb[2] * 255.0) as u8];
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)) {
                                self.inline_edit_idx = None;
                            }
                        });
                    } else {
                        let btn_response = child_ui.add(
                            egui::Button::new(egui::RichText::new(&category.name).size(24.0).color(text_color))
                                .fill(egui::Color32::TRANSPARENT) 
                                .sense(egui::Sense::click_and_drag())
                        );
                        if btn_response.drag_started() { self.dragging_category_idx = Some(idx); }
                        if btn_response.dragged() {
                            category.pos[0] += btn_response.drag_delta().x;
                            category.pos[1] += btn_response.drag_delta().y;
                        }
                        if btn_response.drag_stopped() { self.dragging_category_idx = None; }
                        if btn_response.clicked() && !btn_response.dragged() {
                            if self.is_edit_mode {
                                self.inline_edit_idx = Some(idx);
                            } else {
                                self.active_category = Some(idx);
                                self.inline_edit_idx = None;
                            }
                        }
                    }
                }
                // Execute safe deletion
                if let Some(idx) = cat_to_delete {
                    self.categories.remove(idx);
                    self.inline_edit_idx = None;
                }

            } else if let Some(cat_idx) = self.active_category {
                // 3. Render Expanded Terms 
                let back_label = self.t("back");
                let category = &mut self.categories[cat_idx];
                let egui_color = egui::Color32::from_rgb(category.color[0], category.color[1], category.color[2]);
                let text_color = if egui_color.r() as u32 + egui_color.g() as u32 + egui_color.b() as u32 > 382 {
                    egui::Color32::BLACK
                } else { egui::Color32::WHITE };
                // Static Back Button
                let back_rect = egui::Rect::from_min_size(egui::pos2(canvas_rect.min.x + 24.0, canvas_rect.min.y + 24.0), egui::vec2(120.0, 60.0));
                let mut back_ui = canvas_ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(back_rect)
                        .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                );
                if back_ui.add(egui::Button::new(egui::RichText::new(&back_label).size(24.0))).clicked() {
                    self.active_category = None;
                    self.inline_edit_idx = None;
                }
                let mut term_to_delete = None; // Track deletion outside the loop
                // Render Terms
                for (idx, term) in category.terms.iter_mut().enumerate() {
                    let screen_pos = egui::pos2(
                        canvas_rect.min.x + term.pos[0] + self.pan_offset[0],
                        canvas_rect.min.y + term.pos[1] + self.pan_offset[1],
                    );
                    let node_rect = egui::Rect::from_min_size(screen_pos, egui::vec2(160.0, 120.0));
                    // FIX: Manual background paint for terms to guarantee correct color and contrast
                    canvas_ui.painter().rect_filled(node_rect, egui::CornerRadius::same(12), egui_color);
                    let mut term_ui = canvas_ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(node_rect)
                            .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                    );
                    if self.is_edit_mode && self.inline_edit_idx == Some(idx) {
                        // DELETION BUTTON for terms
                        let x_rect = egui::Rect::from_min_size(node_rect.right_top() + egui::vec2(-28.0, 4.0), egui::vec2(24.0, 24.0));
                        let mut x_ui = canvas_ui.new_child(
                            egui::UiBuilder::new()
                                .max_rect(x_rect)
                                .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                        );
                        if x_ui.add(egui::Button::new(egui::RichText::new("X").color(egui::Color32::WHITE).strong()).fill(egui::Color32::RED)).clicked() {
                            term_to_delete = Some(idx);
                        }
                        term_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.add_space(40.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut term.text)
                                    .font(egui::FontId::proportional(20.0))
                                    .text_color(text_color) // Inherits the calculated contrast color
                                    .horizontal_align(egui::Align::Center)
                                    .frame(false)
                            );
                            if ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)) {
                                self.inline_edit_idx = None;
                            }
                        });
                    } else {
                        let btn_response = term_ui.add(
                            egui::Button::new(egui::RichText::new(&term.text).size(24.0).color(text_color))
                                .fill(egui::Color32::TRANSPARENT) // Background handled by painter
                                .sense(egui::Sense::click_and_drag())
                        );
                        if btn_response.drag_started() { self.dragging_term_idx = Some(idx); }
                        if btn_response.dragged() {
                            term.pos[0] += btn_response.drag_delta().x;
                            term.pos[1] += btn_response.drag_delta().y;
                        }
                        if btn_response.drag_stopped() { self.dragging_term_idx = None; }
                        if btn_response.clicked() && !btn_response.dragged() {
                            if self.is_edit_mode {
                                self.inline_edit_idx = Some(idx);
                            } else {
                                self.sentence_dock.push((term.text.clone(), category.color));
                            }
                        }
                    }
                }
                // Execute safe deletion for terms
                if let Some(idx) = term_to_delete {
                    category.terms.remove(idx);
                    self.inline_edit_idx = None;
                }
            }

            // 4. Floating Action Button for Adding New Items directly on the Canvas
            if self.is_edit_mode {
                let fab_rect = egui::Rect::from_min_size(
                    egui::pos2(canvas_rect.max.x - 220.0, canvas_rect.min.y + 24.0),
                    egui::vec2(180.0, 60.0)
                );
                
                let mut fab_ui = canvas_ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(fab_rect)
                        .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                );
                
                // Calculate drop coordinates to place the new item exactly in the center of the viewport
                let drop_x = -self.pan_offset[0] + (canvas_rect.width() / 2.0) - 80.0;
                let drop_y = -self.pan_offset[1] + (canvas_rect.height() / 2.0) - 60.0;

                if self.active_category.is_none() {
                    if fab_ui.add(egui::Button::new(egui::RichText::new(&add_category_label).size(20.0)).fill(egui::Color32::DARK_GRAY)).clicked() {
                        self.categories.push(Category {
                            name: new_category_label.clone(),
                            color: [100, 100, 100],
                            pos: [drop_x, drop_y],
                            terms: vec![],
                        });
                        self.inline_edit_idx = Some(self.categories.len() - 1);
                    }
                } else if let Some(cat_idx) = self.active_category {
                    if fab_ui.add(egui::Button::new(egui::RichText::new(&add_word_label).size(20.0)).fill(egui::Color32::DARK_GRAY)).clicked() {
                        self.categories[cat_idx].terms.push(Term {
                            text: new_word_label.clone(),
                            pos: [drop_x, drop_y],
                        });
                        self.inline_edit_idx = Some(self.categories[cat_idx].terms.len() - 1);
                    }
                }
            }
            
        });
    }
}