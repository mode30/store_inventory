//
use core::fmt;
#[allow(unused_imports)]
use std::{collections::HashMap, error::Error, io, iter, num::ParseFloatError};
// fn main() {
//     println!("Hello, world!");
//
//     let new_product = product::Product::new(
//         "PROD-2345".to_string(),
//         "towel".to_owned(),
//         product_category::_ProductCategory::Clothing,
//         334,
//         34456,
//         product_status::_ProductStatus::InStock,
//     );
//     let mut inventory_new = inventory::Inventory::new();
//
//     let add_product = inventory_new.add_product(new_product.unwrap());
//     println!("product:{:?}", add_product);
//
//     let new_product_2 = product::Product::new(
//         "PROD-2342".to_string(),
//         "rug".to_owned(),
//         product_category::_ProductCategory::Clothing,
//         334,
//         243313,
//         product_status::_ProductStatus::InStock,
//     );
//     let mut inventory_new_1 = inventory::Inventory::new();
//     let add_product_1 = inventory_new_1.add_product(new_product_2.unwrap());
//     println!("product:{:?}", add_product_1);
//
//     // new_product
// }
//
// pub mod product {
//     use core::fmt;
//
//     use crate::{product_category::_ProductCategory, product_status::_ProductStatus};
//
//     #[allow(dead_code)]
//     #[derive(Debug)]
//     pub struct Product {
//         id: String,
//         name: String,
//         category: _ProductCategory,
//         quantity: u32,
//         price: u64,
//         status: _ProductStatus,
//     }
//
//     impl Product {
//         pub fn sell(&mut self, quantity: u32) -> Result<u32, String> {
//             // if amount
//             if quantity > self.quantity {
//                 return Err(String::from("amount cannot be greater than quantity"));
//             }
//             let profit = quantity * self.price as u32;
//             self.quantity = self.quantity - quantity;
//             Ok(profit)
//         }
//         pub fn restock(&mut self, user_quantity: u32) {
//             self.quantity += user_quantity;
//
//             let status = match user_quantity {
//                 x if x == 0 => _ProductStatus::OutOfStock,
//                 x if x <= 5 => _ProductStatus::LowStock(x),
//                 _ => _ProductStatus::InStock,
//             };
//             self.status = status;
//         }
//         pub fn new(
//             id: String,
//             name: String,
//             category: _ProductCategory,
//             quantity: u32,
//             price: u64,
//             _status: _ProductStatus,
//         ) -> Result<Self, String> {
//             if price == 0 {
//                 return Err(String::from("price cannot be empty"));
//             }
//             if name.is_empty() {
//                 return Err(String::from("name cannot be empty"));
//             }
//             let id_name = name.contains("PROD-");
//             if !id_name || name.len() > 11 {
//                 return Err(String::from(
//                     "id must have PROD- or length od id is greater than 10",
//                 ));
//             }
//
//             let status = match quantity {
//                 x if x == 0 => _ProductStatus::OutOfStock,
//                 x if x <= 5 => _ProductStatus::LowStock(x),
//                 _ => _ProductStatus::InStock,
//             };
//             Ok(Self {
//                 id,
//                 name,
//                 category,
//                 quantity,
//                 price,
//                 status,
//             })
//         }
//     }
//
//     impl fmt::Display for Product {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(
//                 f,
//                 "product id:{:?}\n name:{:?}\ncategory:{:?}\nprice:{:?}\nstatus:{:?}\n",
//                 self.id, self.name, self.category, self.quantity, self.price, self.status
//             )
//         }
//     }
// }
// pub mod product_status {
//     #[allow(dead_code)]
//     #[derive(Debug)]
//     pub enum _ProductStatus {
//         InStock,
//         LowStock(u32),
//         OutOfStock,
//         Discontinued,
//     }
//
//     #[allow(dead_code)]
//     impl _ProductStatus {
//         pub fn needs_reorder(&self) -> bool {
//             match self {
//                 _ProductStatus::OutOfStock => {
//                     println!("needs reorder:");
//                     true
//                 }
//                 _ProductStatus::LowStock(n) => {
//                     println!("reorder needed:{}", n);
//                     true
//                 }
//                 _ProductStatus::InStock => false,
//                 _ProductStatus::Discontinued => false,
//             }
//         }
//     }
// }
//
// pub mod unit_of_measure {
//
//     pub enum _UnitOfMeasure {
//         Each,
//         Kilogram,
//         Meter,
//         Liter,
//     }
// }
// pub mod product_category {
//
//     #[allow(unused_variables)]
//     #[allow(dead_code)]
//     #[derive(Debug)]
//     pub enum _ProductCategory {
//         Electronics,
//         Furniture,
//         Clothing,
//         Food,
//         Books,
//     }
//
//     #[allow(dead_code)]
//     impl _ProductCategory {
//         pub fn to_aisle<'a>(&self) -> &'a str {
//             match self {
//                 _ProductCategory::Electronics => "aisle 1",
//                 _ProductCategory::Furniture => "aisle 2",
//                 _ProductCategory::Clothing => "aisle 3",
//                 _ProductCategory::Food => "aisle 4",
//                 _ProductCategory::Books => "aisle 5",
//             }
//         }
//     }
// }
//
// // impl _ProductStatus
//
// pub mod inventory {
//
//     // use std::iter::Product;
//
//     #[allow(dead_code)]
//     #[allow(unused_variables)]
//     #[allow(unused_imports)]
//     use core::fmt;
//
//     #[allow(unused_imports)]
//     use std::{
//         collections::{self, HashMap},
//         fmt::write,
//     };
//
//
//     #[allow(unused_imports)]
//     use crate::{inventory, product};
//
//     #[allow(dead_code)]
//     pub struct Inventory {
//         // products:HashMap<&'a str,product::Product>,
//         products: HashMap<u32, product::Product>,
//         // products:HashMap<product::Product>,
//     }
//
//     impl Inventory {
//
//
//     impl fmt::Display for Inventory {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(f, "description:{}", self.products)
//         }
//     }
//         pub fn new() -> Self {
//             Self {
//                 products: HashMap::new(),
//             }
//         }
//         pub fn add_product(
//             &mut self,
//             collections: product::Product,
//         ) -> Result<(), inventory_errors::InventoryErrors> {
//             let result = self.products.insert(1, collections);
//             let _output = result.ok_or(String::from("cannot add product"));
//             Ok(())
//         }
//
//         // pub fn remove_product(&mut self,collection:Inventory,id:u8)->Result<Self,inventory_errors::InventoryErrors>{
//         //     let product_to_remove=collec
//         //     // let result=collection.products.iter().for_each(|elements| println!("elements:{}",elements));
//         //
//         // }
//     }
//     mod product_category {}
//
//     mod inventory_errors {
//         use core::fmt;
//         use std::{error::Error, num::ParseIntError};
//
//         #[derive(Debug)]
//         pub enum InventoryErrors {
//             ProductNotFound(String),
//             InsufficientStock { requested: u32, available: u32 },
//             DuplicateProduct(String),
//             InvalidProductData(String),
//             PriceMismatch { expected: u64, actual: u64 },
//         }
//
//         impl fmt::Display for InventoryErrors {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 match self {
//                     InventoryErrors::ProductNotFound(s) => write!(f, "product not found:{}", s),
//                     InventoryErrors::InsufficientStock {
//                         requested,
//                         available,
//                     } => {
//                         write!(f, "insufficient stock:{}:{}", requested, available)
//                     }
//                     InventoryErrors::DuplicateProduct(s) => write!(f, "product not found:{}", s),
//                     InventoryErrors::InvalidProductData(s) => write!(f, "product not found:{}", s),
//                     InventoryErrors::PriceMismatch { expected, actual } => {
//                         write!(f, "product not found:{}:{}", expected, actual)
//                     }
//                 }
//             }
//         }
//
//         impl Error for InventoryErrors {}
//
//         impl From<ParseIntError> for InventoryErrors {
//             fn from(error: ParseIntError) -> Self {
//                 InventoryErrors::InvalidProductData(error.to_string())
//             }
//         }
//     }
//
//     // impl fmt::Display for Inventory{
//     //
//     //     fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
//     //         for products in inventory::Inventory{
//     //         write!(
//     //             f,"{}"
//     //         },self.products)
//     //     }
//     // }
// }
//

