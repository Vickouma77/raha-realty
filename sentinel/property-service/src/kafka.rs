use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde::Serialize;
use std::time::Duration;
#[derive(Serialize)]
pub enum PropertyEvent {
    Created { id: uuid::Uuid, address: String },
    Updated { id: uuid::Uuid, address: String },
    Deleted { id: uuid::Uuid },
}

pub async fn produce_event(event: PropertyEvent) -> Result<(), Box<dyn std::error::Error>> {
    let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .create()?;

    let topic = "property-events";
    let payload = serde_json::to_string(&event)?;

    let record = FutureRecord::to(topic)
        .payload(&payload)
        .key(&format!("{}", uuid::Uuid::new_v4()));

    producer.send(record, Duration::from_secs(0)).await?;
    Ok(())
}