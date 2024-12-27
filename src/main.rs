/*
Publisher                 Message Broker                 Subscriber
  |                          |                             |
  |                          |    subscribe(topic)         |
  |                          |<----------------------------|
  |                          |                             |
  |                          |    confirm subscription     |
  |                          |---------------------------->|
  |                          |                             |
  |    publish(topic,msg)    |                             |
  |------------------------->|                             |
  |                          |                             |
  |                          |    notify(msg)              |
  |                          |---------------------------->|
  |                          |                             |
  |                          |    acknowledge              |
  |                          |<----------------------------|
  |                          |                             |
  |    confirm publish       |                             |
  |<-------------------------|                             |
  |                          |                             |
  |                          |    unsubscribe(topic)       |
  |                          |<----------------------------|
  |                          |                             |
  |                          |    confirm unsubscribe      |
  |                          |---------------------------->|
  |                          |                             |
*/

struct Broker {

}

impl Broker {
    fn new() -> Self {
        Self {

        }
    }

}

trait Publisher {
    fn publish() {

    }
}

trait Subscriber {
    fn subscribe() {

    }

    fn unsubscribe() {

    }
}


fn main() {
    let broker = Broker::new();
}
