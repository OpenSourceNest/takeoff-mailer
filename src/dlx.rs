// Handle Dead Letter Exchange (DLX) messages

use lapin::{
    Channel, Error, ExchangeKind,
    options::*,
    types::{AMQPValue, FieldTable, ShortString},
};

use crate::QUEUE_NAME;

const QUARANTINE_QUEUE_NAME: &str = "email_failed_queue";
const DLX_EXCHANGE_NAME: &str = "email_dlx";
const DLX_ROUTING_KEY: &str = "email_failed_key";

pub async fn setup_dlx(channel: &mut Channel) -> Result<(), Error> {
    // Declare the Dead Letter Exchange (The "Quarantine" Exchange)
    channel
        .exchange_declare(
            DLX_EXCHANGE_NAME,
            ExchangeKind::Direct,
            ExchangeDeclareOptions {
                durable: true,
                ..ExchangeDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Declare the "Quarantine" Queue (Where failed messages sit)
    channel
        .queue_declare(
            QUARANTINE_QUEUE_NAME,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Bind the Quarantine Queue to the DLX
    channel
        .queue_bind(
            QUARANTINE_QUEUE_NAME,
            DLX_EXCHANGE_NAME,
            DLX_ROUTING_KEY,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Declare the MAIN Queue with DLX arguments
    let mut queue_args = FieldTable::default();
    queue_args.insert(
        ShortString::from("x-dead-letter-exchange"),
        AMQPValue::LongString(DLX_EXCHANGE_NAME.into()),
    );
    queue_args.insert(
        ShortString::from("x-dead-letter-routing-key"),
        AMQPValue::LongString(DLX_ROUTING_KEY.into()),
    );

    channel
        .queue_declare(
            QUEUE_NAME,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
            queue_args,
        )
        .await?;

    Ok(())
}
