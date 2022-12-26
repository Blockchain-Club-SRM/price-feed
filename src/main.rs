use server::startup::Application;
use server::configuration::get_configuration;

#[tokio::main]
async fn main()->std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await.expect("Failed to build application.");
    let port = application.port();
    println!("Listening on port {}", port);
    application.run_until_stopped().await?;
    Ok(())
}