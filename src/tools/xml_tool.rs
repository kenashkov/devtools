use super::{two_pane, Tool};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::io::Cursor;

#[derive(Default)]
pub struct XmlTool {
    input: String,
    output: String,
}

impl XmlTool {
    fn format(&mut self) {
        let mut reader = Reader::from_str(&self.input);
        reader.config_mut().trim_text(true);

        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(e) => {
                    if let Err(err) = writer.write_event(e) {
                        self.output = format!("Write error: {err}");
                        return;
                    }
                }
                Err(e) => {
                    self.output = format!("Parse error at {}: {e}", reader.buffer_position());
                    return;
                }
            }
        }
        let bytes = writer.into_inner().into_inner();
        self.output = String::from_utf8(bytes).unwrap_or_else(|_| "Invalid UTF-8".into());
    }

    fn minify(&mut self) {
        let mut reader = Reader::from_str(&self.input);
        reader.config_mut().trim_text(true);
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(e) => {
                    if let Err(err) = writer.write_event(e) {
                        self.output = format!("Write error: {err}");
                        return;
                    }
                }
                Err(e) => {
                    self.output = format!("Parse error at {}: {e}", reader.buffer_position());
                    return;
                }
            }
        }
        let bytes = writer.into_inner().into_inner();
        self.output = String::from_utf8(bytes).unwrap_or_else(|_| "Invalid UTF-8".into());
    }
}

impl Tool for XmlTool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut do_format = false;
        let mut do_minify = false;
        let XmlTool { input, output } = self;
        two_pane(ui, "XML Formatter", input, output, |ui| {
            if ui.button("Format").clicked() {
                do_format = true;
            }
            if ui.button("Minify").clicked() {
                do_minify = true;
            }
        });
        if do_format {
            self.format();
        }
        if do_minify {
            self.minify();
        }
    }
}
