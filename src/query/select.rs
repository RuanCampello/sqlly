pub struct SelectQuery {
    table: Option<String>,
    conditions: Vec<String>,
    columns: Vec<String>,
}

impl SelectQuery {
    pub fn new() -> Self {
        SelectQuery {
            table: None,
            conditions: Vec::new(),
            columns: Vec::new(),
        }
    }
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }
    pub fn at(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }
    pub fn select(mut self, columns: &[&str]) -> Self {
        if columns.is_empty() {
            self.columns.push("*".to_string());
        } else {
            self.columns
                .extend(columns.iter().map(|&col| col.to_string()));
        }
        self
    }

    pub fn build(self) -> String {
        let mut query = String::new();

        if let Some(table) = self.table {
            query.push_str(&format!(
                "SELECT {} FROM {}",
                self.columns.join(", "),
                table
            ));
        } else {
            query.push_str("SELECT ");
            query.push_str(&self.columns.join(","));
        }
        if !self.conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        query.push_str(";");
        query
    }
}
