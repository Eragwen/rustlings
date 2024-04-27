// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        panic!("This option is none!");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    (value_a, value_b) = (value_b, value_a);
    println!("value a: {}; value b: {}", value_a, value_b);
}

// ! Writeup ! 
// Remplacement de my_option.unwrap() par panic!() pour dire que l'option est None
// Ajout de la virgule après -3 dans my_arr car oubli 
// Remplacement de la variable my_empty_vec qui voulait créer un vecteur vide,
// utilisation de Vec::new() pour créer un vecteur vide plus explicitement.
// Pour le swap de valeur, j'ai utilisé un tuple.