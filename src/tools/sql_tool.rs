use super::{two_pane, Tool};
use sqlformat::{FormatOptions, Indent, QueryParams};
use sqlparser::dialect::{
    Dialect, GenericDialect, MsSqlDialect, MySqlDialect, PostgreSqlDialect,
};
use sqlparser::parser::Parser;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SqlDialect {
    MariaDb,
    MySql,
    PostgreSql,
    Oracle,
    MsSql,
    Generic,
}

impl SqlDialect {
    const ALL: &'static [SqlDialect] = &[
        SqlDialect::MariaDb,
        SqlDialect::MySql,
        SqlDialect::PostgreSql,
        SqlDialect::Oracle,
        SqlDialect::MsSql,
        SqlDialect::Generic,
    ];

    fn label(self) -> &'static str {
        match self {
            SqlDialect::MariaDb => "MariaDB",
            SqlDialect::MySql => "MySQL",
            SqlDialect::PostgreSql => "PostgreSQL",
            SqlDialect::Oracle => "Oracle",
            SqlDialect::MsSql => "MS SQL Server",
            SqlDialect::Generic => "Generic",
        }
    }

    fn build(self) -> Box<dyn Dialect> {
        match self {
            // sqlparser has no dedicated MariaDB dialect — it's a MySQL superset, use MySqlDialect.
            SqlDialect::MariaDb | SqlDialect::MySql => Box::new(MySqlDialect {}),
            SqlDialect::PostgreSql => Box::new(PostgreSqlDialect {}),
            // sqlparser has no Oracle dialect either — Generic is the safest fallback.
            SqlDialect::Oracle | SqlDialect::Generic => Box::new(GenericDialect {}),
            SqlDialect::MsSql => Box::new(MsSqlDialect {}),
        }
    }
}

pub struct SqlTool {
    input: String,
    output: String,
    uppercase: bool,
    indent: u8,
    dialect: SqlDialect,
    validate: bool,
}

impl Default for SqlTool {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            uppercase: true,
            indent: 2,
            dialect: SqlDialect::MariaDb,
            validate: true,
        }
    }
}

impl SqlTool {
    fn format(&mut self) {
        if self.validate {
            let dialect = self.dialect.build();
            if let Err(e) = Parser::parse_sql(&*dialect, &self.input) {
                self.output = format!(
                    "-- Parse error ({}): {}\n\n{}",
                    self.dialect.label(),
                    e,
                    self.format_pretty()
                );
                return;
            }
        }
        self.output = self.format_pretty();
    }

    fn format_pretty(&self) -> String {
        let opts = FormatOptions {
            indent: Indent::Spaces(self.indent),
            uppercase: Some(self.uppercase),
            lines_between_queries: 2,
            ignore_case_convert: None,
        };
        sqlformat::format(&self.input, &QueryParams::None, &opts)
    }
}

impl Tool for SqlTool {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut do_format = false;
        let SqlTool {
            input,
            output,
            uppercase,
            indent,
            dialect,
            validate,
        } = self;
        two_pane(ui, "SQL Formatter", input, output, |ui| {
            if ui.button("Format").clicked() {
                do_format = true;
            }
            ui.label("Dialect:");
            egui::ComboBox::from_id_salt("sql_dialect")
                .selected_text(dialect.label())
                .show_ui(ui, |ui| {
                    for d in SqlDialect::ALL {
                        ui.selectable_value(dialect, *d, d.label());
                    }
                });
            ui.checkbox(validate, "Validate");
            ui.checkbox(uppercase, "UPPERCASE keywords");
            ui.label("Indent:");
            ui.add(egui::DragValue::new(indent).range(1..=8));
        });
        if do_format {
            self.format();
        }
    }
}
