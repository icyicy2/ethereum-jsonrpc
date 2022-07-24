use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListEntry {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum MessageCall {
    #[serde(rename_all = "camelCase")]
    Legacy {
        from: Option<Address>,
        to: Option<Address>,
        gas: Option<U64>,
        gas_price: Option<U256>,
        value: Option<U256>,
        data: Option<Bytes>,
    },
    #[serde(rename_all = "camelCase")]
    EIP2930 {
        from: Option<Address>,
        to: Option<Address>,
        gas: Option<U64>,
        gas_price: Option<U256>,
        value: Option<U256>,
        data: Option<Bytes>,
        access_list: Option<Vec<AccessListEntry>>,
    },
    #[serde(rename_all = "camelCase")]
    EIP1559 {
        from: Option<Address>,
        to: Option<Address>,
        gas: Option<U64>,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
        value: Option<U256>,
        data: Option<Bytes>,
        access_list: Option<Vec<AccessListEntry>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum TransactionMessage {
    #[serde(rename_all = "camelCase")]
    Legacy {
        chain_id: Option<U64>,
        nonce: U64,
        to: Option<Address>,
        gas: U64,
        gas_price: U256,
        value: U256,
        input: Bytes,
    },
    #[serde(rename_all = "camelCase")]
    EIP2930 {
        chain_id: U64,
        nonce: U64,
        to: Option<Address>,
        gas: U64,
        gas_price: U256,
        value: U256,
        input: Bytes,
        access_list: Vec<AccessListEntry>,
    },
    #[serde(rename_all = "camelCase")]
    EIP1559 {
        chain_id: U64,
        nonce: U64,
        to: Option<Address>,
        gas: U64,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: U256,
        value: U256,
        input: Bytes,
        access_list: Vec<AccessListEntry>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(flatten)]
    pub message: TransactionMessage,
    /// RLP encoded representation of the transaction.
    pub v: U64,
    pub r: H256,
    pub s: H256,

    pub from: Address,
    pub hash: H256,
    pub transaction_index: Option<U64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// Tx is either a transaction or a transaction hash.
pub enum Tx {
    /// Transaction.
    Transaction(Box<Transaction>),
    /// Transaction hash.
    Hash(H256),
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_ser_de_hexbytes_option() {
        let call_data = MessageCall::Legacy {
            from: None,
            to: Some(Address::from([0; 20])),
            gas: None,
            gas_price: None,
            value: None,
            data: None,
        };
        let hexstring = r#"{"from":null,"to":"0x0000000000000000000000000000000000000000","gas":null,"gasPrice":null,"value":null,"data":null}"#;
        assert_eq!(serde_json::to_string(&call_data).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_str::<MessageCall>(hexstring).unwrap(),
            call_data
        );

        let call_data_with_data = MessageCall::Legacy {
            from: None,
            to: Some(Address::from([0; 20])),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes::from(&b"Hello Akula"[..])),
        };

        let hexstring_with_data = r#"{"from":null,"to":"0x0000000000000000000000000000000000000000","gas":null,"gasPrice":null,"value":null,"data":"0x48656c6c6f20416b756c61"}"#;
        assert_eq!(
            serde_json::to_string(&call_data_with_data).unwrap(),
            hexstring_with_data
        );
        assert_eq!(
            serde_json::from_str::<MessageCall>(hexstring_with_data).unwrap(),
            call_data_with_data
        );
    }

    #[test]
    fn test_tx_ser() {
        let tx = Transaction {
            message: TransactionMessage::Legacy {
                chain_id: Some(2_u64.into()),
                nonce: 12_u64.into(),
                gas: 21000_u64.into(),
                gas_price: 20_000_000_000_u64.into(),
                to: Some(hex!("727fc6a68321b754475c668a6abfb6e9e71c169a").into()),
                value: 10.as_u256() * 1_000_000_000 * 1_000_000_000,
                input: hex!("a9059cbb000000000213ed0f886efd100b67c7e4ec0a85a7d20dc971600000000000000000000015af1d78b58c4000").to_vec().into(),
            },
            v: 40_u64.into(),
            r: hex!("be67e0a07db67da8d446f76add590e54b6e92cb6b8f9835aeb67540579a27717").into(),
            s: hex!("2d690516512020171c1ec870f6ff45398cc8609250326be89915fb538e7bd718").into(),
            from: Address::repeat_byte(0xAA),
            hash: H256::repeat_byte(0xBB),
            transaction_index: Some(0x42.into()),
        };
        let serialized = r#"{"chainId":"0x2","nonce":"0xc","to":"0x727fc6a68321b754475c668a6abfb6e9e71c169a","gas":"0x5208","gasPrice":"0x4a817c800","value":"0x8ac7230489e80000","input":"0xa9059cbb000000000213ed0f886efd100b67c7e4ec0a85a7d20dc971600000000000000000000015af1d78b58c4000","v":"0x28","r":"0xbe67e0a07db67da8d446f76add590e54b6e92cb6b8f9835aeb67540579a27717","s":"0x2d690516512020171c1ec870f6ff45398cc8609250326be89915fb538e7bd718","from":"0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","hash":"0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","transactionIndex":"0x42"}"#;

        assert_eq!(serde_json::to_string(&tx).unwrap(), serialized);
        assert_eq!(serde_json::from_str::<Transaction>(serialized).unwrap(), tx);
    }
}