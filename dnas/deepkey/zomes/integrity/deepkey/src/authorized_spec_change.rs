use crate::AuthoritySpec;
use hdi::prelude::*;
pub type Authorization = (u8, Signature);
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AuthorizedSpecChange {
    pub new_spec: AuthoritySpec,
    // Signature of the content of the authority_spec field,
    // signed by throwaway RootKey on Create,
    // or according to previous AuthSpec upon Update.
    pub authorization_of_new_spec: Vec<Authorization>,
}
impl AuthorizedSpecChange {
    pub fn new(new_spec: AuthoritySpec, authorization_of_new_spec: Vec<Authorization>) -> Self {
        Self {
            new_spec,
            authorization_of_new_spec,
        }
    }
    pub fn as_new_spec_ref(&self) -> &AuthoritySpec {
        &self.new_spec
    }
    pub fn as_authorization_of_new_spec_ref(&self) -> &Vec<Authorization> {
        &self.authorization_of_new_spec
    }
}
pub fn validate_create_authorized_spec_change(
    _action: EntryCreationAction,
    _authorized_spec_change: AuthorizedSpecChange,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_authorized_spec_change(
    _action: Update,
    _authorized_spec_change: AuthorizedSpecChange,
    _original_action: EntryCreationAction,
    _original_authorized_spec_change: AuthorizedSpecChange,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Authorized Spec Changes cannot be updated",
    )))
}
pub fn validate_delete_authorized_spec_change(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_authorized_spec_change: AuthorizedSpecChange,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Authorized Spec Changes cannot be deleted",
    )))
}
