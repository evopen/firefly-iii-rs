use reqwest::Method;
use serde::Serialize;

use crate::Client;
use async_trait::async_trait;
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    #[default]
    Asset,
    Expense,
    Revenue,
    Cash,
    Liability,
}

#[derive(Debug, Clone, Default, Serialize)]
pub enum Role {
    #[default]
    #[serde(rename = "defaultAsset")]
    Default,
    #[serde(rename = "sharedAsset")]
    Shared,
    #[serde(rename = "savingAsset")]
    Saving,
    #[serde(rename = "ccAsset")]
    CreditCard,
    #[serde(rename = "cashWalletAsset")]
    CashWallet,
}

#[derive(Debug, Clone, Serialize, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct CreateAccountParams {
    pub name: String,
    pub r#type: Type,
    #[serde(rename = "account_role")]
    pub role: Option<Role>,
}

#[async_trait]
pub trait Accounts {
    async fn list_accounts(&self) -> serde_json::Value;
    async fn create_account(&self, params: CreateAccountParams) -> serde_json::Value;
}

#[async_trait]
impl Accounts for Client {
    async fn list_accounts(&self) -> serde_json::Value {
        let req = self
            .request_builder(Method::GET, "/api/v1/accounts")
            .build()
            .unwrap();

        self.send(req).await
    }
    async fn create_account(&self, params: CreateAccountParams) -> serde_json::Value {
        let req = self
            .request_builder(Method::POST, "/api/v1/accounts")
            .json(&params)
            .build()
            .unwrap();

        self.send(req).await
    }
}

#[cfg(test)]
mod test {
    use crate::accounts::{CreateAccountParamsBuilder, Role, Type};
    use crate::{Accounts, Client};

    #[tokio::test]
    async fn test_list_accounts() {
        let client = Client::new("https://ff.hk.whoisdhh.com", "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwianRpIjoiN2MwMGY0MTI4OGJmYzg3YmIxZDE4ZWE5ZTE3ODBiNDM4OTI4MjZlNjg2YzhhM2U1OTBhYmJhODhkNWEyNDFhNWYyZGRiY2JkZjliMmJlMWMiLCJpYXQiOjE2NzMyNDUxOTIuNzA0NjE2LCJuYmYiOjE2NzMyNDUxOTIuNzA0NjE5LCJleHAiOjE3MDQ3ODExOTIuNjY2Nzg3LCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.MBRcS75lttLN7j_CTFu-vxMaAHLTqAgFDyvRRWa-98-AV7vOfAU1SLG7iFBJhjYI96iNqoRtznC8yvNJPcaYQ_aUn0MQvkL8CwREgnb7SEuFEvfWSFwO_KgSPS6kLEMYrrxvz6URvH5ASIEDLJMOMyhUKjqeeaifDD1QUWJLHeG8o7Kk_RQXUScd8ogvHR9cKw_lGuxKdaTjPIe1ncFkAeS9gOXaeU2CcP3u4B8mvqPggkpzBUEfpBxXZwLOArCiFzNJPhUHP0LXfPZ17ie-Qmfw0VqfBymyzafFTIRalYx_x6k8z4FHUDJ_Vmt0-u_7b2HOxPIsU1k6DbB4859x-72Xv9qrdSuZpfo99-HBBNPKdionSLFfTf6F0D6pp8_qIt5ar1gGBTt6xltOSyRd6OSa3W2mnNLCD4l_8Pdvhnwk_-N1wbzLAB0VU7nbv_oof38ukGrFJiDHZrNsOq5fQJDCmXjhpY5cnsnpx50wyqgKfjyG9mgVW2u0pARbrGcHX6rnAQGFVcZk0DAJwa2LEG1QfcEGsK0kgF6DcKlVdoL4LC4J_kC84tPeVBLNDCSSfKy5bJR_4APHMQV089NphLT1xtMBUltpKPVnjr7EKBGYceIagW_EHQGDfxGLlvYDbd093neZja7W7bhw5lTTbs84ovxeAUMZVUcUvTmm_7U");
        let accounts = client.list_accounts().await;
        dbg!(&accounts);
    }

    #[tokio::test]
    async fn test_create_account() {
        let client = Client::new("https://ff.hk.whoisdhh.com", "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwianRpIjoiN2MwMGY0MTI4OGJmYzg3YmIxZDE4ZWE5ZTE3ODBiNDM4OTI4MjZlNjg2YzhhM2U1OTBhYmJhODhkNWEyNDFhNWYyZGRiY2JkZjliMmJlMWMiLCJpYXQiOjE2NzMyNDUxOTIuNzA0NjE2LCJuYmYiOjE2NzMyNDUxOTIuNzA0NjE5LCJleHAiOjE3MDQ3ODExOTIuNjY2Nzg3LCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.MBRcS75lttLN7j_CTFu-vxMaAHLTqAgFDyvRRWa-98-AV7vOfAU1SLG7iFBJhjYI96iNqoRtznC8yvNJPcaYQ_aUn0MQvkL8CwREgnb7SEuFEvfWSFwO_KgSPS6kLEMYrrxvz6URvH5ASIEDLJMOMyhUKjqeeaifDD1QUWJLHeG8o7Kk_RQXUScd8ogvHR9cKw_lGuxKdaTjPIe1ncFkAeS9gOXaeU2CcP3u4B8mvqPggkpzBUEfpBxXZwLOArCiFzNJPhUHP0LXfPZ17ie-Qmfw0VqfBymyzafFTIRalYx_x6k8z4FHUDJ_Vmt0-u_7b2HOxPIsU1k6DbB4859x-72Xv9qrdSuZpfo99-HBBNPKdionSLFfTf6F0D6pp8_qIt5ar1gGBTt6xltOSyRd6OSa3W2mnNLCD4l_8Pdvhnwk_-N1wbzLAB0VU7nbv_oof38ukGrFJiDHZrNsOq5fQJDCmXjhpY5cnsnpx50wyqgKfjyG9mgVW2u0pARbrGcHX6rnAQGFVcZk0DAJwa2LEG1QfcEGsK0kgF6DcKlVdoL4LC4J_kC84tPeVBLNDCSSfKy5bJR_4APHMQV089NphLT1xtMBUltpKPVnjr7EKBGYceIagW_EHQGDfxGLlvYDbd093neZja7W7bhw5lTTbs84ovxeAUMZVUcUvTmm_7U");
        let accounts = client
            .create_account(
                CreateAccountParamsBuilder::default()
                    .name("test")
                    .r#type(Type::Asset)
                    .role(Role::Default)
                    .build()
                    .unwrap(),
            )
            .await;
        dbg!(&accounts);
    }
}
