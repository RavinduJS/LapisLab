use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::process::Child;
#[derive(Debug)]
enum Difficulty{
    Peaceful,
    Easy,
    Normal,
    Hard
}

#[derive(Debug)]
enum GameMode{
    Creative,
    Survival,
    Spectator,
    Adventure
}

pub struct Commands{
    thread_handle:Arc<Mutex<Option<Child>>>
}
impl Commands {
    async fn save_all(&self) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> = self.thread_handle.lock().await;
    }
    async fn save_off(&self) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> = self.thread_handle.lock().await;

    }
    async fn save_on(&self) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> = self.thread_handle.lock().await;

    }
    async fn stop(&self) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> = self.thread_handle.lock().await;

    }
    async fn reload(&self) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> = self.thread_handle.lock().await;

    }
    async fn seed(&self) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> = self.thread_handle.lock().await;

    }
    async fn difficulty(&self,dif:Difficulty) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> = self.thread_handle.lock().await;
        let  mutex = local_command_thread_handle.as_mut().unwrap();
        let value = mutex.stdin.as_mut().unwrap();
        let c = format!("difficulty {:?}",dif);
        let _ = value.write_all(c.as_bytes()).await;
    }
    async fn default_game_mode(&self, mode:GameMode) {
        let mut local_command_thread_handle: tokio::sync::MutexGuard<'_, Option<Child>> =self.thread_handle.lock().await;
        let  mutex = local_command_thread_handle.as_mut().unwrap();
        let value = mutex.stdin.as_mut().unwrap();
        let c = format!("difficulty {:?}",mode);
        let _ = value.write_all(c.as_bytes()).await;
    }
}