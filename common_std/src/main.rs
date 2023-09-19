#[derive(Clone)]
struct Data {
    data: Box<[f64; 1024]>,
}

/*
impl Data {
    fn new(value: f64) -> Data {
        Data{data: Box<[value, 1024]>}
    }
}*/

use std::rc::Rc;

#[derive(Clone)]
struct Offer {
    product: Rc<String>
}

impl Offer {
    fn promote(&self) {
        println!("Buy {}!", self.product);
        println!("{} offers remaining", Rc::strong_count(&self.product));
    }
}

fn main() {

    let text = String::from("Nuka-Cola");
    {
        let rc_text = Rc::new(text.clone());

        let offer1 = Offer{product: Rc::clone(&rc_text)};

        {
            let offer2 = offer1.clone();
            offer2.promote();
        }
        offer1.promote();
    }

    println!("{}", text);

    let data = Data { data: Box::new([8.; 1024]) };
    process_data(data);

}
fn process_data(data: Data) {
    println!("Processed the data");
}

