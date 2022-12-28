-- Add migration script here

CREATE TABLE
    market_data (
        id TEXT NOT NULL UNIQUE,
        PRIMARY KEY (id),
        symbol TEXT NOT NULL,
        name TEXT,
        image TEXT,
        current_price FLOAT,
        market_cap FLOAT,
        market_cap_rank INTEGER,
        fully_diluted_valuation FLOAT,
        total_volume FLOAT,
        high_24h FLOAT,
        low_24h FLOAT,
        price_change_24h FLOAT,
        price_change_percentage_24h FLOAT,
        market_cap_change_24h FLOAT,
        market_cap_change_percentage_24h FLOAT,
        circulating_supply FLOAT,
        total_supply FLOAT,
        max_supply FLOAT,
        ath FLOAT,
        ath_change_percentage FLOAT,
        ath_date TEXT,
        atl FLOAT,
        atl_change_percentage FLOAT,
        atl_date TEXT,
        last_updated TEXT,
        created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
    );