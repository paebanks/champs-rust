use crate::event::{errors::CustomError, models::{event::Event, use_case::event::GetEventResponse}, repository::EventDbTrait};
use mongodb::{error::Result as MongoResult, Client, bson::{doc, oid::ObjectId, Document}, Collection};

const COLLECTION_NAME: &str = "events";


pub struct EventMongo {
    client: Client,
    db_name: String,
}

impl EventMongo {
    pub async fn new(uri: &str, db_name: &str) -> MongoResult<Self> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self { client, db_name: db_name.into() })
    }
}
pub trait EventDbTrait {
    async fn create_event(&self, event: Event) -> Result<InsertOneResult, MongoError>;
    // Add more methods for CRUD operations as needed
}

#[async_trait]
impl EventDbTrait for EventMongo {
    async fn create_event(&self, event: Event) -> Result<InsertOneResult, MongoError> {
        self.collection.insert_one(event, None).await
    }
    // Implement other methods from EventDbTrait using EventMongo's methods
}