#[derive(Debug)]
enum ProductCategory {
    Electronics,
    Furniture,
    Clothing,
    Food,
    Books,
}

#[derive(Debug)]
enum ProductStatus {
    InStock,
    LowStock(u32),
    OutOfStock,
    Discontinued,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Product {
    id: String,
    name: String,
    category: ProductCategory,
    quantity: u32,
    price: f64,
    status: ProductStatus,
}
// struct Inventory {
//     products: HashMap<u32, Product>,
// }

#[derive(Debug)]
#[allow(dead_code)]
enum ErrorHandling {
    ProductNotFound(String),
    InsufficientStock { requested: u32, available: u32 },
    DuplicateProduct(String),
    InvalidProductData(String),
    PriceMismatch { expected: u64, actual: u64 },
    InvalidEntry(String),
    InvalidEntryInt(std::num::ParseFloatError),
}

#[derive(Debug)]
struct Inventory {
    containers: Vec<Product>,
}
fn main() {
    let product_1 = Product::default();
    let product_2 = Product::default();
    let product_3 = Product::default();
    let product_4 = Product::default();
    let product_5 = Product::default();
    let product_6 = Product::new(
        "PROD-1234".to_string(),
        "Defacto_chair".to_owned(),
        ProductCategory::Furniture,
        1000,
        12.5,
        ProductStatus::OutOfStock,
    );

    let product_7 = Product::new(
        "PROD-4567".to_string(),
        "Defacto_table".to_owned(),
        ProductCategory::Electronics,
        50,
        12.5,
        ProductStatus::OutOfStock,
    );
    let product_8 = Product::new(
        "PROD-1234".to_string(),
        "Defacto_chair".to_owned(),
        ProductCategory::Food,
        1000,
        12.5,
        ProductStatus::OutOfStock,
    );
    let product_9 = Product::new(
        "PROD-1234".to_string(),
        "Elfwood lamp".to_owned(),
        ProductCategory::Clothing,
        1000,
        3.5,
        ProductStatus::OutOfStock,
    );

    let mut new_container = Inventory::new();
    new_container.add_products(product_9.unwrap());
    new_container.add_products(product_1);
    new_container.add_products(product_2);
    new_container.add_products(product_3);
    new_container.add_products(product_4);
    new_container.add_products(product_5);
    new_container.add_products(product_6.unwrap());
    new_container.add_products(product_7.unwrap());
    new_container.add_products(product_8.unwrap());

    for products in new_container.containers.iter() {
        println!("products:{:?}", products);
    }
}

#[allow(dead_code)]
impl ProductCategory {
    fn to_aisle<'a>(&self) -> &'a str {
        match self {
            ProductCategory::Electronics => "aisle 1",
            ProductCategory::Furniture => "aisle 2",
            ProductCategory::Clothing => "aisle 3",
            ProductCategory::Food => "aisle 4",
            ProductCategory::Books => "aisle 5",
        }
    }
}

impl Inventory {
    fn new() -> Self {
        // fn new(containers: Product) -> Self {
        Self {
            containers: Vec::new(),
        }
    }

