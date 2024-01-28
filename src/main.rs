use arrow_flight::ActionType;
use arrow_flight::FlightClient;
use futures::stream::TryStreamExt;
use lazy_static::lazy_static;

use std::env::var;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    lazy_static! {
        static ref DREMIO_URL: String = var("DREMIO_FLIGHT_URL").unwrap();
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

    let actions: Vec<ActionType> = client
        .list_actions()
        .await
        .expect("error listing actions")
        .try_collect() // use TryStreamExt to collect stream
        .await
        .expect("error gathering actions");
    print!("{:?}", actions);
    Ok(())
}
