use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::{
    configuration::Settings,
    gecko_client::GeckoClient, routes::{health_check, get_coin_market_details},
};
pub struct Application {
    port: u16,
    server: Server,
}
pub struct ApplicationBaseUrl(String);

pub fn run(
    listner: TcpListener,
    gecko_client: GeckoClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let gecko_client = web::Data::new(gecko_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/market", web::get().to(get_coin_market_details))
            // .route(
            //     "/nft/{address}",
            //     web::get().to(get_native_balance_by_wallet),
            // )
            // .service(
            //     web::scope("/{chain}")
            //         .route(
            //             "/balance/{address}",
            //             web::get().to(get_native_balance_by_wallet),
            //         )
            //         .service(
            //             web::scope("/nft/{address}")
            //                 .route("", web::get().to(get_nfts_by_wallet))
            //                 .route("/collections", web::get().to(get_nft_collection_by_wallet))
            //                 .route("/transactions", web::get().to(get_nft_transfers_by_wallet)),
            //         )
            //         .service(
            //             web::scope("/transaction/{address}")
            //                 .route("", web::get().to(get_transactions_by_wallet))
            //                 .route(
            //                     "/verbose",
            //                     web::get().to(get_verbose_transactions_by_wallet),
            //                 ),
            //         )
            //         .service(
            //             web::scope("/token/{address}")
            //                 .route("", web::get().to(get_token_balance_by_wallet))
            //                 .route(
            //                     "/transactions",
            //                     web::get().to(get_token_transaction_by_wallet),
            //                 ),
            //         ),
            // )
            .app_data(gecko_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listner)?
    .run();
    Ok(server)
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let timeout = configuration.gecko_client.timeout();
        let gecko_client = GeckoClient::new(
            configuration.gecko_client.url,
            timeout,
        );
        let address = configuration.application.url();
        let listner = TcpListener::bind(address)?;
        let port = listner.local_addr().unwrap().port();
        let server = run(listner, gecko_client, configuration.application.base_url)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
