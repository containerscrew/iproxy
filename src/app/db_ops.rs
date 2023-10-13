use async_trait::async_trait;
use bson::oid::ObjectId;
use crate::models::GeoLocation;
use mongodb::error::Error;
use mongodb::results::DeleteResult;

#[async_trait]
pub trait DbOps {
    async fn insert_ip(&self, geolocation: &GeoLocation) -> Result<ObjectId, Error>;
    async fn get_ip(&self, ip: String) ->  Result<Option<GeoLocation>, Error>;
    async fn delete_ip(&self, ip: String) -> Result<DeleteResult, Error>;
}