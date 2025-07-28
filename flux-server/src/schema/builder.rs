pub struct SchemaBuilder {
    pub tables: Vec<TableSchema>,
}

#[derive(Clone, Debug)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
}

#[derive(Clone, Debug)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: String,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        SchemaBuilder { tables: Vec::new() }
    }

    pub fn add_table(&mut self, name: &str) -> &mut Self {
        self.tables.push(TableSchema {
            name: name.to_string(),
            columns: Vec::new(),
        });
        self
    }

    pub fn add_column(
        &mut self,
        table_name: &str,
        column_name: &str,
        data_type: &str,
    ) -> &mut Self {
        if let Some(table) = self.tables.iter_mut().find(|t| t.name == table_name) {
            table.columns.push(ColumnSchema {
                name: column_name.to_string(),
                data_type: data_type.to_string(),
            });
        }
        self
    }

    pub fn build(self) -> Vec<TableSchema> {
        self.tables
    }
}

impl TableSchema {
    pub fn to_create_table_sql(&self) -> String {
        let columns: Vec<String> = self
            .columns
            .iter()
            .map(|c| format!("{} {}", c.name, c.data_type))
            .collect();
        format!(
            "CREATE TABLE IF NOT EXISTS {} ({});",
            self.name,
            columns.join(", ")
        )
    }
}
