use super::Tool;
use base64::{engine::general_purpose, Engine as _};

#[derive(Default)]
pub struct JwtTool {
    input: String,
    header: String,
    payload: String,
    signature: String,
    error: Option<String>,
}

impl JwtTool {
    fn decode(&mut self) {
        self.error = None;
        self.header.clear();
        self.payload.clear();
        self.signature.clear();

        let token = self.input.trim();
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            self.error = Some(format!("Expected 3 segments, got {}", parts.len()));
            return;
        }

        match decode_segment(parts[0]) {
            Ok(s) => self.header = pretty_json(&s),
            Err(e) => {
                self.error = Some(format!("Header: {e}"));
                return;
            }
        }
        match decode_segment(parts[1]) {
            Ok(s) => self.payload = pretty_json(&s),
            Err(e) => {
                self.error = Some(format!("Payload: {e}"));
                return;
            }
        }
        self.signature = parts[2].to_owned();
    }
}

fn decode_segment(seg: &str) -> Result<String, String> {
    let bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(seg.as_bytes())
        .map_err(|e| e.to_string())?;
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

fn pretty_json(s: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(s) {
        Ok(v) => serde_json::to_string_pretty(&v).unwrap_or_else(|_| s.to_owned()),
        Err(_) => s.to_owned(),
    }
}

impl Tool for JwtTool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("JWT Decoder");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Decode").clicked() {
                self.decode();
            }
            if ui.button("Clear").clicked() {
                self.input.clear();
                self.header.clear();
                self.payload.clear();
                self.signature.clear();
                self.error = None;
            }
        });
        ui.add_space(4.0);

        ui.label("Token:");
        egui::ScrollArea::vertical()
            .id_salt("jwt_input")
            .max_height(100.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.input)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_width(f32::INFINITY)
                        .desired_rows(3),
                );
            });

        if let Some(err) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, err);
        }

        ui.add_space(8.0);
        ui.columns(2, |cols| {
            cols[0].label("Header");
            cols[0].add(
                egui::TextEdit::multiline(&mut self.header.clone())
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(8),
            );
            cols[1].label("Payload");
            cols[1].add(
                egui::TextEdit::multiline(&mut self.payload.clone())
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(8),
            );
        });

        ui.add_space(4.0);
        ui.label("Signature");
        ui.add(
            egui::TextEdit::singleline(&mut self.signature.clone())
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY),
        );
    }
}
