extern crate rust_thrift_example;
extern crate thrift;

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TIoChannel, TTcpChannel};

use rust_thrift_example::{ExampleServiceSyncClient, TExampleServiceSyncClient};

fn main() {
    match run() {
        Ok(()) => println!("client ran successfully"),
        Err(e) => {
            println!("client failed with error {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> thrift::Result<()> {
    // build our client and connect to the host:port
    let mut c = TTcpChannel::new();
    c.open("127.0.0.1:9000")?;
    let (i_chan, o_chan) = c.split()?;

    // build the input/output protocol
    let i_prot = TBinaryInputProtocol::new(i_chan, true);
    let o_prot = TBinaryOutputProtocol::new(o_chan, true);

    // use the input/output protocol to create a Thrift client
    let mut client = ExampleServiceSyncClient::new(i_prot, o_prot);

    // make service calls
    let res = client.hello("Danny".to_owned())?;
    println!("{}", res);

    // done!
    Ok(())
}
