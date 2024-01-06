use arrow_flight::flight_service_client::FlightServiceClient;
use arrow_flight::Ticket;
use tonic::Request;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a channel to the Dremio server
    let channel = tonic::transport::Channel::from_static("http://[dremio-server-address]")
        .connect()
        .await?;

    // Create a client using the channel
    let mut client = FlightServiceClient::new(channel);

    // Create a ticket with your SQL query
    let ticket = Ticket {
        ticket: "[Your SQL query here]".into(),
    };

    // Send a get_flight_info request to the server
    let request = Request::new(ticket);
    let response = client.get_flight_info(request).await?;

    // Get the first endpoint from the response
    let endpoint = response
        .into_inner()
        .flight_descriptor
        .unwrap()
        .endpoint
        .unwrap()[0]
        .clone();

    // Use the endpoint to create a ticket for the stream
    let ticket = Ticket {
        ticket: endpoint.ticket.ticket,
    };

    // Create a stream from the ticket
    let request = Request::new(ticket);
    let mut stream = client.do_get(request).await?.into_inner();

    // Collect the data from the stream into a Polars DataFrame
    let mut df = None;
    while let Some(flight_data) = stream.message().await? {
        let record_batch = flight_data.record_batch()?;
        let temp_df = DataFrame::try_from(record_batch)?;
        df = match df {
            Some(df) => Some(df.vstack(&temp_df)?),
            None => Some(temp_df),
        };
    }

    // Use the DataFrame
    if let Some(df) = df {
        println!("{:?}", df);
    }

    Ok(())
}