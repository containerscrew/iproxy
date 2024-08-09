// use std::time::Duration;
// use bson::oid::ObjectId;
// use mongodb::{Client, Collection, Database, IndexModel};
// use mongodb::error::Error;
// use async_trait::async_trait;
// use bson::{doc, to_document};
// use crate::app::db_ops::DbOps;
// use mongodb::options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion};
// use mongodb::results::{DeleteResult, UpdateResult};
// use crate::models::{GeoLocation};
//
// // Required fields for mongodb client and collection setup
// #[derive(Clone)]
// pub struct Db {
//     client: Client,
//     database: Database,
//     collection: Collection<GeoLocation>,
// }
//
// // Initiate the constructor for Db struct
// impl Db {
//     pub async fn new(connection_url: String, database: String, collection: String) -> Result<Self, mongodb::error::Error> {
//         let mut client_options = ClientOptions::parse(connection_url).await?;
//         client_options.server_selection_timeout = Some(Duration::from_secs(3));
//         client_options.app_name = Some("ipfinder".to_string());
//
//         // Set the server_api field of the client_options object to Stable API version 1
//         let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//         client_options.server_api = Some(server_api);
//
//         // Create a new client and connect to the server
//         let client = Client::with_options(client_options)?;
//
//         let database = client.database(&database);
//
//         // Send a ping to confirm a successful connection
//         database.run_command(doc! { "ping": 1 }, None).await?;
//
//         let collection = database.collection(&collection);
//
//         Ok(Db { client, database, collection })
//     }
//
//     // Creates an index on the "ip" field to force the values to be unique.
//     pub async fn create_ips_index(&self) {
//         let options = IndexOptions::builder().unique(true).build();
//         let model = IndexModel::builder()
//             .keys(doc! { "query": 1 })
//             .options(options)
//             .build();
//         self.client
//             .database(self.database.name())
//             .collection::<GeoLocation>(self.collection.name())
//             .create_index(model, None)
//             .await
//             .expect("creating an index should succeed");
//     }
// }
//
// #[async_trait]
// impl DbOps for Db {
//     async fn insert_ip(&self, geolocation: &GeoLocation) -> Result<ObjectId, Error> {
//         let result = self.collection.insert_one(geolocation, None).await?;
//         Ok(result.inserted_id.as_object_id().unwrap())
//     }
//     // async fn get_ip(&self, ip: String) -> Result<Option<GeoLocation>, Error> {
//     //     let get_geolocation = self.collection.find_one(doc! { "query": &ip }, None).await?;
//     //     Ok(get_geolocation)
//     // }
//     // async fn delete_ip(&self, ip: String) -> Result<DeleteResult, Error> {
//     //     let delete_result = self.collection.delete_one(doc! { "query": &ip }, None).await?;
//     //     Ok(delete_result)
//     // }
//     // async fn update_ip(&self, ip: String, geolocation: &GeoLocation) -> Result<UpdateResult, Error> {
//     //     let geolocation_bson = to_document(geolocation)?;
//     //
//     //     let update_result = self.collection.update_one(doc! {"query": &ip}, geolocation_bson, None).await?;
//     //     Ok(update_result)
//     // }
// }