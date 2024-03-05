// // // fn main() {
// // //     let s1 = String::from("hello ");
// // //     let s2: String = String::from("world");
// // //     let s3 = concatenate_strings(s1, s2);
// // //     println!("{}", s3);
// // // }

// // // fn concatenate_strings(s1: String, s2: String) -> String {
// // //     let mut result: String = String::from("");
// // //     result.push_str(&s1);
// // //     result.push_str(&s2);
// // //     result
// // // }
// // fn main() {
// //     let book = Book {
// //         title: String::from("The value of shit"),
// //         author: String::from("Nigga singh"),
// //         publication_year: 1969,
// //     };
// //     println!(
// //         "The book '{}' was written by {} in the year {}",
// //         book.title, book.author, book.publication_year
// //     );

// //     let mut book = Book {
// //         title: String::from("The value of shit"),
// //         author: String::from("Nigga singh"),
// //         publication_year: 1969,
// //     };
// //     book.publication_year = 1996;
// //     println!(
// //         "The book '{}' was written by {} in the year {}",
// //         book.title, book.author, book.publication_year
// //     );

// //     let book_data = get_book_data(book);
// //     for data in book_data {
// //         println!("{data}");
// //     }

// //     let book = create_book(String::from("Joe mama"), String::from("Joe daddy"), 1969);
// //     // let book_data = get_book_data(b1);
// //     // for data in book_data {
// //     //     println!("{data}");
// //     // }
// //     println!("{:?}", book);
// // }

// // fn create_book(title: String, author: String, publication_year: u32) -> Book {
// //     let book = Book {
// //         title,
// //         author,
// //         publication_year,
// //     };
// //     book
// // }
// // #[derive(Debug)]
// // struct Book {
// //     title: String,
// //     author: String,
// //     publication_year: u32,
// // }

// // fn get_book_data(book: Book) -> [String; 3] {
// //     let title = book.title;
// //     let author = book.author;
// //     let pub_year = book.publication_year;

// //     let data: [String; 3] = [title, author, pub_year.to_string()];
// //     data
// // }
// fn main() {
//     let current_weather = Weather::Sunny;
//     let msg = Message::Write(String::from("Hello Rusty ass Hoe"));
//     process_messaging(msg);

//     let mypet = Animal::Cat("Melo".to_string());
//     if let Animal::Cat(name) = mypet {
//         println!("My car name is {name}");
//     } else {
//         println!("My pet ni koshka");
//     }

//     let msg = Message::Write(String::from("Nigga is sleeping"));
//     msg.call()
// }

// enum Weather {
//     Sunny,
//     Cloudy,
//     Rainy,
//     Snowy,
// }

// enum Animal {
//     Dog(String),
//     Cat(String),
//     Bird(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn process_messaging(msg: Message) {
//     match msg {
//         Message::Quit => {
//             println!("THe quit variant has no data");
//         }
//         Message::Move { x, y } => {
//             println!("Move to coordinates x: {x}, y:{y}");
//         }
//         Message::Write(text) => {
//             println!("Text message:{text}");
//         }
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to {r} ,{g},{b}");
//         }
//     }
// }

// impl Message {
//     fn call(&self) {
//         match self {
//             Message::Quit => println!("Quit"),
//             Message::Move { x, y } => println!("Move to {x} and {y}"),
//             Message::Write(text) => {
//                 println!("shit {text}")
//             }
//             Message::ChangeColor(r, g, b) => {
//                 println!("colors {r},{g},{b}")
//             }
//         }
//     }
// }

fn main() {
    // enum Options<T> {
    //     Some(T),
    //     None,
    // }
    let num = 12.0;
    let sqrot = find_sq_root(num);
    match sqrot {
        Some(value) => println!("SQrt of {num} is {value}"),
        None => {
            println!("It ain't doin shit")
        }
    }
    let a = 10.;
    let b = 0.;
    let divres = divide(a, b);
    match divres {
        Ok(res) => {
            println!("{a} divide by {b} is {res}")
        }
        Err(mesg) => {
            println!("{}", mesg)
        }
    }
}

fn find_sq_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero not possible".to_string())
    } else {
        Ok(a / b)
    }
}
