

//lets do pricing as follows

//LARGE => $12
//MEDIUM => $8
//SMALL => $5

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
enum Coffee{
    Columbian,
    Arabica,
    Robusta,
    Excelsa,
    BreakfastBlend,
    MidnightRoast
}


//this could represent Columbian Dark
struct CofffeItem{
    name : Coffee,
    roast : Roast,
    description : String
}





//this would be represented by an object like 2 bags og Columbian Dark size L which would then be 24 as price
struct ItemOrder{
    coffee : CofffeItem,
    size : Size,
    quantity : f32,
    price : f32
}


impl ItemOrder{
    fn new(new_coffee : CofffeItem, new_size : Size, new_quantity : f32) -> Self{

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

struct CustomerOrder{
    id : i32,
    customer_id : i32,
    items : Vec<ItemOrder>,
    total_price : f32
}   

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


impl CustomerOrder{

}