    fn add_products(&mut self, product: Product) {
        // fn add_products(&mut self, product: Product) -> Result<(), io::Error> {
        self.containers.push(product);
        // Ok(())
    }
}
impl Product {
    fn new(
        id: String,
        name: String,
        category: ProductCategory,
        quantity: u32,
        price: f64,
        status: ProductStatus,
    ) -> Result<Self, ErrorHandling> {
        if name.is_empty() || id.is_empty() || price <= 0.0 {
            return Err(ErrorHandling::InvalidEntry("incorrect input".to_owned()));
        }

        Ok(Self {
            id,
            name,
            category,
            quantity,
            price,
            status,
        })
    }

    #[allow(dead_code)]
    fn sell(&mut self, user_quantity: i32) -> Result<(), ErrorHandling> {
        if user_quantity <= 0 {
            return Err(ErrorHandling::InvalidEntry(
                "user quantity cannot be less than or equals 0".to_owned(),
            ));
        }
        if user_quantity > self.quantity as i32 {
            return Err(ErrorHandling::PriceMismatch {
                expected: user_quantity as u64,
                actual: self.quantity as u64,
            });
        } else {
            self.quantity as i32 - user_quantity
        };

        self.status = match self.quantity {
            x if x >= 50 => {
                println!("current value:{}", x);
                ProductStatus::InStock
            }
            x if x <= 10 => {
                println!("current value:{}", x);
                ProductStatus::OutOfStock
            }
            x if x < 50 => {
                println!("current value:{}", x);
                ProductStatus::LowStock(x)
            }
            _ => ProductStatus::Discontinued,
        };
        Ok(())
    }

    #[allow(dead_code)]
    fn restock(&mut self, amount: i32) -> Result<(), ErrorHandling> {
        if amount <= 0 {
            return Err(ErrorHandling::InvalidEntry(
                "amount cannot be less than amount in stock".to_string(),
            ));
        }
        self.quantity += amount as u32;
        Ok(())
    }
}

impl Error for ErrorHandling {}

impl fmt::Display for ErrorHandling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorHandling::ProductNotFound(s) => write!(f, "missing products:{}", s),
            ErrorHandling::InsufficientStock {
                requested,
                available,
            } => write!(
                f,
                "insufficient stock:{} available:{}",
                requested, available
            ),
            ErrorHandling::DuplicateProduct(s) => write!(f, "duplicate products :{}", s),
            ErrorHandling::InvalidProductData(s) => write!(f, "missing products:{}", s),
            ErrorHandling::PriceMismatch { expected, actual } => {
                write!(f, "expected price:{}\nactual price:{}\n", expected, actual)
            }
            ErrorHandling::InvalidEntry(s) => write!(f, "meta data missing:{}", s),
            ErrorHandling::InvalidEntryInt(i) => {
                write!(f, "meta data is incorrect,wrong int input:{}", i)
            }
        }
    }
}

impl From<ParseFloatError> for ErrorHandling {
    fn from(error: ParseFloatError) -> Self {
        ErrorHandling::InvalidEntryInt(error)
    }
}

impl Default for Product {
    fn default() -> Self {
        Product {
            id: "PROD-3434".to_owned(),
            name: "rug".to_string(),
            category: ProductCategory::Furniture,
            quantity: 1300,
            price: 123.5,
            status: ProductStatus::InStock,
        }
    }
}
