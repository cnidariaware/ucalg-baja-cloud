use std::path::PathBuf;


#[derive(Debug, Clone)]
pub struct YamlConfig {
    pub file_path: Option<PathBuf>
}

#[derive(Debug, Clone)]
pub struct SpreadSheetConfig {
    pub file_path: Option<PathBuf>
}

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub connection: Option<String>,
    pub max_connection: u8,
    pub schema: Option<String>
}

#[derive(Debug, Clone)]
pub enum StorageConfig {
    Yaml(YamlConfig),
    SpreadSheet(SpreadSheetConfig),
    Postgres(PostgresConfig),
    // for tests
    InMemory
}