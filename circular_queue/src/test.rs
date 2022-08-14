#[cfg(test)]
mod tests {
    use crate::MyCircularQueue as Queue;

    #[test]
    fn example1() {
        let mut queue = Queue::new(3);
        assert!(queue.en_queue(1));
        assert!(queue.en_queue(2));
        assert!(queue.en_queue(3));
        assert!(!queue.en_queue(4));
        assert_eq!(3, queue.rear());
        assert!(queue.is_full());
        assert!(queue.de_queue());
        assert!(queue.en_queue(4));
        assert_eq!(4, queue.rear());
    }

    #[test]
    fn example2() {
        let mut queue = Queue::new(6);
        assert!(queue.en_queue(6));
        assert_eq!(6, queue.rear());
        assert_eq!(6, queue.rear());
        assert!(queue.de_queue());
        assert!(queue.en_queue(5));
        assert_eq!(5, queue.rear());
        assert!(queue.de_queue());
        assert_eq!(-1, queue.front());
        assert!(!queue.de_queue());
        assert!(!queue.de_queue());
        assert!(!queue.de_queue());
    }

    #[test]
    fn example3() {
        let mut queue = Queue::new(2);
        assert!(queue.en_queue(1));
        assert!(queue.en_queue(2));
        assert!(queue.de_queue());
        assert!(queue.en_queue(3));
        assert!(queue.de_queue());
        assert!(queue.en_queue(3));
        assert!(queue.de_queue());
        assert!(queue.en_queue(3));
        assert!(queue.de_queue());
        assert_eq!(3, queue.front());
    }

}
