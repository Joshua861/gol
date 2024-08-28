use std::fs;

#[cfg(not(debug_assertions))]
use dirs::data_dir;
use lazy_static::lazy_static;
use nannou::text::Font;

#[cfg(not(debug_assertions))]
lazy_static! {
    pub static ref BASE_DIR: String = format!(
        "{}/stuff_made_by_lily/GOL",
        data_dir().unwrap().to_str().unwrap()
    );
}

#[cfg(debug_assertions)]
lazy_static! {
    pub static ref BASE_DIR: String = ".".to_string();
}

// #[macro_export]
// /// Randomly returns true or false based on the given chance.
// ///
// /// For example: `chance!(5 in 10)` has a 50% chance of returning true.
// macro_rules! chance {
//     ($one:tt in $two:tt) => {
//         $crate::utils::chance_fn($one, $two)
//     };
//     ($($_:tt)*) => {
//         panic!(
//             "Incorrect usage of the `chance` macro. \
//              Expected format: `[NUMBER] in [NUMBER]`. \
//              Example: `chance!(5 in 10)`."
//         )
//     };
// }
//
// /// Don't use this.
// pub fn chance_fn(one: usize, two: usize) -> bool {
//     rand::thread_rng().gen_range(0..=two) <= one
// }

pub fn load_font(name: &str) -> Font {
    Font::from_bytes(fs::read(format!("assets/fonts/{}.ttf", name)).unwrap()).unwrap()
}
