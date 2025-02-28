pub struct Stack<T> {
    pub items: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn push_items(&mut self, slice: Vec<T>) {
        self.items.extend(slice);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<T> {
        self.items.last().cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
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
        let mut stack = Stack::new();
        let value = i8::MAX;

        stack.push(value);
        assert_eq!(stack.items.len(), 1);
        assert_eq!(stack.items[0], i8::MAX);
    }

    #[test]
    fn push_items() {
        let mut stack = Stack::new();
        let value = i8::MAX;

        stack.push(value);
        let items = vec![1, 2, 3];
        stack.push_items(items);
        assert_eq!(stack.items.len(), 4);
        assert_eq!(stack.items[0], i8::MAX);
        assert_eq!(stack.items[3], 3);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), None);

        let value = i8::MAX;
        stack.push(value);
        assert_eq!(stack.pop(), Some(value));
        assert!(stack.items.is_empty());
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);

        let value = i8::MAX;
        stack.push(value);
        assert_eq!(stack.peek(), Some(value));
        assert!(!stack.items.is_empty());
    }

    #[test]
    fn is_empty() {
        let stack = Stack::<i8>::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn size() {
        let mut stack = Stack::new();
        let value = i8::MAX;

        stack.push(value);
        let items = vec![1, 2, 3];
        stack.push_items(items);
        assert_eq!(stack.size(), 4);
    }

    #[test]
    fn clear() {
        let mut stack = Stack::new();
        let value = i8::MAX;

        stack.push(value);
        let items = vec![1, 2, 3];
        stack.push_items(items);

        stack.clear();
        assert!(stack.is_empty());
    }
}
