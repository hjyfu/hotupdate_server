use eframe::egui;
use super::data::{Pak, Data};
use native_dialog::FileDialog;
use std::fs;
use std::path::Path;
use std::fs::copy;
use std::process::Command; // 引入Command
use std::fs::write; // 引入write函数

pub fn render_ui(
    ctx: &egui::Context,
    ip_input: &mut String,
    port_input: &mut String,
    input_text: &mut String,
    selected_file_path: &mut String,
    selected_json_path: &mut String,

    json_content: &mut String,
    log_message: &mut String
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {

            ui.vertical(|ui| {
                // Left side of the UI
                ui.heading("Hot Update Server");
                // 新增的UI组件：Start Server按钮
                if ui.button("Start Server").clicked() {
                    // 启动同一目录下的main.exe
                    if let Err(err) = Command::new("./server/main.exe").spawn() {
                        eprintln!("Failed to start main.exe: {:?}", err);
                    }
                }
                ui.label("port: ");
                ui.text_edit_singleline(port_input);
                if ui.button("Save Port Setting").clicked() && !port_input.is_empty() {
                    let setting_path = "./server/setting.ini";
                    if let Err(e) = write(setting_path, format!("port={}", port_input)) {
                        eprintln!("Error writing to setting.ini: {:?}", e);
                    }
                }


                ui.group(|ui| {
                    ui.label("Enter IP Address:");
                    ui.text_edit_singleline(ip_input);

                    ui.label("Enter the version number:");
                    ui.text_edit_singleline(input_text);



                    if ui.button("Choose File").clicked() {
                        let result = FileDialog::new()
                            .add_filter("All Files", &["*"])
                            .show_open_single_file()
                            .unwrap();

                        if let Some(path) = result {
                            *selected_file_path = path.to_string_lossy().into_owned();
                        }
                    }
                    ui.text_edit_singleline(selected_file_path);
                    if ui.button("Select JSON").clicked() {
                        let result = FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .show_open_single_file()
                            .unwrap();

                        if let Some(path) = result {
                            *selected_json_path = path.to_string_lossy().into_owned();
                            *json_content = fs::read_to_string(&path).unwrap_or_else(|_| "".to_string());
                        }
                    }
                    ui.text_edit_singleline(selected_json_path);


                    if ui.button("Confirm").clicked() && !selected_file_path.is_empty() && !selected_json_path.is_empty() {

                        let path = Path::new(&selected_json_path);

                        let parent_folder = path.parent().unwrap().file_stem().unwrap().to_string_lossy();

                        let file_name = Path::new(&selected_file_path).file_name().unwrap().to_string_lossy().to_string();

                        // Copy file to the selected JSON's directory
                        let destination = format!("{}/{}", path.parent().unwrap().display(), &file_name);
                        if let Err(e) = copy(&selected_file_path, &destination) {
                            eprintln!("Error copying file: {:?}", e);
                        }

                        let new_pak = Pak {
                            name: file_name.clone(),
                            version: input_text.clone(),
                            url: format!("http://{}:8000/{}/{}", ip_input, parent_folder, file_name),
                        };

                        let mut data = match fs::read_to_string(&selected_json_path) {
                            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                            Err(_) => Data { pak: vec![] },
                        };

                        data.pak.push(new_pak);
                        let serialized_data = serde_json::to_string(&data).unwrap_or_default();
                        if let Err(e) = fs::write(&selected_json_path, &serialized_data) {
                            eprintln!("Error writing to file: {:?}", e);
                        }
                        // 重新读取JSON的内容
                        *json_content = fs::read_to_string(&selected_json_path).unwrap_or_else(|_| "".to_string());

                        // 清空所有输入框的内容
                        input_text.clear();
                        ip_input.clear();
                        selected_file_path.clear();
                        selected_json_path.clear();
                        *log_message = format!("{} has been added to the JSON file.", file_name);
                    }
                });
                ui.group(|ui| {
                    ui.label("Log:");
                    ui.monospace(log_message.as_str());
                });

            });
            ui.group(|ui| {
                ui.label("JSON Content:");
                ui.vertical(|ui| {
                    ui.monospace(json_content.as_str());
                });
            });
        });
    });
}
