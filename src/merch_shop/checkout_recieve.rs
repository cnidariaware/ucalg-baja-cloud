use std::sync::Arc;

use crate::{ArcString, utils::database::Database};
use actix_web::{
    HttpRequest, HttpResponse, Responder, post,
    web::{self, Data},
};
use darkicewolf50_actix_setup::log_incoming_proxy;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use tokio::sync::Mutex;
use umya_spreadsheet::{Worksheet, reader, writer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInfo {
    pub order_id: Option<ArcString>,
    email: String,
    phone: Option<String>,
    name: String,
    sub_team: Option<String>,
    ship_full_name: Option<String>,
    ship_street_addr: Option<String>,
    ship_unit_number: Option<String>,
    ship_city: Option<String>,
    ship_province: Option<String>,
    ship_country: Option<String>,
    ship_postal_code: Option<String>,
    ship_phone: Option<String>,
    additional_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    order_id: Option<ArcString>,
    item_id: String,
    colour: Option<String>,
    size: Option<String>,
    quantity: u8,
    price: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    customer_info: CustomerInfo,
    cart_items: Vec<OrderItem>,
    pub order_id: Option<ArcString>,
    coupon_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSuccess {
    success: bool,
    failure: Option<String>,
    testing: Option<OrderRequest>,
}

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
    data_state: Data<Mutex<Database>>,
    mut order_request: web::Json<OrderRequest>,
    req: HttpRequest,
) -> impl Responder {
    log_incoming_proxy("POST", "/shop/recieve_order", &req);

    let database = data_state.lock().await;

    match database.connection.is_some() {
        true => (),
        false => {
            return HttpResponse::InternalServerError().json(OrderSuccess {
                success: false,
                failure: Some("Database doesnt exist, please try again later".to_string()),
                testing: None,
            });
        }
    }
    order_request.give_uuid();

    let mut book = reader::xlsx::lazy_read(&database.connection.as_ref().unwrap()).unwrap();

    let orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();
    // Finds "newest row"
    let mut order_row_insert = orders_sheet.get_highest_row() + 1;
    // Customer Info Part of incoming order
    let orders = order_request.cart_items.clone();

    let mut order_total: f32 = 0.0;

    // Writes all orders in the vec to the sheet
    for order in orders {
        database.write_order(&order, orders_sheet, &order_row_insert);
        order_row_insert += 1;
        order_total += order.price;
    }

    // let customer_sheet = book.get_sheet_by_name("customer_info").unwrap();
    database.write_customer(
        &order_request.customer_info,
        &order_total,
        &order_request.coupon_code,
        book.get_sheet_by_name_mut("customer_info").unwrap(),
    );

    // save to workbook
    writer::xlsx::write(&book, &database.connection.as_ref().unwrap()).unwrap();

    HttpResponse::Ok().json(OrderSuccess {
        success: true,
        failure: None,
        testing: Some(order_request.into_inner()),
    })
}

impl Database {
    /// writes merch items to the orders sheet of the xl database
    ///
    /// # Params
    ///
    /// - order -The merch item that will be written into the row.
    /// - orders_sheet - The sheet where merch orders are written to.
    /// - row_insert - The integer of the row which the merch item should be written into.
    ///
    /// # Returns
    ///
    /// - unit type as the function cannot handle errors
    ///
    /// # Example
    ///
    /// ```rust
    /// // how a user would use this function, replace ex_crate with actual path to use function
    /// // this is how a public but internal module would be used by an outside user (ex_crate needs to be changed)
    /// // let result = crate::front_of_house::other_module_linked_in_modrs::example_funct(5, 10);
    /// // assert_eq!(result, 15);
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@darkicewolf50.dev>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    pub fn write_order(
        &self,
        order: &OrderItem,
        orders_sheet: &mut Worksheet,
        row_insert: &u32,
    ) -> () {
        // order id
        orders_sheet
            .get_cell_mut(format!("A{}", row_insert))
            .set_value_string(order.order_id.as_deref().unwrap_or_default());

        // item id
        orders_sheet
            .get_cell_mut(format!("B{}", row_insert))
            .set_value_string(order.item_id.as_str());

        // size
        orders_sheet
            .get_cell_mut(format!("C{}", row_insert))
            .set_value(order.size.as_deref().unwrap_or_default());

        // quantity
        orders_sheet
            .get_cell_mut(format!("D{}", row_insert))
            .set_value_number(order.quantity);

        // colour
        orders_sheet
            .get_cell_mut(format!("E{}", row_insert))
            .set_value_string(order.colour.as_deref().unwrap_or_default());

        // price
        orders_sheet
            .get_cell_mut(format!("F{}", row_insert))
            .set_value_number(order.price);
        ()
    }

    /// writes customer information to the customer sheet of the xl database
    ///
    /// # Params
    ///
    /// - customer_info - The shipping and contact information for the customer.
    /// - order_total - The total of this order, which is identified by the order's uuid.
    /// - coupon - a coupon string for flash sales and internal use
    /// - customer_sheet - the xl sheet where these parameters are written
    ///
    /// # Returns
    ///
    /// - Unit as this function cannot handle errors, so it must succeed.
    ///
    /// # Example
    ///
    /// ```rust
    /// // how a user would use this function, replace ex_crate with actual path to use function
    /// // this is how a public but internal module would be used by an outside user (ex_crate needs to be changed)
    /// // let result = crate::front_of_house::other_module_linked_in_modrs::example_funct(5, 10);
    /// // assert_eq!(result, 15);
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@darkicewolf50.dev>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    pub fn write_customer(
        &self,
        customer_info: &CustomerInfo,
        order_total: &f32,
        coupon: &Option<String>,
        customer_sheet: &mut Worksheet,
    ) {
        // Finds "newest row"
        let customer_row_insert = customer_sheet.get_highest_row() + 1;

        // order id
        customer_sheet
            .get_cell_mut(format!("A{}", customer_row_insert))
            .set_value(customer_info.order_id.as_deref().unwrap_or_default());

        // email
        customer_sheet
            .get_cell_mut(format!("B{}", customer_row_insert))
            .set_value_string(&customer_info.email);
        // phone
        customer_sheet
            .get_cell_mut(format!("C{}", customer_row_insert))
            .set_value_string(customer_info.phone.as_deref().unwrap_or_default());
        // name
        customer_sheet
            .get_cell_mut(format!("D{}", customer_row_insert))
            .set_value_string(&customer_info.name);
        // subteam
        customer_sheet
            .get_cell_mut(format!("E{}", customer_row_insert))
            .set_value_string(customer_info.sub_team.as_deref().unwrap_or_default());

        // Order Total
        customer_sheet
            .get_cell_mut(format!("F{}", customer_row_insert))
            .set_value_number(*order_total);

        // Coupon if input
        customer_sheet
            .get_cell_mut(format!("G{}", customer_row_insert))
            .set_value_string(coupon.as_deref().unwrap_or_default());

        // shipping

        // shipping full name
        customer_sheet
            .get_cell_mut(format!("I{}", customer_row_insert))
            .set_value_string(customer_info.ship_full_name.as_deref().unwrap_or_default());

        // shipping street address
        customer_sheet
            .get_cell_mut(format!("J{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_street_addr
                    .as_deref()
                    .unwrap_or_default(),
            );

        // shipping unit number
        customer_sheet
            .get_cell_mut(format!("K{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_unit_number
                    .as_deref()
                    .unwrap_or_default(),
            );

        // shipping city
        customer_sheet
            .get_cell_mut(format!("L{}", customer_row_insert))
            .set_value_string(customer_info.ship_city.as_deref().unwrap_or_default());

        // shipping provice
        customer_sheet
            .get_cell_mut(format!("M{}", customer_row_insert))
            .set_value_string(customer_info.ship_province.as_deref().unwrap_or_default());

        // shipping country
        customer_sheet
            .get_cell_mut(format!("N{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_country
                    .as_deref()
                    .unwrap_or_else(|| "Canada"),
            );

        // shipping postal code
        customer_sheet
            .get_cell_mut(format!("O{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_postal_code
                    .as_deref()
                    .unwrap_or_default(),
            );

        // shipping phone number
        customer_sheet
            .get_cell_mut(format!("P{}", customer_row_insert))
            .set_value_string(customer_info.ship_phone.as_deref().unwrap_or_default());

        customer_sheet
            .get_cell_mut(format!("Q{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .additional_notes
                    .as_deref()
                    .unwrap_or_default(),
            );
    }
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
    /// use ucalg_baja_cloud::merch_shop::checkout_recieve::{OrderRequest, CustomerInfo, OrderItem};
    ///
    /// let mut test_order = OrderRequest::_new_for_test();
    /// test_order.give_uuid();
    ///
    /// assert!(test_order.get_order_id().is_some());
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
    /// use ucalg_baja_cloud::merch_shop::checkout_recieve::{OrderRequest, CustomerInfo, OrderItem};
    ///
    /// let mut test_order = OrderRequest::_new_for_test();
    /// test_order.give_uuid();
    ///
    /// assert!(test_order.get_order_id().is_some());
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
