use std::{
    env,
    path::{Path, PathBuf},
};
use umya_spreadsheet::writer;

pub struct Database {
    pub connection: Option<PathBuf>,
}

impl Database {
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
    pub fn new() -> Database {
        let xl_path = match env::var("MERCH_DATABASE") {
            Ok(path_value) => PathBuf::from(path_value),
            Err(_) => PathBuf::from("./Database/Merch.xlsx"),
        };

        let mut database = Database {
            connection: Some(xl_path),
        };

        if !&database.connection.clone().unwrap().exists() {
            let _ = database.database_initialize_xl();

            return database;
        }

        database
    }

    /// Gets the private field connection
    ///
    /// # Params
    ///
    /// - A database instance
    ///
    /// # Returns
    ///
    /// - The connection private field.
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
    pub fn get_connection(&self) -> &Option<PathBuf> {
        &self.connection
    }

    /// Checks if the database is present, otherwise it creates one and initalizes it
    ///
    /// # Params
    ///
    /// - A database instance
    ///
    /// # Returns
    ///
    /// - Unit or an error string to send back to the front ned to explain the error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ucalg_baja_cloud::database::Database;
    /// let mut database = Database::new();
    ///
    /// assert!(database.check_path_xl().is_ok());
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@darkicewolf50.dev>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    pub fn check_path_xl(&mut self) -> Result<(), String> {
        let xl_path = Path::new("./Database/Merch.xlsx");

        if Path::exists(&xl_path) {
            self.connection = Some(xl_path.into());
            return Ok(());
        } else {
            // Recreating Path
            self.connection = Some(xl_path.into());
            match self.database_initialize_xl() {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }

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
    /// - Brock <brock@darkicewolf50.dev>
    /// semi-permanent email, do not need to respond but try to be a good alumni
    pub fn database_initialize_xl(&mut self) -> Result<(), String> {
        println!("Creating New Sheet");

        let mut book = umya_spreadsheet::new_file_empty_worksheet();

        let order_sheet = match book.new_sheet("orders") {
            Ok(p) => p,
            Err(_) => return Err("Cannot create orders sheet".to_string()),
        };

        order_sheet.get_cell_mut("A1").set_value("Order Id");
        order_sheet.get_cell_mut("B1").set_value("Item Id");
        order_sheet.get_cell_mut("C1").set_value("Size");
        order_sheet.get_cell_mut("D1").set_value("Quantity");
        order_sheet.get_cell_mut("E1").set_value("Colour");
        order_sheet.get_cell_mut("F1").set_value("Price");

        let custmer_sheet = match book.new_sheet("customer_info") {
            Ok(p) => p,
            Err(_) => return Err("Cannot create customer info sheet".to_string()),
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

        match writer::xlsx::write(&book, self.connection.as_ref().unwrap().clone().as_path()) {
            Ok(_) => (),
            Err(_) => return Err("Cannot create xl sheet".to_string()),
        };

        Ok(())
    }
}
