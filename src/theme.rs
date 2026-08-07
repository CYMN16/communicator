//! Design tokens for the "Soft-Tech" Communicator interface.
//!
//! Everything visual in the app is derived from this module: colours, spacing,
//! corner radii, elevation and type scale. Keeping them in one place means the
//! canvas, the dock and the chrome always agree with each other.

use egui::{Color32, CornerRadius, FontId, Margin, Shadow, Stroke, TextStyle, epaint};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Spatial tokens
// ---------------------------------------------------------------------------

/// The base rhythm of the layout. Every gap is a multiple of this.
pub const SPACE: f32 = 8.0;
/// Comfortable padding between tappable elements (anti "fat-finger").
pub const GUTTER: f32 = 24.0;
/// Nodes snap to this grid so the canvas can never become cluttered.
pub const GRID: f32 = 24.0;

/// Default size of a category / term node on the canvas.
pub const NODE_SIZE: egui::Vec2 = egui::vec2(184.0, 144.0);

/// Corner radii.
pub const RADIUS_NODE: u8 = 22;
pub const RADIUS_CARD: u8 = 18;
pub const RADIUS_PILL: u8 = 99;

// ---------------------------------------------------------------------------
// Themes
// ---------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    /// Maximum contrast for low-vision users: black canvas, white ink, yellow focus.
    HighContrast,
}

impl Theme {
    pub const ALL: [Self; 3] = [Self::Light, Self::Dark, Self::HighContrast];

    pub fn palette(self) -> Palette {
        match self {
            Self::Light => Palette {
                canvas: Color32::from_rgb(0xEE, 0xF1, 0xF6),
                canvas_dot: Color32::from_rgb(0xD3, 0xDA, 0xE6),
                surface: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                surface_sunken: Color32::from_rgb(0xF2, 0xF5, 0xFA),
                outline: Color32::from_rgb(0xDD, 0xE3, 0xED),
                ink: Color32::from_rgb(0x17, 0x20, 0x33),
                ink_muted: Color32::from_rgb(0x5C, 0x6A, 0x82),
                accent: Color32::from_rgb(0x3B, 0x6E, 0xF6),
                danger: Color32::from_rgb(0xD9, 0x3A, 0x3A),
                shadow: Color32::from_black_alpha(38),
                dark_mode: false,
                high_contrast: false,
            },
            Self::Dark => Palette {
                canvas: Color32::from_rgb(0x0E, 0x12, 0x18),
                canvas_dot: Color32::from_rgb(0x24, 0x2C, 0x38),
                surface: Color32::from_rgb(0x17, 0x1D, 0x26),
                surface_sunken: Color32::from_rgb(0x11, 0x16, 0x1D),
                outline: Color32::from_rgb(0x2B, 0x34, 0x42),
                ink: Color32::from_rgb(0xE7, 0xEE, 0xF8),
                ink_muted: Color32::from_rgb(0x94, 0xA3, 0xB8),
                accent: Color32::from_rgb(0x74, 0x9F, 0xFF),
                danger: Color32::from_rgb(0xF2, 0x6B, 0x6B),
                shadow: Color32::from_black_alpha(120),
                dark_mode: true,
                high_contrast: false,
            },
            Self::HighContrast => Palette {
                canvas: Color32::BLACK,
                canvas_dot: Color32::from_rgb(0x44, 0x44, 0x44),
                surface: Color32::BLACK,
                surface_sunken: Color32::from_rgb(0x0A, 0x0A, 0x0A),
                outline: Color32::WHITE,
                ink: Color32::WHITE,
                ink_muted: Color32::from_rgb(0xD0, 0xD0, 0xD0),
                accent: Color32::from_rgb(0xFF, 0xD4, 0x00),
                danger: Color32::from_rgb(0xFF, 0x5C, 0x5C),
                shadow: Color32::TRANSPARENT,
                dark_mode: true,
                high_contrast: true,
            },
        }
    }
}

