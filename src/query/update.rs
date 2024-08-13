pub struct UpdateQuery {
    table: Option<String>,
    updates: Vec<String>,
    conditions: Vec<String>,
}

impl UpdateQuery {
    pub fn new() -> Self {
        UpdateQuery {
            table: None,
            updates: Vec::new(),
            conditions: Vec::new(),
        }
    }
    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }
    pub fn set(mut self, column: &str, value: &str) -> Self {
        self.updates.push(format!("{} = {}", column, value));
        self
    }
    pub fn at(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn build(self) -> String {
        let mut query = String::new();

        if let Some(table) = self.table {
            query.push_str(&format!("UPDATE {} SET {}", table, self.updates.join(", ")))
        } else {
            panic!("Table name must be set")
        }
        if !self.conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")))
        }
        query.push_str(";");
        query
    }
}
