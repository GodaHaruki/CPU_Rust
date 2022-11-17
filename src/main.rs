use memory::stack::Stack as Stack;

fn main() {
    let stack = Stack::<&str> {
        value: "Hello",
        child: Box::new(Some(Stack {
            value: "World",
            child: Box::new(None),
        })),
    };
    println!("{:#?}", stack);
  
    println!("{}, {}", stack.at(0), stack.at(1));
}
