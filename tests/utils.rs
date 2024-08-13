use sqlparser::{dialect::GenericDialect, parser::Parser};

pub fn is_valid_sql(query: &str) -> bool {
    let dialect = GenericDialect {};
    Parser::parse_sql(&dialect, query).is_ok()
}
