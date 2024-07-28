fn main() {
    // Unsigned ints overflow (or throw error when debugging, luckily)
    // println!("1 - 2 = {}", 1u32 - 2);

    println!("My power level is over {}!!", 9e3);
    // Scientific notation must be a float, so the following line doesn't work.
    // let power_level: i32 = 9e3;

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // both bits true
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // either bit true
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // one bit true
    println!("1 LEFT_SHIFT 5 is {}", 1u32 << 5); // im too smooth brain
    println!("0x80 RIGHT_SHIFT 2 is 0x{:x}", 0x80u32 >> 2); // for this low-level stuff

    // Underscores are 'ignored', to improve legibility.
    println!("One miljard can be written as {}.", 1_000_000_000);
}
