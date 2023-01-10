use reqwest::Method;
use serde::Serialize;

use crate::Client;
use async_trait::async_trait;
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Accounts,
    ExpenseAccounts,
    RevenueAccounts,
    Deposits,
    Transfers,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
pub struct DestroyParams {
    #[serde(rename = "objects")]
    pub r#type: Type,
}

#[async_trait]
pub trait Data {
    async fn destroy(&self, params: DestroyParams) -> serde_json::Value;
}

#[async_trait]
impl Data for Client {
    async fn destroy(&self, params: DestroyParams) -> serde_json::Value {
        let req = self
            .request_builder(Method::DELETE, "/api/v1/data/destroy")
            .query(&params)
            .build()
            .unwrap();

        self.send(req).await
    }
}

#[cfg(test)]
mod test {
    use crate::data::{Data, DestroyParamsBuilder, Type};
    use crate::Client;

    #[tokio::test]
    async fn test_delete_accounts() {
        let client = Client::new("https://ff.hk.whoisdhh.com", "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwianRpIjoiN2MwMGY0MTI4OGJmYzg3YmIxZDE4ZWE5ZTE3ODBiNDM4OTI4MjZlNjg2YzhhM2U1OTBhYmJhODhkNWEyNDFhNWYyZGRiY2JkZjliMmJlMWMiLCJpYXQiOjE2NzMyNDUxOTIuNzA0NjE2LCJuYmYiOjE2NzMyNDUxOTIuNzA0NjE5LCJleHAiOjE3MDQ3ODExOTIuNjY2Nzg3LCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.MBRcS75lttLN7j_CTFu-vxMaAHLTqAgFDyvRRWa-98-AV7vOfAU1SLG7iFBJhjYI96iNqoRtznC8yvNJPcaYQ_aUn0MQvkL8CwREgnb7SEuFEvfWSFwO_KgSPS6kLEMYrrxvz6URvH5ASIEDLJMOMyhUKjqeeaifDD1QUWJLHeG8o7Kk_RQXUScd8ogvHR9cKw_lGuxKdaTjPIe1ncFkAeS9gOXaeU2CcP3u4B8mvqPggkpzBUEfpBxXZwLOArCiFzNJPhUHP0LXfPZ17ie-Qmfw0VqfBymyzafFTIRalYx_x6k8z4FHUDJ_Vmt0-u_7b2HOxPIsU1k6DbB4859x-72Xv9qrdSuZpfo99-HBBNPKdionSLFfTf6F0D6pp8_qIt5ar1gGBTt6xltOSyRd6OSa3W2mnNLCD4l_8Pdvhnwk_-N1wbzLAB0VU7nbv_oof38ukGrFJiDHZrNsOq5fQJDCmXjhpY5cnsnpx50wyqgKfjyG9mgVW2u0pARbrGcHX6rnAQGFVcZk0DAJwa2LEG1QfcEGsK0kgF6DcKlVdoL4LC4J_kC84tPeVBLNDCSSfKy5bJR_4APHMQV089NphLT1xtMBUltpKPVnjr7EKBGYceIagW_EHQGDfxGLlvYDbd093neZja7W7bhw5lTTbs84ovxeAUMZVUcUvTmm_7U");
        client
            .destroy(
                DestroyParamsBuilder::default()
                    .r#type(Type::Accounts)
                    .build()
                    .unwrap(),
            )
            .await;
    }
}
