use proto::greet_v1::{
    greet_service_server::{GreetService, GreetServiceServer},
    GreetRequest, GreetResponse,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct MyGreetService {}

#[tonic::async_trait]
impl GreetService for MyGreetService {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        let req = request.into_inner();
        println!("reqest name : {}", req.name);

        let reply = GreetResponse {
            greeting: format!("Hello! {}", req.name).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greet = MyGreetService::default();

    Server::builder()
        .add_service(GreetServiceServer::new(greet))
        .serve(addr)
        .await?;
    Ok(())
}
