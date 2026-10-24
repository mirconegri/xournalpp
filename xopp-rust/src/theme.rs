use egui::{Color32, Visuals};

// Application theme settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    // Clean modern light palette
    Light,
    // Pure AMOLED black canvas dark palette
    Dark,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Dark
    }
}

// Configures modern visual aesthetics for egui
pub struct Theme;

impl Theme {
    // Generate an egui Visuals style matching the theme mode
    pub fn apply(mode: ThemeMode, ctx: &egui::Context) {
        let mut visuals = match mode {
            ThemeMode::Dark => Visuals::dark(),
            ThemeMode::Light => Visuals::light(),
        };

        // Modern rounded button geometry
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
        visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        visuals.widgets.active.rounding = egui::Rounding::same(8.0);

        match mode {
            ThemeMode::Dark => {
                visuals.panel_fill = Color32::from_rgb(24, 24, 27);
                visuals.window_fill = Color32::from_rgb(28, 28, 35);
                visuals.extreme_bg_color = Color32::BLACK;
            }
            ThemeMode::Light => {
                visuals.panel_fill = Color32::from_rgb(255, 255, 255);
                visuals.window_fill = Color32::from_rgb(248, 250, 252);
                visuals.extreme_bg_color = Color32::from_rgb(241, 245, 249);
            }
        }

        ctx.set_visuals(visuals);
    }

    // Canvas surrounding background color
    pub fn canvas_background(mode: ThemeMode) -> Color32 {
        match mode {
            ThemeMode::Dark => Color32::BLACK,
            ThemeMode::Light => Color32::from_rgb(226, 232, 240),
        }
    }
}
