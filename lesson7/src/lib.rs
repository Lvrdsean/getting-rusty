use rand::{CryptoRng, ErrorKind::Transient, Rng};

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
