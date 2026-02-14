#![allow(unused)]

use utoipa::ToSchema;

#[derive(ToSchema)]
struct _MerchItem {
    name: String,
    category: String,
    sizes_available: Vec<String>,
    price: f32,
    colours: Vec<String>,
    description: String,
    url_images: Vec<String>,
    additional_details: String,
    material: String,
    cleaning: String,
    size_guide_img_url: String,
}

#[utoipa::path(
    get,
    path = "/shop/merch",
    summary = "List all merch items",
    description = "Returns the complete catalog and their details of merch items sold in the shop.",
    tags = ["Merch"],
    responses(
        (status = 200, description = "Merch Details, using the MerchItem schema", body = [_MerchItem])
    )
)]
pub fn _get_merch() {}

#[derive(ToSchema)]
struct _OrderRequest {
    customer_info: _CustomerInfo,
    cart_items: Vec<_OrderItem>,
    order_id: Option<String>,
    coupon_code: Option<String>,
}

#[derive(ToSchema)]
struct _OrderItem {
    order_id: Option<String>,
    item_id: String,
    colour: Option<String>,
    size: Option<String>,
    quantity: u8,
    price: f32,
}

#[derive(ToSchema)]
struct _CustomerInfo {
    order_id: Option<String>,
    email: String,
    phone: Option<String>,
    name: String,
    sub_team: Option<String>,
    order_total: f32,
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

#[derive(ToSchema)]
struct _OrderSuccess {
    success: bool,
    failure: Option<String>,
    testing: Option<_OrderRequest>,
}

#[utoipa::path(
    post,
    path = "/shop/recieve_order",
    summary = "Receive a merch order",
    description = "Receives order details including customer and cart items, \
                   writes them to the Excel order and customer sheets, \
                   and returns whether the operation was successful.",
    tags = ["Merch"],

    request_body(
        content = _OrderRequest,
        description = "Order details including customer information, cart items, and optional coupon code",
        content_type = "application/json"
    ),

    responses(
        (
            status = 200,
            description = "Order processed successfully",
            body = _OrderSuccess
        ),
        (
            status = 500,
            description = "Internal error while processing the order",
            body = _OrderSuccess
        )
    )
)]
pub fn _recieve_order() {}
