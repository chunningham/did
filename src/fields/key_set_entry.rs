use crate::fields::{PublicKeyEncoding, Subject, VerificationMethod, VerificationMethodType};
use serde::{Deserialize, Serialize};

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
