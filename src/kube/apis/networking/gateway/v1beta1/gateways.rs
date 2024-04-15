// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --api-version=v1beta1 --schema=disabled -f -
// kopium version: 0.17.2

use k8s_openapi::{apimachinery::pkg::apis::meta::v1::Condition, NamespaceResourceScope};
use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

impl k8s_openapi::Resource for Gateway {
    const API_VERSION: &'static str = "gateway.networking.k8s.io/v1beta1";

    const GROUP: &'static str = "gateway.networking.k8s.io";

    const KIND: &'static str = "Gateway";

    const VERSION: &'static str = "v1beta1";

    const URL_PATH_SEGMENT: &'static str = "gateways";

    type Scope = NamespaceResourceScope;
}

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default)]
#[kube(group = "gateway.networking.k8s.io", version = "v1beta1", kind = "Gateway", plural = "gateways")]
#[kube(namespaced)]
#[kube(derive = "Default")]
#[kube(status = "GatewayStatus")]
#[kube(schema = "disabled")]
pub struct GatewaySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayAddresses>>,
    #[serde(rename = "gatewayClassName")]
    pub gateway_class_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<GatewayInfrastructure>,
    pub listeners: Vec<GatewayListeners>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayAddresses {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayInfrastructure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parametersRef")]
    pub parameters_ref: Option<GatewayInfrastructureParametersRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayInfrastructureParametersRef {
    pub group: String,
    pub kind: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListeners {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRoutes")]
    pub allowed_routes: Option<GatewayListenersAllowedRoutes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    pub name: String,
    pub port: i32,
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayListenersTls>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListenersAllowedRoutes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<GatewayListenersAllowedRoutesKinds>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<GatewayListenersAllowedRoutesNamespaces>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListenersAllowedRoutesKinds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListenersAllowedRoutesNamespaces {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<GatewayListenersAllowedRoutesNamespacesFrom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<GatewayListenersAllowedRoutesNamespacesSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GatewayListenersAllowedRoutesNamespacesFrom {
    All,
    Selector,
    Same,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListenersAllowedRoutesNamespacesSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListenersTls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateRefs")]
    pub certificate_refs: Option<Vec<GatewayListenersTlsCertificateRefs>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayListenersTlsMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayListenersTlsCertificateRefs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GatewayListenersTlsMode {
    Terminate,
    Passthrough,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayStatusAddresses>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<GatewayStatusListeners>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayStatusAddresses {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayStatusListeners {
    #[serde(rename = "attachedRoutes")]
    pub attached_routes: i32,
    pub conditions: Vec<Condition>,
    pub name: String,
    #[serde(rename = "supportedKinds")]
    pub supported_kinds: Vec<GatewayStatusListenersSupportedKinds>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayStatusListenersSupportedKinds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
}

