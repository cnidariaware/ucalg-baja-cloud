use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;
use umya_spreadsheet::writer;

use crate::utils::ArcVec;
use crate::utils::Database;
use crate::utils::database::ConnectionConfig;
use crate::utils::database::MerchDatabase;
use crate::utils::database::{Sheets, SpreadSheet};
use crate::utils::merch::CustomerInfo;
use crate::utils::merch::MerchItem;
use crate::utils::merch::OrderItem;
use crate::utils::types::BajaError;
use crate::utils::types::BajaResult;

use umya_spreadsheet::reader::xlsx::lazy_read;

impl Database for SpreadSheet {
    // type Output = Arc<PathBuf>;

    /// Creates a new connection or xl sheet if one is not already present
    ///
    /// # Params
    ///
    /// - Nothing
    ///
    /// # Returns
    ///
    /// - Intializes the Database with a connection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ucalg_baja_cloud::database::Database;
    /// let database = Database::new();
    ///
    /// assert!(database.get_connection().is_some());
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@darkicewolf50.dev>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    fn new() -> SpreadSheet {
        let xl_path = PathBuf::from(
            #[cfg(debug_assertions)]
            "./Database/Merch.xlsx",
            #[cfg(not(debug_assertions))]
            "/Merch/Merch.xlsx",
        );

        let mut database = SpreadSheet {
            file_path: Some(xl_path),
            sheets: None,
        };

        if !&database.file_path.as_deref().unwrap().exists() {
            let _ = database.init_merch_database();

            database.new_sheets();
            return database;
        }

        database.new_sheets();
        database
    }

    fn get_connection(&self) -> Option<ConnectionConfig> {
        self.file_path
            .as_ref()
            .map(|path| ConnectionConfig::SpreadSheet(Arc::from(path.to_path_buf())))
    }

    fn init_database(&mut self) -> BajaResult<()> {
        self.init_merch_database()
    }

    async fn save(&self) -> BajaResult<()> {
        // 1. Lock your inner state
        let sheets = self
            .sheets
            .as_ref()
            .ok_or_else(|| BajaError::Error("Not initialized".to_string()))?
            .lock()
            .await;

        // 2. Re-construct or access your master Workbook struct and dump to file
        if let Some(path) = &self.file_path {
            // Assuming your setup allows reconstructing/holding the complete `book`
            umya_spreadsheet::writer::xlsx::write(&sheets.book, path)
                .map_err(|e| BajaError::Error(e.to_string()))?;
        }
        Ok(())
    }
}

