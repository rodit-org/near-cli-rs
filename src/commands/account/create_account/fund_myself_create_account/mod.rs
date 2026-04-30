mod add_key;
mod sign_as;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = crate::GlobalContext)]
#[interactive_clap(output_context = NewAccountContext)]
pub struct NewAccount {
    #[interactive_clap(skip_default_input_arg)]
    /// What is the new account ID?
    new_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(skip_default_input_arg)]
    /// Enter the amount for the account:
    initial_balance: crate::types::near_token::NearToken,
    #[interactive_clap(subcommand)]
    access_key_mode: add_key::AccessKeyMode,
}

#[derive(Debug, Clone)]
pub struct NewAccountContext {
    global_context: crate::GlobalContext,
    new_account_id: near_primitives::types::AccountId,
    initial_balance: crate::types::near_token::NearToken,
}

impl NewAccountContext {
    pub fn from_previous_context(
        previous_context: crate::GlobalContext,
        scope: &<NewAccount as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context,
            new_account_id: scope.new_account_id.clone().into(),
            initial_balance: scope.initial_balance,
        })
    }
}

impl NewAccount {
    pub fn input_new_account_id(
        _context: &crate::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::account_id::AccountId>> {
        Err(crate::common::non_interactive_input_required("new account ID").into())
    }

    fn input_initial_balance(
        _context: &crate::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::near_token::NearToken>> {
        Err(crate::common::non_interactive_input_required("initial balance").into())
    }
}

#[derive(Clone)]
pub struct AccountPropertiesContext {
    pub global_context: crate::GlobalContext,
    pub account_properties: AccountProperties,
    pub on_before_sending_transaction_callback:
        crate::transaction_signature_options::OnBeforeSendingTransactionCallback,
}

#[derive(Debug, Clone)]
pub struct AccountProperties {
    pub new_account_id: near_primitives::types::AccountId,
    pub public_key: near_crypto::PublicKey,
    pub initial_balance: crate::types::near_token::NearToken,
}
