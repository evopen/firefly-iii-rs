use derive_builder::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::Client;
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Withdrawal,
    Deposit,
    Transfer,
    Reconciliation,
    #[serde(rename = "opening balance")]
    OpeningBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub r#type: Type,
    pub date: chrono::DateTime<chrono::offset::FixedOffset>,
    pub source_name: String,
    pub destination_name: String,
    pub amount: String,
    pub description: String,
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct StoreTransactionParams {
    pub error_if_duplicate_hash: bool,
    pub apply_rules: bool,
    pub fire_webhooks: bool,
    pub group_title: Option<String>,
    pub transactions: Vec<Transaction>,
}

#[async_trait]
pub trait Transactions {
    async fn list_transactions(&self) -> serde_json::Value;
    async fn store_transaction(&self, params: StoreTransactionParams) -> serde_json::Value;
}

#[async_trait]
impl Transactions for Client {
    async fn list_transactions(&self) -> serde_json::Value {
        let req = self
            .request_builder(Method::GET, "/api/v1/transactions")
            .build()
            .unwrap();

        self.send(req).await
    }
    async fn store_transaction(&self, params: StoreTransactionParams) -> serde_json::Value {
        let req = self
            .request_builder(Method::POST, "/api/v1/transactions")
            .json(&params)
            .build()
            .unwrap();

        self.send(req).await
    }
}

#[cfg(test)]
mod test {
    use crate::transactions::{StoreTransactionParamsBuilder, Transactions};
    use crate::Client;

    #[tokio::test]
    async fn test_store_transaction() {
        let client = Client::new("https://ff.hk.whoisdhh.com", "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwianRpIjoiN2MwMGY0MTI4OGJmYzg3YmIxZDE4ZWE5ZTE3ODBiNDM4OTI4MjZlNjg2YzhhM2U1OTBhYmJhODhkNWEyNDFhNWYyZGRiY2JkZjliMmJlMWMiLCJpYXQiOjE2NzMyNDUxOTIuNzA0NjE2LCJuYmYiOjE2NzMyNDUxOTIuNzA0NjE5LCJleHAiOjE3MDQ3ODExOTIuNjY2Nzg3LCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.MBRcS75lttLN7j_CTFu-vxMaAHLTqAgFDyvRRWa-98-AV7vOfAU1SLG7iFBJhjYI96iNqoRtznC8yvNJPcaYQ_aUn0MQvkL8CwREgnb7SEuFEvfWSFwO_KgSPS6kLEMYrrxvz6URvH5ASIEDLJMOMyhUKjqeeaifDD1QUWJLHeG8o7Kk_RQXUScd8ogvHR9cKw_lGuxKdaTjPIe1ncFkAeS9gOXaeU2CcP3u4B8mvqPggkpzBUEfpBxXZwLOArCiFzNJPhUHP0LXfPZ17ie-Qmfw0VqfBymyzafFTIRalYx_x6k8z4FHUDJ_Vmt0-u_7b2HOxPIsU1k6DbB4859x-72Xv9qrdSuZpfo99-HBBNPKdionSLFfTf6F0D6pp8_qIt5ar1gGBTt6xltOSyRd6OSa3W2mnNLCD4l_8Pdvhnwk_-N1wbzLAB0VU7nbv_oof38ukGrFJiDHZrNsOq5fQJDCmXjhpY5cnsnpx50wyqgKfjyG9mgVW2u0pARbrGcHX6rnAQGFVcZk0DAJwa2LEG1QfcEGsK0kgF6DcKlVdoL4LC4J_kC84tPeVBLNDCSSfKy5bJR_4APHMQV089NphLT1xtMBUltpKPVnjr7EKBGYceIagW_EHQGDfxGLlvYDbd093neZja7W7bhw5lTTbs84ovxeAUMZVUcUvTmm_7U");
        client
            .store_transaction(StoreTransactionParamsBuilder::default().build().unwrap())
            .await;
    }
}
