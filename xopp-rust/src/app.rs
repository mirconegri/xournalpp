use eframe::egui::{self, Color32, Key, Sense, Stroke as EguiStroke, Vec2};
use crate::history::{Action, HistoryManager};
use crate::io::{load_xopp, save_xopp};
use crate::model::{Color, Document, PageBackgroundType, Point, Stroke};
use crate::render::{CanvasRenderer, Viewport};
use crate::theme::{Theme, ThemeMode};
use crate::tools::eraser::{EraserHandler, EraserMode};
use crate::tools::pen::PenHandler;
use crate::tools::shapes::ShapeBuilder;
use crate::tools::ActiveTool;

// Main Xournal++ Rust desktop application state
pub struct XoppApp {
    // Current document model
    pub doc: Document,
    // History stack for undo and redo
    pub history: HistoryManager,
    // Currently active drawing tool
    pub active_tool: ActiveTool,
    // Active ink color
    pub active_color: Color,
    // Active stroke width
    pub active_width: f32,
    // Real-time pen stroke builder
    pub pen_handler: PenHandler,
    // Real-time eraser handler
    pub eraser_handler: EraserHandler,
    // Viewport pan and zoom state
    pub viewport: Viewport,
    // Current theme (Light or AMOLED Dark)
    pub theme_mode: ThemeMode,
    // Interactive shape starting point
    pub shape_start: Option<Point>,
    // Status bar message
    pub status_message: String,
}

impl Default for XoppApp {
    // Initialize application with modern default preferences
    fn default() -> Self {
        let active_color = Color::BLACK;
        let active_width = 1.6;
        Self {
            doc: Document::default(),
            history: HistoryManager::default(),
            active_tool: ActiveTool::Pen,
            active_color,
            active_width,
            pen_handler: PenHandler::new(active_color, active_width, false),
            eraser_handler: EraserHandler::default(),
            viewport: Viewport::default(),
            theme_mode: ThemeMode::Dark,
            shape_start: None,
            status_message: String::from("Ready"),
        }
    }
}

impl eframe::App for XoppApp {
    // Main UI update and rendering cycle
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Theme::apply(self.theme_mode, ctx);
        self.handle_shortcuts(ctx);

        self.render_top_panel(ctx);
        self.render_sidebar(ctx);
        self.render_bottom_panel(ctx);
        self.render_central_canvas(ctx);
    }
}

