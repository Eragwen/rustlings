// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    let _ = y; // Fix without deleting this line.
}

// ! Writeup ! 
// Le code ne marche pas car la valeur de y est déjà utilisée dans le match.
// Pour corriger cela, on peut utiliser '_' pour ignorer la valeur de y à la fin du match.