/// A resolved set of colours for one theme.
#[derive(Clone, Copy)]
pub struct Palette {
    pub canvas: Color32,
    pub canvas_dot: Color32,
    pub surface: Color32,
    pub surface_sunken: Color32,
    pub outline: Color32,
    pub ink: Color32,
    pub ink_muted: Color32,
    pub accent: Color32,
    pub danger: Color32,
    pub shadow: Color32,
    pub dark_mode: bool,
    pub high_contrast: bool,
}

impl Palette {
    /// Soft elevation. `level` 0 = resting card, 2 = lifted node, 3 = floating menu.
    pub fn elevation(&self, level: u8) -> Shadow {
        if self.high_contrast {
            return Shadow::NONE;
        }
        let level = level.min(3);
        Shadow {
            offset: [0, (2 + level as i32 * 3) as i8],
            blur: 8 + level * 8,
            spread: 0,
            color: self.shadow,
        }
    }

    /// Ink colour that stays legible on top of `bg`.
    pub fn on(&self, bg: Color32) -> Color32 {
        if luminance(bg) > 0.55 {
            if self.high_contrast {
                Color32::BLACK
            } else {
                Color32::from_rgb(0x10, 0x17, 0x25)
            }
        } else {
            Color32::WHITE
        }
    }

    /// A hairline that separates a coloured node from the canvas.
    pub fn node_outline(&self, fill: Color32) -> Stroke {
        if self.high_contrast {
            Stroke::new(2.0_f32, Color32::WHITE)
        } else {
            Stroke::new(1.0_f32, mix(fill, self.on(fill), 0.14))
        }
    }
}

// ---------------------------------------------------------------------------
// Colour helpers
// ---------------------------------------------------------------------------

/// Perceived brightness of a colour, 0.0 – 1.0.
pub fn luminance(c: Color32) -> f32 {
    let f = |v: u8| {
        let v = f32::from(v) / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
}

/// Linear blend: `t = 0` gives `a`, `t = 1` gives `b`.
pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t) as u8;
    Color32::from_rgb(lerp(a.r(), b.r()), lerp(a.g(), b.g()), lerp(a.b(), b.b()))
}

/// Same colour, new opacity (0.0 – 1.0).
pub fn fade(c: Color32, alpha: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (alpha.clamp(0.0, 1.0) * 255.0) as u8)
}

pub fn to_color(rgb: [u8; 3]) -> Color32 {
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}

