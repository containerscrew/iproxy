use std::time::Duration;
use bson::oid::ObjectId;
use mongodb::{Client, Collection};
use mongodb::error::Error;
use async_trait::async_trait;
use bson::doc;
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::results::DeleteResult;
use crate::models::{GeoLocation};


// Required fields for mongodb client and collection setup
#[derive(Clone)]
pub struct Db {
    collection: Collection<GeoLocation>,
}

// Initiate the constructor for Db struct
impl Db {
    pub async fn new(connection_url: String, database: String, collection: String) -> Result<Self, mongodb::error::Error> {
        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows
        let mut options =
            ClientOptions::parse_with_resolver_config(&connection_url, ResolverConfig::cloudflare())
                .await?;

        // If server don't respond in 3 seconds, panic!
        options.server_selection_timeout = Some(Duration::from_secs(2));

        let client = Client::with_options(options)?;

        let database = client.database(&database);

        // Send a ping to confirm a successful connection
        database.run_command(doc! { "ping": 1 }, None).await?;

        let collection = database.collection(&collection);

        Ok(Db { collection })
    }
}

#[async_trait]
pub trait DbOps {
    async fn insert_ip(&self, geolocation: &GeoLocation) -> Result<ObjectId, Error>;
    // To be implemented
    //async fn update_ip(&self) ;
    async fn get_ip(&self, ip: String) ->  Result<Option<GeoLocation>, Error>;
    async fn delete_ip(&self, ip: String) -> Result<DeleteResult, Error>;

}

#[async_trait]
impl DbOps for Db {
    async fn insert_ip(&self, geolocation: &GeoLocation) -> Result<ObjectId, Error> {
        let result = self.collection.insert_one(geolocation, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }
    async fn get_ip(&self, ip: String) ->  Result<Option<GeoLocation>, Error> {
        let get_geolocation = self.collection.find_one(doc! { "query": &ip }, None).await?;
        Ok(get_geolocation)
    }
    async fn delete_ip(&self, ip: String) -> Result<DeleteResult, Error> {
        let delete_result = self.collection.delete_one(doc! { "query": &ip }, None).await?;
        Ok(delete_result)
    }
}
