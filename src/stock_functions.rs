use serde::{Deserialize, Serialize};
use async_graphql::*;
use crate::persistence::model::{StocksEntity, NewStocksEntity, StocksSummaryEntity};
use crate::persistence::repository;
use crate::persistence::connection::create_connection_pool;

pub fn save_stock(symbol: String, shares: i32, price: String, percentage_change: String, action: String) {
    let new_stocks = NewStocksEntity {
        symbol: symbol,
        shares: shares,
        price: price,
        percentage_change: percentage_change,
        action_type: action,
        user_id: 1,
    };
    let pool = create_connection_pool();
    repository::create_stock(
        new_stocks, &mut pool.get().expect("Can't get DB connection")
    ).expect("Error to create a stock");
}

#[derive(Serialize, Deserialize)]
struct Stock {
    id: ID,
    symbol: String,
    shares: i32,
    price: String,
    percentage_change: String,
    action_type: String,
    user_id: ID,
}

#[Object]
impl Stock {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn symbol(&self) -> &String {
        &self.symbol
    }

    async fn shares(&self) -> &i32 {
        &self.shares
    }

    async fn price(&self) -> &String {
        &self.price
    }

    async fn percentage_change(&self) -> &String {
        &self.percentage_change
    }

    async fn action_type(&self) -> &String {
        &self.action_type
    }

    async fn user_id(&self) -> &ID {
        &self.user_id
    }
}

impl From<&StocksEntity> for Stock {
    fn from(entity: &StocksEntity) -> Self {
        Stock {
            id: entity.id.into(),
            symbol: entity.symbol.clone(),
            shares: entity.shares.into(),
            price: entity.price.clone(),
            percentage_change: entity.percentage_change.clone(),
            action_type: entity.action_type.clone(),
            user_id: entity.user_id.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct StockSummary {
    id: ID,
    symbol: String,
    shares: i32,
    total_value: String,
    lowest_price: String,
    highest_price: String,
    average_price: String,
    price_by_hours: String,
    profit_loss: String,
    user_id: ID,
}

#[Object]
impl StockSummary {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn symbol(&self) -> &String {
        &self.symbol
    }

    async fn shares(&self) -> &i32 {
        &self.shares
    }

    async fn total_value(&self) -> &String {
        &self.total_value
    }

    async fn lowest_price(&self) -> &String {
        &self.lowest_price
    }

    async fn highest_price(&self) -> &String {
        &self.highest_price
    }

    async fn average_price(&self) -> &String {
        &self.average_price
    }

    async fn price_by_hours(&self) -> &String {
        &self.price_by_hours
    }

    async fn profit_loss(&self) -> &String {
        &self.profit_loss
    }

    async fn user_id(&self) -> &ID {
        &self.user_id
    }
}

impl From<&StocksSummaryEntity> for StockSummary {
    fn from(entity: &StocksSummaryEntity) -> Self {
        StockSummary {
            id: entity.id.into(),
            symbol: entity.symbol.clone(),
            shares: entity.shares.into(),
            total_value: entity.total_value.clone(),
            lowest_price: entity.lowest_price.clone(),
            highest_price: entity.highest_price.clone(),
            average_price: entity.average_price.clone(),
            price_by_hours: entity.price_by_hours.clone(),
            profit_loss: entity.profit_loss.clone(),
            user_id: entity.user_id.into(),
        }
    }
}
