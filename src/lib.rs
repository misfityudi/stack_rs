pub struct Stack<T>{
    pub items: Vec<T>
}

impl<T> Stack<T>{
    pub fn new()->Self{
        Self{
            items: Vec::new()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
       let stack = Stack::<i8>::new();
       assert!(stack.items.is_empty());
    }
}
