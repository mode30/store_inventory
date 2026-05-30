pub mod pos {

    pub struct Customer {
        id: String,
        name: String,
        email: String,
        phone: Option<String>,
        loyalty_points: u32,
        // purchase_history: Vec<Transaction>,
    }

    impl Customer {
        pub fn new(
            id: String,
            name: String,
            email: String,
            phone: Option<String>,
            loyalty_points: u32,
        ) -> Self {
            Self {
                id,
                name,
                email,
                phone,
                loyalty_points,
            }
        }
    }
}

// use pos::*;

//
use std::{
    fmt::{self},
    // io::Result,
    // ops::Add,
};
// use std::fmt::{Display, Formatter};
#[allow(unused_imports)]
use std::{collections::HashMap, error::Error, io, iter, num::ParseFloatError};

#[derive(Debug)]
// #[derive(Debug)]
#[derive(PartialEq, Clone)]
enum ProductCategory {
    Electronics,
    Furniture,
    Clothing,
    Food,
    Books,
}

// #[derive(Debug)]

#[derive(Debug, PartialEq, Clone)]
enum ProductStatus {
    InStock,
    LowStock(u32),
    OutOfStock,
    Discontinued,
}

#[derive(Debug, PartialEq, Clone)]
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

    let product_10 = Product::new(
        "PROD-1234".to_string(),
        "Iron man".to_owned(),
        ProductCategory::Books,
        1001,
        83.5,
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
    new_container.add_products(product_10.unwrap());

    for (index, products) in new_container.containers.iter().enumerate() {
        println!("{}. products:{}", index, products);
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

    #[allow(dead_code)]
    fn add_products(&mut self, product: Product) {
        // fn add_products(&mut self, product: Product) -> Result<(), io::Error> {
        self.containers.push(product);
        // Ok(())
    }

    #[allow(dead_code)]
    fn get_all_product(&self) -> Result<(), ErrorHandling> {
        // fn get_all_product(&self,containers:Inventory)->Result<(),ErrorHandling>{
        if self.containers.is_empty() {
            return Err(ErrorHandling::InvalidEntry(
                " product list empty".to_string(),
            ));
        }
        for (index, products) in self.containers.iter().enumerate() {
            println!("{}.{}", index, products);
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn get_product(&self) -> Result<bool, ErrorHandling> {
        let all_product = &self.get_all_product()?;
        println!("product list:{:?}", all_product);
        println!("enter number of product to get");
        let user_input_result =
            user_input::atoi32("enter number of product to get").unwrap_or_default();
        let user_input_result = user_input_result as usize;
        let option_result = self.containers.get(user_input_result);
        option_result
            .ok_or_else(|| {
                ErrorHandling::InvalidEntry("error cannot find product at this number".to_owned())
            })
            .map(|product| {
                println!("found product: {:?}", product);
                true
            })

        // option_result.ok_or_else(||ErrorHandling::InvalidEntry(String::from("Invalid entry".to_owned()).map
        // let option_result = self.containers.get(user_input_result).ok_or(ErrorHandling::InvalidEntry(String::from("invalid entry".to_string())))
        // option_result.ok_or_else(||ErrorHandling::)
    }

    #[allow(dead_code)]
    fn clear_db(&mut self) -> Result<(), ErrorHandling> {
        if self.containers.is_empty() {
            return Err(ErrorHandling::ProductNotFound(String::from(
                "field empty and already cleared",
            )));
        }
        self.containers.clear();
        println!("db cleared");
        Ok(())
    }

    #[allow(dead_code)]
    fn remove_product(&mut self) -> Result<Product, ErrorHandling> {
        let all_product = &self.get_all_product()?;
        println!("product list:{:?}", all_product);

        let buffer = user_input::atoi32("enter product to edit").unwrap();
        // let buffer = buffer as usize;
        let num_usize = buffer as usize;
        if num_usize > self.containers.len() {
            return Err(ErrorHandling::ProductNotFound(String::from(
                "error product not found",
            )));
        }
        // let user_input = &self.containers.get(num_usize ).copied();
        let result = self.containers.remove(num_usize);
        Ok(result)
    }

    // fn search_by_id(&self, user_input:&'static str)->Result<&Product, String>
    // fn search_by<T>(&self, user_input: &T) -> Result<&T, ErrorHandling>
    // {
    //     let result=self.containers.iter().find(|id| id==user_input);
    //     result.ok_or(String::from("not found"));

    //     // self.containers
    //     //     .iter()
    //     //     .con
    //     //     // .find_map(|&product| Some(product == user_input))
    //     //     .ok_or(ErrorHandling::InvalidEntry(String::from("not found")));
    //     // Ok(result)
    // }
    fn search_by<F>(&self, user_input: F) -> Option<&Product>
    // fn search_by<T>(&self, user_input: &T) -> Result<&T, ErrorHandling>
    where
        F: Fn(&Product) -> bool,
    {
        self.containers.iter().find(|&product| user_input(product))
        // .ok_or(ErrorHandling::InvalidEntry(String::from("not found")));
        // .find_map(|&product| Some(product == user_input))
        // Ok(result)
    }

    fn filter_by_status(&self, status: ProductStatus) -> Vec<&ProductStatus> {
        self.containers
            .iter()
            .filter(|product| product.status == status)
            .map(|product| &product.status)
            .collect()
    }

    fn update_product(&mut self) -> Result<(), ErrorHandling> {
        let all_product = self.get_all_product();
        println!("products:{:?}", all_product);

        let user_key_input = user_input::atoi32("enter row to edit").unwrap_or(1);
        let user_key_input = user_key_input as usize;
        let id = user_input::user_input("enter id in string and in the format PROD-***")
            .unwrap_or_default();
        let name =
            user_input::user_input("enter name of product").unwrap_or("empty entry".to_owned());
        let category = ProductCategory::Electronics;
        let quantity = user_input::atoi32("enter number").unwrap_or(0);
        let quantity = quantity as u32;
        let price = user_input::atof64("enter price:").unwrap_or_default();
        let status = ProductStatus::InStock;
        let new_product =
            Product::new(id, name, category, quantity, price, status).unwrap_or_default();
        self.containers.insert(user_key_input, new_product);

        Ok(())
        // Ok()
    }

    fn total_inventory(&self) {
        self.containers
            .iter()
            .for_each(|product| println!("product:{}", product));
    }

    // fn total_inventory(&self)->Result<f64,ErrorHandling>{
    //
    //     let mut product_price_sum=0;
    //     let mut product_quantity_sum=0;
    //     for (_,products) in self.containers.iter().enumerate(){
    //
    //         product_price_sum+=products.price;
    //         product_quantity_sum+=products.quantity
    //     }
    //
    //     let product_total=product_price_sum  as f64* product_quantity_sum as f64;
    //     Ok(product_total)
    //
    // }
    // fn sort_by__name_len(&self)->Result<(),ErrorHandling>{
    //     let result=&self.containers.sort_by_key(|product| product.name.len());
    //     // Ok(())
    //     // Ok(result)
    // }
    //
    // fn sort_by__id_len(&self)->Result<(),ErrorHandling>{
    //     let result=&self.containers.sort_by_key(|product| product.id.len());
    //     // Ok(())
    //     // Ok(result)
    // }

    fn sort_id(&self) -> Result<Vec<Product>, ErrorHandling> {
        if self.containers.is_empty() {
            return Err(ErrorHandling::ProductNotFound(
                "no products in fields".to_string(),
            ));
        }
        let mut owned_product: Vec<Product> = self.containers.to_vec();
        owned_product.sort_by_key(|product| product.id.len());
        // owned_product.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(owned_product)
    }
    fn sort_name(&self) -> Result<Vec<Product>, ErrorHandling> {
        let mut owned_product: Vec<Product> = self.containers.to_vec();
        // let mut owned_product: Vec<Product> = self.containers.iter().cloned().collect();
        // owned_product.sort_by(|a, b| a.name.len().cmp(&b.name.len()));
        owned_product.sort_by_key(|product| product.name.len());
        Ok(owned_product)
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
        // let contains_prod = name.contains("PROD");
        // if !contains_prod {
        //     return Err(ErrorHandling::InvalidEntry(
        //         "name must contain PROD-****".to_owned(),
        //     ));
        // }

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
            self.quantity = (self.quantity - user_quantity as u32) as u32
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

    // fn bulk add(){
    //
    // }

    // fn search_by<T>(&self,user_input:T)->Result<&T,ErrorHandling>
    //     where
    //     T:PartialEq
    // {
    //     let result=&self.containers.iter().f

    // }
    fn update_single_product(&mut self, name: String, price: f64) -> Result<(), ErrorHandling> {
        if name.is_empty() {
            return Err(ErrorHandling::InvalidEntry(
                "name cannot be emoty".to_string(),
            ));
        }
        if price <= 0.0 {
            return Err(ErrorHandling::InvalidEntry(
                "price cannot be lower or equals to 0".to_owned(),
            ));
        }

        self.name = name;
        self.price = price;
        Ok(())
        // Self {..,name,price}
    }

    fn total_cost(&self) -> Result<f64, ErrorHandling> {
        let result = self.quantity as f64 * self.price;
        Ok(result)
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
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id:{:?}\nname:{:?}\n,category:{:?}\n,quantity:{:?}\n,price:{:?}\n,status:{:?}\n",
            self.id, self.name, self.category, self.quantity, self.price, self.status
        )
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "inventory list")?;
        for (index, products) in self.containers.iter().enumerate() {
            writeln!(f, "{}.{}", index, products)?
        }
        Ok(())
    }
}

mod user_input {
    use std::io;

    // use crate::user_input;

    #[allow(dead_code)]
    pub fn user_input(prompt: &'static str) -> Result<String, io::Error> {
        println!("{}", prompt);
        let mut user_input = String::new();
        if user_input.is_empty() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "nan".to_string()));
        }
        std::io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();
        Ok(user_input.to_string())
    }

    #[allow(dead_code)]
    pub fn atoi32(prompt: &'static str) -> Result<i32, std::num::ParseIntError> {
        let buffer = user_input(prompt).unwrap_or_default();
        let buffer_converted = buffer.trim().parse::<i32>()?;
        // .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "nan".to_string()))?;
        Ok(buffer_converted)
    }
    pub fn atof64(prompt: &'static str) -> Result<f64, std::num::ParseFloatError> {
        let buffer = user_input(prompt).unwrap_or_default();
        let buffer_converted = buffer.trim().parse::<f64>()?;
        Ok(buffer_converted)
    }
}
