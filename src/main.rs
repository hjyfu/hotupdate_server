mod data;
mod gui;
use eframe::egui;



fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(520.0, 320.0)),
        ..Default::default()
    };

    let mut input_text = String::new();
    let mut ip_input = String::new();
    let mut selected_file_path = String::new();
    let mut selected_json_path = String::new();
    let mut json_content = String::new();
    let mut log_message = String::new();

    eframe::run_simple_native("hotupdate_server", options, move |ctx, _frame| {
        gui::render_ui(ctx, &mut ip_input, &mut input_text, &mut selected_file_path, &mut selected_json_path, &mut json_content, &mut log_message);
    })
}
