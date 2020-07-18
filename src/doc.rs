use crate::fields::{
    string_or_list, Context, KeySetEntry, ServiceEndpoint, Subject, VerificationMethod,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::str::FromStr;
use std::string::{String, ToString};
use void::Void;

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    #[serde(rename = "@context", deserialize_with = "string_or_list")]
    context: Context,

    id: Subject,

    #[serde(skip_serializing_if = "String::is_empty", default)]
    created: String,

    #[serde(skip_serializing_if = "String::is_empty", default)]
    updated: String,

    #[serde(
        rename = "verificationMethod",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    verification_method: Vec<VerificationMethod>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    authentication: Vec<KeySetEntry>,

    #[serde(
        rename = "assertionMethod",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    assertion_method: Vec<KeySetEntry>,

    #[serde(
        rename = "keyAgreement",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    key_agreement: Vec<KeySetEntry>,

    #[serde(
        rename = "capabilityInvocation",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    capability_invocation: Vec<KeySetEntry>,

    #[serde(
        rename = "capabilityDelegation",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    capability_delegation: Vec<KeySetEntry>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    service: Vec<ServiceEndpoint>,

    //#[serde(skip_serializing_if = "Proof::is_empty", default)]
    //pub proof: Proof,
    #[serde(flatten)]
    pub extra: IndexMap<String, Value>,
}

impl Document {
    pub fn new(context: &str, id: &str) -> Self {
        Document {
            context: Context::from_str(context).unwrap(),
            id: Subject::from_str(id).unwrap(),
            created: String::new(),
            updated: String::new(),
            verification_method: Vec::default(),
            authentication: Vec::default(),
            assertion_method: Vec::default(),
            key_agreement: Vec::default(),
            capability_invocation: Vec::default(),
            capability_delegation: Vec::default(),
            service: Vec::default(),
            extra: IndexMap::default(),
        }
    }

    pub fn context(&self) -> &Vec<String> {
        &self.context.as_vec()
    }

    pub fn subject(&self) -> &Subject {
        &self.id
    }

    pub fn verification_method(&self) -> &Vec<VerificationMethod> {
        &self.verification_method
    }

    pub fn authentication(&self) -> &Vec<KeySetEntry> {
        &self.authentication
    }

    pub fn service(&self) -> &Vec<ServiceEndpoint> {
        &self.service
    }
}

impl ToString for Document {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl FromStr for Document {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let doc = serde_json::from_str(s).unwrap();
        Ok(doc)
    }
}
