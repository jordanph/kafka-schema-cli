# schema-registry-cli

Small command line tool written in Rust to deploy topics, and validate and migrate avro schema files to a deployed [Confluent Schema Registry](https://github.com/confluentinc/schema-registry).

## Development

- You must have `rustup` installed on your machine ([install link](https://www.rust-lang.org/tools/install))
- You must have `cmake` installed on your machine (`brew install cmake`)

### Run locally

Point the CLI to your schema registry and brokers by setting the enviroment variables `SCHEMA_REGISTRY_URL` and `BOOTSTRAP_SERVERS`. For example:

```bash
export SCHEMA_REGISTRY_URL=https://kafka-schema-registry.eventus-prod.realestate.com.au
export BOOTSTRAP_SERVERS=localhost:39092
```

Place all the topics that you would like to create in the `topics` folder. The topic name is determine by the folder path that each `config.yaml` file resides in.

For example, the path `./topics/events/property/listings/raw/v1` creates a topic called `events.property.listings.raw.v1`.

The schema subjects will be the `topic name + - + schema_name` with the `-schema.avsc` removed from the name.

For example, the path `./topics/events/property/listings/raw/v1` creates a topic called `events.property.listings.raw.v1` and the schema file `key-schema.avcs` will have a subject `events.property.listings.raw.v1-key`. This schema deployment follows the default [TopicNameStrategy provided by the Schema Registry.](https://docs.confluent.io/current/schema-registry/serdes-develop/index.html#overview)

Run the application:

```bash
cargo run
```

You should see output similar to the following:

```
🔧 Schema registry url: https://kafka-schema-registry.eventus-prod.realestate.com.au
🥾 Kafka Bootstrap servers: localhost:39092

🕵️  Validating schema files before migrating...
----------------------------------------------
⌛ Processing events.property.listings.raw.v1 topic...
🔑 key:
  - ✅ is a valid avro schema!
  - ✅ is a compatible with current schema!
📈 value:
  - ✅ is a valid avro schema!
  - ✅ is a compatible with current schema!
⌛ Processing events.property.listings.raw.v2 topic...
🔑 key:
  - ✅ is a valid avro schema!
  - ✅ is a compatible with current schema!
📈 value:
  - ✅ is a valid avro schema!
  - ✅ is a compatible with current schema!
----------------------------------------------
🙆‍♂️ All schemas passed validation!

🚀 Deploying topics...
----------------------------------------------
📚 events.property.listings.raw.v1:
  - ✅ valid topic config
    - 📋 replication factor: 1
    - ✂️  partitions: 3
    - ⏲️  retention (ms): 604800000
  - ✅ topic already exists, skipping...
📚 events.property.listings.raw.v2:
  - ✅ valid topic config
    - 📋 replication factor: 1
    - ✂️  partitions: 1
    - ⏲️  retention (ms): 604800000
  - ✅ topic already exists, skipping...
----------------------------------------------
🚢 All topics deployed!

🤓 Migrating schemas...
----------------------------------------------
🚧 TBC
----------------------------------------------
```
