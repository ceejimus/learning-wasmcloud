use std::collections::HashMap;

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct HelloActor {}

// fn extract_url_params(query_string: &str) -> HashMap<String, impl IntoIterator<Item = String>> {
fn extract_url_params(query_string: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (n, v) in form_urlencoded::parse(query_string.as_bytes()) {
        let v = String::from(v);
        match map.get_mut(n.as_ref()) {
            Some(vv) => vv.push(v),
            None => {
                map.insert(n.into(), vec![v]);
            }
        }
    }
    map
}

fn handle(req: &HttpRequest) -> RpcResult<HttpResponse> {
    let params = extract_url_params(&req.query_string);

    let name = match params.get("name") {
        Some(name_vals) => &name_vals[0],
        None => "You",
    };

    let msg_id = match params.get("msg") {
        Some(msg_vals) => &msg_vals[0],
        None => "hello",
    };

    let pops = match params.get("pop") {
        Some(pop_vals) => pop_vals.join(","),
        None => String::from("No pops... :("),
    };

    let response = match msg_id {
        "hello" => format!("Hello {}", name),
        "bye" => format!("Goodbye {}", name),
        "hey" => format!("Hey {} what's up?", name),
        _ => format!("I didn't understand that, {}", name),
    };

    let response = format!("{} - {}", response, pops);

    Ok(HttpResponse {
        body: response.as_bytes().to_vec(),
        ..Default::default()
    })
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for HelloActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        handle(req)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_request(query_params: HashMap<String, Vec<String>>) -> HttpRequest {
        let mut query_string = String::new();
        for (k, vv) in query_params {
            for v in vv {
                query_string = format!("{query_string}&{k}={v}");
            }
        }

        HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            query_string,
            header: HashMap::new(),
            body: vec![],
        }
    }

    #[test]
    fn default_works() {
        let request = make_request(HashMap::new());

        let response: HttpResponse = handle(&request).unwrap();

        assert_eq!(response.status_code, 200);
        assert_eq!(
            "Hello You - No pops... :(",
            String::from_utf8(response.body).unwrap()
        )
    }
}
