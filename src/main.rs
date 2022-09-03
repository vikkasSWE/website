#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "website",
        native_options,
        Box::new(|cc| Box::new(website::App::new(cc))),
    );
}

// When compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id",
        web_options,
        Box::new(|cc| Box::new(website::App::new(cc))),
    )
    .expect("failed to start eframe");
}
