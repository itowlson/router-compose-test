#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http0_2_0::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use bindings::wasi::http0_2_0::types::{Fields, OutgoingBody, OutgoingResponse};

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let mut rf = routefinder::Router::new();
        rf.add("/auth", Handler::new(auth_handler)).expect("should have added route");
        rf.add("/greet", Handler::new(greet_handler)).expect("should have added route");
        rf.add("/*", Handler::new(not_found_handler)).expect("should have added route");

        let request_path = request.path_with_query().expect("should have a path");
        let handler = rf
            .best_match(&request_path)
            .expect("path should have matched a route");

        handler.invoke(request, response_out);
    }
}

struct Handler {
    handler: Box<dyn Fn(IncomingRequest, ResponseOutparam)>,
}

impl Handler {
    fn new(handler: impl Fn(IncomingRequest, ResponseOutparam) + 'static) -> Self {
        Self { handler: Box::new(handler) }
    }

    fn invoke(&self, request: IncomingRequest, response_out: ResponseOutparam) {
        (self.handler)(request, response_out);
    }
}

fn auth_handler(request: IncomingRequest, response_out: ResponseOutparam) {
    bindings::auth::handle(request, response_out)
}

fn greet_handler(request: IncomingRequest, response_out: ResponseOutparam) {
    bindings::greet::handle(request, response_out)
}

fn not_found_handler(_: IncomingRequest, response_out: ResponseOutparam) {
    let og = OutgoingResponse::new(Fields::new());
    og.set_status_code(404).unwrap();
    let ogbod = og.body().unwrap();
    ResponseOutparam::set(response_out, Ok(og));
    OutgoingBody::finish(ogbod, None).unwrap();
}

bindings::export!(Component with_types_in bindings);