impl MerchDatabase for SpreadSheet {
    /// Initalizes the xl file with two sheets, one for order items, and the other for customer info.
    ///
    /// # Params
    ///
    /// - A database instance
    ///
    /// # Returns
    ///
    /// - Unit or a string explaining the error so it can be sent to to the front end or dealt with.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ucalg_baja_cloud::database::Database;
    /// let mut database = Database::new();
    ///
    /// assert!(database.database_initialize_xl().is_ok());
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@cnidariaware.ca>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    fn init_merch_database(&mut self) -> BajaResult<()> {
        println!("Creating New Sheet");

        let mut book = umya_spreadsheet::new_file_empty_worksheet();

        let order_sheet = match book.new_sheet("orders") {
            Ok(p) => p,
            Err(_) => return Err(BajaError::Error("Cannot create orders sheet".to_string())),
        };

        order_sheet.get_cell_mut("A1").set_value("Order Id");
        order_sheet.get_cell_mut("B1").set_value("Item Id");
        order_sheet.get_cell_mut("C1").set_value("Size");
        order_sheet.get_cell_mut("D1").set_value("Quantity");
        order_sheet.get_cell_mut("E1").set_value("Colour");
        order_sheet.get_cell_mut("F1").set_value("Price");

        let custmer_sheet = match book.new_sheet("customer_info") {
            Ok(p) => p,
            Err(_) => {
                return Err(BajaError::Error(
                    "Cannot create customer info sheet".to_string(),
                ));
            }
        };

        custmer_sheet.get_cell_mut("A1").set_value("Order Id");
        custmer_sheet.get_cell_mut("B1").set_value("Email");
        custmer_sheet.get_cell_mut("C1").set_value("Phone");
        custmer_sheet.get_cell_mut("D1").set_value("Name");
        custmer_sheet.get_cell_mut("E1").set_value("Subteam");
        custmer_sheet.get_cell_mut("F1").set_value("Order Total");
        custmer_sheet.get_cell_mut("G1").set_value("Coupon Code");
        custmer_sheet
            .get_cell_mut("H1")
            .set_value("Shipping Details");
        custmer_sheet.get_cell_mut("I1").set_value("Full Name");
        custmer_sheet.get_cell_mut("J1").set_value("Street Address");
        custmer_sheet.get_cell_mut("K1").set_value("Unit Number");
        custmer_sheet.get_cell_mut("L1").set_value("City");
        custmer_sheet.get_cell_mut("M1").set_value("Province");
        custmer_sheet
            .get_cell_mut("N1")
            .set_value("Country (Default Canada)");
        custmer_sheet.get_cell_mut("O1").set_value("Postal Code");
        custmer_sheet
            .get_cell_mut("P1")
            .set_value("Phone Number (shipping)");
        custmer_sheet
            .get_cell_mut("Q1")
            .set_value("Additional Notes");

        match writer::xlsx::write(&book, self.file_path.as_ref().unwrap().clone().as_path()) {
            Ok(_) => (),
            Err(_) => return Err(BajaError::Error("Cannot create xl sheet".to_string())),
        };

        Ok(())
    }

    fn get_merch(&self) -> ArcVec<MerchItem> {
        panic!("No Merch Items Stored here, only orders");
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
    /// - Brock <brock@cnidariaware.ca>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    async fn write_order(
        &self,
        order: &OrderItem,
        // orders_sheet: &mut Worksheet,
        // row_insert: &u32,
    ) -> BajaResult<()> {
        let orders_sheet = &mut self
            .sheets
            .as_ref() // Borrow it so we don't move out of `self`
            .ok_or_else(|| BajaError::Error("Sheets not initialized".to_string()))? // Early returns on None
            .lock()
            .await // get lock
            .orders_sheet;

        let row_insert = orders_sheet.get_highest_row() + 1;

        // let orders_sheet = self.sheets.unwrap_or_else(|| return BajaError::Error("Sheets not initailized".to_string();)).lock().await.orders_sheet.as_ref();
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

        self.save().await?;

        Ok(())
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
    async fn write_customer(
        &self,
        // spread_sheet_config: &SpreadSheetConfig,
        customer_info: &CustomerInfo,
        order_total: &f32,
        coupon: &Option<String>,
        // customer_sheet: &mut Worksheet,
    ) -> BajaResult<()> {
        let customer_sheet = &mut self
            .sheets
            .as_ref()
            .ok_or_else(|| BajaError::Error("Sheets not initialized".to_string()))? // Early returns on None
            .lock()
            .await
            .customer_sheet;
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

        self.save().await?;

        Ok(())
    }
}

impl SpreadSheet {
    fn new_sheets(&mut self) {
        Some(Arc::from(Mutex::from(
            Sheets::new(self.file_path.as_deref().unwrap()).unwrap(),
        )));
    }
}

impl Sheets {
    fn new(file_path: &Path) -> Option<Self> {
        Some(Sheets {
            orders_sheet: lazy_read(file_path)
                .unwrap()
                .get_sheet_by_name_mut("orders")
                .cloned()
                .unwrap(),
            customer_sheet: lazy_read(file_path)
                .unwrap()
                .get_sheet_by_name_mut("customer_info")
                .cloned()
                .unwrap(),

            book: lazy_read(file_path).unwrap(),
        })
    }
}
