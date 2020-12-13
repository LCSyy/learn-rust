// the lib crate name is: basic_crates

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { }
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() { }
        pub fn serve_order() {
            super::hosting::seat_at_table();
        }
        pub fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
}
