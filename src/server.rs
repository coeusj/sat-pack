use std::{error::Error, net::ToSocketAddrs, pin::Pin, time::Duration};

use tokio::sync::mpsc;
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, transport::Server};

pub mod simulator;

pub mod pb {
    tonic::include_proto!("grpc.ccsds");
}

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

        let _ = request.into_inner().sat_id.into_bytes();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            let mut sim = simulator::Simulator::new();
            println!("Simulation starting");

            // Safety check: If the gRPC layer drops the receiver, abort immediately
            if tx.is_closed() {
                return;
            }

            let response_packet = SatResponse {
                packet: sim.packet.to_bin_vec()
            };

            // Try to queue the item to the client receiver
            if tx.send(Ok(response_packet)).await.is_err() {
                // Receiver dropped (Client disconnected)
                return;
            }

            loop {
                // Safety check: If the gRPC layer drops the receiver, abort immediately
                if tx.is_closed() {
                    break;
                }

                let response_packet = SatResponse {
                    packet: sim.update().to_bin_vec()
                };

                // Try to queue the item to the client receiver
                if tx.send(Ok(response_packet)).await.is_err() {
                    // Receiver dropped (Client disconnected)
                    break;
                }

                tokio::time::sleep(Duration::from_millis(sim.loop_delay)).await;
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

    println!("🛰️ Server listening on {}", addr);

    Server::builder()
        .add_service(pb::sat_server::SatServer::new(server))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Ctrl+C signal handler failed")
}