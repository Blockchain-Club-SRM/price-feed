use sqlx::PgPool;

use crate::{domains::Currency,
    configuration::Settings, 
    startup::get_connection_pool, 
    gecko_client::GeckoClient, 
    routes::{coin_market_details, store_market_data}
};

pub enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

pub async fn run_worker_until_stopped(configuration:Settings) -> Result<(), anyhow::Error> {
    let connection_pool = get_connection_pool(&configuration.database);
    let gecko_client = configuration.gecko_client.client();
    worker_loop(connection_pool, gecko_client).await
}

async fn worker_loop(pool: PgPool, gecko_client:GeckoClient) -> Result<(),anyhow::Error> {
    let mut count =1;
    
    loop {
        match try_execute_task(&pool, &gecko_client, count).await {
            Ok(ExecutionOutcome::EmptyQueue) => {
                println!("Empty queue, waiting for 360 seconds");
                tokio::time::sleep(std::time::Duration::from_secs(360)).await;
            }
            Err(e) => {
                println!("Error: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(ExecutionOutcome::TaskCompleted) => {
                println!("Task completed successfully");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                count += 1;
            }
            
        }
    }
}

async fn try_execute_task(pool: &PgPool, client: &GeckoClient, count: u16) -> Result<ExecutionOutcome, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let result = coin_market_details(client, &Currency::USD, count).await?;
    if result.is_empty() {
        return Ok(ExecutionOutcome::EmptyQueue);
    };
    for data in &result {
        if let Some(data) = data {
            if let Err(e) = store_market_data(&mut transaction, data).await{
                println!("Skipping a coin data because of Error: {}", e);
            }
        }
    }
    transaction.commit().await?;
    Ok(ExecutionOutcome::TaskCompleted)
}

