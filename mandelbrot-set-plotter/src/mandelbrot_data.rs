use eframe::egui::Vec2;
use rayon::prelude::*;
use strum::{Display, EnumIter};

#[derive(PartialEq, Clone, Copy, EnumIter, Display)]
pub enum ColorMode {
    #[strum(serialize = "Trippy")]
    Trippy,
    #[strum(serialize = "Blue Green Red")]
    Bgr,
}

pub struct MandelbrotData {
    center_re: f64,
    center_im: f64,
    zoom: f64,
    color_mode: ColorMode,
}

impl MandelbrotData {
    pub fn new() -> Self {
        Self {
            center_re: -0.5,
            center_im: 0.0,
            zoom: 3.5,
            color_mode: ColorMode::Trippy,
        }
    }

    pub fn handle_drag(&mut self, width: f64, height: f64, delta: Vec2) {
        let aspect_ratio = width as f64 / height as f64;

        self.center_re -= (delta.x as f64 / width as f64) * self.zoom * aspect_ratio;
        self.center_im -= (delta.y as f64 / width as f64) * self.zoom;
    }

    pub fn handle_zoom(&mut self, scroll: f32) {
        self.zoom *= if scroll > 0.0 { 0.9 } else { 1.1 };
    }

    pub fn generate_pixel_buffer(&self, width: usize, height: usize) -> Vec<u8> {
        let mut pixels = vec![0_u8; width * height * 4];
        let max_iter = 255;

        pixels
            .par_chunks_exact_mut(4) // 4 because of RGBA
            .enumerate()
            .for_each(|(i, pixel)| {
                let px = i % width;
                let py = i / width;

                let (c_real, c_imaginary) =
                    self.screen_coord_to_complex_coord(px, py, width, height);
                let iter = MandelbrotData::compute_mandelbrot_iteration_for_pixel(
                    c_real,
                    c_imaginary,
                    max_iter,
                );

                (pixel[0], pixel[1], pixel[2]) = match self.color_mode {
                    ColorMode::Trippy => MandelbrotData::coloring_trippy(iter, max_iter),
                    ColorMode::Bgr => MandelbrotData::coloring_bgr(iter, max_iter),
                };

                pixel[3] = 255;
            });

        pixels
    }

    pub fn get_color_mode(&mut self) -> &mut ColorMode {
        &mut self.color_mode
    }
}

// =========== private helpers ===========
impl MandelbrotData {
    fn screen_coord_to_complex_coord(
        &self,
        px: usize,
        py: usize,
        width: usize,
        height: usize,
    ) -> (f64, f64) {
        let aspect_ratio = width as f64 / height as f64;

        // normalise to (-0.5 to 0.5) * zoom * ap + center
        let c_real = ((px as f64 / width as f64) - 0.5) * self.zoom * aspect_ratio + self.center_re; // used for x
        // normalise to (-0.5 to 0.5) * zoom + center
        let c_imaginary = ((py as f64 / height as f64) - 0.5) * self.zoom + self.center_im; // used for y

        (c_real, c_imaginary)
    }

    fn compute_mandelbrot_iteration_for_pixel(c_real: f64, c_imaginary: f64, max_iter: u32) -> u32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut iter = 0;

        // the points that stayed will be colored black
        // escaped points will have colors based on which iteration they escaped
        const ESCAPE_RADIUS: f64 = 4.0;
        while x * x + y * y <= ESCAPE_RADIUS && iter < max_iter {
            // z(new) = z(old)^2 + complex
            // z = x + yc
            // z(new) = (x^2 - y^2) + (2xyc)
            let x_new = x * x - y * y + c_real;
            y = 2.0 * x * y + c_imaginary;
            x = x_new;
            iter += 1;
        }
        iter
    }

    fn coloring_trippy(iter: u32, max_iter: u32) -> (u8, u8, u8) {
        if iter == max_iter {
            (0, 0, 0) // inside points are black
        } else {
            let t = iter as f64 * 0.1;

            let r = ((t + 0.0).sin() * 127.5 + 127.5) as u8;
            let g = ((t + 2.0).sin() * 127.5 + 127.5) as u8;
            let b = ((t + 4.0).sin() * 127.5 + 127.5) as u8;

            (r, g, b)
        }
    }

    fn coloring_bgr(iter: u32, max_iter: u32) -> (u8, u8, u8) {
        if iter == max_iter {
            (0, 0, 0) // inside points are black
        } else {
            // normalize the iteration count to a 0.0 -> 1.0 range
            let t = iter as f64 / max_iter as f64;

            // blue at start, green at medium depth, red at deepth
            let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
            let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
            let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;

            (r, g, b)
        }
    }
}
