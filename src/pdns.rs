// https://doc.powerdns.com/authoritative/backends/remote.html#api

/*
Each JSON query has two sections, ‘method’ and ‘parameters’.
 */


// https://doc.powerdns.com/authoritative/backends/remote.html#lookup

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Query {
    method: String,
    parameters: Parameters,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Parameters {
    Init(InitReq),
    Lookup(LookupReq),
}

// Connection string
//
// parameters: path, timeout (default 2000ms)
// https://doc.powerdns.com/authoritative/backends/remote.html#unix-connector
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct InitReq {
    path: String,
    timeout: Option<String>,
}

// Lookup query
//
// Parameters: qtype, qname, zone_id
// Optional parameters: remote, local, real-remote
// https://doc.powerdns.com/authoritative/backends/remote.html#lookup
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LookupReq {
    qtype: String,
    qname: String,
    #[serde(rename = "zone-id")]
    zone_id: String,
    remote: Option<String>,
    local: Option<String>,
    #[serde(rename = "real-remote")]
    real_remote: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Response {
    Init(InitRes),
    Lookup(LookupRes),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct InitRes {
    result: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LookupRes {
    result: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_query_method_initialize() {
        let query_json = r#"{"method":"initialize", "parameters":{"path":"/path/to/socket", "timeout":"2000"}}"#;

        let query_from_json: Query = serde_json::from_str(&query_json).unwrap();

        let init = InitReq {
            path: "/path/to/socket".to_string(),
            timeout: Some("2000".to_string()),
        };

        let query = Query {
            method: "initialize".to_string(),
            parameters: Parameters::Init(init),
        };

        assert_eq!(query, query_from_json);
    }

    #[test]
    fn deserialize_query_method_lookup() {
        let query_json = r#"{"method":"lookup", "parameters":{"qtype":"ANY", "qname":"www.example.com.", "remote":"192.0.2.24", "local":"192.0.2.1", "real-remote":"192.0.2.24", "zone-id":"-1"}}"#;

        let query_from_json: Query = serde_json::from_str(&query_json).unwrap();

        let lookup = LookupReq {
            qtype: "ANY".to_string(),
            qname: "www.example.com.".to_string(),
            zone_id: "-1".to_string(),
            remote: Some("192.0.2.24".to_string()),
            local: Some("192.0.2.1".to_string()),
            real_remote: Some("192.0.2.24".to_string()),
        };

        let query = Query {
            method: "lookup".to_string(),
            parameters: Parameters::Lookup(lookup),
        };

        assert_eq!(query, query_from_json);
    }

    #[test]
    fn deserialize_response_result_true() {
        let response_json = r#"{"result":true}"#;

        let response_from_json: Response = serde_json::from_str(&response_json).unwrap();

        let response = Response::Init(
            InitRes {
                result: true
            }
        );

        assert_eq!(response, response_from_json);
    }

    #[test]
    fn deserialize_response_result_false() {
        let response_json = r#"{"result":false}"#;

        let response_from_json: Response = serde_json::from_str(&response_json).unwrap();

        let response = Response::Init(
            InitRes {
                result: false
            }
        );

        assert_eq!(response, response_from_json);
    }
}
