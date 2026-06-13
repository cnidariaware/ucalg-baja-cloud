use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use umya_spreadsheet::Worksheet;

#[derive(Debug, Clone)]
pub struct SpreadSheet {
    pub file_path: Option<PathBuf>,
    pub sheets: Option<Arc<Mutex<Sheets>>>,
}

#[derive(Debug, Clone)]
pub struct Sheets {
    pub orders_sheet: Worksheet,
    pub customer_sheet: Worksheet,
    pub book: umya_spreadsheet::Spreadsheet,
}

#[derive(Debug, Clone)]
pub struct Postgres {
    pub connection: Option<String>,
    pub max_connection: u8,
    pub schema: Option<String>,
}

#[derive(Debug, Clone)]
pub enum StorageConfig {
    SpreadSheetConfig(SpreadSheet),
    PostgresConfig(Postgres),
    // for tests
    InMemory,
}

#[derive(Debug, Clone)]
pub enum ConnectionConfig {
    SpreadSheet(Arc<PathBuf>),
    Other(String),
    Todo(()),
}
