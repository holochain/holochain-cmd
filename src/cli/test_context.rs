use tempfile::tempdir;

use holochain_core::{
    context::Context,
    logger::Logger,
    persister::SimplePersister,
};
use holochain_agent::Agent;
use holochain_cas_implementations::{cas::file::FilesystemStorage, eav::file::EavFileStorage};
use std::sync::{Arc, Mutex};


#[derive(Clone, Debug)]
pub struct TestLogger {
    pub log: Vec<String>,
}

impl Logger for TestLogger {
    fn log(&mut self, msg: String) {
        self.log.push(msg);
    }
    fn dump(&self) -> String {
        format!("{:?}", self.log)
    }
}

/// create a test logger
pub fn test_logger() -> Arc<Mutex<TestLogger>> {
    Arc::new(Mutex::new(TestLogger { log: Vec::new() }))
}

/// create a test context and TestLogger pair so we can use the logger in assertions
pub fn test_context(agent_name: &str) -> Arc<Context> {
    let agent = Agent::from(agent_name.to_owned());
    let logger = test_logger();
    Arc::new(
        Context::new(
            agent,
            logger.clone(),
            Arc::new(Mutex::new(SimplePersister::new())),
            FilesystemStorage::new(tempdir().unwrap().path().to_str().unwrap()).unwrap(),
            EavFileStorage::new(tempdir().unwrap().path().to_str().unwrap().to_string())
                .unwrap(),
        ).unwrap(),
    )
}