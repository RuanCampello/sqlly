use sqlly::query::query::Query;
mod utils;
use utils::is_valid_sql;

#[test]
fn test_update_query() {
    let update_query = Query::update()
        .table("contries")
        .at("id = 1")
        .set("name", "first_contry");

    assert!(is_valid_sql(&update_query.build()));
}
