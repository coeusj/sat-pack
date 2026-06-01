use std::{error::Error, net::ToSocketAddrs, pin::Pin, time::Duration};

use tokio::sync::mpsc;
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, transport::Server};

// 1. Fixed module path generation matching your proto package
pub mod pb {
    tonic::include_proto!("grpc.ccsds");
}

// Bring types safely into scope based on how tonic generates packages
use pb::{SatRequest, SatResponse};

pub struct SimSatServer {}

type SatResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SatResponse, Status>> + Send>>;

#[tonic::async_trait]
impl pb::sat_server::Sat for SimSatServer {
    type SatPacketsStream = ResponseStream;

    async fn sat_packets(
        &self,
        request: Request<SatRequest>
    ) -> SatResult<Self::SatPacketsStream> {
        println!("Client connected from {:?}", request.remote_addr());

        // Extract the target ID before moving ownership into the spawn block
        let sat_id_bytes = request.into_inner().sat_id.into_bytes();

        let (tx, rx) = mpsc::channel(128);
        
        // 2. Fixed Spawn Block: Clean async loops with explicit sleep throttling
        tokio::spawn(async move {
            loop {
                // Safety check: If the gRPC layer drops the receiver, abort immediately
                if tx.is_closed() {
                    break;
                }

                let response_packet = SatResponse {
                    packet: sat_id_bytes.clone(),
                };

                // Try to queue the item to the client receiver
                if tx.send(Ok(response_packet)).await.is_err() {
                    // Receiver dropped (Client disconnected)
                    break;
                }

                // 3. Replaced `.throttle()` with clean, native async sleep
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
            println!("Client disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::SatPacketsStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = SimSatServer {};
    let addr = "localhost:50051".to_socket_addrs()?.next().unwrap();

    println!("🚀 Server successfully listening on {}", addr);

    Server::builder()
        .add_service(pb::sat_server::SatServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}