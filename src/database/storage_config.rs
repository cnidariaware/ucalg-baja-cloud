use crate::utils::{
    ArcVec, Database,
    database::{ConnectionConfig, MerchDatabase, SpreadSheet, StorageConfig},
    merch::{CustomerInfo, MerchItem, OrderItem},
    types::BajaResult,
};

impl Database for StorageConfig {
    fn new() -> StorageConfig {
        StorageConfig::SpreadSheetConfig(SpreadSheet::new())
    }

    fn init_database(&mut self) -> BajaResult<()> {
        todo!();
        Ok(())
    }

    fn get_connection(&self) -> Option<ConnectionConfig> {
        match &self {
            StorageConfig::SpreadSheetConfig(s) => SpreadSheet::get_connection(&s),
            StorageConfig::PostgresConfig(s) => todo!(),
            StorageConfig::InMemory => todo!(),
        }
    }

    async fn save(&self) -> BajaResult<()> {
        todo!();
        Ok(())
    }
}

impl MerchDatabase for StorageConfig {
    fn init_merch_database(&mut self) -> BajaResult<()> {
        todo!()
    }

    fn get_merch(&self) -> ArcVec<MerchItem> {
        todo!()
    }

    async fn write_order(&self, order: &OrderItem) -> BajaResult<()> {
        todo!()
    }

    async fn write_customer(
        &self,
        customer_info: &CustomerInfo,
        order_total: &f32,
        coupon: &Option<String>,
    ) -> BajaResult<()> {
        todo!()
    }
}
