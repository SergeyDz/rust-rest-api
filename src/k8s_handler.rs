use k8s_openapi::api::core::v1::Namespace;
use kube::{Client, api::ListParams, Api};
use anyhow::Result;

pub struct K8sHandler {
    client: Client,
}

impl K8sHandler {
    pub async fn new() -> Result<Self> {
        let client = Client::try_default().await?;
        Ok(K8sHandler { client })
    }

    pub async fn list_namespaces(&self) -> Result<Vec<String>> {
        let namespaces: Api<Namespace> = Api::all(self.client.clone());
        let lp = ListParams::default();
        let ns_list = namespaces.list(&lp).await?;
        
        Ok(ns_list.iter().map(|ns| ns.metadata.name.clone().unwrap_or_default()).collect())
    }
}