pub fn to_hex(rgb: [u8; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

/// Parse `#RRGGBB` / `RRGGBB` / `#RGB`. Returns `None` for anything else.
pub fn from_hex(text: &str) -> Option<[u8; 3]> {
    let hex = text.trim().trim_start_matches('#');
    let expand = |c: char| u8::from_str_radix(&c.to_string(), 16).ok().map(|v| v * 17);
    match hex.len() {
        3 => {
            let mut it = hex.chars();
            Some([
                expand(it.next()?)?,
                expand(it.next()?)?,
                expand(it.next()?)?,
            ])
        }
        6 => {
            let byte = |range: std::ops::Range<usize>| u8::from_str_radix(hex.get(range)?, 16).ok();
            Some([byte(0..2)?, byte(2..4)?, byte(4..6)?])
        }
        _ => None,
    }
}

/// Pre-attentive category colours: distinguishable by hue *and* by lightness,
/// so they still read apart for the most common colour-vision deficiencies.
pub const CATEGORY_PRESETS: [(&str, [u8; 3]); 10] = [
    ("Blue", [0x2F, 0x6F, 0xD0]),
    ("Teal", [0x0E, 0x8F, 0x8F]),
    ("Green", [0x2E, 0x9E, 0x5B]),
    ("Lime", [0x7A, 0xA8, 0x1F]),
    ("Amber", [0xE0, 0x9F, 0x11]),
    ("Orange", [0xE0, 0x6C, 0x1E]),
    ("Red", [0xD1, 0x3F, 0x4A]),
    ("Pink", [0xD3, 0x50, 0x92]),
    ("Purple", [0x7B, 0x54, 0xC8]),
    ("Slate", [0x50, 0x5E, 0x72]),
];

// ---------------------------------------------------------------------------
// Style application
// ---------------------------------------------------------------------------

/// Push the design tokens into egui's global style.
pub fn apply(ctx: &egui::Context, theme: Theme) {
    let pal = theme.palette();
    let mut style = (*ctx.style()).clone();

    // --- Type scale: generous, because the reader may be across the room. ---
    style.text_styles = [
        (TextStyle::Small, FontId::proportional(15.0)),
        (TextStyle::Body, FontId::proportional(19.0)),
        (TextStyle::Monospace, FontId::monospace(17.0)),
        (TextStyle::Button, FontId::proportional(19.0)),
        (TextStyle::Heading, FontId::proportional(26.0)),
    ]
    .into();

    // --- Spacing: 24pt gutters everywhere. ---
    style.spacing.item_spacing = egui::vec2(SPACE * 1.5, SPACE * 1.5);
    style.spacing.button_padding = egui::vec2(18.0, 12.0);
    style.spacing.window_margin = Margin::same(GUTTER as i8);
    style.spacing.menu_margin = Margin::same(12);
    style.spacing.interact_size = egui::vec2(48.0, 44.0);
    style.spacing.slider_width = 220.0;
    style.spacing.scroll.bar_width = 12.0;

    // --- Visuals ---
    let v = &mut style.visuals;
    v.dark_mode = pal.dark_mode;
    v.panel_fill = pal.surface;
    v.window_fill = pal.surface;
    v.faint_bg_color = pal.surface_sunken;
    v.extreme_bg_color = pal.surface_sunken;
    v.override_text_color = Some(pal.ink);
    v.hyperlink_color = pal.accent;
    v.warn_fg_color = Color32::from_rgb(0xE0, 0x9F, 0x11);
    v.error_fg_color = pal.danger;
    v.window_corner_radius = CornerRadius::same(RADIUS_CARD);
    v.menu_corner_radius = CornerRadius::same(RADIUS_CARD);
    v.window_stroke = Stroke::new(1.0_f32, pal.outline);
    v.window_shadow = pal.elevation(3);
    v.popup_shadow = pal.elevation(2);
    v.selection = egui::style::Selection {
        bg_fill: fade(pal.accent, 0.35),
        stroke: Stroke::new(1.0_f32, pal.ink),
    };

    let radius = CornerRadius::same(14);
    let w = &mut v.widgets;
    w.noninteractive.corner_radius = radius;
    w.noninteractive.bg_fill = pal.surface_sunken;
    w.noninteractive.weak_bg_fill = pal.surface_sunken;
    w.noninteractive.bg_stroke = Stroke::new(1.0_f32, pal.outline);
    w.noninteractive.fg_stroke = Stroke::new(1.0_f32, pal.ink_muted);

    w.inactive.corner_radius = radius;
    // `bg_fill` is the slider rail / radio well: it has to read against a card,
    // while `weak_bg_fill` is the button face, which should not.
    w.inactive.bg_fill = mix(pal.surface_sunken, pal.ink, 0.16);
    w.inactive.weak_bg_fill = pal.surface_sunken;
    w.inactive.bg_stroke = Stroke::new(1.0_f32, pal.outline);
    w.inactive.fg_stroke = Stroke::new(1.0_f32, pal.ink);
    w.inactive.expansion = 0.0;

    w.hovered.corner_radius = radius;
    w.hovered.bg_fill = mix(pal.surface_sunken, pal.accent, 0.14);
    w.hovered.weak_bg_fill = mix(pal.surface_sunken, pal.accent, 0.14);
    w.hovered.bg_stroke = Stroke::new(1.5_f32, fade(pal.accent, 0.7));
    w.hovered.fg_stroke = Stroke::new(1.2_f32, pal.ink);
    w.hovered.expansion = 1.0;

    w.active.corner_radius = radius;
    w.active.bg_fill = mix(pal.surface_sunken, pal.accent, 0.28);
    w.active.weak_bg_fill = mix(pal.surface_sunken, pal.accent, 0.28);
    w.active.bg_stroke = Stroke::new(2.0_f32, pal.accent);
    w.active.fg_stroke = Stroke::new(1.5_f32, pal.ink);
    w.active.expansion = 0.0;

    w.open.corner_radius = radius;
    w.open.bg_fill = pal.surface_sunken;
    w.open.weak_bg_fill = pal.surface_sunken;
    w.open.bg_stroke = Stroke::new(1.0_f32, pal.outline);
    w.open.fg_stroke = Stroke::new(1.0_f32, pal.ink);

    ctx.set_style(style);
}

/// The card frame used by floating windows and panels.
pub fn card_frame(pal: &Palette) -> egui::Frame {
    egui::Frame::new()
        .fill(pal.surface)
        .stroke(Stroke::new(1.0_f32, pal.outline))
        .corner_radius(CornerRadius::same(RADIUS_CARD))
        .inner_margin(Margin::same(GUTTER as i8))
        .shadow(pal.elevation(3))
}

/// Paint the soft dot-grid backdrop that makes the snap grid legible.
pub fn paint_canvas_backdrop(
    painter: &egui::Painter,
    rect: egui::Rect,
    pan: egui::Vec2,
    pal: &Palette,
    tint: Option<Color32>,
) {
    painter.rect_filled(rect, 0, pal.canvas);

    if let Some(tint) = tint {
        // A whisper of the active category's colour: "you are inside this node".
        painter.rect_filled(
            rect,
            0,
            fade(tint, if pal.high_contrast { 0.0 } else { 0.06 }),
        );
    }

    if pal.high_contrast {
        return; // Dots would only add noise at maximum contrast.
    }

    let step = GRID * 2.0;
    let start_x = rect.min.x + pan.x.rem_euclid(step);
    let start_y = rect.min.y + pan.y.rem_euclid(step);
    let mut shapes = Vec::new();
    let mut y = start_y - step;
    while y < rect.max.y + step {
        let mut x = start_x - step;
        while x < rect.max.x + step {
            if rect.contains(egui::pos2(x, y)) {
                shapes.push(epaint::Shape::circle_filled(
                    egui::pos2(x, y),
                    1.5,
                    pal.canvas_dot,
                ));
            }
            x += step;
        }
        y += step;
    }
    painter.extend(shapes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trips() {
        for (_, rgb) in CATEGORY_PRESETS {
            assert_eq!(from_hex(&to_hex(rgb)), Some(rgb));
        }
    }

    #[test]
    fn hex_accepts_what_a_carer_might_type() {
        assert_eq!(from_hex("#FF8000"), Some([0xFF, 0x80, 0x00]));
        assert_eq!(from_hex("ff8000"), Some([0xFF, 0x80, 0x00]));
        assert_eq!(from_hex("  #F80  "), Some([0xFF, 0x88, 0x00]));
        assert_eq!(from_hex(""), None);
        assert_eq!(from_hex("#12345"), None);
        assert_eq!(from_hex("#GGGGGG"), None);
    }

    #[test]
    fn every_preset_stays_legible_under_its_own_label() {
        // Whatever colour is chosen, the label picked for it must contrast.
        for theme in Theme::ALL {
            let pal = theme.palette();
            for (name, rgb) in CATEGORY_PRESETS {
                let fill = to_color(rgb);
                let ink = pal.on(fill);
                let contrast = (luminance(fill) - luminance(ink)).abs();
                assert!(
                    contrast > 0.25,
                    "{name} on {theme:?} has contrast {contrast}"
                );
            }
        }
    }

    #[test]
    fn mixing_moves_between_the_ends() {
        let black = Color32::BLACK;
        let white = Color32::WHITE;
        assert_eq!(mix(black, white, 0.0), black);
        assert_eq!(mix(black, white, 1.0), white);
        assert!(mix(black, white, 0.5).r() > 100);
        // Out-of-range factors are clamped rather than wrapping.
        assert_eq!(mix(black, white, -3.0), black);
        assert_eq!(mix(black, white, 9.0), white);
    }
}
