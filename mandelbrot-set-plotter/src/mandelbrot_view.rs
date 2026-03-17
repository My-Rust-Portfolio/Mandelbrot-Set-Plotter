use eframe::egui;

pub struct MandelbrotView {
    texture: Option<egui::TextureHandle>,
}

impl MandelbrotView {
    pub fn new() -> Self {
        Self { texture: None }
    }

    pub fn is_texture_none(&self) -> bool {
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
}
