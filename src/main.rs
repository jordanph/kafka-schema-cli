use walkdir::WalkDir;
use avro_rs::Schema;
use std::fs::read_to_string;
use clients::schema_registry_client::SchemaRegistryClient;

mod clients;

#[tokio::main]
async fn main() {
  let mut any_errors = false;
 
  let schema_registry_client = SchemaRegistryClient {
    base_url: "http://localhost:8081"
  };

  println!("🕵️ Validating schema files before migrating...");
  println!("----------------------------------------------");

  for entry in WalkDir::new("./schemas")
  .follow_links(true)
  .into_iter()
  .filter_map(|e| e.ok()) {
    let f_name = entry.file_name().to_string_lossy();

    if f_name.ends_with(".avsc") {
      println!("⌛ Processing {} AVRO file...", f_name);

      let raw_schema_or_err = read_to_string(entry.path()); 

      match raw_schema_or_err {
        Ok(raw_schema) => match Schema::parse_str(&raw_schema) {
          Ok(schema) => {
            match &schema {
              Schema::Record { name, .. } => {
                println!("  - ✅ {} is a valid AVRO schema file!", f_name);

                let schema_subject_name = format!("{}.{}", name.namespace.clone().unwrap_or_else(|| "".to_string()), name.name);

                match schema_registry_client.check_schema_compatibility(&schema_subject_name, &schema.canonical_form()).await  {
                  Ok(is_compatible) => {
                    if is_compatible {
                      println!("  - ✅ {} is a compatible migration!", f_name)
                    } else {
                      any_errors = true;
                      println!("  - ❌ {} is a not compatible migration with the existing schema!", f_name)
                    }
                  }
                  Err(error) => {
                    any_errors = true;
                    println!("❌ Unexpected Error: {:?}", error)
                  }
                }
              }
              _ => println!("-  ⚠️ {} is not a record type. Skipping compatibility check...", f_name)
            }
          },
          Err(err) => {
            any_errors = true;
            println!("  - ❌ {} is an invalid AVRO schema file - {}", f_name, err)
          }
        },
        Err(err) => {
          any_errors = true;
          println!("❌ Error: {}", err)
        }
      }
    }
  }

  println!("----------------------------------------------");

  if any_errors {
    println!("🙅‍♂️ One or more schemas failed validation...");
    std::process::exit(1)
  } else {
    println!("🙆‍♂️ All schemas passed validation!");
    std::process::exit(0)
  }
}
