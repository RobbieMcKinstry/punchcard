use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::{read_yaml, WorkflowEvents};
use crate::bindings::concurrency::Concurrency;
use crate::bindings::defaults::Defaults;
use crate::bindings::jobs::Job;
use crate::bindings::OneOrDictionary;

pub fn load_workflow<T: AsRef<Path>>(file: T) -> Result<Workflow, serde_yaml::Error> {
    let loaded = read_yaml(file);
    let result: Workflow = serde_yaml::from_str(&loaded)?;
    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Workflow {
    pub name: Option<String>,
    pub run_name: Option<String>,
    pub on: WorkflowEvents,
    pub env: Option<IndexMap<String, String>>,
    pub defaults: Option<Defaults>,
    pub jobs: IndexMap<String, Job>,
    pub concurrency: Option<OneOrDictionary<String, Concurrency>>,
}
