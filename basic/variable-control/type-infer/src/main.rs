fn main() {
    let need_clue = "24";
    println!("\nThis time we'll infer and parse our String Value.\nThe Value is {need_clue}.\n");

    let need_clue: u32 = need_clue.parse().expect("Not a number!");
    println!("This is the answer, {need_clue}.");
    println!("We made it. Thanks for everyone.\n");
}
