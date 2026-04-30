#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::SignedMetaTransactionContext)]
#[interactive_clap(output_context = RelayerAccountIdContext)]
pub struct RelayerAccountId {
    #[interactive_clap(skip_default_input_arg)]
    /// What is the relayer account ID?
    relayer_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: crate::network_for_transaction::NetworkForTransactionArgs,
}

#[derive(Clone)]
pub struct RelayerAccountIdContext(crate::commands::ActionContext);

impl RelayerAccountIdContext {
    pub fn from_previous_context(
        previous_context: super::SignedMetaTransactionContext,
        scope: &<RelayerAccountId as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let get_prepopulated_transaction_after_getting_network_callback: crate::commands::GetPrepopulatedTransactionAfterGettingNetworkCallback =
            std::sync::Arc::new({
                let signer_id: near_primitives::types::AccountId =
                    scope.relayer_account_id.clone().into();
                let signed_delegate_action = previous_context.signed_delegate_action.clone();

                move |_network_config| {
                    let actions = vec![signed_delegate_action.clone().into()];

                    Ok(crate::commands::PrepopulatedTransaction {
                        signer_id: signer_id.clone(),
                        receiver_id: signed_delegate_action.delegate_action.sender_id.clone(),
                        actions,
                    })
                }
            });

        let on_before_signing_callback: crate::commands::OnBeforeSigningCallback =
            std::sync::Arc::new({
                move |prepopulated_unsigned_transaction, _network_config| {
                    prepopulated_unsigned_transaction.actions =
                        vec![near_primitives::transaction::Action::Delegate(Box::new(
                            previous_context.signed_delegate_action.clone(),
                        ))];
                    Ok(())
                }
            });

        Ok(Self(crate::commands::ActionContext {
            global_context: previous_context.global_context,
            interacting_with_account_ids: vec![scope.relayer_account_id.clone().into()],
            get_prepopulated_transaction_after_getting_network_callback,
            on_before_signing_callback,
            on_before_sending_transaction_callback: std::sync::Arc::new(
                |_signed_transaction, _network_config| Ok(String::new()),
            ),
            on_after_sending_transaction_callback: std::sync::Arc::new(
                |_outcome, _network_config| Ok(()),
            ),
            sign_as_delegate_action: false,
            on_sending_delegate_action_callback: None,
        }))
    }
}

impl From<RelayerAccountIdContext> for crate::commands::ActionContext {
    fn from(item: RelayerAccountIdContext) -> Self {
        item.0
    }
}

impl RelayerAccountId {
    fn input_relayer_account_id(
        context: &super::SignedMetaTransactionContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::account_id::AccountId>> {
        let known_accounts = crate::common::get_used_account_list(&context.global_context.config.credentials_home_dir)
            .into_iter()
            .map(|account| account.account_id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(color_eyre::eyre::eyre!(
            "Missing required argument <relayer-account-id>. Provide it explicitly to run non-interactively. Known local accounts: {known_accounts}"
        ))
    }
}
