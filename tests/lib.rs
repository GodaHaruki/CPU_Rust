#[cfg(test)]
pub mod tests {
    use memory::*;
    
    #[test]
    fn test_stack(){
        let stack = stack::Stack::<&str> {
            value: "Hello",
            node: Some(Box::new(stack::Stack {
                value: "World",
                node: None,
            })),
        };
        
        assert_eq!(format!("{}, {}", stack.at(&0).unwrap(), stack.at(&1).unwrap()), "Hello, World");
    }
}
