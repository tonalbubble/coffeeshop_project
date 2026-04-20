pub mod model;
pub mod database;
use crate::database::Database;


use model::{Coffee, Roast, ItemOrder, CustomerOrder, Size, Inventory};

fn main() {
    //first, initialize database:
    db_init();


    println!("--- Initializing Shop Inventory ---");
    let mut shop_inventory = Inventory::new();


    //adding inventory
    shop_inventory.add_stock(Coffee::Columbian, 50);
    shop_inventory.add_stock(Coffee::MidnightRoast, 20);
    //shop_inventory.print_inventory();


    //simulating customer getting to page
    println!("\n--- Creating New Customer Order ---");
    let mut current_session_order = CustomerOrder::new(101, 5001);


   
    // Link: /add?coffee=Arabica&size=Large&qty=2, user clicks this link
    println!("Adding 2 Large Arabica...");
    current_session_order.add_item(Coffee::Arabica, Roast::Medium, Size::Large, 2.0);
    shop_inventory.reduce_stock(Coffee::Arabica, 2);


    println!("\n--- Final Receipt ---");
    println!("Order ID: {}", current_session_order.id);
    println!("Customer ID: {}", current_session_order.customer_id);


    for item in &current_session_order.items {
        println!(
            "Item: {:?} ({:?}) | Size: {:?} | Qty: {} | Subtotal: ${:.2}",
            item.coffee, item.roast.to_str(), item.size.to_str(), item.quantity, item.price
        );
    }

    println!("---------------------------");
    println!("TOTAL ORDER PRICE: ${:.2}", current_session_order.total_price);
    println!("---------------------------");

}

//this function makes sure our database is initialized with empty tables ready to go.
pub fn db_init(){
    let db = match Database::new("coffee.db".to_string()){
        Ok(db) => {
            println!("Successfully connected to {}",{&db.name});
            db
        },
        Err(e) => {
            println!("Error connecting to database: {e}");
            return;
        }
    };

    match db.drop_tables(){
        Ok(a) => println!("Tables dropped successfully"),
        Err(e) => println!("Error dropping tables: {e}")
    };

    match db.create_tables(){
        Ok(a) => println!("Tables created successfully"),
        Err(e) => println!("Error creating tables: {e}")
    };
}


struct hello{

}



fn calculate(){

}