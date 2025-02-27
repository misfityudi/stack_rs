pub struct Stack<T> {
    pub items: Vec<T>,
}

impl<T:Clone> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<T> {
        self.items.last().cloned()
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
    fn push() {
        let mut stack = Stack::<i8>::new();
        let value = i8::MAX;

        stack.push(value);
        assert_eq!(stack.items.len(), 1);
        assert_eq!(stack.items[0], i8::MAX);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::<i8>::new();
        assert_eq!(stack.pop(), None);

        let value = i8::MAX;
        stack.push(value);
        assert_eq!(stack.pop(), Some(value));
        assert!(stack.items.is_empty());
    }

    #[test]
    fn peek() {
        let mut stack = Stack::<i8>::new();
        assert_eq!(stack.peek(), None);

        let value = i8::MAX;
        stack.push(value);
        assert_eq!(stack.peek(), Some(value));
        assert!(!stack.items.is_empty());
    }
}
