use crate::database::{fetch_counter, get_db_path, initialize_db, update_counter};
use crux_core::capability::{CapabilityContext, Operation};
use crux_core::macros::Capability;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DatabaseOperation {
    /// Fetches the counter from the database
    FetchCounter,
    /// Updates the counter in the database
    UpdateCounter(isize),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum DatabaseOutput {
    /// When the database operation succeeds with no return value
    Succeeded,
    /// The database operation failed
    Failed(String),
    /// The operation returns the current counter.
    Counter(isize),
}

impl Operation for DatabaseOperation {
    type Output = DatabaseOutput;
}

#[derive(Capability)]
pub struct Database<Ev> {
    context: CapabilityContext<DatabaseOperation, Ev>,
}

impl<Ev> Database<Ev>
where
    Ev: 'static,
{
    pub fn new(context: CapabilityContext<DatabaseOperation, Ev>) -> Self {
        Self { context }
    }

    pub fn fetch_counter<F>(&self, event: F)
    where
        F: Fn(DatabaseOutput) -> Ev + Send + 'static,
    {
        let ctx = self.context.clone();
        self.context.spawn(async move {
            let result: Result<isize, String> = (|| {
                let path = get_db_path().map_err(|e| e.to_string())?;
                let path_str = path
                    .to_str()
                    .ok_or_else(|| "Invalid path string".to_string())?;
                let connection = initialize_db(path_str).map_err(|e| e.to_string())?;
                let counter = fetch_counter(&connection, 1).map_err(|e| e.to_string())?;
                Ok(counter)
            })();

            let output = match result {
                Ok(counter) => DatabaseOutput::Counter(counter),
                Err(e) => DatabaseOutput::Failed(e.to_string()),
            };

            ctx.update_app(event(output));
        });
    }

    pub fn update_counter<F>(&self, value: isize, event: F)
    where
        F: Fn(DatabaseOutput) -> Ev + Send + 'static,
    {
        let ctx = self.context.clone();
        self.context.spawn(async move {
            let result: Result<(), String> = (|| {
                let path = get_db_path().map_err(|e| e.to_string())?;
                let path_str = path
                    .to_str()
                    .ok_or_else(|| "Invalid path string".to_string())?;
                let connection = initialize_db(path_str).map_err(|e| e.to_string())?;
                update_counter(&connection, 1, value).map_err(|e| e.to_string())?;
                Ok(())
            })();

            let output = match result {
                Ok(()) => DatabaseOutput::Succeeded,
                Err(e) => DatabaseOutput::Failed(e),
            };

            ctx.update_app(event(output));
        });
    }
}
