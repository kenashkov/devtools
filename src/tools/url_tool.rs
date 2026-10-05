use super::{two_pane, Tool};

#[derive(Default)]
pub struct UrlTool {
    input: String,
    output: String,
}

impl Tool for UrlTool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut do_encode = false;
        let mut do_decode = false;
        let UrlTool { input, output } = self;
        two_pane(ui, "URL Encode/Decode", input, output, |ui| {
            if ui.button("Encode").clicked() {
                do_encode = true;
            }
            if ui.button("Decode").clicked() {
                do_decode = true;
            }
        });
        if do_encode {
            self.output = urlencoding::encode(&self.input).into_owned();
        }
        if do_decode {
            self.output = match urlencoding::decode(&self.input) {
                Ok(s) => s.into_owned(),
                Err(e) => format!("Decode error: {e}"),
            };
        }
    }
}
