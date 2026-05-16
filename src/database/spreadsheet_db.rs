
use crate::utils::Database;
use crate::utils::database::SpreadSheetConfig;
use crate::utils::database::MerchDatabase;

impl Database for SpreadSheetConfig {
    fn new() -> SpreadSheetConfig {
        todo!();
    }

    fn get_connection (&self) {
        todo!();
    }

    fn init_database(&mut self) -> Result<(), crate::core::error::BajaError> {
        todo!();
    }
}
impl MerchDatabase for SpreadSheetConfig {
    fn init_merch_database(&mut self) -> Result<(), String> {
        todo!();
    }

    fn get_merch(&self) -> crate::utils::ArcVec<crate::utils::merch::MerchItem> {
        todo!();
    }
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
    fn write_order(
        spread_sheet_config: &SpreadSheetConfig,
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
        spread_sheet_config: &SpreadSheetConfig,
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
