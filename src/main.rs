use server_scaffold::configuration::get_configuration;
use server_scaffold::startup::Application;
use server_scaffold::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("scaffold".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration.clone()).await?;

    application.run_until_stopped().await?;

    Ok(())
}
