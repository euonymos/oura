use std::thread::sleep;
use std::time::Duration;

use kafka::{client::RequiredAcks, producer::Producer};
use serde::Deserialize;

use crate::{
    pipelining::{BootstrapResult, SinkProvider, StageReceiver},
    utils::WithUtils,
};

use super::run::producer_loop;

#[derive(Debug, Deserialize, Clone)]
pub enum PartitionStrategy {
    ByBlock,
    Random,
}

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub brokers: Vec<String>,
    pub topic: String,
    pub ack_timeout_secs: Option<u64>,
    pub paritioning: Option<PartitionStrategy>,
}

impl SinkProvider for WithUtils<Config> {
    fn bootstrap(&self, input: StageReceiver) -> BootstrapResult {

        let retry = || -> Result<Producer, String> {
            for _ in 1..10 {
                let mut builder = Producer::from_hosts(self.inner.brokers.clone());

                if let Some(timeout) = self.inner.ack_timeout_secs {
                    builder = builder
                        .with_ack_timeout(Duration::from_secs(timeout))
                        .with_required_acks(RequiredAcks::One)
                };

                match builder.create() {
                    Ok(producer) => return Ok(producer),
                    Err(err) => {
                        println!("error while creating producer: {}", err);
                        println!("waiting 3 secs...");
                        sleep(Duration::from_secs(3))
                    }
                };
            }
            Err(String::from("giving up"))
        };

        let producer = retry()?;
        let topic = self.inner.topic.clone();
        let partitioning = self
            .inner
            .paritioning
            .clone()
            .unwrap_or(PartitionStrategy::Random);

        let utils = self.utils.clone();

        let handle = std::thread::spawn(move || {
            producer_loop(input, producer, topic, partitioning, utils)
                .expect("producer loop failed")
        });

        Ok(handle)
    }
}
