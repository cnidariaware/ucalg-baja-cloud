use umya_spreadsheet::Worksheet;

use crate::utils::{ArcVec, merch::{CustomerInfo, MerchItem, OrderItem}, types::BajaResult};

pub trait Database {
    type Output; // must specify return type here
    
    fn new() -> Self;

    fn init_database(&mut self) -> BajaResult<()>;

    fn get_connection (&self) -> Self::Output;
}

pub trait MerchDatabase: Database {
    fn init_merch_database(&mut self) -> BajaResult<()>;

    fn get_merch(&self) -> ArcVec<MerchItem>;


    fn write_order(
        order: &OrderItem,
        orders_sheet: &mut Worksheet,
        row_insert: &u32,
    ); // turn into result later in case of failure

    fn write_customer(
        customer_info: &CustomerInfo,
        order_total: &f32,
        coupon: &Option<String>,
        customer_sheet: &mut Worksheet,
    ); // turn into result later

    // todo add readers, query and delete for admin pannel
}

// #[async_trait]
// pub trait DataPort: Send + Sync + 'static {
//     async fn
// }