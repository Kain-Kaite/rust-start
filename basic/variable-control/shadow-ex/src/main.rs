fn main() {
    let name = "Kain";
    println!("\nlet us practice on the Shadowing concept, My name is {name}.");

    let name = "カイン";
    println!("Now My name is {name}.\n");

    {
        println!("In the brackets. My name is {name}.\n");

        let name = "スペースを探せ";
        println!("What about NOW? My name is \"{name}\".");

        let name = "                 ";
        let name = name.len();
        println!("I pressed the Space bar {name} times.\n");
    }

    println!("Finally I came back. My name is {name}.");
    println!("Thanks very much.\n");

}