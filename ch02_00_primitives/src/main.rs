fn main() {
    // Variables can be type annotated.
    let _rustacean: bool = true;

    // Ints and floats can be annotated regularly or using suffix annotation.
    let _installed_percentage: f32 = 0.98;
    let _installed_packages = 64_419u16;
    // Default to f64 and i32 respectively.

    // Can also be inferred (Rust features!)
    let mut _inferred_u128 = 12;
    // Compiler will infer this is u128 because of this line:
    _inferred_u128 = 69420u128;

    // Types can't be changed
    let mut _integer = 12;
    // _integer = false;

    // But they can be overshadowed.
    let _integer = false;
}
