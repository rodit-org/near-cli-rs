
#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = crate::GlobalContext)]
#[interactive_clap(output_context = CallFunctionContext)]
pub struct CallFunction {
    #[interactive_clap(skip_default_input_arg)]
    /// What is the contract account ID?
    contract_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subargs)]
    /// Select function
    function: Function,
}

#[derive(Debug, Clone)]
pub struct CallFunctionContext {
    global_context: crate::GlobalContext,
    contract_account_id: near_primitives::types::AccountId,
}

impl CallFunctionContext {
    pub fn from_previous_context(
        previous_context: crate::GlobalContext,
        scope: &<CallFunction as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context,
            contract_account_id: scope.contract_account_id.clone().into(),
        })
    }
}

impl CallFunction {
    pub fn input_contract_account_id(
        context: &crate::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::account_id::AccountId>> {
        let known_accounts = crate::common::get_used_account_list(&context.config.credentials_home_dir)
            .into_iter()
            .map(|account| account.account_id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(color_eyre::eyre::eyre!(
            "Missing required argument <contract-account-id>. Provide it explicitly to run non-interactively. Known local accounts: {known_accounts}"
        ))
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CallFunctionContext)]
#[interactive_clap(output_context = FunctionContext)]
pub struct Function {
    #[interactive_clap(skip_default_input_arg)]
    /// What is the name of the function?
    function_name: String,
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_default_input_arg)]
    /// How do you want to pass the function call arguments?
    function_args_type: super::call_function_args_type::FunctionArgsType,
    /// Enter the arguments to this function:
    function_args: String,
    #[interactive_clap(named_arg)]
    /// Enter gas for function call
    prepaid_gas: PrepaidGas,
}

#[derive(Clone)]
pub struct FunctionContext {
    global_context: crate::GlobalContext,
    contract_account_id: near_primitives::types::AccountId,
    function_name: String,
    function_args: Vec<u8>,
}

impl FunctionContext {
    pub fn from_previous_context(
        previous_context: CallFunctionContext,
        scope: &<Function as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let function_args = super::call_function_args_type::function_args(
            scope.function_args.clone(),
            scope.function_args_type.clone(),
        )?;
        Ok(Self {
            global_context: previous_context.global_context,
            contract_account_id: previous_context.contract_account_id,
            function_name: scope.function_name.clone(),
            function_args,
        })
    }
}

impl Function {
    fn input_function_args_type(
        _context: &CallFunctionContext,
    ) -> color_eyre::eyre::Result<Option<super::call_function_args_type::FunctionArgsType>> {
        super::call_function_args_type::input_function_args_type()
    }

    fn input_function_name(
        context: &CallFunctionContext,
    ) -> color_eyre::eyre::Result<Option<String>> {
        super::input_call_function_name(&context.global_context, &context.contract_account_id)
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = FunctionContext)]
#[interactive_clap(output_context = PrepaidGasContext)]
pub struct PrepaidGas {
    #[interactive_clap(skip_default_input_arg)]
    /// Enter gas for function call:
    gas: crate::common::NearGas,
    #[interactive_clap(named_arg)]
    /// Enter deposit for a function call
    attached_deposit: Deposit,
}

#[derive(Debug, Clone)]
pub struct PrepaidGasContext {
    global_context: crate::GlobalContext,
    contract_account_id: near_primitives::types::AccountId,
    function_name: String,
    function_args: Vec<u8>,
    gas: crate::common::NearGas,
}

impl PrepaidGasContext {
    pub fn from_previous_context(
        previous_context: FunctionContext,
        scope: &<PrepaidGas as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context.global_context,
            contract_account_id: previous_context.contract_account_id,
            function_name: previous_context.function_name,
            function_args: previous_context.function_args,
            gas: scope.gas,
        })
    }
}

