use super::{two_pane, Tool};

#[derive(Default)]
pub struct JsonTool {
    input: String,
    output: String,
    indent: u8,
}

impl JsonTool {
    fn format(&mut self) {
        let indent = if self.indent == 0 { 2 } else { self.indent };
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(v) => {
                let indent_bytes = vec![b' '; indent as usize];
                let mut buf = Vec::new();
                let fmt = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
                let mut ser = serde_json::Serializer::with_formatter(&mut buf, fmt);
                if serde::Serialize::serialize(&v, &mut ser).is_ok() {
                    self.output = String::from_utf8(buf).unwrap_or_default();
                } else {
                    self.output = "Serialization failed".into();
                }
            }
            Err(e) => self.output = format!("Parse error: {e}"),
        }
    }

    fn minify(&mut self) {
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(v) => self.output = v.to_string(),
            Err(e) => self.output = format!("Parse error: {e}"),
        }
    }
}

impl Tool for JsonTool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        if self.indent == 0 {
            self.indent = 2;
        }
        let mut do_format = false;
        let mut do_minify = false;
        // Borrow trickery: capture flags inside closure, run after.
        let title = "JSON Formatter";
        let JsonTool {
            input,
            output,
            indent,
        } = self;

        two_pane(ui, title, input, output, |ui| {
            if ui.button("Format").clicked() {
                do_format = true;
            }
            if ui.button("Minify").clicked() {
                do_minify = true;
            }
            ui.label("Indent:");
            ui.add(egui::DragValue::new(indent).range(1..=8));
        });

        if do_format {
            self.format();
        }
        if do_minify {
            self.minify();
        }
    }
}
