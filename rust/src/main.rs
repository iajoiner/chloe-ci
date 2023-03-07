fn main() {
    println!("Hello, Chloe CI!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn cats_are_cats() {
        let result = "cats";
        assert_eq!(result, "cats");
    }

    #[test]
    fn cats_are_not_dogs() {
        let result = "cats";
        assert_ne!(result, "dogs");
    }
}
