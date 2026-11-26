use super::*;

#[test]
fn lifecycle_requests_preserve_the_id() {
    for method in ["initialize", "shutdown"] {
        let request = json!({"jsonrpc":"2.0","id":"test","method":method});
        assert_eq!(
            respond(&request.to_string()),
            Some(json!({"jsonrpc":"2.0","id":"test","result":null}))
        );
    }
}

#[test]
fn reports_protocol_and_connectivity_errors() {
    assert_eq!(respond("{").unwrap()["error"]["code"], -32700);
    assert_eq!(respond("{}").unwrap()["error"]["code"], -32600);
    for (method, code) in [
        ("execute_query", -32601),
        ("test_connection", -32000),
        ("ping", -32000),
    ] {
        let request = json!({"id":2,"jsonrpc":"2.0","method":method});
        assert_eq!(
            respond(&request.to_string()).unwrap()["error"]["code"],
            code
        );
    }
}

#[test]
fn notifications_have_no_reply() {
    assert_eq!(respond(r#"{"jsonrpc":"2.0","method":"initialize"}"#), None);
}
