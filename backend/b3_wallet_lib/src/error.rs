use b3_helper::error::TrapError;
use ic_cdk::export::candid::{CandidType, Deserialize};

use crate::types::RequestId;

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum WalletError {
    UnknownError,
    InvalidTx(String),
    InvalidMsg(String),
    SignError(String),
    LedgerError(String),
    GenerateError(String),
    PublicKeyError(String),
    RequestNotFound(RequestId),
    RequestNotConfirmed(RequestId),
    RequestAlreadyConfirmed(RequestId),
    InvalidSignature(String),
    CyclesMintingError(String),
    CanisterStatusError(String),
    UpdateSettingsError(String),
    InvalidTransaction(String),
    SignerRoleNotAuthorized(String),
    SignerRoleNotFound(String, String),
    SignerNotFound(String),
    TransactionTooOld(u64),
    AlreadySigned(String),
    Processing,
    InvalidMessageLength,
    CallerIsNotOwner,
    CannotRemoveDefaultAccount,
    EcdsaPublicKeyAlreadySet,
    MissingEcdsaPublicKey,
    InvalidEcdsaPublicKey,
    InvalidAccountIdentifier,
    WalletAccountNotExists,
    RequestNotExists,
    InvalidAddress,

}

#[rustfmt::skip]
impl TrapError for WalletError {
    fn to_string(self) -> String {
        match self {
            WalletError::UnknownError => "Unknown error".to_string(),
            WalletError::InvalidMsg(msg) => ["Invalid message: ", &msg].concat(),
            WalletError::InvalidTx(msg) => ["Invalid transaction: ", &msg].concat(),
            WalletError::InvalidSignature(msg) => ["Invalid signature: ", &msg].concat(),
            WalletError::SignError(msg) => ["Sign error: ", &msg].concat(),
            WalletError::SignerNotFound(msg) => [&msg, " is not a signer!"].concat(),
            WalletError::SignerRoleNotFound(signer, role) => ["Signer ", &signer, " does not have role ", &role].concat(),
            WalletError::SignerRoleNotAuthorized(signer) => ["Signer ", &signer, " is not authorized to sign!"].concat(),
            WalletError::LedgerError(msg) => ["Ledger error: ", &msg].concat(),
            WalletError::RequestNotFound(msg) => ["Request not found: ", &msg.to_string()].concat(),
            WalletError::GenerateError(msg) => ["Generation error: ", &msg].concat(),
            WalletError::PublicKeyError(msg) => ["Public key error: ", &msg].concat(),
            WalletError::CyclesMintingError(msg) => ["Cycles minting error: ", &msg].concat(),
            WalletError::CanisterStatusError(msg) => ["Canister status error: ", &msg].concat(),
            WalletError::UpdateSettingsError(msg) => ["Update settings error: ", &msg].concat(),
            WalletError::InvalidTransaction(msg) => ["Invalid transaction: ", &msg].concat(),
            WalletError::TransactionTooOld(nanos) => ["Transaction too old: {} nanoseconds", &nanos.to_string()].concat(),
            WalletError::AlreadySigned(signer) => ["Signer ", &signer, " already signed"].concat(),
            WalletError::Processing => "Processing error".to_string(),
            WalletError::InvalidMessageLength => "Invalid message length".to_string(),
            WalletError::MissingEcdsaPublicKey => "Missing ECDSA public key".to_string(),
            WalletError::CallerIsNotOwner => "Caller is not the owner".to_string(),
            WalletError::CannotRemoveDefaultAccount => "Cannot remove default account!".to_string(),
            WalletError::EcdsaPublicKeyAlreadySet => "Public key already exists".to_string(),
            WalletError::InvalidEcdsaPublicKey => "Invalid ECDSA public key!".to_string(),
            WalletError::InvalidAccountIdentifier => "Invalid account identifier!".to_string(),
            WalletError::WalletAccountNotExists => "Wallet account does not exist!".to_string(),
            WalletError::RequestNotExists => "Request does not exist!".to_string(),
            WalletError::RequestNotConfirmed(request_id) => ["Request ", &request_id.to_string(), " not confirmed!"].concat(),
            WalletError::RequestAlreadyConfirmed(request_id) => ["Request ", &request_id.to_string(), " already confirmed!"].concat(),
            WalletError::InvalidAddress => "Invalid address!".to_string(),
        }
    }
}
