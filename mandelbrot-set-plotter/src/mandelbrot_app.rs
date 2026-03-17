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
    last_gen_time: f64,
}

impl MandelbrotApp {
    pub fn new() -> Self {
        Self {
            data: MandelbrotData::new(),
            view: MandelbrotView::new(),
            last_width: 0,
            last_height: 0,
            last_gen_time: 0_f64,
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
                let start_time = std::time::Instant::now();
                let buffer = self.data.generate_pixel_buffer(width, height, time);
                self.last_gen_time = start_time.elapsed().as_secs_f64() * 1000.0;
                self.view.update_texture(width, height, &buffer, ctx);
            }

            self.view.draw(ui, &rect);
            self.draw_profiling(ui);
        });
    }
}

impl MandelbrotApp {
    fn draw_profiling(&self, ui: &mut egui::Ui) -> egui::Response {
        const TEXT_WIDTH: f32 = 200.0;
        const TEXT_HEIGHT: f32 = 50.0;

        ui.put(
            // Position text at the top-left corner with a small margin
            egui::Rect::from_min_size(
                egui::pos2(
                    self.last_width as f32 - TEXT_WIDTH,
                    self.last_height as f32 - TEXT_HEIGHT,
                ),
                egui::vec2(TEXT_WIDTH, TEXT_HEIGHT),
            ),
            egui::Label::new(
                egui::RichText::new(format!(
                    "Render Time: {:.2} ms\nResolution: {}x{}",
                    self.last_gen_time, self.last_width, self.last_height
                ))
                .color(egui::Color32::WHITE)
                .background_color(egui::Color32::from_black_alpha(150))
                .heading(),
            ),
        )
    }
}
