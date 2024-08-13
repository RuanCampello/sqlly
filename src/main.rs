use sqlly::query::query::Query;

fn main() {
    let select_query = Query::select()
        .from("users")
        .select(&["id", "name", "email"])
        .at("age > 30")
        .at("active = true");

    print!("{}", select_query.build())
}
