// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

// ! Writeup ! 
// Le problème ici est que la struct Book<'a> a des références "author" et "title"
// qui n'avait pas de durée de vie définie. Donc, elle pouvait être détruite avant l'utilisation de book.title et book.author.
// Pour le résoudre, ajout de la durée de vie 'a à la struct Book donc aussi à ses champs author et title
// pour indiquer que author et title doivent vivre au moins aussi longtemps que la struct Book. 