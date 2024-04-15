// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --api-version=v1alpha2 --schema=disabled -f -
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "gateway.networking.k8s.io", version = "v1alpha2", kind = "ReferenceGrant", plural = "referencegrants")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct ReferenceGrantSpec {
    pub from: Vec<ReferenceGrantFrom>,
    pub to: Vec<ReferenceGrantTo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReferenceGrantFrom {
    pub group: String,
    pub kind: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReferenceGrantTo {
    pub group: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

