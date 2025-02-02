use hdk::prelude::*;
use crate::change_rule::entry::Authorization;
#[cfg(test)]
use ::fixt::prelude::*;
#[cfg(test)]
use crate::change_rule::entry::AuthorizationVecFixturator;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const KEY_REGISTRATION_INDEX: EntryDefIndex = EntryDefIndex(6);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct KeyGeneration {
    new_key: AgentPubKey,
    new_key_signing_of_author: Signature,
    // Ensure the generator has the same author as the KeyRegistration.
    generator: HeaderHash,
    generator_signature: Signature,
}

#[cfg(test)]
fixturator!(
    KeyGeneration;
    constructor fn new(AgentPubKey, Signature, HeaderHash, Signature);
);

impl KeyGeneration {
    pub fn new(new_key: AgentPubKey, new_key_signing_of_author: Signature, generator: HeaderHash, generator_signature: Signature) -> Self {
        Self { new_key, new_key_signing_of_author, generator, generator_signature }
    }

    pub fn as_new_key_ref(&self) -> &AgentPubKey {
        &self.new_key
    }

    pub fn as_new_key_signing_of_author_ref(&self) -> &Signature {
        &self.new_key_signing_of_author
    }

    pub fn as_generator_ref(&self) -> &HeaderHash {
        &self.generator
    }

    pub fn as_generator_signature_ref(&self) -> &Signature {
        &self.generator_signature
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct KeyRevocation {
    prior_key_registration: HeaderHash,
    // To be validated according to the change rule of the generator of the prior key.
    revocation_authorization: Vec<Authorization>,
}

#[cfg(test)]
fixturator!(
    KeyRevocation;
    constructor fn new(HeaderHash, AuthorizationVec);
);

impl KeyRevocation {
    pub fn new(prior_key_registration: HeaderHash, revocation_authorization: Vec<Authorization>) -> Self {
        Self { prior_key_registration, revocation_authorization }
    }

    pub fn as_prior_key_registration_ref(&self) -> &HeaderHash {
        &self.prior_key_registration
    }

    pub fn as_revocation_authorization_ref(&self) -> &[Authorization] {
        &self.revocation_authorization
    }
}

#[hdk_entry(id = "key_registration")]
#[derive(Clone)]
pub enum KeyRegistration {
    Create(KeyGeneration),
    CreateOnly(KeyGeneration), // Keys for hosted web users may be of this type, cannot revoke
    Update(KeyRevocation, KeyGeneration),
    Delete(KeyRevocation)
}

#[cfg(test)]
fixturator!(
    KeyRegistration;
    // @todo support Update as a variant (it has two inner types).
    variants [ Create(KeyGeneration) Delete(KeyRevocation) ];
);

impl TryFrom<&Element> for KeyRegistration {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // All CRUD are allowed for a KeyRegistration.
            Header::Create(_) | Header::Update(_) | Header::Delete(_) => {
                Ok(match element.entry() {
                    ElementEntry::Present(serialized) => match Self::try_from(serialized) {
                        Ok(deserialized) => deserialized,
                        Err(e) => return Err(crate::error::Error::Wasm(e)),
                    }
                    __ => return Err(crate::error::Error::EntryMissing),
                })
            },
            _ => Err(crate::error::Error::WrongHeader),
        }

    }
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::KEY_REGISTRATION_INDEX;
    use super::KeyRegistration;

    #[test]
    fn key_registration_index_test() {
        assert_eq!(
            KEY_REGISTRATION_INDEX,
            entry_def_index!(KeyRegistration).unwrap()
        )
    }
}