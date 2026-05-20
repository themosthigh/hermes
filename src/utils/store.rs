use relm4::Sender;
use std::sync::RwLock;

#[derive(Debug)]
pub struct LocalStore<T> {
    pub data: RwLock<T>,

    // Holds a pool of update senders tied to child event loops
    pub subscribers: RwLock<Vec<Sender<T>>>,
}

impl<T: Clone + 'static + std::fmt::Debug> LocalStore<T> {
    pub fn new(initial: T) -> Self {
        Self {
            data: RwLock::new(initial),
            subscribers: RwLock::new(Vec::new()),
        }
    }

    // automatic updates
    pub fn update<F>(&self, mutate: F)
    where
        F: FnOnce(&mut T),
    {
        let current_data = {
            let mut data_guard = self.data.write().unwrap();
            mutate(&mut data_guard);
            data_guard
        };

        // notify subscribers
        for subscriber in self.subscribers.read().unwrap().iter() {
            subscriber.send(current_data.clone()).unwrap();
        }
    }

    // subcribe hook for children
    pub fn subscribe(&self, sender: Sender<T>) {
        let current = self.data.read().unwrap().clone();
        let _ = sender.send(current);
        self.subscribers.write().unwrap().push(sender);
    }
}
