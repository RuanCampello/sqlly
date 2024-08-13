use sqlly::query::query::Query;

fn main() {
    let select_query = Query::select()
        .from("users")
        .select(&["id", "name", "email"])
        .where_condition("age > 30")
        .where_condition("active = true");

    print!("{}", select_query.build())
}
