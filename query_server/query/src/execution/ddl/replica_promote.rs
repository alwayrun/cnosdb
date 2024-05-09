use async_trait::async_trait;
use coordinator::ReplicationCmdType;
use spi::query::execution::{Output, QueryStateMachineRef};
use spi::query::logical_planner::ReplicaPromote;
use spi::{QueryError, Result};

use super::DDLDefinitionTask;

pub struct ReplicaPromoteTask {
    stmt: ReplicaPromote,
}

impl ReplicaPromoteTask {
    #[inline(always)]
    pub fn new(stmt: ReplicaPromote) -> Self {
        Self { stmt }
    }
}

#[async_trait]
impl DDLDefinitionTask for ReplicaPromoteTask {
    async fn execute(&self, query_state_machine: QueryStateMachineRef) -> Result<Output> {
        let (replica_id, node_id) = (self.stmt.replica_id, self.stmt.node_id);
        let tenant = query_state_machine.session.tenant();

        let meta = query_state_machine.meta.clone();
        let coord = query_state_machine.coord.clone();
        let all_info = coordinator::get_replica_all_info(meta, tenant, replica_id).await?;

        if let Some(info) = all_info.replica_set.by_node_id(node_id) {
            let cmd_type = ReplicationCmdType::PromoteLeader(replica_id, info.id);
            coord.replication_manager(tenant, cmd_type).await?;

            Ok(Output::Nil(()))
        } else {
            Err(QueryError::ReplicaNotFound {
                replica_id,
                node_id,
            })
        }
    }
}
