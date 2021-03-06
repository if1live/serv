extern crate futures;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serv;

use hyper::server::{const_service, Http};
use std::sync::atomic::*;

struct State {
    counter: AtomicUsize,
}

#[derive(Serialize)]
struct CounterResp {
    counter: usize,
}
fn counter(s: &State, _req: serv::Empty) -> serv::error::Result<CounterResp> {
    let counter = s.counter.fetch_add(1, Ordering::SeqCst);
    Ok(CounterResp { counter })
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let state = State {
        counter: Default::default(),
    };
    let service = const_service(serv::sync::serv_state(state, counter));

    let server = Http::new().bind(&addr, service).unwrap();
    eprintln!("listen: {}", server.local_addr().unwrap());
    server.run().unwrap();
}
