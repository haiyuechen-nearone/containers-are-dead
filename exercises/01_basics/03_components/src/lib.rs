use std::ops::Add;

// Generates Rust bindings for the WIT world at compile time.
wit_bindgen::generate!({
    path: "wit",
    world: "example",
    generate_all,
});

// This component should implement the interface laid out in the WIT file.
// If you get stuck, ask a neighbor or grab a trainer!
struct Component;

// #[unsafe(no_mangle)]
// unsafe extern "C" fn add_random(num: u64) -> u64 {
//     let r: u64 = wasi::random::random::get_random_u64();
//     num + r
// }

impl Guest for Component {
    fn add_random(num: u64) -> u64 {
        let r: u64 = wasi::random::random::get_random_u64();
        num.saturating_add(r)
    }
}

export!(Component);
