use crate::mandelbrot_data::ColorMode;
use eframe::egui;
use strum::IntoEnumIterator;

pub struct MandelbrotView {
    texture: Option<egui::TextureHandle>,
}

impl MandelbrotView {
    pub fn new() -> Self {
        Self { texture: None }
    }

    pub fn needs_update(&self) -> bool {
        self.texture.is_none()
    }

    pub fn update_texture(
        &mut self,
        width: usize,
        height: usize,
        data: &[u8],
        ctx: &egui::Context,
    ) {
        let image = egui::ColorImage::from_rgba_unmultiplied([width, height], &data);

        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::LINEAR);
        } else {
            self.texture =
                Some(ctx.load_texture("mandelbrot", image, egui::TextureOptions::LINEAR));
        }
    }

    pub fn draw(&self, ui: &egui::Ui, rect: &egui::Rect) {
        if let Some(texture) = &self.texture {
            ui.painter().image(
                texture.id(),
                *rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }

    // draws settings, returns true if color option is changed
    pub fn draw_color_settings_changed(
        &self,
        ctx: &egui::Context,
        current_mode: &mut ColorMode,
    ) -> bool {
        let mut settings_changed = false;

        egui::Window::new("Color Settings")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Color Palette");
                ui.separator();

                for mode in ColorMode::iter() {
                    if ui
                        .radio_value(current_mode, mode, mode.to_string())
                        .changed()
                    {
                        settings_changed = true;
                    }
                }
            });

        settings_changed
    }
}
