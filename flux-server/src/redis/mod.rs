use redis::{Client, Commands, PubSub};
use std::sync::Arc;

pub struct RedisClient {
    client: Arc<Client>,
}

impl RedisClient {
    pub fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(url)?;
        Ok(RedisClient {
            client: Arc::new(client),
        })
    }

    pub fn publish(&self, channel: &str, message: &str) -> Result<(), redis::RedisError> {
        let mut con = self.client.get_connection()?;
        con.publish(channel, message)?;
        Ok(())
    }

    pub fn subscribe<F>(&self, channel: &str, callback: F) -> Result<(), redis::RedisError>
    where
        F: Fn(String) + Send + 'static,
    {
        let mut con = self.client.get_connection()?;
        // let mut pubsub = con.as_pubsub();
        // pubsub.subscribe(channel)?;

        // std::thread::spawn(move || loop {
        //     let msg = pubsub.get_message().unwrap();
        //     let payload: String = msg.get_payload().unwrap();
        //     callback(payload);
        // });

        Ok(())
    }
}
