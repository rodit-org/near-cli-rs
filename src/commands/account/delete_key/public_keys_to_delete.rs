#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::DeleteKeysCommandContext)]
#[interactive_clap(output_context = PublicKeyListContext)]
pub struct PublicKeyList {
    #[interactive_clap(skip_default_input_arg)]
    /// Enter the public keys you wish to delete (separated by comma):
    public_keys: crate::types::public_key_list::PublicKeyList,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: crate::network_for_transaction::NetworkForTransactionArgs,
}

#[derive(Debug, Clone)]
pub struct PublicKeyListContext {
    global_context: crate::GlobalContext,
    owner_account_id: near_primitives::types::AccountId,
    public_keys: Vec<near_crypto::PublicKey>,
}

impl PublicKeyListContext {
    pub fn from_previous_context(
        previous_context: super::DeleteKeysCommandContext,
        scope: &<PublicKeyList as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context.global_context,
            owner_account_id: previous_context.owner_account_id,
            public_keys: scope.public_keys.clone().into(),
        })
    }
}

impl From<PublicKeyListContext> for crate::commands::ActionContext {
    fn from(item: PublicKeyListContext) -> Self {
        let get_prepopulated_transaction_after_getting_network_callback: crate::commands::GetPrepopulatedTransactionAfterGettingNetworkCallback =
            std::sync::Arc::new({
                let owner_account_id = item.owner_account_id.clone();

                move |_network_config| {
                    Ok(crate::commands::PrepopulatedTransaction {
                        signer_id: owner_account_id.clone(),
                        receiver_id: owner_account_id.clone(),
                        actions: item
                            .public_keys
                            .clone()
                            .into_iter()
                            .map(|public_key| {
                                near_primitives::transaction::Action::DeleteKey(Box::new(
                                    near_primitives::transaction::DeleteKeyAction { public_key },
                                ))
                            })
                            .collect(),
                    })
                }
            });

        Self {
            global_context: item.global_context,
            interacting_with_account_ids: vec![item.owner_account_id],
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

impl PublicKeyList {
    pub fn input_public_keys(
        context: &super::DeleteKeysCommandContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::public_key_list::PublicKeyList>> {
        let known_networks = context.global_context.config.network_names().join(", ");
        Err(color_eyre::eyre::eyre!(
            "Missing required argument <public-keys>. Automatic key selection is disabled for non-interactive mode. Provide a comma-separated list of public keys. Networks available: {known_networks}"
        ))
    }
}


