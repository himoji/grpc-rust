use std::error::Error;

use tonic::{Request, Result};

use proto::calculator_client::CalculatorClient;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;

    let req = proto::CalcReq {
        a: 4,
        b: 5,
        c: String::from("+"),
    };
    let request = Request::new(req);

    let response = client.func(request).await?;

    println!("Resp: {:?}", response.get_ref().result);

    Ok(())
}
