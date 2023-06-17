use serde::Deserialize;
use std::fmt;



#[derive(Debug, Deserialize, Clone)]
///we create a struct of the product which contains the
//name of the product and the rate of the product

pub struct ProductInfo {
    name: String,
    pub rate: u64
}
//we implement the decalred struct and create a new
///function that will create a new instance of the product. i.e if we call the 
///function, we can create a new product with it

impl ProductInfo {
    pub fn new() -> ProductInfo{
        ProductInfo{
            name: String::from(""),
            rate: 0
        }
    }
}

//we implement the formatter interface on the product list to make it look more neater 
impl fmt::Display for ProductInfo{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, " {} {} sat per unit", self.name, self.rate)
    }
}