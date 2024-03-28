use crate::error::Result;
use std::collections::HashMap;

use bpx_api_types::{
    capital::{Balance, Deposit, DepositAddress, RequestWithdrawalPayload, Withdrawal},
    Blockchain,
};

use crate::BpxClient;

impl BpxClient {
    pub  fn get_balances(&self) -> Result<HashMap<String, Balance>> {
        let url = format!("{}/api/v1/capital", self.base_url);
        let res = self.get(url)?;
        res.json().map_err(Into::into)
    }

    pub  fn get_deposits(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Deposit>> {
        let mut url = format!("{}/wapi/v1/capital/deposits", self.base_url);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("&{}={}", k, v));
            }
        }
        let res = self.get(url)?;
        res.json().map_err(Into::into)
    }

    pub  fn get_deposit_address(&self, blockchain: Blockchain) -> Result<DepositAddress> {
        let url = format!(
            "{}/wapi/v1/capital/deposit/address?blockchain={}",
            self.base_url, blockchain
        );
        let res = self.get(url)?;
        res.json().map_err(Into::into)
    }

    pub  fn get_withdrawals(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Withdrawal>> {
        let mut url = format!("{}/wapi/v1/capital/withdrawals", self.base_url);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("{}={}&", k, v));
            }
        }
        let res = self.get(url)?;
        res.json().map_err(Into::into)
    }

    pub  fn request_withdrawal(&self, payload: RequestWithdrawalPayload) -> Result<()> {
        let endpoint = format!("{}/wapi/v1/capital/withdrawals", self.base_url);
        self.post(endpoint, payload).map(|_| ())
    }
}
