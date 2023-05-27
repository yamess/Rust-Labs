use log::{info};
use chrono;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
use tonic::{transport::Server, Request, Response, Status};
use product::product_server::{Product, ProductServer};
use product::{ProductRequest, ProductResponse};


fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::BrightBlue)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // .chain(fern::log_file("output.txt")?)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

pub mod product {
    tonic::include_proto!("product");
}

#[derive(Default)]
pub struct ProductService {}

#[tonic::async_trait]
impl Product for ProductService {
   async fn get_product(&self, _request: Request<ProductRequest>) -> Result<Response<ProductResponse>, Status>{
       let reply = ProductResponse{
           id: 1,
           name: "Product 1".to_string(),
           description: "Description 1".to_string(),
           price: 100.0,
       };
       info!("Get product: {:?}", reply);
       Ok(Response::new(reply))
   }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger().expect("Failed to initialize logger.");
    info!("Starting gRPC server...");
    let addr = "[::1]:50051".parse()?;
    let product_service = ProductService::default();

    Server::builder()
        .add_service(ProductServer::new(product_service))
        .serve(addr)
        .await?;
    info!("gRPC server started.");
    Ok(())
}
