// use arrow_flight::BasicAuth;
use arrow_flight::FlightClient;
use arrow_flight::FlightDescriptor;
use arrow_flight::Ticket;
use lazy_static::lazy_static;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Config
    lazy_static! {
        static ref DREMIO_URL: String = var("DREMIO_FLIGHT_URL").unwrap();
    }
    lazy_static! {
        static ref DREMIO_USER: String = var("DREMIO_USER").unwrap();
    }
    lazy_static! {
        static ref DREMIO_PASS: String = var("DREMIO_PASS").unwrap();
    }
    lazy_static! {
        static ref DREMIO_AUTH: String = var("DREMIO_AUTH").unwrap();
    }
    let channel = tonic::transport::Channel::from_static(&DREMIO_URL)
        .connect()
        .await
        .expect("error connecting");

    let mut client = FlightClient::new(channel);

    client.add_header(
        "authorization",
        format!("Basic {}", DREMIO_AUTH.as_str()).as_str(),
    )?;
    Ok(())
}
