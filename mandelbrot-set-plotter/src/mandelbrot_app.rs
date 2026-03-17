use eframe::egui;

use crate::mandelbrot_data::MandelbrotData;
use crate::mandelbrot_view::MandelbrotView;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

pub struct MandelbrotApp {
    data: MandelbrotData,
    view: MandelbrotView,
    last_width: usize,
    last_height: usize,
}

impl MandelbrotApp {
    pub fn new() -> Self {
        Self {
            data: MandelbrotData::new(),
            view: MandelbrotView::new(),
            last_width: 0,
            last_height: 0,
        }
    }
}

impl MandelbrotApp {
    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([SCREEN_WIDTH, SCREEN_HEIGHT])
                .with_title("Mandelbrot Set Plotter"),
            ..Default::default()
        };

        eframe::run_native(
            "Mandelbrot Set Plotter",
            options,
            Box::new(|_cc| Ok(Box::new(Self::new()))),
        )
    }
}

// =========== private helpers ===========
impl eframe::App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let width = size.x as usize;
            let height = size.y as usize;

            let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
            let mut update = self.view.needs_update();

            let time = ctx.input(|i| i.time);
            if self.data.is_animated() {
                update = true;
                ctx.request_repaint();
            }

            if self.last_width != width || self.last_height != height {
                update = true;
                self.last_width = width;
                self.last_height = height;
            }

            if response.dragged() {
                let delta = response.drag_delta();
                self.data.handle_drag(width as f64, height as f64, delta);
                update = true;
            }

            let scroll = ctx.input(|i| i.smooth_scroll_delta.y);

            if scroll != 0.0 {
                self.data.handle_zoom(scroll);
                update = true;
            }

            if self
                .view
                .draw_color_settings_changed(ctx, self.data.get_color_mode())
            {
                update = true;
            }

            if update {
                self.view.update_texture(
                    width,
                    height,
                    &self.data.generate_pixel_buffer(width, height, time),
                    ctx,
                );
            }

            self.view.draw(ui, &rect);
        });
    }
}
