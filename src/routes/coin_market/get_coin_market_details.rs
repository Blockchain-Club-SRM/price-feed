use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{domains::Currency, gecko_client::GeckoClient};

use super::{CoinFetchError, StoreTokenError};

#[derive(serde::Deserialize, Debug)]
pub struct PathData {
    currency: String,
    page: u16,
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
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CoinFetchError> {
    let page = parameters.page;
    let currency = parameters
        .into_inner()
        .currency
        .try_into()
        .map_err(CoinFetchError::ValidationError)?;
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    let result = coin_market_details(&gecko_client, &currency, page).await?;
    for data in &result {
        if let Some(data) = data {
            store_market_data(&mut transaction, &data).await.context("Failed to store market data")?;
        }
    }
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a market data.")?;
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

pub async fn store_market_data(
    transaction: &mut Transaction<'_, Postgres>,
    data: &MarketData,
) -> Result<(), StoreTokenError> {
    sqlx::query!(
        r#"
            INSERT INTO market_data (
                id,
                symbol,
                name,
                image,
                current_price,
                market_cap,
                market_cap_rank,
                fully_diluted_valuation,
                total_volume,
                high_24h,
                low_24h,
                price_change_24h,
                price_change_percentage_24h,
                market_cap_change_24h,
                market_cap_change_percentage_24h,
                circulating_supply,
                total_supply,
                max_supply,
                ath,
                ath_change_percentage,
                ath_date,
                atl,
                atl_change_percentage,
                atl_date,
                last_updated
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                $12,
                $13,
                $14,
                $15,
                $16,
                $17,
                $18,
                $19,
                $20,
                $21,
                $22,
                $23,
                $24,
                $25
            )
            ON CONFLICT (id) DO UPDATE SET
                symbol = $2,
                name = $3,
                image = $4,
                current_price = $5,
                market_cap = $6,
                market_cap_rank = $7,
                fully_diluted_valuation = $8,
                total_volume = $9,
                high_24h = $10,
                low_24h = $11,
                price_change_24h = $12,
                price_change_percentage_24h = $13,
                market_cap_change_24h = $14,
                market_cap_change_percentage_24h = $15,
                circulating_supply = $16,
                total_supply = $17,
                max_supply = $18,
                ath = $19,
                ath_change_percentage = $20,
                ath_date = $21,
                atl = $22,
                atl_change_percentage = $23,
                atl_date = $24,
                last_updated = $25
            "#,
        data.id,
        data.symbol,
        data.name,
        data.image,
        data.current_price,
        data.market_cap,
        data.market_cap_rank,
        data.fully_diluted_valuation,
        data.total_volume,
        data.high_24h,
        data.low_24h,
        data.price_change_24h,
        data.price_change_percentage_24h,
        data.market_cap_change_24h,
        data.market_cap_change_percentage_24h,
        data.circulating_supply,
        data.total_supply,
        data.max_supply,
        data.ath,
        data.ath_change_percentage,
        data.ath_date,
        data.atl,
        data.atl_change_percentage,
        data.atl_date,
        data.last_updated
    )
    .execute(transaction)
    .await
    .map_err(StoreTokenError)?;
    Ok(())
}
