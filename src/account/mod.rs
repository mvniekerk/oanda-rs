pub mod details;
pub mod instruments;
pub mod summary;

use std::error::Error;
use std::fmt::Debug;
use crate::client::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json;

use self::details::AccountDetails;
use self::details::Details;
use self::instruments::AccountInstruments;
use self::instruments::Instrument;
use self::summary::AccountSummary;
use self::summary::Summary;

fn none() -> Option<&'static Client<'static>> {
    None
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Accounts<'a> {
    pub accounts: Vec<Account<'a>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Account<'a> {
    pub id: String,
    pub tags: Vec<String>,
    #[serde(default = "none")]
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub client: Option<&'a Client<'a>>,
}

impl <'a> Debug for Account<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("id", &self.id)
            .field("tags", &self.tags)
            .finish()
    }
}

impl<'a> Account<'a> {
    pub async fn details(&self) -> Result<Details, Box<dyn Error>> {
        let input = self.client().get(format!("accounts/{}", self.id).as_str()).await?;
        let result: AccountDetails = serde_json::from_str(&input)
            .map_err(|e| {
                eprintln!("Error deserializing account details: {}", e);
                eprintln!("Input: {}", input);
                e
            })?;

        Ok(result.account)
    }

    pub async fn instruments(&self) -> Result<Vec<Instrument>, Box<dyn Error>> {
        let input = self
            .client()
            .get(format!("accounts/{}/instruments", self.id).as_str())
            .await?;
        let result: AccountInstruments = serde_json::from_str(&input)?;

        Ok(result.instruments)
    }

    pub async fn summary(&self) -> Result<Summary, Box<dyn Error>> {
        let input = self
            .client()
            .get(format!("accounts/{}/summary", self.id).as_str())
            .await?;
        let result: AccountSummary = serde_json::from_str(&input)?;

        Ok(result.account)
    }

    fn client(&self) -> &'a Client<'a> {
        self.client.expect("Account cannot refer to a client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// # TODO: Move integration tests to `tests/`
    #[tokio::test]
    async fn it_can_read_account_details() {
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();

        let client = Client::new(&url, &key);
        let accounts = client.accounts().await.unwrap();

        let account = accounts.first().unwrap().clone();
        let details = account.details().await.unwrap();

        assert_eq!(details.alias.unwrap(), "Testv20")
    }

    #[tokio::test]
    async fn it_can_read_account_summary() {
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();

        let client = Client::new(&url, &key);
        let accounts = client.accounts().await.unwrap();

        let account = accounts.first().unwrap().clone();
        let summary = account.summary().await.unwrap();

        assert_eq!(summary.alias.unwrap(), "Testv20")
    }

    #[tokio::test]
    async fn it_can_read_account_instruments() {
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();

        let client = Client::new(&url, &key);
        let accounts = client.accounts().await.unwrap();

        let account = accounts.first().unwrap().clone();
        let instruments = account.instruments().await.unwrap();
        /// the result here is a list of all USD_* tradable currencies because
        /// the test account is USD, just make sure we find one we "expect"
        assert_eq!(instruments.into_iter().any(|x| x.name == "USD_DKK"), true)
    }
}
