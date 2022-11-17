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
    println!("{:#?}", stack);
  
    println!("{}, {}", stack.at(&0).unwrap(), stack.at(&1).unwrap());
}
