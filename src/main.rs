#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod tools;

use app::DevToolsApp;

const ICON_SVG: &str = include_str!("../assets/icon.svg");
const ICON_SIZE: u32 = 256;

fn load_icon() -> Option<egui::IconData> {
    use resvg::{tiny_skia, usvg};

    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(ICON_SVG, &opt).ok()?;
    let svg_size = tree.size();
    let scale = ICON_SIZE as f32 / svg_size.width().max(svg_size.height());

    let mut pixmap = tiny_skia::Pixmap::new(ICON_SIZE, ICON_SIZE)?;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    Some(egui::IconData {
        rgba: pixmap.data().to_vec(),
        width: ICON_SIZE,
        height: ICON_SIZE,
    })
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1100.0, 700.0])
        .with_min_inner_size([700.0, 450.0])
        .with_title("DevTools");

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "DevTools",
        options,
        Box::new(|cc| Ok(Box::new(DevToolsApp::new(cc)))),
    )
}
