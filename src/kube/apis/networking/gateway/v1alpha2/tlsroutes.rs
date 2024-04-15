// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --api-version=v1alpha2 --schema=disabled -f -
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "gateway.networking.k8s.io", version = "v1alpha2", kind = "TLSRoute", plural = "tlsroutes")]
#[kube(namespaced)]
#[kube(status = "TLSRouteStatus")]
#[kube(schema = "disabled")]
pub struct TLSRouteSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentRefs")]
    pub parent_refs: Option<Vec<TLSRouteParentRefs>>,
    pub rules: Vec<TLSRouteRules>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TLSRouteParentRefs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TLSRouteRules {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRefs")]
    pub backend_refs: Option<Vec<TLSRouteRulesBackendRefs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TLSRouteRulesBackendRefs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TLSRouteStatus {
    pub parents: Vec<TLSRouteStatusParents>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TLSRouteStatusParents {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(rename = "controllerName")]
    pub controller_name: String,
    #[serde(rename = "parentRef")]
    pub parent_ref: TLSRouteStatusParentsParentRef,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TLSRouteStatusParentsParentRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

