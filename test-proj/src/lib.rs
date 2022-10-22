
fn print_value(v: i32) -> i32 {
    println!("i got value {}", v);
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn add(left: usize, right: usize) -> usize {
        left + right
    }

    fn greeting(name: &str) -> String {
        format!("Hello {}", name)
    }
    #[test]
    #[ignore]
    fn test_ok() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_error() {
        let result = add(2, 2);
        panic!("make this tes fail");
    }


    #[test]
    #[should_panic(expected ="too ")]
    fn test_panic() {
        Guess::new(800);
    }

    #[test]
    fn print_ok() {
        let v = print_value(4);
        assert_eq!(10, v);
    }

    #[test]
    fn print_failure() {
        let v = print_value(8);
        assert_eq!(5, v);
    }

    #[test]
    fn test_add() {
        assert_eq!(4, add_two(2));
    }
}