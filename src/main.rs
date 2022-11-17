use memory::stack::Stack as Stack;

fn main() {
    let stack = Stack::<&str> {
        value: "Hello",
        node: Some(Box::new(Stack {
            value: "World",
            node: None,
        })),
    };
    println!("{:#?}", stack);
  
    println!("{}, {}", stack.at(&0).unwrap(), stack.at(&1).unwrap());
}
