mod mandelbrot_app;
mod mandelbrot_data;
mod mandelbrot_view;
use mandelbrot_app::MandelbrotApp;

fn main() -> eframe::Result<()> {
    MandelbrotApp::run()
}
