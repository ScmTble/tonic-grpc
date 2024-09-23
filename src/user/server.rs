use tonic::{transport::Server, Request, Response, Status};

pub mod user {
    tonic::include_proto!("user");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("user_descriptor");
}
#[derive(Default)]
pub struct UserService {}

#[tonic::async_trait]
impl user::user_service_server::UserService for UserService {
    async fn hello(
        &self, 
        req: Request<user::HelloRequest>) -> Result<Response<user::HelloResponse>,Status > {

       let resp = user::HelloResponse {
            message: format!("Hello {}!", req.into_inner().name),
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(user::FILE_DESCRIPTOR_SET)
    .build_v1alpha()
    .unwrap();

    let addr = "[::]:8080".parse().unwrap();
    let user_svc = UserService::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(service)
        .add_service(user::user_service_server::UserServiceServer::new(user_svc))
        .serve(addr)
        .await?;

    Ok(())
}
