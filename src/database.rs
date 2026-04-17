use rusqlite::{Connection, Statement, Error};


//database struct: name is .db file name, conn is connection
pub struct Database{
    pub name: String,
    pub conn: Connection
}

impl Database{
    //new create a database if we can successfully connect, otherwise propagate the error
    pub fn new(name: String) -> Result<Database, Error>{
        match Connection::open(&name){
            Ok(c) => Ok(Database{
                name:name,
                conn:c
            }),
            Err(e) => {
                Err(e)
            }
        }
    }
    //creates the tables that we will need
    pub fn create_tables(&self) -> Result<(),Error>{
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS product(
                product_id INT PRIMARY KEY,
                name STRING NOT NULL,
                base_price DECIMAL NOT NULL
                )",())?; //error propagation
        Ok(())
    }

    //drop tables if we need to (not commonly needed)
    pub fn drop_tables(&self)-> Result<(),Error>{
        self.conn.execute(
            "DROP TABLE IF EXISTS product;",())?;
        Ok(())
    }
}
