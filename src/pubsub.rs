use crate::{prelude::*, SyncStatus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum EthSubscriptionKind {
    Sync,
    Block,
}

#[derive(Serialize, Deserialize)]
pub enum EthSubscriptionResult {
    Sync(SyncStatus),
    Block(Block),
}

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "eth"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "eth"))]
pub trait PubsubApi {
    #[subscription(name = "subscribe", item = EthSubscriptionResult)]
    fn sub(&self, kind: EthSubscriptionKind);
}
