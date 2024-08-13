use sqlly::query::query::Query;
mod utils;
use utils::is_valid_sql;

#[test]
fn test_select_query() {
    let select_query = Query::select()
        .from("users")
        .select(&["id", "name", "email"])
        .at("age > 30")
        .at("active = true");

    assert!(is_valid_sql(&select_query.build()));
}
