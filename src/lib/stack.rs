#[derive(Debug, Clone, PartialEq)]
struct Stack<T: Sized> {
    value: T,
    node: Option<Box<Graph<T>>>,
}

impl<T: Sized + std::cmp::PartialEq + Clone> Graph<T> {
    /// It will Clone Graph<T>
    fn at(&self, index: usize) -> Result<T, String> {
        let n = self.clone();
        self.at_inner(Some(Box::new(n)), &index, 0)
    }
  
    fn at_next(&self, index) -> Result<T, String>{
      at_next_inner(&self, st, index, 0)
    }
  
    fn at_next_inner(&self, st: &Graph<T>, index: &usize, current_index: usize) -> Result<T, String> {
      if *index == current_index {
        return Ok(st.value);
      } else if *index > current_index && st.node == None {
        return Err("Index Error".to_string());
      }
      let st = st.node.as_ref().unwrap();
      self.at_next_inner(&*st, index, current_index + 1)
    }

    fn at_inner(
        &self,
        st: Option<Box<Graph<T>>>,
        index: &usize,
        current_index: usize,
    ) -> Result<T, String> {
        let st = st.unwrap();
        if *index == current_index {
            return Ok(st.value);
        } else if st.node == None && *index > current_index {
            return Err("Index Error".to_string());
        };
        self.at_inner(st.node, index, current_index + 1)
    }

    fn next(&self) -> &Graph<T>{
        let n = self.node.as_ref().unwrap();
        &*n
    }

    fn next_clone(&self) -> Graph<T>{
        let n = self.clone();
        let n = n.node.unwrap();
        *n
    }
}

fn main() {
    let g = Graph {
        value: "Hello",
        node: Some(Box::new(Graph {
            value: "World",
            node: None,
        })),
    };
    println!("{:#?}", g);

    println!("{}, {}", g.at(0).unwrap(), g.at(1).unwrap()); // Maybe Faster: println!("{}, {}", g.value, g.next().value)
}
