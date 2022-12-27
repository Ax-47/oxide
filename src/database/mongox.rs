use std::{env};
use dotenv::dotenv;
use mongodb::{
    bson::{doc,Document, extjson::de::Error},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use crate::{
    database::models::*,

};
pub struct MongoRepo {
    col: Collection<GuildData>,
 
}
impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let database = match env::var("DATABASE") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let collection = match env::var("COLLECTION") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database(database.as_str());
        let col: Collection<GuildData> = db.collection(collection.as_str());
        MongoRepo { col}
    }

    pub async fn create_guild_data(&self, guild: String) -> Result<InsertOneResult, Error> {
        let new_doc = GuildData {
            guild_id: Some(guild),
            join: None,
            leave:None ,
            music_dash: None,
            vc: None,
            choose:None,
        };
        let guild_data = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating guild_data");

        Ok(guild_data)
    }

    pub async fn get_guildid_data(&self, id: &String) -> Result<GuildData, GuildData> {

        let guild_data_detail = self
            .col
            .find_one(doc! {"guild_id": id}, None)
            .await
            .ok();
        if guild_data_detail.is_none(){
            let err=GuildData{
                guild_id:None,
                join:None,
                leave:None,
                music_dash:None,
                vc:None,
                choose:None,

            };
            return Err(err)
        }
        Ok(guild_data_detail.unwrap().unwrap())
    }
    pub async fn update_guild(&self, id: &String, guild_dataname: Document) -> Result<UpdateResult, Error> {
       
        let filter = doc! {"guild_id": id};
        let updated_doc = self
            .col
            .update_one(filter, guild_dataname, None)
            .await
            .ok()
            ;
            
        Ok(updated_doc.unwrap())
    }
    pub async fn delete_guild_data(&self, id: &String) -> Result<DeleteResult, Error> {
        
        let filter = doc! {"guild_id": id};
        let guild_data_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting guild_data");

        Ok(guild_data_detail)
    }

}