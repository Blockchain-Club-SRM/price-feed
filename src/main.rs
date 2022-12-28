use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use server::startup::Application;
use server::configuration::get_configuration;
use server::market_data_worker::run_worker_until_stopped;

#[tokio::main]
async fn main()-> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));
    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task =>  report_exit("Background worker", o),
    };
    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            println!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            print!("{} has exited with an error: {}", task_name, e)
        }
        Err(e) => {
            print!("{} has panicked: {}", task_name, e)
        }
    }
}