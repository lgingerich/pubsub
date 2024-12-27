use std::collections::HashMap;

struct Message {
    topic: String,
    content: String,
}

struct Publisher {
    topic: String,
}

struct Subscriber {
    name: String,
}

struct Broker {
    topics: HashMap<String, Vec<Subscriber>>,
}

impl Publisher {
    fn new(topic: String) -> Self {
        println!("Created publisher for topic: {}", topic);
        Publisher { topic }
    }

    fn publish(&self, broker: &mut Broker, content: String) {
        println!("Published message {} to topic {}", content, self.topic);
        let message = Message {
            topic: self.topic.clone(),
            content: content.clone(),
        };
        broker.handle_message(message);
    }
}

impl Subscriber {
    fn new(name: String) -> Self {
        println!("Created subscriber: {}", name);
        Subscriber { name }
    }

    fn receive(&self, message: &Message) {
        println!(
            "Subscriber {} on topic {} received message: {}",
            self.name, message.topic, message.content
        );
    }
}

impl Broker {
    fn new() -> Self {
        Broker {
            topics: HashMap::new(),
        }
    }

    fn subscribe(&mut self, topic: String, subscriber: Subscriber) {
        self.topics.entry(topic).or_default().push(subscriber);
    }

    fn handle_message(&mut self, message: Message) {
        if let Some(subscribers) = self.topics.get(&message.topic) {
            for subscriber in subscribers {
                subscriber.receive(&message);
            }
        }
    }
}

fn main() {
    let mut broker = Broker::new();
    let publisher = Publisher::new("topic1".to_string());
    let subscriber = Subscriber::new("subscriber1".to_string());

    // Subscribe to topic
    broker.subscribe("topic1".to_string(), subscriber);

    // Publish message
    publisher.publish(&mut broker, "Hello, world!".to_string());
}