impl PrepaidGas {
    fn input_gas(
        _context: &FunctionContext,
    ) -> color_eyre::eyre::Result<Option<crate::common::NearGas>> {
        Ok(Some("100 Tgas".parse()?))
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = PrepaidGasContext)]
#[interactive_clap(output_context = DepositContext)]
pub struct Deposit {
    #[interactive_clap(skip_default_input_arg)]
    /// Enter deposit for a function call:
    deposit: crate::types::near_token::NearToken,
    #[interactive_clap(named_arg)]
    /// What is the signer account ID?
    sign_as: SignerAccountId,
}

#[derive(Debug, Clone)]
pub struct DepositContext {
    global_context: crate::GlobalContext,
    contract_account_id: near_primitives::types::AccountId,
    function_name: String,
    function_args: Vec<u8>,
    gas: crate::common::NearGas,
    deposit: crate::types::near_token::NearToken,
}

impl DepositContext {
    pub fn from_previous_context(
        previous_context: PrepaidGasContext,
        scope: &<Deposit as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context.global_context,
            contract_account_id: previous_context.contract_account_id,
            function_name: previous_context.function_name,
            function_args: previous_context.function_args,
            gas: previous_context.gas,
            deposit: scope.deposit,
        })
    }
}

impl Deposit {
    fn input_deposit(
        _context: &PrepaidGasContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::near_token::NearToken>> {
        Ok(Some("0 NEAR".parse()?))
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = DepositContext)]
#[interactive_clap(output_context = SignerAccountIdContext)]
pub struct SignerAccountId {
    #[interactive_clap(skip_default_input_arg)]
    /// What is the signer account ID?
    signer_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: crate::network_for_transaction::NetworkForTransactionArgs,
}

#[derive(Debug, Clone)]
pub struct SignerAccountIdContext {
    global_context: crate::GlobalContext,
    contract_account_id: near_primitives::types::AccountId,
    function_name: String,
    function_args: Vec<u8>,
    gas: crate::common::NearGas,
    deposit: crate::types::near_token::NearToken,
    signer_account_id: near_primitives::types::AccountId,
}

impl SignerAccountIdContext {
    pub fn from_previous_context(
        previous_context: DepositContext,
        scope: &<SignerAccountId as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context.global_context,
            contract_account_id: previous_context.contract_account_id,
            function_name: previous_context.function_name,
            function_args: previous_context.function_args,
            gas: previous_context.gas,
            deposit: previous_context.deposit,
            signer_account_id: scope.signer_account_id.clone().into(),
        })
    }
}

impl From<SignerAccountIdContext> for crate::commands::ActionContext {
    fn from(item: SignerAccountIdContext) -> Self {
        let get_prepopulated_transaction_after_getting_network_callback: crate::commands::GetPrepopulatedTransactionAfterGettingNetworkCallback =
            std::sync::Arc::new({
                let signer_account_id = item.signer_account_id.clone();
                let receiver_account_id = item.contract_account_id.clone();

                move |_network_config| {
                    Ok(crate::commands::PrepopulatedTransaction {
                        signer_id: signer_account_id.clone(),
                        receiver_id: receiver_account_id.clone(),
                        actions: vec![near_primitives::transaction::Action::FunctionCall(
                            Box::new(near_primitives::transaction::FunctionCallAction {
                                method_name: item.function_name.clone(),
                                args: item.function_args.clone(),
                                gas: near_primitives::gas::Gas::from_gas(item.gas.as_gas()),
                                deposit: item.deposit.into(),
                            }),
                        )],
                    })
                }
            });

        Self {
            global_context: item.global_context,
            interacting_with_account_ids: vec![item.signer_account_id, item.contract_account_id],
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

impl SignerAccountId {
    pub fn input_signer_account_id(
        context: &DepositContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::account_id::AccountId>> {
        let known_accounts = crate::common::get_used_account_list(&context.global_context.config.credentials_home_dir)
            .into_iter()
            .map(|account| account.account_id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(color_eyre::eyre::eyre!(
            "Missing required argument <signer-account-id>. Provide it explicitly to run non-interactively. Known local accounts: {known_accounts}"
        ))
    }
}
