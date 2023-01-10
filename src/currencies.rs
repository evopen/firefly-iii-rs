use reqwest::Method;

use crate::Client;
use async_trait::async_trait;

#[async_trait]
pub trait Currencies {
    async fn list_currencies(&self) -> serde_json::Value;
    async fn enable_currency(&self, currency: &str) -> serde_json::Value;
    async fn disable_currency(&self, currency: &str) -> serde_json::Value;
    async fn set_default_currency(&self, currency: &str) -> serde_json::Value;
}

#[async_trait]
impl Currencies for Client {
    async fn list_currencies(&self) -> serde_json::Value {
        let req = self
            .request_builder(Method::GET, "/api/v1/currencies")
            .build()
            .unwrap();

        let res = self
            .client
            .execute(req)
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
        res
    }

    async fn enable_currency(&self, currency: &str) -> serde_json::Value {
        let req = self
            .request_builder(
                Method::POST,
                format!("/api/v1/currencies/{}/enable", currency),
            )
            .build()
            .unwrap();

        self.send(req).await
    }

    async fn disable_currency(&self, currency: &str) -> serde_json::Value {
        let req = self
            .request_builder(
                Method::POST,
                format!("/api/v1/currencies/{}/disable", currency),
            )
            .build()
            .unwrap();

        self.send(req).await
    }

    async fn set_default_currency(&self, currency: &str) -> serde_json::Value {
        let req = self
            .request_builder(
                Method::POST,
                format!("/api/v1/currencies/{}/default", currency),
            )
            .build()
            .unwrap();

        self.send(req).await
    }
}

#[cfg(test)]
mod test {
    use crate::{currencies::Currencies, Client};

    #[tokio::test]
    async fn test_list_currencies() {
        let client = Client::new("https://ff.hk.whoisdhh.com",  "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwianRpIjoiN2MwMGY0MTI4OGJmYzg3YmIxZDE4ZWE5ZTE3ODBiNDM4OTI4MjZlNjg2YzhhM2U1OTBhYmJhODhkNWEyNDFhNWYyZGRiY2JkZjliMmJlMWMiLCJpYXQiOjE2NzMyNDUxOTIuNzA0NjE2LCJuYmYiOjE2NzMyNDUxOTIuNzA0NjE5LCJleHAiOjE3MDQ3ODExOTIuNjY2Nzg3LCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.MBRcS75lttLN7j_CTFu-vxMaAHLTqAgFDyvRRWa-98-AV7vOfAU1SLG7iFBJhjYI96iNqoRtznC8yvNJPcaYQ_aUn0MQvkL8CwREgnb7SEuFEvfWSFwO_KgSPS6kLEMYrrxvz6URvH5ASIEDLJMOMyhUKjqeeaifDD1QUWJLHeG8o7Kk_RQXUScd8ogvHR9cKw_lGuxKdaTjPIe1ncFkAeS9gOXaeU2CcP3u4B8mvqPggkpzBUEfpBxXZwLOArCiFzNJPhUHP0LXfPZ17ie-Qmfw0VqfBymyzafFTIRalYx_x6k8z4FHUDJ_Vmt0-u_7b2HOxPIsU1k6DbB4859x-72Xv9qrdSuZpfo99-HBBNPKdionSLFfTf6F0D6pp8_qIt5ar1gGBTt6xltOSyRd6OSa3W2mnNLCD4l_8Pdvhnwk_-N1wbzLAB0VU7nbv_oof38ukGrFJiDHZrNsOq5fQJDCmXjhpY5cnsnpx50wyqgKfjyG9mgVW2u0pARbrGcHX6rnAQGFVcZk0DAJwa2LEG1QfcEGsK0kgF6DcKlVdoL4LC4J_kC84tPeVBLNDCSSfKy5bJR_4APHMQV089NphLT1xtMBUltpKPVnjr7EKBGYceIagW_EHQGDfxGLlvYDbd093neZja7W7bhw5lTTbs84ovxeAUMZVUcUvTmm_7U");
        let currencies = client.list_currencies().await;
        dbg!(&currencies);
    }

    #[tokio::test]
    async fn test_enable_currency() {
        let client = Client::new("https://ff.hk.whoisdhh.com",  "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIxIiwianRpIjoiN2MwMGY0MTI4OGJmYzg3YmIxZDE4ZWE5ZTE3ODBiNDM4OTI4MjZlNjg2YzhhM2U1OTBhYmJhODhkNWEyNDFhNWYyZGRiY2JkZjliMmJlMWMiLCJpYXQiOjE2NzMyNDUxOTIuNzA0NjE2LCJuYmYiOjE2NzMyNDUxOTIuNzA0NjE5LCJleHAiOjE3MDQ3ODExOTIuNjY2Nzg3LCJzdWIiOiIxIiwic2NvcGVzIjpbXX0.MBRcS75lttLN7j_CTFu-vxMaAHLTqAgFDyvRRWa-98-AV7vOfAU1SLG7iFBJhjYI96iNqoRtznC8yvNJPcaYQ_aUn0MQvkL8CwREgnb7SEuFEvfWSFwO_KgSPS6kLEMYrrxvz6URvH5ASIEDLJMOMyhUKjqeeaifDD1QUWJLHeG8o7Kk_RQXUScd8ogvHR9cKw_lGuxKdaTjPIe1ncFkAeS9gOXaeU2CcP3u4B8mvqPggkpzBUEfpBxXZwLOArCiFzNJPhUHP0LXfPZ17ie-Qmfw0VqfBymyzafFTIRalYx_x6k8z4FHUDJ_Vmt0-u_7b2HOxPIsU1k6DbB4859x-72Xv9qrdSuZpfo99-HBBNPKdionSLFfTf6F0D6pp8_qIt5ar1gGBTt6xltOSyRd6OSa3W2mnNLCD4l_8Pdvhnwk_-N1wbzLAB0VU7nbv_oof38ukGrFJiDHZrNsOq5fQJDCmXjhpY5cnsnpx50wyqgKfjyG9mgVW2u0pARbrGcHX6rnAQGFVcZk0DAJwa2LEG1QfcEGsK0kgF6DcKlVdoL4LC4J_kC84tPeVBLNDCSSfKy5bJR_4APHMQV089NphLT1xtMBUltpKPVnjr7EKBGYceIagW_EHQGDfxGLlvYDbd093neZja7W7bhw5lTTbs84ovxeAUMZVUcUvTmm_7U");
        client.disable_currency("USD").await;
        client.enable_currency("RMB").await;
        client.set_default_currency("RMB").await;
    }
}
