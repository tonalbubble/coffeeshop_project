

enum Size{
    Small,
    Medium,
    Large
}


enum Roast{
    Light,
    Medium,
    Dark
}

struct Cofffe{
    name : String,
    roast : Roast,
    price : f32
}


struct ItemOrder{
    coffee : Cofffe,
    size : Size,
    quantity : i32
}


struct CustomerOrder{
    id : i32,
    customer_id : i32,
    items : Vec<ItemOrder>,
    total : f32
}   

pub struct Customer{
    id : i32,
    name : String,
    email : String
}