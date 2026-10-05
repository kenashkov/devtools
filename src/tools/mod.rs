pub mod base64_tool;
pub mod hash_tool;
pub mod json_tool;
pub mod jwt_tool;
pub mod sql_tool;
pub mod url_tool;
pub mod uuid_tool;
pub mod xml_tool;

pub trait Tool {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub fn two_pane(
    ui: &mut egui::Ui,
    title: &str,
    input: &mut String,
    output: &str,
    on_action: impl FnOnce(&mut egui::Ui),
) {
    ui.heading(title);
    ui.separator();

    egui::TopBottomPanel::top("toolbar")
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                on_action(ui);
                if ui.button("Clear").clicked() {
                    input.clear();
                }
                if ui.button("Copy output").clicked() {
                    ui.ctx().copy_text(output.to_owned());
                }
            });
            ui.add_space(4.0);
        });

    let avail = ui.available_size();
    let half = avail.x / 2.0 - 4.0;

    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(half, avail.y),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.label("Input");
                egui::ScrollArea::both()
                    .id_salt("input_scroll")
                    .show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(input)
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .desired_width(f32::INFINITY),
                        );
                    });
            },
        );

        ui.separator();

        ui.allocate_ui_with_layout(
            egui::vec2(half, avail.y),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.label("Output");
                egui::ScrollArea::both()
                    .id_salt("output_scroll")
                    .show(ui, |ui| {
                        let mut out = output.to_owned();
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut out)
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .desired_width(f32::INFINITY)
                                .interactive(true),
                        );
                    });
            },
        );
    });
}
