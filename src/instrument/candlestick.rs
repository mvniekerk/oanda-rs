use chrono::DateTime;
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

use super::candlestick_data::CandlestickData;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Candlestick {
    /// The start time of the candlestick
    pub time: DateTime<Utc>,
    /// The candlestick data based on bids. Only provided if bid-based candles
    /// were requested.
    pub bid: Option<CandlestickData>,
    /// The candlestick data based on asks. Only provided if ask-based candles
    /// were requested.
    pub ask: Option<CandlestickData>,
    /// The candlestick data based on midpoints. Only provided if midpoint-based
    /// candles were requested.
    pub mid: Option<CandlestickData>,
    /// The number of prices created during the time-range represented by the
    /// candlestick.
    pub volume: i32,
    /// A flag indicating if the candlestick is complete. A complete candlestick
    /// is one whose ending time is not in the future.
    pub complete: bool,
}
