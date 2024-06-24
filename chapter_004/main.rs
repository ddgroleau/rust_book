fn main() {
    { 
        let s = "string";
        println!("{s}");
    }

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    let y = s.clone();
    println!("{y} {s}");
}