impl XoppApp {
    // Global keyboard shortcut dispatcher
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());

        if input.modifiers.command && input.key_pressed(Key::Z) {
            self.undo();
        }
        if input.modifiers.command && input.key_pressed(Key::Y) {
            self.redo();
        }
        if input.modifiers.command && input.key_pressed(Key::O) {
            self.open_file_dialog();
        }
        if input.modifiers.command && input.key_pressed(Key::S) {
            self.save_file_dialog();
        }
    }

    // Top menubar and modern action toolbar
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_toolbar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                // Main application menu buttons
                ui.menu_button("File", |ui| {
                    if ui.button("New Document").clicked() {
                        self.doc = Document::default();
                        self.status_message = String::from("New document created");
                        ui.close_menu();
                    }
                    if ui.button("Open (.xopp)...").clicked() {
                        self.open_file_dialog();
                        ui.close_menu();
                    }
                    if ui.button("Save (.xopp)...").clicked() {
                        self.save_file_dialog();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.add_enabled(self.history.can_undo(), egui::Button::new("Undo (Ctrl+Z)")).clicked() {
                        self.undo();
                        ui.close_menu();
                    }
                    if ui.add_enabled(self.history.can_redo(), egui::Button::new("Redo (Ctrl+Y)")).clicked() {
                        self.redo();
                        ui.close_menu();
                    }
                    if ui.button("Clear Page").clicked() {
                        if let Some(page) = self.doc.current_page_mut() {
                            page.active_layer_mut().strokes.clear();
                        }
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Zoom In (Ctrl++)").clicked() {
                        self.viewport.zoom = (self.viewport.zoom * 1.2).min(5.0);
                    }
                    if ui.button("Zoom Out (Ctrl+-)").clicked() {
                        self.viewport.zoom = (self.viewport.zoom / 1.2).max(0.2);
                    }
                    if ui.button("Reset Zoom (100%)").clicked() {
                        self.viewport.zoom = 1.0;
                    }
                });

                ui.separator();

                // Interactive tool selection buttons
                self.tool_button(ui, "🖊 Pen", ActiveTool::Pen);
                self.tool_button(ui, "🖍 Highlighter", ActiveTool::Highlighter);
                self.tool_button(ui, "🧹 Eraser", ActiveTool::Eraser);
                self.tool_button(ui, "📏 Line", ActiveTool::Line);
                self.tool_button(ui, "↗ Arrow", ActiveTool::Arrow);
                self.tool_button(ui, "▭ Rect", ActiveTool::Rectangle);
                self.tool_button(ui, "◯ Circle", ActiveTool::Ellipse);

                ui.separator();

                // Color palette presets
                self.color_swatch(ui, Color::BLACK, "Black");
                self.color_swatch(ui, Color::BLUE, "Blue");
                self.color_swatch(ui, Color::RED, "Red");
                self.color_swatch(ui, Color::GREEN, "Green");
                self.color_swatch(ui, Color::rgb(249, 115, 22), "Orange");
                self.color_swatch(ui, Color::rgb(147, 51, 234), "Purple");

                ui.separator();

                // Stroke thickness presets
                self.width_button(ui, "Fine", 1.0);
                self.width_button(ui, "Medium", 2.0);
                self.width_button(ui, "Thick", 4.0);

                ui.separator();

                // Undo and Redo quick actions
                if ui.add_enabled(self.history.can_undo(), egui::Button::new("⮌")).clicked() {
                    self.undo();
                }
                if ui.add_enabled(self.history.can_redo(), egui::Button::new("⮎")).clicked() {
                    self.redo();
                }

                // Dark Mode toggle button
                let dark_icon = if self.theme_mode == ThemeMode::Dark { "🌙 Dark" } else { "☀️ Light" };
                if ui.button(dark_icon).clicked() {
                    self.theme_mode = match self.theme_mode {
                        ThemeMode::Dark => ThemeMode::Light,
                        ThemeMode::Light => ThemeMode::Dark,
                    };
                }
            });
            ui.add_space(4.0);
        });
    }

    // Left sidebar for page management
    fn render_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("page_sidebar").resizable(false).default_width(140.0).show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("Pages");
            ui.label(format!("Page {} of {}", self.doc.current_page_index + 1, self.doc.pages.len()));

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("➕ Add").clicked() {
                    self.doc.add_page();
                }
                if ui.add_enabled(self.doc.pages.len() > 1, egui::Button::new("🗑 Del")).clicked() {
                    self.doc.remove_page(self.doc.current_page_index);
                }
            });

            ui.add_space(12.0);
            ui.separator();
            ui.label("Ruling Type:");
            if let Some(page) = self.doc.current_page_mut() {
                ui.radio_value(&mut page.background.bg_type, PageBackgroundType::Ruled, "Ruled");
                ui.radio_value(&mut page.background.bg_type, PageBackgroundType::Lined, "Lined");
                ui.radio_value(&mut page.background.bg_type, PageBackgroundType::Grid, "Grid");
                ui.radio_value(&mut page.background.bg_type, PageBackgroundType::Plain, "Plain");
                ui.radio_value(&mut page.background.bg_type, PageBackgroundType::Dotted, "Dotted");
            }

            ui.add_space(12.0);
            ui.separator();
            ui.label("Navigation:");
            for i in 0..self.doc.pages.len() {
                let selected = i == self.doc.current_page_index;
                if ui.selectable_label(selected, format!("Page {}", i + 1)).clicked() {
                    self.doc.current_page_index = i;
                }
            }
        });
    }

    // Bottom status bar
    fn render_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.0}%", self.viewport.zoom * 100.0));
                });
            });
        });
    }

    // Central drawing canvas
    fn render_central_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Theme::canvas_background(self.theme_mode)))
            .show(ctx, |ui| {
                let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

                // Handle canvas panning via right mouse button or drag
                if response.dragged_by(egui::PointerButton::Secondary) || response.dragged_by(egui::PointerButton::Middle) {
                    self.viewport.pan += response.drag_delta();
                }

                // Handle mouse scroll zooming
                let scroll = ui.input(|i| i.raw_scroll_delta.y);
                if scroll != 0.0 {
                    let zoom_factor = if scroll > 0.0 { 1.1 } else { 0.9 };
                    self.viewport.zoom = (self.viewport.zoom * zoom_factor).clamp(0.2, 5.0);
                }

                let current_page = match self.doc.current_page() {
                    Some(p) => p.clone(),
                    None => return,
                };

                let page_screen_rect = self.viewport.page_screen_rect(&current_page);

                // Handle pointer interaction on the paper
                if let Some(hover_pos) = response.hover_pos() {
                    let doc_pos = self.viewport.screen_to_doc(hover_pos);
                    let doc_point = Point::new(doc_pos.x, doc_pos.y, 1.0);

                    if response.drag_started_by(egui::PointerButton::Primary) && page_screen_rect.contains(hover_pos) {
                        self.on_pointer_down(doc_point);
                    } else if response.dragged_by(egui::PointerButton::Primary) {
                        self.on_pointer_move(doc_point);
                    } else if response.drag_stopped() {
                        self.on_pointer_up(doc_point);
                    }
                }

                // Render page, rulings, existing strokes, and live preview stroke
                let preview = self.pen_handler.current_stroke();
                CanvasRenderer::render_page(&painter, &current_page, &self.viewport, preview);
            });
    }

    // Dispatcher for pointer down events
    fn on_pointer_down(&mut self, point: Point) {
        match self.active_tool {
            ActiveTool::Pen => {
                self.pen_handler.is_highlighter = false;
                self.pen_handler.color = self.active_color;
                self.pen_handler.width = self.active_width;
                self.pen_handler.start(point);
            }
            ActiveTool::Highlighter => {
                self.pen_handler.is_highlighter = true;
                self.pen_handler.color = Color::rgba(self.active_color.r, self.active_color.g, self.active_color.b, 100);
                self.pen_handler.width = self.active_width * 3.5;
                self.pen_handler.start(point);
            }
            ActiveTool::Eraser => {
                if let Some(page) = self.doc.current_page_mut() {
                    let layer = page.active_layer_mut();
                    self.eraser_handler.erase_at(layer, &point);
                }
            }
            ActiveTool::Line | ActiveTool::Arrow | ActiveTool::Rectangle | ActiveTool::Ellipse => {
                self.shape_start = Some(point);
            }
            ActiveTool::Select => {}
        }
    }

    // Dispatcher for pointer drag events
    fn on_pointer_move(&mut self, point: Point) {
        match self.active_tool {
            ActiveTool::Pen | ActiveTool::Highlighter => {
                self.pen_handler.update(point);
            }
            ActiveTool::Eraser => {
                if let Some(page) = self.doc.current_page_mut() {
                    let layer = page.active_layer_mut();
                    self.eraser_handler.erase_at(layer, &point);
                }
            }
            _ => {}
        }
    }

    // Dispatcher for pointer release events
    fn on_pointer_up(&mut self, point: Point) {
        match self.active_tool {
            ActiveTool::Pen | ActiveTool::Highlighter => {
                if let Some(stroke) = self.pen_handler.finish() {
                    let page_idx = self.doc.current_page_index;
                    let layer_idx = self.doc.current_page().map(|p| p.active_layer_index).unwrap_or(0);
                    if let Some(page) = self.doc.current_page_mut() {
                        page.active_layer_mut().add_stroke(stroke.clone());
                        self.history.push(Action::AddStroke { page_idx, layer_idx, stroke });
                        self.doc.is_modified = true;
                    }
                }
            }
            ActiveTool::Line => {
                if let Some(start) = self.shape_start.take() {
                    let stroke = ShapeBuilder::build_line(start, point, self.active_color, self.active_width);
                    self.commit_shape(stroke);
                }
            }
            ActiveTool::Arrow => {
                if let Some(start) = self.shape_start.take() {
                    let stroke = ShapeBuilder::build_arrow(start, point, self.active_color, self.active_width);
                    self.commit_shape(stroke);
                }
            }
            ActiveTool::Rectangle => {
                if let Some(start) = self.shape_start.take() {
                    let stroke = ShapeBuilder::build_rectangle(start, point, self.active_color, self.active_width);
                    self.commit_shape(stroke);
                }
            }
            ActiveTool::Ellipse => {
                if let Some(start) = self.shape_start.take() {
                    let stroke = ShapeBuilder::build_ellipse(start, point, self.active_color, self.active_width);
                    self.commit_shape(stroke);
                }
            }
            _ => {}
        }
    }

    // Commit a geometric shape to the active page layer
    fn commit_shape(&mut self, stroke: Stroke) {
        let page_idx = self.doc.current_page_index;
        let layer_idx = self.doc.current_page().map(|p| p.active_layer_index).unwrap_or(0);
        if let Some(page) = self.doc.current_page_mut() {
            page.active_layer_mut().add_stroke(stroke.clone());
            self.history.push(Action::AddStroke { page_idx, layer_idx, stroke });
            self.doc.is_modified = true;
        }
    }

    // Undo the last modification
    fn undo(&mut self) {
        if let Some(action) = self.history.pop_undo() {
            match action {
                Action::AddStroke { page_idx, layer_idx, .. } => {
                    if let Some(page) = self.doc.pages.get_mut(page_idx) {
                        if let Some(layer) = page.layers.get_mut(layer_idx) {
                            layer.strokes.pop();
                        }
                    }
                }
                Action::RemoveStroke { page_idx, layer_idx, stroke, stroke_idx } => {
                    if let Some(page) = self.doc.pages.get_mut(page_idx) {
                        if let Some(layer) = page.layers.get_mut(layer_idx) {
                            if stroke_idx <= layer.strokes.len() {
                                layer.strokes.insert(stroke_idx, stroke);
                            }
                        }
                    }
                }
            }
        }
    }

    // Redo the last undone modification
    fn redo(&mut self) {
        if let Some(action) = self.history.pop_redo() {
            match action {
                Action::AddStroke { page_idx, layer_idx, stroke } => {
                    if let Some(page) = self.doc.pages.get_mut(page_idx) {
                        if let Some(layer) = page.layers.get_mut(layer_idx) {
                            layer.add_stroke(stroke);
                        }
                    }
                }
                Action::RemoveStroke { page_idx, layer_idx, stroke_idx, .. } => {
                    if let Some(page) = self.doc.pages.get_mut(page_idx) {
                        if let Some(layer) = page.layers.get_mut(layer_idx) {
                            if stroke_idx < layer.strokes.len() {
                                layer.strokes.remove(stroke_idx);
                            }
                        }
                    }
                }
            }
        }
    }

    // Display open file picker dialog
    fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Xournal++ Documents", &["xopp", "xoj"])
            .pick_file()
        {
            match load_xopp(&path) {
                Ok(doc) => {
                    self.doc = doc;
                    self.status_message = format!("Loaded: {}", path.display());
                }
                Err(err) => {
                    self.status_message = format!("Error loading file: {}", err);
                }
            }
        }
    }

    // Display save file picker dialog
    fn save_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Xournal++ Documents", &["xopp"])
            .save_file()
        {
            match save_xopp(&self.doc, &path) {
                Ok(_) => {
                    self.doc.is_modified = false;
                    self.status_message = format!("Saved to: {}", path.display());
                }
                Err(err) => {
                    self.status_message = format!("Error saving file: {}", err);
                }
            }
        }
    }

    // Helper widget for tool selection buttons
    fn tool_button(&mut self, ui: &mut egui::Ui, label: &str, tool: ActiveTool) {
        let is_selected = self.active_tool == tool;
        if ui.selectable_label(is_selected, label).clicked() {
            self.active_tool = tool;
        }
    }

    // Helper widget for palette color buttons
    fn color_swatch(&mut self, ui: &mut egui::Ui, color: Color, tooltip: &str) {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::click());
        let egui_col = Color32::from_rgb(color.r, color.g, color.b);
        let border_stroke = if self.active_color == color {
            EguiStroke::new(2.0, Color32::from_rgb(56, 189, 248))
        } else {
            EguiStroke::new(1.0, Color32::from_gray(120))
        };
        ui.painter().rect_filled(rect, 4.0, egui_col);
        ui.painter().rect_stroke(rect, 4.0, border_stroke);
        if response.on_hover_text(tooltip).clicked() {
            self.active_color = color;
        }
    }

    // Helper widget for stroke width preset buttons
    fn width_button(&mut self, ui: &mut egui::Ui, label: &str, width: f32) {
        let is_selected = (self.active_width - width).abs() < 0.1;
        if ui.selectable_label(is_selected, label).clicked() {
            self.active_width = width;
        }
    }
}
