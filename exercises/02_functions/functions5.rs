// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}

// ! Writeup !
// Il manquait le mot clé return pour retourner la valeur de num * num 
// mais possible aussi de juste enlever le ";"