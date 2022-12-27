use actix_web::{web, HttpResponse};
// use anyhow::Context;

use crate::{domains::Currency, gecko_client::GeckoClient};

use super::CoinFetchError;

#[derive(serde::Deserialize,Debug)]
pub struct PathData {
    currency: String,
    page: u16
}
// impl TryFrom<PathData> for Params {
//     type Error = String;

//     fn try_from(value: PathData) -> Result<Self, Self::Error> {
//         let page = value.page;
//         let currency = Currency::try_from(value.currency)?;
//         Ok(Self { currency })
//     }
// }

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MarketData {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub current_price: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Option<i32>,
    pub fully_diluted_valuation: Option<f64>,
    pub total_volume: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub market_cap_change_percentage_24h: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: Option<f64>,
    pub ath_change_percentage: Option<f64>,
    pub ath_date: Option<String>,
    pub atl: Option<f64>,
    pub atl_change_percentage: Option<f64>,
    pub atl_date: Option<String>,
    // pub roi: Option<f64>,
    pub last_updated: Option<String>,
}

pub async fn get_coin_market_details(
    parameters: web::Query<PathData>,
    gecko_client: web::Data<GeckoClient>,
) -> Result<HttpResponse, CoinFetchError> {
    let page = parameters.page;
    let currency = parameters
        .into_inner()
        .currency
        .try_into()
        .map_err(CoinFetchError::ValidationError)?;
    let result = coin_market_details(&gecko_client, &currency, page).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn coin_market_details(
    client: &GeckoClient,
    currency: &Currency,
    page: u16,
) -> Result<Vec<Option<MarketData>>, CoinFetchError> {
    let result = client
    .get_request(&format!("coins/markets?vs_currency={}&order=market_cap_desc&per_page=250&page={}&sparkline=false", currency.as_str(),page))
        .await?
        .json::<Vec<Option<MarketData>>>().await.map_err(|e| CoinFetchError::UnexpectedError(anyhow::anyhow!(e.to_string())))?;
    Ok(result)
}
