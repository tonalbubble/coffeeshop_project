
use std::collections::HashMap;

//lets do pricing as follows

//LARGE => $12
//MEDIUM => $8
//SMALL => $5

use std::hash::RandomState;

enum Size{
    Small,
    Medium,
    Large
}


impl Size{
    //so we can calculate price on the backend inside the struct when we create a new item order
    fn price(&self) -> f32{
        match self {
            Size::Small => 5.0,

            Size::Medium => 8.0,

            Size::Large => 12.0
        }
    }
}


enum Roast{
    Light,
    Medium,
    Dark
}

//coffee enum just so we have fixed types, if we were building this to dynamically add coffees
//then string could be better but this will be easier to manage errors


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Coffee{
    Columbian,
    Arabica,
    Robusta,
    Excelsa,
    BreakfastBlend,
    MidnightRoast
}

struct Inventory{
    stock : HashMap<Coffee, i32>
}


//basically just gonna do number of bags available
//disregard the large,small,medium that we can implement later
//so with the simulation just gonna remove one bag per purchase
impl Inventory{
    fn new() -> Self{

        let mut stock = HashMap::new();

        stock.insert(Coffee::Arabica, 100);
        stock.insert(Coffee::Arabica, 100);
        stock.insert(Coffee::Arabica, 100);

        Inventory { stock }

    }

    fn add_stock(&mut self, coffee : Coffee, amount : i32){
        //or insert checks if a value exists at the location at thekey, returns mutable reference to the value
        let inventory_add = self.stock.entry(coffee).or_insert(0);

        //dereference the pointer here
        *inventory_add += amount
    } 


    //if not enough coffee to remove from return false here
    fn reduce_stock(&mut self, coffee : Coffee, amount : i32) -> bool{

        //get_mut return mutable reference for the value at the key location in the hashmap
        if let Some(current_stock) = self.stock.get_mut(&coffee){
            if(*current_stock >= amount){
                *current_stock -= amount;
                return true;
            }
            
        }
        false
    }


    fn print_inventory(&self){
        println!("--------INVENTORY---------");
        for(coffee, amount) in &self.stock{
            println!("{:?}, {}", coffee, amount);
        }
    }

}

impl Coffee{
    fn description(&self) -> &'static str{
        match self {
            Coffee::Columbian => "Smooth and balanced with mild acidity",
            Coffee::Arabica => "Sweet and complex with fruity notes",
            Coffee::Robusta => "Strong and bitter with high caffeine",
            Coffee::Excelsa => "Tart and fruity with a unique profile",
            Coffee::BreakfastBlend => "Light and bright, perfect for mornings",
            Coffee::MidnightRoast => "Dark and bold with deep flavor",
            
        }
    }
}


//this could represent Columbian Dark
//this struct represents what we have on the menu

/* 
struct CofffeItem{
    name : Coffee,
    roast : Roast,
    //description will always be the same for the coffees so instead of lifetimes used static
    description : &'static str
}


impl CofffeItem{
    fn new(new_name : Coffee, new_roast : Roast) -> Self{

        let new_description = Coffee::description(&new_name);

        CofffeItem{
            name : new_name,
            roast : new_roast,
            description : new_description
        }

    }
}
*/


//might need lifetimes('a things) for the coffeeitem in the parameters
//this would be represented by an object like 2 bags og Columbian Dark size L which would then be 24 as price
struct ItemOrder{
    coffee : Coffee,
    size : Size,
    quantity : f32,
    price : f32
}


//
impl ItemOrder{

    fn new(new_coffee : Coffee, new_size : Size, new_quantity : f32) -> Self{

        let total_price = new_size.price() * new_quantity;

        ItemOrder{
            coffee : new_coffee,
            size : new_size,
            quantity : new_quantity,
            price : total_price
        }
    }

}

//the vector will contain different item orders, 

//customerOrder struct will basically be like a receipt of everything they bought
struct CustomerOrder{
    id : i32,
    customer_id : i32,
    items : Vec<ItemOrder>,
    total_price : f32
}   


impl CustomerOrder{
    fn new(new_id : i32, customer_id : i32, ) -> Self{

        CustomerOrder{
            id : new_id,
            customer_id : customer_id,
            items : Vec::new(),
            total_price : 0.0
        }
    }

    fn add_item(&mut self, coffee: Coffee, roast: Roast, size: Size, quantity: f32){


        //let coffee_item = CofffeItem::new(coffee, roast);
        let item = ItemOrder::new(coffee, size, quantity);


        self.total_price += item.price;
        self.items.push(item);


    }
}




/*
pub struct Customer{
    id : i32,
    name : String,
    email : String
}

impl Customer{
    fn new(new_id : i32, new_name : String, new_email : String) -> Self
    {
        Customer{
            id : new_id,
            name : new_name,
            email : new_email
        }
    }
}
*/
