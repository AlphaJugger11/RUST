use std::io;

#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    fn new(name: String, description: String, price: f64, quantity: i32) -> Self {
        Product {
            name,
            description,
            price,
            quantity,
        }
    }
}

struct Inventory {
    products: Vec<Product>,
    users: Vec<(String, String)>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: Vec::new(),
            users: vec![("admin".to_string(), "password123".to_string())],
        }
    }

    fn authenticate(&self, username: &str, password: &str) -> bool {
        self.users
            .iter()
            .any(|(u, p)| u == username && p == password)
    }
    /////////////////  ADDING PRODUCT CODE     ////////////////////////
    fn add_prod(&mut self, product: Product) {
        self.products.push(product);
    }
    /////////////////  DELETING PRODUCT USING INDEX  ///////////////////
    fn del_prod(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.products.len() {
            return Err("Index out of range");
        }

        self.products.remove(index);
        Ok(())
    }
    ///          MODIFYING THE PRODUCT WALA CODE /////////////////////////
    fn mod_prod(&mut self, index: usize, new_product: Product) -> Result<(), &'static str> {
        if index >= self.products.len() {
            return Err("Index out of range");
        }

        self.products[index] = new_product;
        Ok(())
    }
    /////////////// Generating Report //////////////////////
    fn gen_report(&self) {
        println!("Inventory Report:");
        println!(
            "{:<10} {:<30} {:<10} {:<10}",
            "Name", "Description", "Price", "Quantity"
        );
        for product in &self.products {
            println!(
                "{:<10} {:<30} {:<10.2} {:<10}",
                product.name, product.description, product.price, product.quantity
            );
        }
    }
    //////////////////  UI controlling code ///////
    fn ui(&mut self) {
        loop {
            println!("1. Add Product");
            println!("2. Delete Product");
            println!("3. Modify Product");
            println!("4. Generate Report");
            println!("5. Exit");

            let choice: u32 = match get_user_input("Enter your choice:").trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };

            match choice {
                1 => {
                    let name = get_user_input("Enter product name:");
                    let description = get_user_input("Enter product description:");
                    let price: f64 = get_user_input("Enter product price:")
                        .trim()
                        .parse()
                        .unwrap_or(0.0);
                    let quantity: i32 = get_user_input("Enter product quantity:")
                        .trim()
                        .parse()
                        .unwrap_or(0);

                    let new_product = Product::new(name, description, price, quantity);
                    self.add_prod(new_product);
                    println!("Product added successfully!");
                }
                2 => {
                    self.gen_report();
                    let index: usize = get_user_input(
                        "Enter the index of the product to delete (starts from 0): ",
                    )
                    .trim()
                    .parse()
                    .unwrap_or(usize::MAX);

                    match self.del_prod(index) {
                        Ok(_) => println!("Product deleted successfully!"),
                        Err(err) => println!("Error: {}", err),
                    }
                }
                3 => {
                    self.gen_report();
                    let index: usize = get_user_input(
                        "Enter the index of the product to modify (starts from 0): ",
                    )
                    .trim()
                    .parse()
                    .unwrap_or(usize::MAX);

                    if index == usize::MAX {
                        println!("Invalid index");
                        continue;
                    }

                    let name = get_user_input("Enter new product name:");
                    let description = get_user_input("Enter new product description:");
                    let price: f64 = get_user_input("Enter new product price:")
                        .trim()
                        .parse()
                        .unwrap_or(0.0);
                    let quantity: i32 = get_user_input("Enter new product quantity:")
                        .trim()
                        .parse()
                        .unwrap_or(0);

                    let new_product = Product::new(name, description, price, quantity);

                    match self.mod_prod(index, new_product) {
                        Ok(_) => println!("Product modified successfully!"),
                        Err(err) => println!("Error: {}", err),
                    }
                }
                4 => {
                    self.gen_report();
                }
                5 => {
                    println!("Exiting the system. Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid choice, you must enter a number between 1 and 5.");
                }
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    input.trim().to_string()
}

fn main() {
    runner();
}
////////////////// Main runner function /////////////
fn runner() {
    let mut inventory = Inventory::new();

    // Basic authentication
    let username = get_user_input("Enter your username (admin): ");
    let password = get_user_input("Enter your password (password123): ");

    if inventory.authenticate(&username, &password) {
        println!("Authentication successful");
        inventory.ui();
    } else {
        println!("Authentication failed. Exiting the system. ");
    }
}
