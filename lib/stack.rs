#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T: Sized> {
    pub value: T,
    pub node: Option<Box<Stack<T>>>,
}

impl<T: Sized + std::cmp::PartialEq + Clone> Stack<T> {
    // fn at(&self, index: usize) -> Result<T, String> {
    //     let n = self.clone();
    //     self.at_inner(Some(Box::new(n)), &index, 0)
    // }

    // fn at_inner(
    //     &self,
    //     st: Option<Box<Stack<T>>>,
    //     index: &usize,
    //     current_index: usize,
    // ) -> Result<T, String> {
    //     let st = st.unwrap();
    //     if *index == current_index {
    //         return Ok(st.value);
    //     } else if st.node == None && *index > current_index {
    //         return Err("Index Error".to_string());
    //     };
    //     self.at_inner(st.node, index, current_index + 1)
    // }

    pub fn at(&self, index: &usize) -> Result<T, String> {
        self.at_inner(&self, index, 0)
    }

    fn at_inner(&self, st: &Stack<T>, index: &usize, current_index: usize) -> Result<T, String> {
        if *index == current_index {
            return Ok(st.value.clone());
        } else if *index > current_index && st.node.is_none() {
            return Err("Index Error".to_string());
        }
        let st = st.node.as_ref().unwrap();
        self.at_inner(st, index, current_index + 1)
    }

    pub fn next(&self) -> &Stack<T> {
        let n = self.node.as_ref().unwrap();
        &*n
    }

    pub fn next_clone(&self) -> Stack<T> {
        let n = self.clone();
        let n = n.node.unwrap();
        *n
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn read_stack_value(){
        let stack = Stack::<&str>{
            value : "Hello",
            node : Some(Box::new(Stack::<&str>{
                value: "World",
                node : None,
            }));
        assert_eq!(format!("{}, {}" ,stack.at(&0).unwrap(), stack.at(&1).unwrap()), "Hello, World);
    }
