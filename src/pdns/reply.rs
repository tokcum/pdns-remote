/// Serialize PowerDNS replies.
/// https://doc.powerdns.com/authoritative/backends/remote.html#api
///
/// Serde supports different Enum representations.
/// For replies send via PowerDNS ABI the 'untagged' approach works best.
/// https://serde.rs/enum-representations.html#untagged
///
/// PowerDNS ABI uses camelCase to name methods.
/// https://serde.rs/attr-rename.html#serialize-fields-as-camelcase
///
use serde::Serialize;

#[cfg(test)]
use serde::Deserialize;

/// `Reply` is the counterpart to `Query` but is never serialized.
#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged)]
#[cfg_attr(test, derive(Deserialize))]
pub enum Reply {
    Initialize(InitializeReply),
    Lookup(LookupReply),
    List(),
    GetDomainInfo(),
    DirectBackendCmd(),
    GetAllDomains(),
}

#[derive(Serialize, PartialEq, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub struct InitializeReply {
    result: bool,
}

#[derive(Serialize, PartialEq, Debug)]
#[cfg_attr(test, derive(Deserialize))]
pub struct LookupReply {
    result: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_response_result_true() {
        let response_json = r#"{"result":true}"#;

        let response_from_json: Reply = serde_json::from_str(&response_json).unwrap();

        let response = Reply::Initialize(
            InitializeReply {
                result: true
            }
        );

        assert_eq!(response, response_from_json);
    }

    #[test]
    fn deserialize_response_result_false() {
        let response_json = r#"{"result":false}"#;

        let response_from_json: Reply = serde_json::from_str(&response_json).unwrap();

        let response = Reply::Initialize(
            InitializeReply {
                result: false
            }
        );

        assert_eq!(response, response_from_json);
    }
}
