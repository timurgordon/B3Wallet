use b3_helper::error::TrapError;
use ic_cdk::export::candid::{CandidType, Deserialize};

use crate::types::RequestId;

#[rustfmt::skip]
#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub enum WalletError {
    UnknownError,
    InvalidRequest,
    InvalidNetwork,
    MissingAddress,
    InvalidTx(String),
    InvalidMsg(String),
    SignError(String),
    LedgerError(String),
    GenerateError(String),
    PublicKeyError(String),
    RequestNotFound(RequestId),
    RequestNotConfirmed(RequestId),
    RequestAlreadyConfirmed(RequestId),
    BitcoinGetBalanceError(String),
    BitcoinGetUtxosError(String),
    BitcoinSendTransactionError(String),
    BitcoinGetCurrentFeePercentilesError(String),
    CyclesMintingError(String),
    CanisterStatusError(String),
    UpdateSettingsError(String),
    InvalidTransaction(String),
    SignerRoleNotAuthorized(String),
    SignerRoleNotFound(String, String),
    SignerNotFound(String),
    SignerAlreadyExists(String),
    SignerDoesNotExist(String),
    TransactionTooOld(u64),
    AlreadySigned(String),
    ExecutionError(String),
    NotifyTopUpError(String),
    RecoverableSignatureError(String),
    DeadlineExceeded,
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
    InvalidEvmTransactionType,
    NotSignedTransaction,
    InvalidController,
    InvalidSignature(String),
    InvalidMessage(String),
    InvalidPublicKey(String),
    InvalidRecoveryId(String),
}

#[rustfmt::skip]
impl TrapError for WalletError {
    fn to_string(self) -> String {
        match self {
            WalletError::UnknownError => "Unknown error".to_string(),
            WalletError::InvalidRequest => "Invalid request".to_string(),
            WalletError::InvalidNetwork => "Invalid network".to_string(),
            WalletError::MissingAddress => "Missing address".to_string(),
            WalletError::ExecutionError(msg) => ["Execution error: ", &msg].concat(),
            WalletError::InvalidMsg(msg) => ["Invalid message: ", &msg].concat(),
            WalletError::InvalidTx(msg) => ["Invalid transaction: ", &msg].concat(),
            WalletError::SignError(msg) => ["Sign error: ", &msg].concat(),
            WalletError::SignerNotFound(msg) => [&msg, " is not a signer!"].concat(),
            WalletError::SignerRoleNotFound(signer, role) => ["Signer ", &signer, " does not have role ", &role].concat(),
            WalletError::SignerRoleNotAuthorized(signer) => ["Signer ", &signer, " is not authorized to sign!"].concat(),
            WalletError::SignerAlreadyExists(signer) => ["Signer ", &signer, " already exists!"].concat(),
            WalletError::SignerDoesNotExist(signer) => ["Signer ", &signer, " does not exist!"].concat(),
            WalletError::LedgerError(msg) => ["Ledger error: ", &msg].concat(),
            WalletError::RequestNotFound(msg) => ["Request not found: ", &msg.to_string()].concat(),
            WalletError::GenerateError(msg) => ["Generation error: ", &msg].concat(),
            WalletError::PublicKeyError(msg) => ["Public key error: ", &msg].concat(),
            WalletError::CyclesMintingError(msg) => ["Cycles minting error: ", &msg].concat(),
            WalletError::CanisterStatusError(msg) => ["Canister status error: ", &msg].concat(),
            WalletError::UpdateSettingsError(msg) => ["Update settings error: ", &msg].concat(),
            WalletError::InvalidTransaction(msg) => ["Invalid transaction: ", &msg].concat(),
            WalletError::TransactionTooOld(nanos) => ["Transaction too old: ", &nanos.to_string(), " nanoseconds"].concat(),
            WalletError::AlreadySigned(signer) => ["Signer ", &signer, " already signed"].concat(),
            WalletError::BitcoinGetBalanceError(msg) => ["Bitcoin get balance error: ", &msg].concat(),
            WalletError::BitcoinGetUtxosError(msg) => ["Bitcoin get utxos error: ", &msg].concat(),
            WalletError::BitcoinGetCurrentFeePercentilesError(msg) => ["Bitcoin get current fee percentiles error: ", &msg].concat(),
            WalletError::BitcoinSendTransactionError(msg) => ["Bitcoin send transaction error: ", &msg].concat(),
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
            WalletError::NotifyTopUpError(msg) => ["Notify top up error: ", &msg].concat(),
            WalletError::RecoverableSignatureError(msg) => ["Recoverable signature error: ", &msg].concat(),
            WalletError::InvalidController => "Invalid controller!".to_string(),
            WalletError::InvalidAddress => "Invalid address!".to_string(),
            WalletError::InvalidEvmTransactionType => "Invalid EVM transaction type!".to_string(),
            WalletError::NotSignedTransaction => "Not signed transaction!".to_string(),
            WalletError::InvalidMessage(msg) => ["Invalid message: ", &msg].concat(),
            WalletError::InvalidPublicKey(msg) => ["Invalid public key: ", &msg].concat(),
            WalletError::InvalidRecoveryId(msg) => ["Invalid recovery id: ", &msg].concat(),
            WalletError::InvalidSignature(msg) => ["Invalid signature: ", &msg].concat(),
            WalletError::DeadlineExceeded => "Deadline exceeded!".to_string(),
        }
    }
}
