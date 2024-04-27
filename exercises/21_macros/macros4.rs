// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}

// ! Writeup !
// La macro `my_macro!` est définie avec deux règles,
// le problème est que les règles n'ont pas de séparateur entre elles.
// Ajout d'un point-virgule après chaque règle pour les séparer