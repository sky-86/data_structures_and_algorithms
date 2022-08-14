mod test;

struct MyCircularQueue {
    // head and tail will point to elements in array
    head: Option<usize>,
    tail: Option<usize>,
    queue: Vec<Option<i32>>,
    length: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            head: None,
            tail: None,
            queue: vec![None; k as usize],
            length: k as usize,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        use std::cmp::Ordering;
        if self.is_full() {
            return false
        }

        // Move tail; Insert
        match self.tail {
            Some(mut tail) => {
                tail = match tail.cmp(&(self.length-1)) {
                    Ordering::Less => tail + 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => panic!("Error: tail is greater than length"),
                };
                self.tail = Some(tail);
                self.queue[tail] = Some(value);
            },
            None => {
                self.queue[0] = Some(value);
                self.tail = Some(0);
            }
        }
        true
    }

    fn de_queue(&mut self) -> bool {
        use std::cmp::Ordering;
        if self.is_empty() {
            return false
        }
        // remove from head; move head
        match self.head {
            Some(mut head) => {
                self.queue[head] = None;
                head = match head.cmp(&(self.length-1)) {
                    Ordering::Less => head + 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => panic!("Error: head is greater than length"),
                };
                self.head = Some(head);
            },
            None => {
                self.queue[0] = None;
                self.head = Some(1);
            }
        }

        if self.is_empty() {
            self.head = None;
            self.tail = None;
        }

        true
    }

    fn front(&self) -> i32 {
        let head = self.head.unwrap_or(0);
        self.queue[head].unwrap_or(-1)
    }

    fn rear(&self) -> i32 {
        let tail = self.tail.unwrap_or(0);
        self.queue[tail].unwrap_or(-1)
    }

    fn is_empty(&self) -> bool {
        for i in &self.queue {
            if i.is_some() {
                return false
            }
        }
        true
    }

    fn is_full(&self) -> bool {
        for i in &self.queue {
            if i.is_none() {
                return false
            }
        }
        true
    }
}
