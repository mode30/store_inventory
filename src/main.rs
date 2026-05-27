fn main() {
    println!("Hello, world!");
}

pub mod inventory_struct {
    #[allow(dead_code)]
    pub struct Product {
        id: String,
        name: String,
        category: _ProductCategory,
        quantity: u32,
        price: u64,
        status: _ProductStatus,
    }

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
    pub enum _ProductStatus {
        InStock,
        LowStock(u32),
        OutOfStock,
        Discontinued,
    }

    pub enum _UnitOfMeasure {
        Each,
        Kilogram,
        Meter,
        Liter,
    }

    // impl _ProductStatus
}

mod enum_implementation_block {
    use crate::inventory_struct::_ProductCategory;
    use crate::inventory_struct::_ProductStatus;
    use crate::inventory_struct::Product;

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

    impl Product {
        pub fn new()->Result<Self,{
            Self{

            }
        }
    }
}

mod producr_category {}

mod error_handling {}
