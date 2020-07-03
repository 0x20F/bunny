pub trait Book {
    fn url_encode(a: &str) {
        println!("You encoded a thing {}", a);
    }
}


pub struct Github {}

impl Book for Github {}