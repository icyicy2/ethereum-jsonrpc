use crate::{prelude::*, SyncStatus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EthSubscriptionKind {
    Syncing,
    NewHeads,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EthSubscriptionResult {
    Syncing(SyncStatus),
    NewHeads(Option<Block>),
}

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "eth"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "eth"))]
pub trait PubsubApi {
    #[subscription(name = "subscribe", item = EthSubscriptionResult)]
    fn sub(&self, kind: EthSubscriptionKind);
}
