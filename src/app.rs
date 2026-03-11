use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct Category {
    name: String,
    color: [u8; 3], // Store as standard RGB array for easy serialization
    terms: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CommunicatorApp {
// Persistent Data
    categories: Vec<Category>,

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
}
impl Default for CommunicatorApp {
    fn default() -> Self {
        Self {
            categories: vec![
                Category {
                    name: "People".to_string(),
                    color: [41, 128, 185],
                    terms: vec!["I".to_string(), "You".to_string(), "They".to_string()],
                },
                Category {
                    name: "Actions".to_string(),
                    color: [39, 174, 96],
                    terms: vec!["Want".to_string(), "Go".to_string(), "Stop".to_string()],
                },
            ],
            active_plane: "Home".to_string(),
            sentence_dock: vec![],
            ghost_menu_open: false,
            settings_open: false,
            active_category: None,
            editing_category_idx: None,
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
}

impl eframe::App for CommunicatorApp {
    // 2. The Persistence Hook
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // Top Navigation & Ghost Menu Toggle
        egui::TopBottomPanel::top("top_plane_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Current Plane: {}", self.active_plane)).size(18.0).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Menu").clicked() {
                        self.ghost_menu_open = !self.ghost_menu_open;
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
                    ui.heading("Sentence Builder");
                    if ui.button("Clear").clicked() {
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
            egui::Window::new("Ghost Menu")
                .vscroll(true)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Planes of Existence");
                    ui.separator();
                    if ui.button("Home").clicked() { self.active_plane = "Home".to_string(); }
                    if ui.button("Hospital").clicked() { self.active_plane = "Hospital".to_string(); }
                    ui.separator();
                    if ui.button("Settings").clicked() {
                        self.settings_open = true;
                        self.ghost_menu_open = false; // Close ghost menu when opening settings
                    }
                });
        }

        // 3. Customization Interface (Settings)
        if self.settings_open {
            egui::Window::new("Settings & Customization")
                .open(&mut self.settings_open)
                .default_width(700.0) // Wider to accommodate the split view
                .default_height(500.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.columns(2, |columns| {
                        // Left Column: Master List of Categories
                        columns[0].heading("Categories");
                        columns[0].add_space(8.0);
                        
                        egui::ScrollArea::vertical().id_salt("category_list_scroll").show(&mut columns[0], |ui| {
                            for (idx, category) in self.categories.iter().enumerate() {
                                let is_selected = self.editing_category_idx == Some(idx);
                                
                                // Large selectable labels for dexterity
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
                            if ui.button(egui::RichText::new("+ Add New Category").size(18.0)).clicked() {
                                self.categories.push(Category {
                                    name: "New Category".to_string(),
                                    color: [100, 100, 100],
                                    terms: vec![],
                                });
                                // Automatically select the newly created category
                                self.editing_category_idx = Some(self.categories.len() - 1);
                            }
                        });

                        // Right Column: Detail Editor
                        columns[1].heading("Editor");
                        columns[1].add_space(8.0);

                        if let Some(idx) = self.editing_category_idx {
                            if idx < self.categories.len() {
                                let category = &mut self.categories[idx];
                                
                                egui::ScrollArea::vertical().id_salt("category_edit_scroll").show(&mut columns[1], |ui| {
                                    ui.group(|ui| {
                                        ui.label("Category Name:");
                                        ui.text_edit_singleline(&mut category.name);
                                        
                                        ui.add_space(8.0);
                                        ui.label("Category Color:");
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
                                    ui.heading("Vocabulary Terms");
                                    ui.add_space(8.0);

                                    // Term Management
                                    let mut term_to_remove = None;
                                    for (term_idx, term) in category.terms.iter_mut().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.text_edit_singleline(term);
                                            if ui.button("X").clicked() {
                                                term_to_remove = Some(term_idx);
                                            }
                                        });
                                        ui.add_space(4.0);
                                    }

                                    if let Some(r_idx) = term_to_remove {
                                        category.terms.remove(r_idx);
                                    }

                                    ui.add_space(8.0);
                                    if ui.button("+ Add Term").clicked() {
                                        category.terms.push("New Word".to_string());
                                    }
                                    
                                    ui.add_space(24.0);
                                    ui.separator();
                                    if ui.button(egui::RichText::new("Delete Entire Category").color(egui::Color32::RED)).clicked() {
                                        // We will handle the actual deletion outside the mutable borrow below
                                        self.editing_category_idx = Some(usize::MAX); 
                                    }
                                });
                            }
                        } else {
                            columns[1].label(egui::RichText::new("Select a category on the left to edit its details and vocabulary.").italics());
                        }
                    });
                });
                
            // Handle category deletion safely outside the main UI borrow
            if self.editing_category_idx == Some(usize::MAX) {
                 // To implement safe deletion, we would track the actual index to delete.
                 // For now, this acts as a placeholder for the logic.
                 self.editing_category_idx = None;
            }
        }

        // Main Interaction Grid (Semantic Zoom)
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(cat_idx) = self.active_category {
                // Expanded View: Show Terms inside the Category
                let category = &self.categories[cat_idx];
                
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("< Back").size(24.0)).clicked() {
                        self.active_category = None;
                    }
                    ui.add_space(16.0);
                    ui.heading(&category.name);
                });
                ui.add_space(12.0);

                let egui_color = egui::Color32::from_rgb(category.color[0], category.color[1], category.color[2]);
                let text_color = if egui_color.r() as u32 + egui_color.g() as u32 + egui_color.b() as u32 > 382 {
                    egui::Color32::BLACK
                } else {
                    egui::Color32::WHITE
                };

                ui.horizontal_wrapped(|ui| {
                    for term in &category.terms {
                        let btn = egui::Button::new(egui::RichText::new(term).size(24.0).color(text_color))
                            .fill(egui_color)
                            .min_size(egui::vec2(160.0, 120.0));
                        
                        if ui.add(btn).clicked() {
                            self.sentence_dock.push((term.clone(), category.color));
                        }
                    }
                });
            } else {
                // Root View: Show Categories
                ui.heading("Categories");
                ui.add_space(12.0);
                
                ui.horizontal_wrapped(|ui| {
                    // Use enumerate to capture the index for the click state
                    for (idx, category) in self.categories.iter().enumerate() {
                        let egui_color = egui::Color32::from_rgb(category.color[0], category.color[1], category.color[2]);
                        
                        let text_color = if egui_color.r() as u32 + egui_color.g() as u32 + egui_color.b() as u32 > 382 {
                            egui::Color32::BLACK
                        } else {
                            egui::Color32::WHITE
                        };

                        let btn = egui::Button::new(egui::RichText::new(&category.name).size(24.0).color(text_color))
                            .fill(egui_color)
                            .min_size(egui::vec2(160.0, 120.0));
                        
                        // Transition into Semantic Zoom instead of adding a term
                        if ui.add(btn).clicked() {
                            self.active_category = Some(idx);
                        }
                    }
                });
            }
        });
    }
}