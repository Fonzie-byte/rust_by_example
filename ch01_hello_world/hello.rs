use std::io;

// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.
    println!("What's your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Could not read name");
    let name = name.trim();

    // Print text to the console.
    println!("Hello {}!", name);
    if name == "Ferris" {
        println!("You're a Rustacean!");
    } else {
        println!("I'm a Rustacean!");
    }

    // This is an example of a line comment.
    // There are two slashes at the beginning of the line.
    // And nothing written after these will be read by the compiler.

    println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But block comments
     * are extremely useful for temporarily disabling chunks of code.
     * /* Block comments can be /* nested, */ */ so it takes only a few
     * keystrokes to comment out everything in this main() function.
     * /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + 90 /* + 5 */;
    println!("Is `x` 10 or 100? x = {}", x);

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    let month = "February";
    let days = match month {
        "April" | "June" | "September" | "November" => 30,
        "February" => 28,
        _ => 31,
    };
    println!("{} days", days);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject}は {object}が {verb}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base {b:<16} {}", 7878, b = "10"); // 7878
    println!("Base {b:<16} {:b}", 7878, b = "2 (binary)"); // 1111011000110
    println!("Base {b:<16} {:o}", 7878, b = "8 (octal)"); // 17306
    println!("Base {b:<16} {:x}", 7878, b = "16 (hexadecimal)"); // 1ec6
    println!("Base {b:<16} {:X}", 7878, b = "16 (HEXADECIMAL)"); // 1EC6

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
                                          // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:>length$}", number = 1, length = 4);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", name);
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // DONE ^ Tried uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1.3", 4 white spaces and a "1.3".
    let number: f64 = 1.3;
    let length: usize = 7;
    println!("{number:>length$}");

    let pi = 3.141592653589;
    println!("Pi is roughly {pi:.3}");
}
