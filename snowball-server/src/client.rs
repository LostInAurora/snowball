use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[derive(Clone)]
// Clone trait is derived here, when Client is cloned, a new vector will be created
// Can not derive copy, because Vec<String> doesn't implement Clone trait
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    // This is used to send the message to the client
    pub sender: Option<mpsc::UnboundedSender<Result<Message, warp::Error>>>,
}