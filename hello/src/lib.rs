use std::collections::HashMap;

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct HelloActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for HelloActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        handle_http_request(req)
    }
}

fn handle_http_request(req: &HttpRequest) -> RpcResult<HttpResponse> {
    let params = parse_url_params(&req.query_string);

    let name = match params.get("name") {
        Some(vv) => &vv[0],
        None => "You",
    };

    let msg_id = match params.get("msg") {
        Some(vv) => &vv[0],
        None => "hello",
    };

    let response = match msg_id {
        "hello" => format!("Hello {}", name),
        "bye" => format!("Goodbye {}", name),
        "hey" => format!("Yoyoyo {}, what's up?", name),
        _ => format!("Say what, {}?", name),
    };

    Ok(HttpResponse {
        body: response.as_bytes().to_vec(),
        ..Default::default()
    })
}

fn parse_url_params(query_string: &str) -> HashMap<String, Vec<String>> {
    let mut params: HashMap<String, Vec<String>> = HashMap::new();

    for (n, v) in form_urlencoded::parse(query_string.as_bytes()) {
        let v: String = v.into();
        match params.get_mut(n.as_ref()) {
            Some(vv) => vv.push(v),
            None => {
                params.insert(n.into(), vec![v]);
            }
        }
    }

    params
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_handle_request() {
        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            query_string: "name=test".to_string(),
            header: HashMap::new(),
            body: vec![],
        };

        let response: HttpResponse = handle_http_request(&request).unwrap();

        assert_eq!(response.status_code, 200);
        assert_eq!(
            String::from_utf8(response.body).unwrap(),
            "Hello test".to_string()
        )
    }
}
