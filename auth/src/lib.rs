#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http0_2_0::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use bindings::wasi::http0_2_0::types::{Fields, OutgoingBody, OutgoingResponse};

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        println!("Handling request to {:?}://{:?}/{:?}", request.scheme(), request.authority(), request.path_with_query());

        let response = OutgoingResponse::new(Fields::new());
        let body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));

        let resp_stm = body.write().unwrap();
        resp_stm.write("You are authenticated!  On to plunder".as_bytes()).unwrap();
        resp_stm.flush().unwrap();
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
