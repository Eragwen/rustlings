// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// Pour "blue", type &str donc on utilise string_slice car c'est une chaine littérale
// Pour "red", type String donc on utilise string car c'est une chaine allouée grâce à to_string()
// Pour "hi", type String donc on utilise string car c'est une chaine allouée grâce à String::from()
// Pour "rust is fun!", type String donc on utilise string car c'est une chaine allouée grâce à to_owned()
// Pour "nice weather", type String donc on utilise string car c'est une chaine allouée grâce à into()
// Pour "Interpolation Station", type String donc on utilise string car c'est une chaine allouée grâce à format!()
// Pour "a", type &str donc on utilise string_slice car c'est une chaine littérale. C'est une référence à une partie de la chaine grâce à &String
// Pour "hello there", type &str donc on utilise string_slice car c'est une chaine littérale, trim() renvoie une référence à une partie de la chaine
// Pour "Happy Tuesday!", type String donc on utilise string car c'est une chaine allouée grâce à to_string()
// Pour "mY sHiFt KeY iS sTiCkY", type String donc on utilise string car c'est une chaine allouée grâce à to_lowercase()