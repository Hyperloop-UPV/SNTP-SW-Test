use ntp_server::protocol::Stratum;
use ntp_server::server::NtpServer;
use tracing::span::Attributes;
use tracing::{Id, Level, Subscriber, info};
use tracing_subscriber::layer::{Context, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, filter::LevelFilter};

/// Loguea la IP de cada solicitud que recibe el servidor.
struct RequestLogger;

impl<S: Subscriber> Layer<S> for RequestLogger {
    fn on_new_span(&self, attrs: &Attributes<'_>, _id: &Id, _ctx: Context<'_, S>) {
        // `ntp_server` abre este span con la IP del cliente por cada paquete recibido.
        if attrs.metadata().name() == "handle_request" {
            attrs.record(&mut |_: &_, ip: &dyn std::fmt::Debug| {
                info!("Solicitud SNTP recibida de {ip:?}");
            });
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::layer())
        .with(RequestLogger)
        .init();

    let server = NtpServer::builder()
        .listen("0.0.0.0:8123")
        .stratum(Stratum(2))
        .build()
        .await?;

    println!("SNTP server listening on UDP port 8123");

    server.run().await
}
