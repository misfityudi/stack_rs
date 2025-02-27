pub struct Stack<T>{
    pub items: Vec<T>
}

impl<T> Stack<T>{
    pub fn new()->Self{
        Self{
            items: Vec::new()
        }
    }

    pub fn push(&mut self, value: T){
        self.items.push(value);
    }
}


#[cfg(test)]
mod tests {
    use std::i8;

    use super::*;

    #[test]
    fn new() {
       let stack = Stack::<i8>::new();
       assert!(stack.items.is_empty());
    }

    #[test]
    fn push(){
        let mut stack = Stack::<i8>::new();
        let value= i8::MAX;
        
        stack.push(value);
        assert_eq!(stack.items.len(), 1);
        assert_eq!(stack.items[0], i8::MAX);
    }
}
