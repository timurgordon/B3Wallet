use std::borrow::{Borrow, BorrowMut};

use crate::account::WalletAccount;
use crate::counter::WalletCounters;
use crate::error::WalletError;
use crate::ledger::public_keys::PublicKeys;
use crate::request::Request;
use crate::types::{AccountId, ConfirmedRequests, PendingRequestMap, RequestId, WalletAccountMap};
use b3_helper::types::{AccountsCounter, Environment, Subaccount};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct State {
    pub accounts: WalletAccountMap,
    pub counters: WalletCounters,
    pub pending_requests: PendingRequestMap,
    pub confirmed_requests: ConfirmedRequests,
}

impl Default for State {
    fn default() -> Self {
        State {
            confirmed_requests: ConfirmedRequests::new(),
            pending_requests: PendingRequestMap::new(),
            counters: WalletCounters::new(),
            accounts: WalletAccountMap::new(),
        }
    }
}

impl State {
    // Init Functions

    pub fn init_wallet(&mut self) {
        if self.counters.total() > 0 {
            return;
        }

        let mut account = WalletAccount::from(Subaccount::default());

        account.rename("Main Account".to_owned());

        self.accounts.insert("default".to_owned(), account);
    }

    // New Functions

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> Subaccount {
        let env = opt_env.unwrap_or(Environment::Production);

        let counter = self.account_counter(&env);

        Subaccount::new(env, counter)
    }

    // Insert Functions

    pub fn insert_account(
        &mut self,
        mut account: WalletAccount,
        opt_name: Option<String>,
    ) -> AccountId {
        if let Some(name) = opt_name {
            account.rename(name);
        } else {
            let env = account.environment();

            let name = self.counters.generate_next_account_name(env);

            account.rename(name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);

        id
    }

    // Account Functions

    pub fn account(&self, id: &String) -> Result<&WalletAccount, WalletError> {
        self.accounts
            .get(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn account_mut(&mut self, id: &String) -> Result<&mut WalletAccount, WalletError> {
        self.accounts
            .get_mut(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn accounts_public_keys(&self) -> Vec<PublicKeys> {
        self.accounts
            .iter()
            .map(|(_, account)| account.public_keys())
            .collect()
    }

    pub fn accounts(&self) -> Vec<WalletAccount> {
        self.accounts
            .iter()
            .map(|(_, account)| account.clone())
            .collect()
    }

    pub fn accounts_len(&self) -> usize {
        self.accounts.len()
    }

    pub fn account_status(&self) -> AccountsCounter {
        self.counters.clone().into()
    }

    pub fn account_counter(&self, env: &Environment) -> u64 {
        self.counters.account(env)
    }

    pub fn remove_account(&mut self, id: &String) -> Result<(), WalletError> {
        if id == "default" {
            return Err(WalletError::CannotRemoveDefaultAccount);
        }

        self.accounts
            .remove(id)
            .ok_or(WalletError::WalletAccountNotExists)?;

        Ok(())
    }

    pub fn hide_account(&mut self, id: &String) -> Result<(), WalletError> {
        let account = self.account_mut(id)?;

        account.hide();

        Ok(())
    }

    pub fn unhide_account(&mut self, id: &String) -> Result<(), WalletError> {
        let account = self.account_mut(id)?;

        account.unhide();

        Ok(())
    }

    // Confirmed Functions

    pub fn confirm_request(&mut self, request_id: RequestId) -> Result<(), WalletError> {
        let request = self
            .pending_requests
            .remove(&request_id)
            .ok_or(WalletError::RequestNotExists)?;

        self.confirmed_requests.push(request);

        Ok(())
    }

    pub fn confirmed(&self, request_id: RequestId) -> Result<&Request, WalletError> {
        self.confirmed_requests
            .iter()
            .find(|request| request.id() == request_id)
            .ok_or(WalletError::RequestNotConfirmed(request_id))
    }

    pub fn confirmed_requests(&self) -> &ConfirmedRequests {
        self.confirmed_requests.borrow()
    }

    pub fn confirmed_requests_mut(&mut self) -> &mut ConfirmedRequests {
        self.confirmed_requests.borrow_mut()
    }

    pub fn insert_confirmed(&mut self, request: Request) {
        self.confirmed_requests.push(request);
    }

    pub fn reset(&mut self) {
        self.accounts.clear();
        self.pending_requests.clear();
        self.confirmed_requests.clear();
        self.counters = WalletCounters::new();

        self.init_wallet();
    }
}
