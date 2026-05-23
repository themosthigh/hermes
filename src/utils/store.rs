use relm4::{ComponentSender, Sender, SimpleComponent};
use std::fmt::Debug;
use std::sync::RwLock;

#[derive(Debug)]
pub struct LocalStore<T> {
    pub data: RwLock<T>,
    pub source: RwLock<String>,

    // Holds a pool of update senders tied to child event loops
    pub subscribers: RwLock<Vec<Sender<T>>>,
}

impl<T: Clone + 'static + Debug + Send> LocalStore<T> {
    pub fn new(initial: T) -> Self {
        Self {
            data: RwLock::new(initial),
            subscribers: RwLock::new(Vec::new()),
            source: RwLock::new(String::new()),
        }
    }

    pub fn get_current(&self) -> T {
        self.data.read().unwrap().clone()
    }

    pub fn get_source(&self) -> String {
        self.source.read().unwrap().clone()
    }

    // Automatic updates
    pub fn update<F>(&self, mutate: F)
    where
        F: FnOnce(&mut T),
    {
        let current_data = {
            let mut data_guard = self.data.write().unwrap();
            mutate(&mut data_guard);
            data_guard
        };

        // Notify subscribers
        for subscriber in self.subscribers.read().unwrap().iter() {
            subscriber.send(current_data.clone()).unwrap();
        }
    }

    pub fn update_source(&self, source: String) -> &Self {
        {
            let mut data_guard = self.source.write().unwrap();
            *data_guard = source;
        };
        self
    }

    // Subcribe hook for children
    pub fn subscribe(&self, sender: Sender<T>) {
        let current = self.data.read().unwrap().clone();
        let _ = sender.send(current);
        self.subscribers.write().unwrap().push(sender);
    }

    // Connect to a child
    pub fn connect<C>(
        &self,
        sender: &ComponentSender<C>,
        mut map_msg: impl FnMut(T) -> C::Input + Send + 'static,
    ) where
        C: SimpleComponent,
    {
        // Create a channel to send data to the child
        let (channel_sender, channel_receiver) = relm4::channel::<T>();

        // Subscribe to the store
        let current_data = self.data.read().unwrap().clone();
        let _ = channel_sender.send(current_data);
        self.subscribers.write().unwrap().push(channel_sender);

        // Send messages to the child
        let input_sender = sender.input_sender().clone();
        relm4::spawn_local(async move {
            while let Some(data) = channel_receiver.recv().await {
                let input = map_msg(data);
                let _ = input_sender.send(input);
            }
        });
    }
}
