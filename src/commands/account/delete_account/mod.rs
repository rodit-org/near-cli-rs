#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = crate::GlobalContext)]
#[interactive_clap(output_context = DeleteAccountContext)]
pub struct DeleteAccount {
    #[interactive_clap(skip_default_input_arg)]
    /// What Account ID to be deleted?
    account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Enter the beneficiary ID to delete this account ID
    beneficiary: BeneficiaryAccount,
}

#[derive(Debug, Clone)]
pub struct DeleteAccountContext {
    global_context: crate::GlobalContext,
    account_id: near_primitives::types::AccountId,
}

impl DeleteAccountContext {
    pub fn from_previous_context(
        previous_context: crate::GlobalContext,
        scope: &<DeleteAccount as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context,
            account_id: scope.account_id.clone().into(),
        })
    }
}

impl DeleteAccount {
    pub fn input_account_id(
        context: &crate::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::account_id::AccountId>> {
        crate::common::input_signer_account_id_from_used_account_list(
            &context.config.credentials_home_dir,
            "What Account ID to be deleted?",
        )
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = DeleteAccountContext)]
#[interactive_clap(output_context = BeneficiaryAccountContext)]
pub struct BeneficiaryAccount {
    #[interactive_clap(skip_default_input_arg)]
    /// Specify a beneficiary:
    beneficiary_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: crate::network_for_transaction::NetworkForTransactionArgs,
}

#[derive(Debug, Clone)]
pub struct BeneficiaryAccountContext {
    global_context: crate::GlobalContext,
    account_id: near_primitives::types::AccountId,
    beneficiary_account_id: near_primitives::types::AccountId,
}

impl BeneficiaryAccountContext {
    pub fn from_previous_context(
        previous_context: DeleteAccountContext,
        scope: &<BeneficiaryAccount as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context.global_context,
            account_id: previous_context.account_id,
            beneficiary_account_id: scope.beneficiary_account_id.clone().into(),
        })
    }
}

impl From<BeneficiaryAccountContext> for crate::commands::ActionContext {
    fn from(item: BeneficiaryAccountContext) -> Self {
        let get_prepopulated_transaction_after_getting_network_callback: crate::commands::GetPrepopulatedTransactionAfterGettingNetworkCallback =
            std::sync::Arc::new({
                let account_id = item.account_id.clone();

                move |_network_config| {
                    Ok(crate::commands::PrepopulatedTransaction {
                        signer_id: account_id.clone(),
                        receiver_id: account_id.clone(),
                        actions: vec![near_primitives::transaction::Action::DeleteAccount(
                            near_primitives::transaction::DeleteAccountAction {
                                beneficiary_id: item.beneficiary_account_id.clone(),
                            },
                        )],
                    })
                }
            });

        Self {
            global_context: item.global_context,
            interacting_with_account_ids: vec![item.account_id],
            get_prepopulated_transaction_after_getting_network_callback,
            on_before_signing_callback: std::sync::Arc::new(
                |_prepopulated_unsigned_transaction, _network_config| Ok(()),
            ),
            on_before_sending_transaction_callback: std::sync::Arc::new(
                |_signed_transaction, _network_config| Ok(String::new()),
            ),
            on_after_sending_transaction_callback: std::sync::Arc::new(
                |_outcome_view, _network_config| Ok(()),
            ),
            sign_as_delegate_action: false,
            on_sending_delegate_action_callback: None,
        }
    }
}

impl BeneficiaryAccount {
    pub fn input_beneficiary_account_id(
        _context: &DeleteAccountContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::account_id::AccountId>> {
        Err(crate::common::non_interactive_input_required("beneficiary account ID").into())
    }
}
