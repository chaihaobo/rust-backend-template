use crate::service::hello::{HelloService, HelloServiceImpl};

pub struct AppContext {
    pub service: ServiceContext,
}

pub struct ServiceContext {
    pub hello_service: Box<dyn HelloService>,
}


impl AppContext {
    pub fn new() -> Self {
        Self {
            service: ServiceContext {
                hello_service: Box::new(HelloServiceImpl::new()),
            },
        }
    }
}


