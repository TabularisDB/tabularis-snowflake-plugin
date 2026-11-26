use serde_json::{json, Value};

fn error(id: &Value, code: i32, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}

pub fn respond(line: &str) -> Option<Value> {
    let Ok(request) = serde_json::from_str::<Value>(line) else {
        return Some(error(&Value::Null, -32700, "Parse error"));
    };
    let id = &request["id"];
    let method = request["method"].as_str();
    if request["jsonrpc"] != "2.0"
        || method.is_none()
        || !(id.is_number() || id.is_string() || id.is_null())
    {
        return Some(error(&Value::Null, -32600, "Invalid request"));
    }
    request.get("id")?;
    Some(match method {
        Some("initialize" | "shutdown") => json!({"jsonrpc": "2.0", "id": id, "result": null}),
        Some("ping" | "test_connection") => error(
            id,
            -32000,
            "Snowflake connectivity is planned for the first implementation milestone.",
        ),
        _ => error(id, -32601, "Method not found"),
    })
}

#[cfg(test)]
mod tests;
