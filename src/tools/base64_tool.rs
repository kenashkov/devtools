use super::{two_pane, Tool};
use base64::{engine::general_purpose, Engine as _};

pub struct Base64Tool {
    input: String,
    output: String,
    url_safe: bool,
}

impl Default for Base64Tool {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            url_safe: false,
        }
    }
}

impl Base64Tool {
    fn encode(&mut self) {
        self.output = if self.url_safe {
            general_purpose::URL_SAFE.encode(self.input.as_bytes())
        } else {
            general_purpose::STANDARD.encode(self.input.as_bytes())
        };
    }

    fn decode(&mut self) {
        let trimmed = self.input.trim();
        let result = if self.url_safe {
            general_purpose::URL_SAFE.decode(trimmed.as_bytes())
        } else {
            general_purpose::STANDARD.decode(trimmed.as_bytes())
        };
        self.output = match result {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(e) => format!("Decoded but not UTF-8: {e}"),
            },
            Err(e) => format!("Decode error: {e}"),
        };
    }
}

impl Tool for Base64Tool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut do_encode = false;
        let mut do_decode = false;
        let Base64Tool {
            input,
            output,
            url_safe,
        } = self;
        two_pane(ui, "Base64", input, output, |ui| {
            if ui.button("Encode").clicked() {
                do_encode = true;
            }
            if ui.button("Decode").clicked() {
                do_decode = true;
            }
            ui.checkbox(url_safe, "URL-safe");
        });
        if do_encode {
            self.encode();
        }
        if do_decode {
            self.decode();
        }
    }
}
