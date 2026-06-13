use crate::utils::{
    ArcVec,
    database::db_types::ConnectionConfig,
    merch::{CustomerInfo, MerchItem, OrderItem},
    types::BajaResult,
};

pub trait Database {
    fn new() -> Self;

    fn init_database(&mut self) -> BajaResult<()>;

    fn get_connection(&self) -> Option<ConnectionConfig>;

    fn save(&self) -> impl std::future::Future<Output = BajaResult<()>> + Send;
}

// #[async_trait]
pub trait MerchDatabase: Database {
    fn init_merch_database(&mut self) -> BajaResult<()>;

    fn get_merch(&self) -> ArcVec<MerchItem>;

    fn write_order(
        &self,
        order: &OrderItem,
    ) -> impl std::future::Future<Output = BajaResult<()>> + Send;

    fn write_customer(
        &self,
        customer_info: &CustomerInfo,
        order_total: &f32,
        coupon: &Option<String>,
    ) -> impl std::future::Future<Output = BajaResult<()>> + Send;

    // todo add readers, query and delete for admin pannel
}

// #[async_trait]
// pub trait DataPort: Send + Sync + 'static {
//     async fn
// }
