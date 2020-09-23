use crate::TopicConfig;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;

pub struct KafkaClient {
    admin_client: AdminClient<DefaultClientContext>,
}

impl<'a> KafkaClient {
    pub fn new(bootstrap_servers: &'a str) -> KafkaClient {
        let client: AdminClient<_> = consumer_config("create_topic", bootstrap_servers)
            .create()
            .unwrap();

        KafkaClient {
            admin_client: client,
        }
    }

    pub async fn create_topic(
        &self,
        topic_name: &str,
        topic_config: &TopicConfig,
    ) -> Result<
        Result<std::string::String, (std::string::String, rdkafka::error::RDKafkaError)>,
        rdkafka::error::KafkaError,
    > {
        let retention_ms = topic_config.config.retention_ms.to_string();

        let new_topic = NewTopic {
            name: topic_name,
            num_partitions: topic_config.partitions,
            replication: TopicReplication::Fixed(topic_config.replication_factor),
            config: vec![("retention.ms", &retention_ms)],
        };

        self.admin_client
            .create_topics(&[new_topic], &AdminOptions::new())
            .await
            .map(|vectors| vectors.first().unwrap().to_owned())
    }
}

fn consumer_config(group_id: &str, bootstrap_servers: &str) -> ClientConfig {
    let mut config = ClientConfig::new();

    config.set("group.id", group_id);
    config.set("client.id", "rdkafka_integration_test_client");
    config.set("bootstrap.servers", bootstrap_servers);
    config.set("enable.partition.eof", "false");
    config.set("session.timeout.ms", "6000");
    config.set("enable.auto.commit", "false");
    config.set("statistics.interval.ms", "500");
    config.set("api.version.request", "true");
    config.set("debug", "all");
    config.set("auto.offset.reset", "earliest");
    config.set("security.protocol", "PLAINTEXT");

    config
}
