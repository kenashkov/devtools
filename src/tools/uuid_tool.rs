use super::Tool;
use uuid::{Uuid, NoContext, Timestamp};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum UuidVersion {
    Nil,
    V1,
    V3,
    V4,
    V5,
    V6,
    V7,
    Max,
}

impl UuidVersion {
    const ALL: &'static [UuidVersion] = &[
        UuidVersion::Nil,
        UuidVersion::V1,
        UuidVersion::V3,
        UuidVersion::V4,
        UuidVersion::V5,
        UuidVersion::V6,
        UuidVersion::V7,
        UuidVersion::Max,
    ];

    fn label(self) -> &'static str {
        match self {
            UuidVersion::Nil => "Nil (all-zero)",
            UuidVersion::V1 => "v1 (time + MAC)",
            UuidVersion::V3 => "v3 (MD5 of name)",
            UuidVersion::V4 => "v4 (random)",
            UuidVersion::V5 => "v5 (SHA-1 of name)",
            UuidVersion::V6 => "v6 (reordered time)",
            UuidVersion::V7 => "v7 (Unix ts + random)",
            UuidVersion::Max => "Max (all-FF)",
        }
    }

    fn requires_name(self) -> bool {
        matches!(self, UuidVersion::V3 | UuidVersion::V5)
    }

    fn supports_count(self) -> bool {
        !matches!(self, UuidVersion::Nil | UuidVersion::Max)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum NamespacePreset {
    Dns,
    Url,
    Oid,
    X500,
}

impl NamespacePreset {
    const ALL: &'static [NamespacePreset] = &[
        NamespacePreset::Dns,
        NamespacePreset::Url,
        NamespacePreset::Oid,
        NamespacePreset::X500,
    ];

    fn label(self) -> &'static str {
        match self {
            NamespacePreset::Dns => "DNS",
            NamespacePreset::Url => "URL",
            NamespacePreset::Oid => "OID",
            NamespacePreset::X500 => "X.500",
        }
    }

    fn uuid(self) -> Uuid {
        match self {
            NamespacePreset::Dns => Uuid::NAMESPACE_DNS,
            NamespacePreset::Url => Uuid::NAMESPACE_URL,
            NamespacePreset::Oid => Uuid::NAMESPACE_OID,
            NamespacePreset::X500 => Uuid::NAMESPACE_X500,
        }
    }
}

pub struct UuidTool {
    version: UuidVersion,
    count: u32,
    namespace: NamespacePreset,
    name: String,
    output: String,
    error: Option<String>,
}

impl Default for UuidTool {
    fn default() -> Self {
        Self {
            version: UuidVersion::V4,
            count: 1,
            namespace: NamespacePreset::Dns,
            name: String::new(),
            output: String::new(),
            error: None,
        }
    }
}

impl UuidTool {
    fn generate(&mut self) {
        self.error = None;
        let mut buf = String::new();

        // v1/v6 need a fake "node id" (MAC address). Generate one per call.
        let node = {
            let mut rand = [0u8; 6];
            getrandom_bytes(&mut rand);
            // Set the locally-administered + multicast bits to mark it as not-real-MAC.
            rand[0] |= 0x02;
            rand
        };

        let n = if self.version.supports_count() {
            self.count.max(1)
        } else {
            1
        };

        for _ in 0..n {
            let id = match self.version {
                UuidVersion::Nil => Uuid::nil(),
                UuidVersion::Max => Uuid::max(),
                UuidVersion::V1 => Uuid::new_v1(Timestamp::now(NoContext), &node),
                UuidVersion::V3 => {
                    if self.name.is_empty() {
                        self.error = Some("v3 requires a name".into());
                        return;
                    }
                    Uuid::new_v3(&self.namespace.uuid(), self.name.as_bytes())
                }
                UuidVersion::V4 => Uuid::new_v4(),
                UuidVersion::V5 => {
                    if self.name.is_empty() {
                        self.error = Some("v5 requires a name".into());
                        return;
                    }
                    Uuid::new_v5(&self.namespace.uuid(), self.name.as_bytes())
                }
                UuidVersion::V6 => Uuid::new_v6(Timestamp::now(NoContext), &node),
                UuidVersion::V7 => Uuid::now_v7(),
            };
            buf.push_str(&id.to_string());
            buf.push('\n');
        }
        self.output = buf;
    }
}

fn getrandom_bytes(buf: &mut [u8]) {
    // Reuse uuid's RNG indirectly: a v4 UUID gives us 16 random bytes, of which
    // we use the first 6. Avoids adding `rand` or `getrandom` as a direct dep.
    let bytes = *Uuid::new_v4().as_bytes();
    let n = buf.len().min(bytes.len());
    buf[..n].copy_from_slice(&bytes[..n]);
}

impl Tool for UuidTool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("UUID Generator");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Version:");
            egui::ComboBox::from_id_salt("uuid_version")
                .selected_text(self.version.label())
                .show_ui(ui, |ui| {
                    for v in UuidVersion::ALL {
                        ui.selectable_value(&mut self.version, *v, v.label());
                    }
                });

            if self.version.supports_count() {
                ui.separator();
                ui.label("Count:");
                ui.add(egui::DragValue::new(&mut self.count).range(1..=1000));
            }

            if ui.button("Generate").clicked() {
                self.generate();
            }
            if ui.button("Copy").clicked() {
                ui.ctx().copy_text(self.output.clone());
            }
            if ui.button("Clear").clicked() {
                self.output.clear();
                self.error = None;
            }
        });

        if self.version.requires_name() {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Namespace:");
                egui::ComboBox::from_id_salt("uuid_namespace")
                    .selected_text(self.namespace.label())
                    .show_ui(ui, |ui| {
                        for ns in NamespacePreset::ALL {
                            ui.selectable_value(&mut self.namespace, *ns, ns.label());
                        }
                    });
                ui.label("Name:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.name)
                        .hint_text("e.g. example.com"),
                );
            });
        }

        if let Some(err) = &self.error {
            ui.add_space(4.0);
            ui.colored_label(egui::Color32::LIGHT_RED, err);
        }

        ui.add_space(8.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.output.clone())
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY),
            );
        });
    }
}
