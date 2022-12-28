mod coin_fetch_error;
mod get_coin_market_details;
pub use coin_fetch_error::{CoinFetchError,StoreTokenError};
pub use get_coin_market_details::{get_coin_market_details,coin_market_details,store_market_data};