use lazy_static::lazy_static;
use rand::Rng;

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

#[macro_export]
/// Randomly returns true or false based on the given chance.
///
/// For example: `chance!(5 in 10)` has a 50% chance of returning true.
macro_rules! chance {
    ($one:tt in $two:tt) => {
        $crate::utils::chance_fn($one, $two)
    };
    ($($_:tt)*) => {
        panic!(
            "Incorrect usage of the `chance` macro. \
             Expected format: `[NUMBER] in [NUMBER]`. \
             Example: `chance!(5 in 10)`."
        )
    };
}

/// Don't use this.
pub fn chance_fn(one: usize, two: usize) -> bool {
    rand::thread_rng().gen_range(0..=two) <= one
}

#[macro_export]
macro_rules! time {
    ($name:expr, $block:block) => {
        {
            #[cfg(debug_assertions)]
            let timer = std::time::Instant::now();
            $block
            #[cfg(debug_assertions)]
            println!("Timer '{}' took {}ms", $name, timer.elapsed().as_millis());
        }
    };
}

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
