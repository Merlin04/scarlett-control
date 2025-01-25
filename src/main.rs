fn main() -> eframe::Result {
    env_logger::init();

    let native_opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "scarlett-control",
        native_opts, 
        Box::new(|cc| Ok(Box::new(scarlett_control::ScarlettControlApp::new(cc))))
    )
}
