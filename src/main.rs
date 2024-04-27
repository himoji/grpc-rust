use std::error::Error;

use tonic::{Request, Response, Status};
use tonic::metadata::MetadataValue;
use tonic::transport::Server;

use proto::{CalcReq, CalcResp};
use proto::admin_server::{Admin, AdminServer};
use proto::calculator_server::{Calculator, CalculatorServer};

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer zxczxczxc".parse().unwrap();

    match req.metadata().get("Authorization:") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("Auth token is not valid")),
    }
}

mod client;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct CalculatorService {
    state: State,
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Req count: {}", *count);
    }
}

#[derive(Default, Debug)]
struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_req_count(
        &self,
        _req: tonic::Request<proto::GetCountReq>,
    ) -> Result<Response<proto::CounterResp>, Status> {
        let count = self.state.read().await;
        let response = proto::CounterResp { count: *count };

        Ok(tonic::Response::new(response))
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn func(&self, request: Request<CalcReq>) -> Result<Response<CalcResp>, Status> {
        println!("req: {:?}", request);
        self.increment_counter().await;
        let input = request.get_ref();
        let resp = match input.c.as_str() {
            "+" => CalcResp {
                result: input.a + input.b,
            },
            "-" => CalcResp {
                result: input.a - input.b,
            },
            "/" => {
                if input.b == 0 {
                    return Err(Status::invalid_argument("Cannot divide by zero"));
                }
                CalcResp {
                    result: input.a / input.b,
                }
            }
            "*" => CalcResp {
                result: input.a * input.b,
            },
            &_ => return Err(Status::invalid_argument("Bad function argument")),
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    let state = State::default();

    let calc = CalculatorService {
        state: state.clone(),
    };
    let admin = AdminService {
        state: state.clone(),
    };

    println!("{}", addr);
    Server::builder()
        .add_service(service)
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
