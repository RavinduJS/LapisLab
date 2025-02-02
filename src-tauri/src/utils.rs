use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::{process::Child, sync::Mutex};

pub trait Server {
    fn new(v:&str,n:&str,r:Option<i32>)->Self;
    async fn build(&self);
    fn run(&self)->Arc<Mutex<Option<Child>>>;
    async fn kill(control:Arc<Mutex<Option<Child>>>);
    fn save_metadata(&self)->Result<String, String>;
    fn load_metadata(name:&str);
    async fn get_server_jar(&self)->Result<String, String>;
    async fn get_versions()->Result<String, String>;
}




#[derive(Debug,Serialize,Deserialize)]
pub struct Version{
    project_id:String,
    project_name:String,
    version_groups:Vec<String>,
    versions:Vec<String>
}