use avro_rs::Schema;
use clients::schema_registry_client::SchemaRegistryClient;
use clients::kafka_client::KafkaClient;
use std::fs::read_to_string;
use walkdir::WalkDir;
use serde_derive::Deserialize;
use rdkafka::types::RDKafkaError;

mod clients;

#[derive(Deserialize)]
struct Config {
  retention_ms: i64,
}

#[derive(Deserialize)]
pub struct TopicConfig {
  replication_factor: i32,
  partitions: i32,
  config: Config
}

#[tokio::main]
async fn main() {
    let mut any_errors = false;

    let schema_registry_url =
        std::env::var("SCHEMA_REGISTRY_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());

    let schema_registry_client = SchemaRegistryClient {
        base_url: &schema_registry_url,
    };

    let bootstrap_servers = std::env::var("BOOTSTRAP_SERVERS").unwrap_or_else(|_| "localhost:39092".to_string());

    let kafka_client = KafkaClient::new(&bootstrap_servers);

    println!("🔧 Schema registry url: {}", schema_registry_url);
    println!("🥾 Kafka Bootstrap servers: {}\n", bootstrap_servers);
    println!("🕵️  Validating schema files before migrating...");
    println!("----------------------------------------------");

    for entry in WalkDir::new("./topics")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with("config.yaml") {
            let topic_name = entry
              .path()
              .to_string_lossy()
              .strip_prefix("./topics/")
              .unwrap()
              .strip_suffix(&format!("/{}", &f_name.to_string()))
              .unwrap()
              .replace("/", ".");

            println!("⌛ Processing {} topic...", topic_name);

            for schema_entry in WalkDir::new(entry.path().parent().unwrap())
              .follow_links(true)
              .into_iter()
              .filter_map(|e| e.ok())
              {
                let f_name = schema_entry.file_name().to_string_lossy();

                if f_name.ends_with(".avsc") {
                  let raw_schema_or_err = read_to_string(schema_entry.path());

                  let key_or_value_schema = f_name.strip_suffix("-schema.avsc").unwrap();

                  let schema_emoji = if key_or_value_schema == "key" {
                    "🔑"
                  } else {
                    "📈"
                  };

                  println!("{} {}:", schema_emoji, key_or_value_schema);

                  match raw_schema_or_err {
                      Ok(raw_schema) => match Schema::parse_str(&raw_schema) {
                          Ok(schema) => match &schema {
                              Schema::Record { .. } => {
                                  println!("  - ✅ is a valid avro schema!");
  
                                  let schema_subject_name = format!(
                                      "{}-{}",
                                      &topic_name,
                                      &key_or_value_schema
                                  );
  
                                  match schema_registry_client
                                      .check_schema_compatibility(
                                          &schema_subject_name,
                                          &schema.canonical_form(),
                                      )
                                      .await
                                  {
                                      Ok(is_compatible) => {
                                          if is_compatible {
                                              println!("  - ✅ is a compatible with current schema!")
                                          } else {
                                              any_errors = true;
                                              println!("  - ❌ is a not compatible migration with the existing schema!")
                                          }
                                      }
                                      Err(error) => {
                                          any_errors = true;
                                          println!("  - ❌ Unexpected Error processing: {:?}", error)
                                      }
                                  }
                              }
                              _ => println!("-  ⚠️ is not a record type. Skipping compatibility check..."),
                          },
                          Err(err) => {
                              any_errors = true;
                              println!("  - ❌ is an invalid AVRO schema file - {}", err)
                          }
                      },
                      Err(err) => {
                          any_errors = true;
                          println!("❌ Error while reading file {} - {}", f_name, err)
                      }
                  }
                }
          };
        }
    }

    println!("----------------------------------------------");

    if any_errors {
        println!("🙅‍♂️ One or more schemas failed validation...");
        std::process::exit(1)
    } else {
        println!("🙆‍♂️ All schemas passed validation!\n");
    }

    println!("🚀 Deploying topics...");
    println!("----------------------------------------------");
    for entry in WalkDir::new("./topics")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
          let f_name = entry.file_name().to_string_lossy();

          if f_name.ends_with("config.yaml") {
            let topic_name = entry
                .path()
                .to_string_lossy()
                .strip_prefix("./topics/")
                .unwrap()
                .strip_suffix(&format!("/{}", &f_name.to_string()))
                .unwrap()
                .replace("/", ".");

            println!("📚 {}:", topic_name);

            let topic_config_or_err = read_to_string(entry.path());

            match topic_config_or_err {
              Ok(raw_topic_config) => {
                let topic_config_or_err = serde_yaml::from_str::<TopicConfig>(&raw_topic_config);

                match topic_config_or_err {
                  Ok(topic_config) => {
                    println!("  - ✅ valid topic config");
                    println!("    - 📋 replication factor: {}", topic_config.replication_factor);
                    println!("    - ✂️  partitions: {}", topic_config.partitions);
                    println!("    - ⏲️  retention (ms): {}", topic_config.config.retention_ms);

                    match kafka_client.create_topic(&topic_name, &topic_config).await {
                      Ok(err_or_succ) => match err_or_succ {
                        Ok(_) => println!("  - ✅ deployed topic to broker"),
                        Err((_, err)) => match err {
                          RDKafkaError::TopicAlreadyExists => println!("  - ✅ topic already exists, skipping..."),
                          _ => println!("  - ❌ error while deploying topic to broker - {}", err)
                        }
                      },
                      Err(err) => println!("  - ❌ unexpected error while deploying topic to broker - {}", err)
                    }
                  },
                  Err(err) => println!("  - ❌ invalid topic config - {}", err)
                }
              } ,
              Err(err) => println!("Error loading {} config: {}", topic_name, err)
            }
          }
        }
          
    println!("----------------------------------------------");
    println!("🚢 All topics deployed!\n");

    println!("🤓 Migrating schemas...");
    println!("----------------------------------------------");
    println!("🚧 TBC");
    println!("----------------------------------------------");
    std::process::exit(0)
}
