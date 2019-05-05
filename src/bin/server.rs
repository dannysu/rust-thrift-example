use thrift::protocol::{TBinaryInputProtocolFactory, TInputProtocolFactory};
use thrift::protocol::{TBinaryOutputProtocolFactory, TOutputProtocolFactory};
use thrift::server::TServer;
use thrift::transport::{TBufferedReadTransportFactory, TReadTransportFactory};
use thrift::transport::{TBufferedWriteTransportFactory, TWriteTransportFactory};

use rust_thrift_example::{ExampleServiceSyncHandler, ExampleServiceSyncProcessor};

fn main() {
    match run() {
        Ok(()) => println!("server ran successfully"),
        Err(e) => {
            println!("server failed with error {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> thrift::Result<()> {
    // set listen address
    let listen_address = "127.0.0.1:9000";

    // create input protocol/transport factory
    let i_tran_fact: Box<TReadTransportFactory> = Box::new(TBufferedReadTransportFactory::new());
    let i_prot_fact: Box<TInputProtocolFactory> = Box::new(TBinaryInputProtocolFactory::new());

    // create output protocol/transport factory
    let o_tran_fact: Box<TWriteTransportFactory> = Box::new(TBufferedWriteTransportFactory::new());
    let o_prot_fact: Box<TOutputProtocolFactory> = Box::new(TBinaryOutputProtocolFactory::new());

    // create the server and start listening
    let processor = ExampleServiceSyncProcessor::new(ExampleServiceHandlerImpl {});
    let mut server = TServer::new(
        i_tran_fact,
        i_prot_fact,
        o_tran_fact,
        o_prot_fact,
        processor,
        10,
    );

    println!("binding to {}", listen_address);
    server.listen(&listen_address)
}

#[derive(Default)]
struct ExampleServiceHandlerImpl;
impl ExampleServiceSyncHandler for ExampleServiceHandlerImpl {
    fn handle_hello(&self, name: String) -> thrift::Result<String> {
        Ok(format!("Hello {}!", name))
    }
}
