use bson::oid::ObjectId;
use mongodb::{Client, Collection};
use mongodb::error::Error;
use async_trait::async_trait;
use bson::doc;
use crate::models::{GeoLocation};


// Required fields for mongodb client and collection setup
#[derive(Clone)]
pub struct Db {
    collection: Collection<GeoLocation>,
}

// Initiate the constructor for Db struct
impl Db {
    pub async fn new(connection_url: String, database: String, collection: String) -> Result<Self, mongodb::error::Error> {
        // let options =
        //     ClientOptions::parse_with_resolver_config(&connection_url, ResolverConfig::cloudflare())
        //         .await;
        let client = Client::with_uri_str(connection_url).await?;
        let database = client.database(&database);
        let collection = database.collection(&collection);

        Ok(Db { collection })
    }
}

#[async_trait]
pub trait DbOps {
    async fn insert_ip(&self, geolocation: &GeoLocation) -> Result<ObjectId, Error>;
    async fn get_ip(&self, ip: String) ->  Result<Option<GeoLocation>, Error>;
}

#[async_trait]
impl DbOps for Db {
    async fn insert_ip(&self, geolocation: &GeoLocation) -> Result<ObjectId, Error> {
        let result = self.collection.insert_one(geolocation, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }
    async fn get_ip(&self, ip: String) ->  Result<Option<GeoLocation>, Error> {
        let ip_geolocation = self.collection.find_one(doc! { "query": &ip }, None).await?;
        Ok(ip_geolocation)
    }
}