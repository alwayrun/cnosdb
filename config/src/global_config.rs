use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::check::{CheckConfig, CheckConfigItemResult, CheckConfigResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlobalConfig {
    #[serde(default = "GlobalConfig::default_node_id")]
    pub node_id: u64,
    #[serde(default = "GlobalConfig::default_host")]
    pub host: String,
    #[serde(default = "GlobalConfig::default_cluster_name")]
    pub cluster_name: String,
    #[serde(default = "GlobalConfig::default_store_metrics")]
    pub store_metrics: bool,
}

impl GlobalConfig {
    fn default_node_id() -> u64 {
        1001
    }

    fn default_host() -> String {
        "localhost".to_string()
    }

    fn default_cluster_name() -> String {
        "cluster_xxx".to_string()
    }

    fn default_store_metrics() -> bool {
        true
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            node_id: GlobalConfig::default_node_id(),
            host: GlobalConfig::default_host(),
            cluster_name: GlobalConfig::default_cluster_name(),
            store_metrics: GlobalConfig::default_store_metrics(),
        }
    }
}

impl CheckConfig for GlobalConfig {
    fn check(&self, _: &crate::Config) -> Option<CheckConfigResult> {
        let config_name = Arc::new("global".to_string());
        let mut ret = CheckConfigResult::default();

        if self.cluster_name.is_empty() {
            ret.add_warn(CheckConfigItemResult {
                config: config_name,
                item: "name".to_string(),
                message: "'name' is empty".to_string(),
            });
        }

        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }
}
