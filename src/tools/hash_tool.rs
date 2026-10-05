use super::Tool;
use md5::{Digest as Md5Digest, Md5};
use sha2::{Sha256, Sha512};

#[derive(Default)]
pub struct HashTool {
    input: String,
    md5: String,
    sha256: String,
    sha512: String,
}

impl HashTool {
    fn compute(&mut self) {
        let bytes = self.input.as_bytes();
        self.md5 = hex::encode(Md5::digest(bytes));
        self.sha256 = hex::encode(Sha256::digest(bytes));
        self.sha512 = hex::encode(Sha512::digest(bytes));
    }
}

impl Tool for HashTool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hash (MD5 / SHA-256 / SHA-512)");
        ui.separator();

        let mut do_compute = false;
        ui.horizontal(|ui| {
            if ui.button("Compute").clicked() {
                do_compute = true;
            }
            if ui.button("Clear").clicked() {
                self.input.clear();
                self.md5.clear();
                self.sha256.clear();
                self.sha512.clear();
            }
        });
        ui.add_space(4.0);

        ui.label("Input:");
        egui::ScrollArea::vertical()
            .id_salt("hash_input")
            .max_height(160.0)
            .show(ui, |ui| {
                ui.add_sized(
                    [ui.available_width(), 160.0],
                    egui::TextEdit::multiline(&mut self.input)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            });

        if do_compute {
            self.compute();
        }

        ui.add_space(8.0);
        for (label, value) in [
            ("MD5", &self.md5),
            ("SHA-256", &self.sha256),
            ("SHA-512", &self.sha512),
        ] {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(label).strong().monospace());
                let mut v = value.clone();
                ui.add(
                    egui::TextEdit::singleline(&mut v)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
                if ui.button("Copy").clicked() {
                    ui.ctx().copy_text(value.clone());
                }
            });
        }
    }
}
