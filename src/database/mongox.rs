use std::{env, collections::HashMap};
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc,Document, extjson::de::Error, oid::ObjectId,to_bson},
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
    pub async fn get_guild_data_mail(&self, mail: &String) -> Result<GuildData,&'static str> {
        
        let filter = doc! {"email": mail};
        match self
        .col
        .find_one(filter, None)
        .await
        .ok()
        .expect("msg") {
            Some(u)=>return Ok(u),
            None=>return Err("err"),
           
        }
    }
    pub async fn get_guild_data_a(&self, filter: Document) -> Result<GuildData,&'static str> {
        
        
        let guild_data_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("msg")
            ;

      
        match guild_data_detail {
            Some(u)=>return Ok(u),
            None=>return Err("err"),
        }
    }
    // pub async fn update_guild_data(&self, id: &String, new_guild_data: guild_data) -> Result<UpdateResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let new_doc = doc! {
    //         "$set":
    //             {
    //                 "name": new_guild_data.name,
    //                 "location": new_guild_data.email,
    //                 "title": new_guild_data.password
    //             },
    //     };
    //     let updated_doc = self
    //         .col
    //         .update_one(filter, new_doc, None)
    //         .await
    //         .ok()
    //         .expect("Error updating guild_data");
    //     Ok(updated_doc)
    // }
    pub async fn update_guild_datavalided_drop(&self, id: &String) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let new_doc:Document = doc! {
           
            "$unset": {
                "session_auth":""
        }
        
    };
        let update = new_doc;
        let updated_doc = self
            .col
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("Error updating guild_data");
        Ok(updated_doc)
    }
    
    pub async fn update_guild_datavalided(&self, id: &String, key: &String,volume:&String) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let new_doc:Document = doc! {
            "$set":
                {
                    "valid": true,
                    format!("session_auth.{}",key): volume,

                },
            "$unset": {
                "session_register":"",
                "session_login":""
        }
        
    };
        let update = new_doc;
        let updated_doc = self
            .col
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("Error updating guild_data");
        Ok(updated_doc)
    }
    pub async fn update_guild_datavalided_mail(&self, mail: &String, guild_data: HashMap<String,String>) -> Result<UpdateResult, Error> {
        
        let filter = doc! {"email": mail};
        let new_doc = doc! {
   
            "$set":
                {
                    "session_login": to_bson(&guild_data).unwrap() ,

                }
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            ;
            
        Ok(updated_doc.unwrap())
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

    pub async fn get_all_guild_datas(&self) -> Result<Vec<GuildData>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of guild_datas");
        let mut guild_datas: Vec<GuildData> = Vec::new();
        while let Some(guild_data) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            guild_datas.push(guild_data)
        }
        Ok(guild_datas)
    }
}