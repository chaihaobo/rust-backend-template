use async_trait::async_trait;

#[async_trait]
pub trait HelloService: Send + Sync {
    async fn hello(&self) -> String;
}

#[derive(Clone)]
pub struct HelloServiceImpl;

impl HelloServiceImpl {
    pub fn new() -> Self {
        HelloServiceImpl {}
    }
}

#[async_trait]
impl HelloService for HelloServiceImpl {
    async fn hello(&self) -> String {
        String::from("hello word")
    }
}

