fn main() {
    println!("Hello, world!");

    let new_product=product::Product::new("PROD-2345".to_string(), "towel".to_owned(), product_category::_ProductCategory::Clothing, 334, 34456, product_status::_ProductStatus::InStock);
    let mut inventory_new=inventory::Inventory::new();
    let add_product=inventory_new.add_product(new_product.unwrap());
    println!("product:{:?}",add_product);
    // new_product.
}

pub mod product {
    use crate::{product_category::_ProductCategory, product_status::_ProductStatus};

    #[allow(dead_code)]
    pub struct Product {
        id: String,
        name: String,
        category: _ProductCategory,
        quantity: u32,
        price: u64,
        status: _ProductStatus,
    }

    impl Product {

        pub fn sell(&mut self,amount:u32)->Result<u32,String>{
            // if amount
            if amount >self.quantity{
                return Err(String::from("amount cannot be greater than quantity"))
            }
            Ok(amount * self.price as u32)
        }
    pub fn restock(&mut self,user_quantity:u32){
        self.quantity+=user_quantity;

            let status = match user_quantity {
                x if x == 0 => _ProductStatus::OutOfStock,
                x if x <= 5 => _ProductStatus::LowStock(x),
                _ => _ProductStatus::InStock,
            };
        self.status=status;
    }
        pub fn new(
            id: String,
            name: String,
            category: _ProductCategory,
            quantity: u32,
            price: u64,
            _status: _ProductStatus,
        ) -> Result<Self, String> {
            if price == 0 {
                return Err(String::from("price cannot be empty"));
            }
            if name.is_empty() {
                return Err(String::from("name cannot be empty"));
            }
            let id_name = name.contains("PROD-");
            if !id_name || name.len() > 11 {
                return Err(String::from(
                    "id must have PROD- or length od id is greater than 10",
                ));
            }

            let status = match quantity {
                x if x == 0 => _ProductStatus::OutOfStock,
                x if x <= 5 => _ProductStatus::LowStock(x),
                _ => _ProductStatus::InStock,
            };
            Ok(Self {
                id,
                name,
                category,
                quantity,
                price,
                status,
            })
        }

    }

}
pub mod product_status {

    #[allow(dead_code)]
    pub enum _ProductStatus {
        InStock,
        LowStock(u32),
        OutOfStock,
        Discontinued,
    }

    #[allow(dead_code)]
    impl _ProductStatus {
        pub fn needs_reorder(&self) -> bool {
            match self {
                _ProductStatus::OutOfStock => {
                    println!("needs reorder:");
                    true
                }
                _ProductStatus::LowStock(n) => {
                    println!("reorder needed:{}", n);
                    true
                }
                _ProductStatus::InStock => false,
                _ProductStatus::Discontinued => false,
            }
        }
    }
}

pub mod unit_of_measure {

    pub enum _UnitOfMeasure {
        Each,
        Kilogram,
        Meter,
        Liter,
    }
}
pub mod product_category {

    #[allow(unused_variables)]
    #[allow(dead_code)]
    pub enum _ProductCategory {
        Electronics,
        Furniture,
        Clothing,
        Food,
        Books,
    }

    #[allow(dead_code)]
    impl _ProductCategory {
        pub fn to_aisle<'a>(&self) -> &'a str {
            match self {
                _ProductCategory::Electronics => "aisle 1",
                _ProductCategory::Furniture => "aisle 2",
                _ProductCategory::Clothing => "aisle 3",
                _ProductCategory::Food => "aisle 4",
                _ProductCategory::Books => "aisle 5",
            }
        }
    }
}

// impl _ProductStatus

pub mod inventory{
    // use std::iter::Product;

    use crate::product;

    #[allow(dead_code)]
    pub struct Inventory{
        products:Vec<product::Product>,
    }

    impl Inventory{
        pub fn new()->Self{
            Self { products: Vec::new() }
        }
        pub fn add_product(&mut self,collection:product::Product)->Result<(),String>{
            Ok(self.products.push(collection))

        }

    }

}
mod producr_category {}

mod error_handling {}
