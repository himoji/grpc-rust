use std::error::Error;

use tonic::{Request, Response, Status};
use tonic::transport::Server;

use proto::{CalcReq, CalcResp};
use proto::calculator_server::{Calculator, CalculatorServer};

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService {}
mod client;

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(&self, request: Request<CalcReq>) -> Result<Response<CalcResp>, Status> {
        println!("req: {:?}", request);
        let input = request.get_ref();
        let resp = CalcResp {
            result: input.a + input.b,
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse()?;
    let calc = CalculatorService::default();
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    println!("{}", addr);
    Server::builder()
        .add_service(service)
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
