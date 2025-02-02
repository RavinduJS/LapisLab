use std::{fs::{self, File}, io::{copy, stdout, Cursor, Write}, process::Stdio, sync::Arc,};
use serde::{Deserialize, Serialize};
use crate::utils::{self, Version};
use tokio::{process::{Child, Command}, sync::Mutex};
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PaperMC{
    version:String,
    name:String,
    build:String,
    path:String,
    jar_name:String,
    assigned_ram:i32,
    runs:i32,
    
}
//java -Xms4096M -Xmx4096M -jar server.jar nogui
impl utils::Server for PaperMC {
    fn new(v:&str,n:&str,r:Option<i32>)->Self {
        
        let path = format!("servers/paper/{}/{}",v,n);
        let _ = fs::create_dir_all(&path);

        let default_ram: Option<i32> = Some(4096);
        let default_runs:i32 = 0;

        Self{
            version:v.to_owned(), 
            name:n.to_owned(),
            build:"499".to_owned(),
            path,
            jar_name:"paper-1.20.4-499".to_owned(),
            assigned_ram:r.or(default_ram).unwrap(),
            runs:default_runs
        }
    }

    async fn build(&self) {
        let server_jar = format!("{}.jar",self.jar_name);
    
        let build_command = Command::new("java").
            current_dir(&self.path)
            .arg("-Xms4096M")
            .arg("-Xms4096M")
            .arg("-jar")
            .arg(server_jar)
            .arg("nogui")
            .spawn();
        let _res = build_command.unwrap().wait().await;
        let eula_path = format!("{}/eula.txt",self.path);
        let eula = fs::read_to_string(&eula_path).expect("failed to find eula");
        let valid_eula = eula.replace("eula=false", "eula=true");
        let _ = fs::write(&eula_path, valid_eula);
        println!("eula rewritten")
    }
    
   fn run(&self)->Arc<Mutex<Option<Child>>>{
        let server_jar: String = format!("{}.jar",self.jar_name);
        let ram:String = format!("-Xms{}M",self.assigned_ram);
        let path:String = self.path.clone();
       
        let command_thread_handle:Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let forign_command_thread_handle = command_thread_handle.clone();
        tokio::spawn(async move{
            let mut local_command_thread_handle = forign_command_thread_handle.lock().await;
            let build_command = 
            Command::new("java")
            .current_dir(path)
            .arg(ram.clone())
            .arg(ram.clone())
            .arg("-jar")
            .arg(server_jar.clone())
            .arg("nogui")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn().expect("msg");

        *local_command_thread_handle = Some(build_command);
        });
        println!("running server");
        
        command_thread_handle
    }
    
    async fn kill(forign_command_thread_handle:Arc<Mutex<Option<Child>>>){
        let mut local_command_thread_handle = forign_command_thread_handle.lock().await;
        let  mutex = local_command_thread_handle.as_mut().unwrap();
        let value = mutex.stdin.as_mut().unwrap();
        let _ = value.write_all(b"stop").await;
    }

    fn save_metadata(&self)->Result<String, String> {
        println!("saving metadata");
        let json_string = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        let m_path: String = format!("metadata/server-{}.json",self.name);
        let mut file:File = File::create_new(m_path).map_err(|e| e.to_string())?;
        let _ = file.write_all(json_string.as_bytes());
        Ok("metadata saved".to_string())
    }

    fn load_metadata(name:&str) {
        println!("loading metadata");
        let m_path: String = format!("metadata/{}.json",name);
        let mut file = File::open(m_path).unwrap();
        let mut stdout = stdout();
        let str = &copy(&mut file, &mut stdout).unwrap().to_string();
        let json_obj:PaperMC = serde_json::from_str(str).unwrap();
        println!("{:#?}",json_obj)
    }

    async fn get_server_jar(&self)->Result<String, String> {
        let uri: String = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/{}.jar",self.version,self.build,self.jar_name);
        let jar_path = format!("{}/{}.jar",self.path,self.jar_name);
        println!("starting download {} from {}",jar_path, uri);
        let download_file: reqwest::Response = reqwest::get(uri)
            .await
            .map_err(|e| e.to_string())?;
        println!("downloading");
        let mut file: File = File::create(jar_path)
            .map_err(|e| e.to_string())?;
        let mut b = Cursor::new(download_file.bytes().await.map_err(|e| e.to_string())?);
        copy(&mut b, &mut file).map_err(|e| e.to_string())?;
        Ok("".to_owned())
    }

    async fn get_versions()->Result<String, String> {
        let uri:&str  = "https://api.papermc.io/v2/projects/paper";
        let versions: reqwest::Response = reqwest::get(uri).await.map_err(|e| e.to_string())?;
        let test_string = versions.text().await.unwrap();
        let json_string:Version = serde_json::from_str(&test_string).unwrap();
        println!("{:#?}",json_string);
        Ok("".to_owned())
    }

}



