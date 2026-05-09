use serde::{Deserialize, Serialize};

use crate::utils::{ArcString, ArcVec};


#[derive(Debug, Serialize, Deserialize)]
pub struct MerchItem {
    name: String,
    category: String,
    sizes_available: ArcVec<String>,
    price: f32,
    colours: ArcVec<String>,
    description: String,
    url_images: ArcVec<String>,
    additional_details: String,
    material: String,
    cleaning: String,
    size_guide_img_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInfo {
    pub order_id: Option<ArcString>,
    pub email: String,
    pub phone: Option<String>,
    pub name: String,
    pub sub_team: Option<String>,
    pub ship_full_name: Option<String>,
    pub ship_street_addr: Option<String>,
    pub ship_unit_number: Option<String>,
    pub ship_city: Option<String>,
    pub ship_province: Option<String>,
    pub ship_country: Option<String>,
    pub ship_postal_code: Option<String>,
    pub ship_phone: Option<String>,
    pub additional_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub order_id: Option<ArcString>,
    pub item_id: String,
    pub colour: Option<String>,
    pub size: Option<String>,
    pub quantity: u8,
    pub price: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub customer_info: CustomerInfo,
    pub cart_items: Vec<OrderItem>,
    pub order_id: Option<ArcString>,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSuccess {
    pub success: bool,
    pub failure: Option<String>,
    pub testing: Option<OrderRequest>,
}