// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}

// ! Writeup !
// Il y a 3 tests dans ce fichier : 
// - correct_width_and_height : vérifie la largeur et la hauteur du rectangle, 
//      ajout de rect.width et rect.height dans les assert_eq pour vérifier que les valeurs sont correctes
// - negative_width : vérifie si le programme plante quand on crée un rectangle avec une largeur négative,
//      ajout de #[should_panic] pour vérifier que le programme plante bien
// - negative_height : vérifie si le programme plante quand on crée un rectangle avec une hauteur négative,
//      ajout de #[should_panic] pour vérifier que le programme plante bien