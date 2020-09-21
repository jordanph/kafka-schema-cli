# schema-registry-cli

Small command line tool written in Rust to validate and migrate avro schema files to a deployed [Confluent Schema Registry](https://github.com/confluentinc/schema-registry).

## Development

You must have `rustup` installed on your machine ([install link](https://www.rust-lang.org/tools/install))

### Run locally

Point the CLI to your schema registry by setting the enviroment variable `SCHEMA_REGISTRY_URL`. For example:

```bash
export SCHEMA_REGISTRY_URL=https://kafka-schema-registry.eventus-prod.realestate.com.au
```

Place all the schemas that you would like validated in the `schema` folder.

Run the application:

```bash
cargo run
```

You should see output similar to the following:

```
🕵️ Validating schema files before migrating...
🔧 Schema registry url: http://localhost:8081
----------------------------------------------
⌛ Processing ListingAddressDisplay.avsc AVRO file...
  - ❌ ListingAddressDisplay.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.Geocode
⌛ Processing ListingFeature.avsc AVRO file...
  - ✅ ListingFeature.avsc is a valid AVRO schema file!
  - ✅ ListingFeature.avsc is a compatible migration!
⌛ Processing Embedded.avsc AVRO file...
  - ❌ Embedded.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.Attachment
⌛ Processing LandSize.avsc AVRO file...
  - ✅ LandSize.avsc is a valid AVRO schema file!
  - ✅ LandSize.avsc is a compatible migration!
⌛ Processing Attachment.avsc AVRO file...
  - ✅ Attachment.avsc is a valid AVRO schema file!
  - ✅ Attachment.avsc is a compatible migration!
⌛ Processing ListingAddressEmbedded.avsc AVRO file...
  - ❌ ListingAddressEmbedded.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.AtlasObject
⌛ Processing SoldPriceDisplay.avsc AVRO file...
  - ✅ SoldPriceDisplay.avsc is a valid AVRO schema file!
  - ✅ SoldPriceDisplay.avsc is a compatible migration!
⌛ Processing BuildingSize.avsc AVRO file...
  - ✅ BuildingSize.avsc is a valid AVRO schema file!
  - ✅ BuildingSize.avsc is a compatible migration!
⌛ Processing Delete.avsc AVRO file...
  - ✅ Delete.avsc is a valid AVRO schema file!
  - ✅ Delete.avsc is a compatible migration!
⌛ Processing ListingEvent.avsc AVRO file...
  - ❌ ListingEvent.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.Listing
⌛ Processing Usage.avsc AVRO file...
  - ✅ Usage.avsc is a valid AVRO schema file!
  - ✅ Usage.avsc is a compatible migration!
⌛ Processing SoldPricePrice.avsc AVRO file...
  - ✅ SoldPricePrice.avsc is a valid AVRO schema file!
  - ✅ SoldPricePrice.avsc is a compatible migration!
⌛ Processing ListingAddress.avsc AVRO file...
  - ❌ ListingAddress.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.ListingAddressDisplay
⌛ Processing SoldPrice.avsc AVRO file...
  - ❌ SoldPrice.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.SoldPriceDisplay
⌛ Processing PurchaseDisplay.avsc AVRO file...
  - ✅ PurchaseDisplay.avsc is a valid AVRO schema file!
  - ✅ PurchaseDisplay.avsc is a compatible migration!
⌛ Processing Purchase.avsc AVRO file...
  - ❌ Purchase.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.PurchasePrice
⌛ Processing Listing.avsc AVRO file...
  - ❌ Listing.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.ListingAddress
⌛ Processing PurchasePrice.avsc AVRO file...
  - ✅ PurchasePrice.avsc is a valid AVRO schema file!
  - ✅ PurchasePrice.avsc is a compatible migration!
⌛ Processing AtlasObject.avsc AVRO file...
  - ✅ AtlasObject.avsc is a valid AVRO schema file!
  - ❌ AtlasObject.avsc is a not compatible migration with the existing schema!
⌛ Processing DateSold.avsc AVRO file...
  - ✅ DateSold.avsc is a valid AVRO schema file!
  - ✅ DateSold.avsc is a compatible migration!
⌛ Processing FinancialTerms.avsc AVRO file...
  - ❌ FinancialTerms.avsc is an invalid AVRO schema file - Unknown primitiive type: listings.Purchase
⌛ Processing ParkingSpaces.avsc AVRO file...
  - ✅ ParkingSpaces.avsc is a valid AVRO schema file!
  - ✅ ParkingSpaces.avsc is a compatible migration!
⌛ Processing Geocode.avsc AVRO file...
  - ✅ Geocode.avsc is a valid AVRO schema file!
  - ✅ Geocode.avsc is a compatible migration!
----------------------------------------------
🙅‍♂️ One or more schemas failed validation...
```
