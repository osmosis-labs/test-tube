use cosmrs::{
    crypto::{secp256k1::SigningKey, PublicKey},
    AccountId,
};
use cosmwasm_std::Coin;

pub trait Account {
    fn public_key(&self) -> PublicKey;
    fn address(&self) -> String {
        self.account_id().to_string()
    }
    fn prefix(&self) -> &str;
    fn account_id(&self) -> AccountId {
        self.public_key()
            .account_id(self.prefix())
            .expect("Prefix is constant and must valid")
    }
}
pub struct SigningAccount {
    prefix: String,
    signing_key: SigningKey,
    fee_setting: FeeSetting,
}

impl SigningAccount {
    pub fn new(prefix: String, signing_key: SigningKey, fee_setting: FeeSetting) -> Self {
        SigningAccount {
            prefix,
            signing_key,
            fee_setting,
        }
    }

    pub fn with_prefix(self, prefix: String) -> Self {
        Self {
            prefix,
            signing_key: self.signing_key,
            fee_setting: self.fee_setting,
        }
    }

    pub fn fee_setting(&self) -> &FeeSetting {
        &self.fee_setting
    }

    pub fn with_fee_setting(self, fee_setting: FeeSetting) -> Self {
        Self {
            prefix: self.prefix,
            signing_key: self.signing_key,
            fee_setting,
        }
    }
}

impl Account for SigningAccount {
    fn public_key(&self) -> PublicKey {
        self.signing_key.public_key()
    }

    fn prefix(&self) -> &str {
        &self.prefix
    }
}

impl SigningAccount {
    pub fn signing_key(&'_ self) -> &'_ SigningKey {
        &self.signing_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonSigningAccount {
    prefix: String,
    public_key: PublicKey,
}

impl From<PublicKey> for NonSigningAccount {
    fn from(public_key: PublicKey) -> Self {
        NonSigningAccount {
            prefix: String::from(""),
            public_key,
        }
    }
}
impl From<SigningAccount> for NonSigningAccount {
    fn from(signing_account: SigningAccount) -> Self {
        NonSigningAccount {
            prefix: signing_account.prefix.clone(),
            public_key: signing_account.public_key(),
        }
    }
}

impl NonSigningAccount {
    pub fn new(prefix: String, public_key: PublicKey) -> Self {
        NonSigningAccount { prefix, public_key }
    }

    pub fn with_prefix(self, prefix: String) -> Self {
        Self {
            prefix,
            public_key: self.public_key,
        }
    }
}

impl Account for NonSigningAccount {
    fn public_key(&self) -> PublicKey {
        self.public_key
    }

    fn prefix(&self) -> &str {
        &self.prefix
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeeSetting {
    Auto {
        gas_price: Coin,
        gas_adjustment: f64,
    },
    Custom {
        amount: Coin,
        gas_limit: u64,
    },
}
