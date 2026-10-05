use crate::tools::{
    base64_tool::Base64Tool, hash_tool::HashTool, json_tool::JsonTool, jwt_tool::JwtTool,
    sql_tool::SqlTool, url_tool::UrlTool, uuid_tool::UuidTool, xml_tool::XmlTool, Tool,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ToolKind {
    Json,
    Xml,
    Sql,
    Base64,
    Jwt,
    Url,
    Uuid,
    Hash,
}

impl ToolKind {
    const ALL: &'static [ToolKind] = &[
        ToolKind::Json,
        ToolKind::Xml,
        ToolKind::Sql,
        ToolKind::Base64,
        ToolKind::Jwt,
        ToolKind::Url,
        ToolKind::Uuid,
        ToolKind::Hash,
    ];

    fn label(self) -> &'static str {
        match self {
            ToolKind::Json => "JSON Formatter",
            ToolKind::Xml => "XML Formatter",
            ToolKind::Sql => "SQL Formatter",
            ToolKind::Base64 => "Base64",
            ToolKind::Jwt => "JWT Decoder",
            ToolKind::Url => "URL Encode/Decode",
            ToolKind::Uuid => "UUID Generator",
            ToolKind::Hash => "Hash (SHA/MD5)",
        }
    }
}

pub struct DevToolsApp {
    selected: ToolKind,
    json: JsonTool,
    xml: XmlTool,
    sql: SqlTool,
    base64: Base64Tool,
    jwt: JwtTool,
    url: UrlTool,
    uuid: UuidTool,
    hash: HashTool,
}

impl DevToolsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.all_styles_mut(|style| {
            for (_text_style, font_id) in style.text_styles.iter_mut() {
                font_id.size *= 1.20;
            }
        });
        Self {
            selected: ToolKind::Json,
            json: JsonTool::default(),
            xml: XmlTool::default(),
            sql: SqlTool::default(),
            base64: Base64Tool::default(),
            jwt: JwtTool::default(),
            url: UrlTool::default(),
            uuid: UuidTool::default(),
            hash: HashTool::default(),
        }
    }
}

impl eframe::App for DevToolsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(180.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading("DevTools");
                ui.separator();
                for kind in ToolKind::ALL {
                    let selected = self.selected == *kind;
                    if ui.selectable_label(selected, kind.label()).clicked() {
                        self.selected = *kind;
                    }
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("v0.1.0").weak().small());
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.selected {
            ToolKind::Json => self.json.ui(ui),
            ToolKind::Xml => self.xml.ui(ui),
            ToolKind::Sql => self.sql.ui(ui),
            ToolKind::Base64 => self.base64.ui(ui),
            ToolKind::Jwt => self.jwt.ui(ui),
            ToolKind::Url => self.url.ui(ui),
            ToolKind::Uuid => self.uuid.ui(ui),
            ToolKind::Hash => self.hash.ui(ui),
        });
    }
}
