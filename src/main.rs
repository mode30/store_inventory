fn main() {
    println!("Hello, world!");
}

pub mod product {
    use crate::{inventory_struct::_ProductCategory, product_status::_ProductStatus};

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
        pub fn new(
            id: String,
            name: String,
            category: _ProductCategory,
            quantity: u32,
            price: u64,
            status: _ProductStatus,
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
        // pub fn change_status(&mut self,status:_ProductStatus){
        //     match status{

        //     }
        // }
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

mod producr_category {}

mod error_handling {}
