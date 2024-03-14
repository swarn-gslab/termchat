use std::{collections::HashMap, sync::{Arc, Mutex}};

// use self database::(UserDatabase,Response);
pub struct Message{
    pub token: String,
    pub contents: String,
    
}
#[derive(Default)]
struct InMemoryDatabase{
     message: Arc<Mutex<HashMap<String,Vec<Message>>>>,
}
impl InMemoryDatabase {
    fn add_message(&self,token:String,contents:String){
        let mut message = self.message.lock().unwrap();
        let user_message = message.entry(token).or_insert(Vec::new());
        user_message.push(Message{
            token: token.clone(),
            contents: contents.clone(),
        });
        // TODO:

    }
}