/// Deserialize PowerDNS queries.
/// https://doc.powerdns.com/authoritative/backends/remote.html#api
///
/// Serde supports different Enum representations.
/// For queries received via PowerDNS ABI the 'adjacent tagged' approach works best.
/// https://serde.rs/enum-representations.html#adjacently-tagged
///
/// PowerDNS ABI uses camelCase to name methods.
/// https://serde.rs/attr-rename.html#serialize-fields-as-camelcase

// Queries are only deserialized, never serialized.
use serde::Deserialize;

/// `Query` represents PowerDNS methods.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(tag = "method", content = "parameters", rename_all = "camelCase")]
pub enum Query {
    Initialize(InitializeParams),
    Lookup(LookupParams),
    List(ListParams),
    GetDomainInfo(GetDomainInfoParams),
    DirectBackendCmd(DirectBackendCmdParams),
    GetAllDomains(GetAllDomainsParams),
}

/// Method: initialize
/// https://doc.powerdns.com/authoritative/backends/remote.html#initialize
///
/// Parameters depend on the connection string configured in 'remote-connection-string'.
/// https://doc.powerdns.com/authoritative/backends/remote.html#unix-connector
///
#[derive(Deserialize, PartialEq, Debug)]
pub struct InitializeParams {
    path: String,
    timeout: Option<String>,
}

/// Method: lookup
/// https://doc.powerdns.com/authoritative/backends/remote.html#lookup
///
/// Parameters: qtype, qname, zone_id, remote, local, real-remote
/// Optional: remote, local, real-remote
///
#[derive(Deserialize, PartialEq, Debug)]
pub struct LookupParams {
    qtype: String,
    qname: String,
    #[serde(rename = "zone-id")]
    zone_id: i64,
    remote: Option<String>,
    local: Option<String>,
    #[serde(rename = "real-remote")]
    real_remote: Option<String>,
}

/// Method: list
/// https://doc.powerdns.com/authoritative/backends/remote.html#list
///
/// Parameters: zonename, domain_id
/// Optional: domain_id
///
#[derive(Deserialize, PartialEq, Debug)]
pub struct ListParams {
    zonename: String,
    #[serde(rename = "domain-id")]
    domain_id: Option<i64>,
}

/// Method: getDomainInfo
/// https://doc.powerdns.com/authoritative/backends/remote.html#getdomaininfo
///
/// Parameters: name
///
#[derive(Deserialize, PartialEq, Debug)]
pub struct GetDomainInfoParams {
    name: String,
}

/// Method: directBackendCmd
/// https://doc.powerdns.com/authoritative/backends/remote.html#directbackendcmd
///
/// Parameters: query
///
#[derive(Deserialize, PartialEq, Debug)]
pub struct DirectBackendCmdParams {
    query: String,
}

/// Method: getAllDomains
/// https://doc.powerdns.com/authoritative/backends/remote.html#getalldomains
///
/// Parameters: include_disabled
///
#[derive(Deserialize, PartialEq, Debug)]
pub struct GetAllDomainsParams {
    include_disabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_query_initialize_wo_timeout() {
        let query_json = r#"{"method":"initialize", "parameters":{"path":"/path/to/socket"}}"#;

        let query_from_json: Query = serde_json::from_str(&query_json).unwrap();

        let init = InitializeParams {
            path: "/path/to/socket".to_string(),
            timeout: None,
        };

        let query = Query::Initialize(init);

        assert_eq!(query, query_from_json);
    }

    #[test]
    fn deserialize_query_initialize_w_timeout() {
        let query_json = r#"{"method":"initialize", "parameters":{"path":"/path/to/socket", "timeout":"2000"}}"#;

        let query_from_json: Query = serde_json::from_str(&query_json).unwrap();

        let init = InitializeParams {
            path: "/path/to/socket".to_string(),
            timeout: Some("2000".to_string()),
        };

        let query = Query::Initialize(init);

        assert_eq!(query, query_from_json);
    }

    #[test]
    fn deserialize_query_lookup() {
        let query_json = r#"{"method":"lookup", "parameters":{"qtype":"ANY", "qname":"www.example.com.", "remote":"192.0.2.24", "local":"192.0.2.1", "real-remote":"192.0.2.24", "zone-id":-1}}"#;

        let query_from_json: Query = serde_json::from_str(&query_json).unwrap();

        let lookup = LookupParams {
            qtype: "ANY".to_string(),
            qname: "www.example.com.".to_string(),
            zone_id: -1,
            remote: Some("192.0.2.24".to_string()),
            local: Some("192.0.2.1".to_string()),
            real_remote: Some("192.0.2.24".to_string()),
        };

        let query = Query::Lookup(lookup);

        assert_eq!(query, query_from_json);
    }
}
