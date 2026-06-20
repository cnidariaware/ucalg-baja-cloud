use std::sync::Arc;

use crate::utils::{
    ArcString,
    database::{MerchDatabase, SpreadSheet},
    types::BajaError,
};
use actix_web::{
    HttpRequest, HttpResponse, Responder, post,
    web::{self, Data},
};
use darkicewolf50_actix_setup::log_incoming_proxy;
// use serde_json::json;
use uuid::Uuid;

use crate::utils::merch::{CustomerInfo, OrderItem, OrderRequest, OrderSuccess};

/// Recieves incoming merch orders and writes them to a database
///
/// # Params
///
/// - data_state - The database mutex
/// - order_request - The order information, including the coupon, customer info, and items in the order
/// - req - The details about the incoming request/headers
///
/// # Returns
///
/// - Whether the writing of the order was successfull
///
/// # Example
///
/// ```rust
/// use ucalg_baja_cloud::merch_shop::checkout_recieve::recieve_order;
/// use actix_web::{test, web, App};
/// use serde_json::{json, Value};
/// use tokio::sync::Mutex;
///
/// #[actix_web::test]
/// async fn test_recieve_order() {
///     // Create a test database instance
///     let database = Mutex::new(
///         ucalg_baja_cloud::database::Database::default()
///     );
///
///     let app = test::init_service(
///         App::new()
///             .app_data(web::Data::new(database))
///             .service(recieve_order)
///     ).await;
///
///     let req = test::TestRequest::post()
///         .uri("/recieve_order")
///         .set_json(json!({
///             "customer_info": {
///                 "order_id": "93616598-94e1-4b54-ae94-ccce8393d8bb",
///                 "email": "brock.tomlinson@ucalgary.ca",
///                 "phone": "2509466196",
///                 "name": "Brock",
///                 "sub_team": "Software"
///             },
///             "cart_items": [
///                 {
///                     "order_id": "93616598-94e1-4b54-ae94-ccce8393d8bb",
///                     "item_id": "HERO-2020 HOODIES",
///                     "size": "S",
///                     "quantity": 3,
///                     "price": 36.01
///                 },
///                 {
///                     "order_id": "93616598-94e1-4b54-ae94-ccce8393d8bb",
///                     "item_id": "HERO-2020 HOODIES",
///                     "size": "XL",
///                     "quantity": 10,
///                     "price": 36.01
///                 }
///             ]
///         }))
///         .to_request();
///
///     let resp: Value = test::call_and_read_body_json(&app, req).await;
///
///     // Contract: endpoint always returns an object with `success`
///     assert!(resp.get("success").is_some());
/// }
/// ```
///
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>
/// semi-permanent email, do not need to respond but try to be a good alumni
#[post("/recieve_order")]
pub async fn recieve_order(
    data_state: Data<SpreadSheet>, // impl Database + MerchDatabase
    mut order_request: web::Json<OrderRequest>,
    req: HttpRequest,
) -> impl Responder
where
    // DB: Database + MerchDatabase + 'static,
{
    log_incoming_proxy("POST", "/shop/recieve_order", &req);

    let database = data_state.into_inner();

    // match database.into_inner() {
    //     true => (),
    //     false => {
    //         return HttpResponse::InternalServerError().json(OrderSuccess {
    //             success: false,
    //             failure: Some("Database doesnt exist, please try again later".to_string()),
    //             testing: None,
    //         });
    //     }
    // }
    order_request.give_uuid();

    // let orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();
    // Finds "newest row"
    // let mut order_row_insert = orders_sheet.get_highest_row() + 1;
    // Customer Info Part of incoming order
    let orders = order_request.cart_items.clone();

    let mut order_total: f32 = 0.0;

    // Writes all orders in the vec to the sheet
    for order in orders {
        match database.write_order(&order).await {
            Err(BajaError::Error(e)) => {
                println!("Database Error\n {e}");
                return HttpResponse::InternalServerError().body("Failed to write to the database");
            }
            _ => (),
        }
        order_total += order.price;
    }

    // let customer_sheet = book.get_sheet_by_name("customer_info").unwrap();
    match database
        .write_customer(
            &order_request.customer_info,
            &order_total,
            &order_request.coupon_code,
        )
        .await
    {
        Err(BajaError::Error(e)) => {
            println!("Database Error\n{e}");
            return HttpResponse::InternalServerError().body("Failed to write to the database");
        }
        _ => (),
    }

    // save to workbook
    // writer::xlsx::write(&book, &database.connection.as_ref().unwrap()).unwrap();

    HttpResponse::Ok().json(OrderSuccess {
        success: true,
        failure: None,
        testing: Some(order_request.into_inner()),
    })
}

impl OrderRequest {
    /// Gives the struct a uuid
    ///
    /// # Params
    ///
    /// - self - An instance of the struct object.
    ///
    /// # Returns
    ///
    /// - Unit, this self assigns a uuid to the order struct.
    ///
    /// # Example
    ///
    /// ```
    /// // use crate::utils::merch::{CustomerInfo, OrderItem, OrderRequest};
    ///
    /// // let mut test_order = OrderRequest::_new_for_test();
    /// // test_order.give_uuid();
    ///
    /// // assert!(test_order.get_order_id().is_some());
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@darkicewolf50.dev>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    pub fn give_uuid(&mut self) {
        if self.order_id.is_none() {
            let new_uuid: ArcString = Arc::from(Uuid::new_v4().to_string());

            self.order_id = Some(new_uuid.clone());

            self.customer_info.order_id = Some(new_uuid.clone());

            for order_item in self.cart_items.iter_mut() {
                order_item.order_id = Some(new_uuid.clone());
            }
        }
    }

    // Gets the order_id uuid
    ///
    /// # Params
    ///
    /// - self - An instance of the struct object.
    ///
    /// # Returns
    ///
    /// - The Option which may contain the order_id.
    ///
    /// # Example
    ///
    /// ```
    /// // use crate::utils::merch::{CustomerInfo, OrderItem, OrderRequest};
    ///
    /// // let mut test_order = OrderRequest::_new_for_test();
    /// // test_order.give_uuid();
    ///
    /// // assert!(test_order.get_order_id().is_some());
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@darkicewolf50.dev>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    pub fn get_order_id(&self) -> Option<ArcString> {
        self.order_id.clone()
    }

    // only for testing not a real function, only for testing the above functions
    // only present to get around rust's module privacy
    pub fn _new_for_test() -> Self {
        Self {
            customer_info: CustomerInfo {
                order_id: None,
                email: "test@gmail.com".to_string(),
                phone: Some("1234567890".to_string()),
                name: "Tester".to_string(),
                sub_team: None,
                ship_full_name: None,
                ship_street_addr: None,
                ship_unit_number: None,
                ship_city: None,
                ship_province: None,
                ship_country: None,
                ship_postal_code: None,
                ship_phone: None,
                additional_notes: None,
            },
            cart_items: vec![OrderItem {
                order_id: None,
                item_id: "Test Merch".to_string(),
                colour: None,
                size: None,
                quantity: 1,
                price: 1.00,
            }],
            order_id: None,
            coupon_code: None,
        }
    }
}
