#[derive(Debug)]
enum Queue {
    Node(u32, Box<Queue>),
    Nil,
}

impl Queue {
    fn new() -> Queue {
        Queue::Nil
    }

    fn push(self, elem: u32) -> Queue {
        Queue::Node(elem, Box::new(self))
    }

    fn pop(self) -> (u32, Queue) {
        match self {
            Queue::Node(elem, tail) => (elem, *tail),
            Queue::Nil => {
                eprintln!("error: queue is Nil");
                (0, Queue::Nil)
            }
        }
    }

    fn len(&self) -> u32 {
        match *self {
            Queue::Node(_, ref tail) => 1 + tail.len(),
            Queue::Nil => 0,
        }
    }
}

fn main() {
    let mut queue = Queue::new();

    println!("{:?}", queue);

    queue = queue.push(1);
    println!("Push 1: {:?}", queue);

    queue = queue.push(2);
    println!("Push 2: {:?}", queue);

    queue = queue.push(3);
    println!("Push 3: {:?}", queue);

    println!("Length: {}", queue.len());

    let result = queue.pop();
    queue = result.1;
    println!("1st Pop: {} {:?}", result.0, queue);

    let result = queue.pop();
    queue = result.1;
    println!("2nd Pop: {} {:?}", result.0, queue);

    let result = queue.pop();
    queue = result.1;
    println!("3nd Pop: {} {:?}", result.0, queue);

    let result = queue.pop();
    queue = result.1;
    println!("4nd Pop: {} {:?}", result.0, queue);
}
