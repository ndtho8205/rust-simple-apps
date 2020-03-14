#[derive(Debug)]
struct Product {}

#[derive(Debug)]
struct Config {
    debug_mode: bool,
}

#[derive(Debug)]
struct ProductService<'a> {
    config: &'a Config,
}
impl<'a> ProductService<'a> {
    fn new(config: &Config) -> ProductService {
        ProductService { config }
    }

    fn get_product(&self, id: i32) -> Product {
        if self.config.debug_mode {
            println!("retrieving product for id {}", id)
        }

        Product {}
    }
}

#[derive(Debug)]
struct BasketService<'a> {
    config: &'a Config,
}
impl<'a> BasketService<'a> {
    fn new(config: &Config) -> BasketService {
        BasketService { config }
    }

    fn add_product(&self, product: Product) {
        if self.config.debug_mode {
            println!("adding product {:?}", product);
        }
    }
}

fn main() {
    let config = Config { debug_mode: true };
    let product_service = ProductService::new(&config);
    let basket_service = BasketService::new(&config);

    let product = product_service.get_product(1);
    basket_service.add_product(product);

    println!("{:?}", product_service);
    println!("{:?}", basket_service);
}
