mod model;
pub mod database;
use crate::database::Database;

fn main() {
    //first, initialize database:
    db_init();
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