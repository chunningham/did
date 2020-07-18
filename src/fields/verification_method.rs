use crate::fields::Subject;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::default::Default;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum VerificationMethodType {
    UnknownKey,
    JwsVerificationKey2020,
    EcdsaSecp256k1VerificationKey2019,
    Ed25519VerificationKey2018,
    GpgVerificationKey2020,
    RsaVerificationKey2018,
    X25519KeyAgreementKey2019,
    SchnorrSecp256k1VerificationKey2019,
    EcdsaSecp256k1RecoveryMethod2020,
}

impl Default for VerificationMethodType {
    fn default() -> Self {
        VerificationMethodType::UnknownKey
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PublicKeyEncoding {
    Unknown,
    PublicKeyPem(String),
    // TODO, find a good JWK def crate
    // PublicKeyJwk,
    PublicKeyHex(String),
    PublicKeyBase64(String),
    PublicKeyBase58(String),
    PublicKeyMultibase(String),
    EthereumAddress(String),
}

impl Default for PublicKeyEncoding {
    fn default() -> Self {
        PublicKeyEncoding::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VerificationMethod {
    id: Subject,
    #[serde(rename = "type")]
    vm_type: VerificationMethodType,
    controller: Subject,
    #[serde(flatten)]
    key_data: PublicKeyEncoding,
    #[serde(flatten)]
    pub extra: IndexMap<String, Value>,
}

impl VerificationMethod {
    pub fn subject(&self) -> &Subject {
        &self.id
    }

    pub fn controller(&self) -> &Subject {
        &self.controller
    }

    pub fn kind(&self) -> VerificationMethodType {
        self.vm_type
    }

    pub fn encoding(&self) -> PublicKeyEncoding {
        // TODO can this be a reference?
        self.key_data.clone()
    }

    pub fn data(&self) -> &String {
        match &self.key_data {
            PublicKeyEncoding::PublicKeyHex(s)
            | PublicKeyEncoding::PublicKeyBase58(s)
            | PublicKeyEncoding::PublicKeyBase64(s)
            | PublicKeyEncoding::PublicKeyPem(s)
            | PublicKeyEncoding::PublicKeyMultibase(s)
            | PublicKeyEncoding::EthereumAddress(s) => &s,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum KeySetEntry {
    Method(VerificationMethod),
    Reference(Subject),
}

impl KeySetEntry {
    pub fn subject(&self) -> &Subject {
        match self {
            Self::Method(vm) => vm.subject(),
            Self::Reference(subject) => &subject,
        }
    }

    pub fn kind(&self) -> VerificationMethodType {
        match self {
            Self::Method(vm) => vm.kind(),
            Self::Reference(_) => VerificationMethodType::UnknownKey,
        }
    }

    pub fn encoding(&self) -> PublicKeyEncoding {
        match self {
            Self::Method(vm) => vm.encoding(),
            Self::Reference(_) => PublicKeyEncoding::Unknown,
        }
    }
}
