#![allow(clippy::enum_variant_names, clippy::large_enum_variant)]
use std::{io, str::FromStr};

use color_eyre::eyre::Context;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use near_primitives::account::id::AccountType;

mod using_private_key;
mod using_seed_phrase;
mod using_web_wallet;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct ImportAccountCommand {
    #[interactive_clap(subcommand)]
    import_account_actions: ImportAccountActions,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// How would you like to import the account?
pub enum ImportAccountActions {
    #[strum_discriminants(strum(
        message = "using-web-wallet          - Import existing account using NEAR Wallet (a.k.a. \"sign in\")"
    ))]
    /// Import existing account using NEAR Wallet (a.k.a. "sign in")
    UsingWebWallet(self::using_web_wallet::LoginFromWebWallet),
    #[strum_discriminants(strum(
        message = "using-seed-phrase         - Import existing account using a seed phrase"
    ))]
    /// Import existing account using a seed phrase
    UsingSeedPhrase(self::using_seed_phrase::LoginFromSeedPhrase),
    #[strum_discriminants(strum(
        message = "using-private-key         - Import existing account using a private key"
    ))]
    /// Import existing account using a private key
    UsingPrivateKey(self::using_private_key::LoginFromPrivateKey),
}

pub fn login(
    network_config: crate::config::NetworkConfig,
    credentials_home_dir: std::path::PathBuf,
    key_pair_properties_buf: &str,
    public_key_str: &str,
    error_message: &str,
) -> crate::CliResult {
    let public_key: near_crypto::PublicKey = near_crypto::PublicKey::from_str(public_key_str)?;

    let account_id = loop {
        let account_id_from_cli = input_account_id()?;

        // If the implicit account does not exist on the network, it will still be imported.
        if let AccountType::NearImplicitAccount = account_id_from_cli.get_account_type() {
            let pk_implicit_account =
                near_crypto::PublicKey::from_near_implicit_account(&account_id_from_cli)?;
            if public_key_str == pk_implicit_account.to_string() {
                break account_id_from_cli;
            }
        };

        let access_key_view = crate::common::verify_account_access_key(
            account_id_from_cli.clone(),
            public_key.clone(),
            network_config.clone(),
        );
        if let Err(err @ crate::common::AccountStateError::Cancel) = access_key_view {
            return color_eyre::eyre::Result::Err(color_eyre::eyre::eyre!(err));
        }
        if access_key_view.is_err() {
            tracing::warn!(
                parent: &tracing::Span::none(),
                "{}",
                crate::common::indent_payload(error_message)
            );
            if !confirm_retry_account_id()? {
                break account_id_from_cli;
            }
        } else {
            break account_id_from_cli;
        }
    };
    crate::common::update_used_account_list_as_signer(&credentials_home_dir, &account_id);
    save_access_key(
        account_id,
        key_pair_properties_buf,
        public_key_str,
        network_config,
        credentials_home_dir,
    )?;
    Ok(())
}

fn input_account_id() -> color_eyre::eyre::Result<near_primitives::types::AccountId> {
    eprintln!("Enter account ID:");
    let mut account_id = String::new();
    io::stdin().read_line(&mut account_id)?;
    Ok(account_id.trim().parse()?)
}

fn confirm_retry_account_id() -> color_eyre::eyre::Result<bool> {
    eprintln!("Re-enter account_id? [y/N]");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    let answer = answer.trim().to_ascii_lowercase();
    Ok(matches!(answer.as_str(), "y" | "yes"))
}

fn use_legacy_keychain() -> bool {
    matches!(
        std::env::var("NEAR_CLI_USE_LEGACY_KEYCHAIN")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

fn save_access_key(
    account_id: near_primitives::types::AccountId,
    key_pair_properties_buf: &str,
    public_key_str: &str,
    network_config: crate::config::NetworkConfig,
    credentials_home_dir: std::path::PathBuf,
) -> crate::CliResult {
    if !use_legacy_keychain() {
        let storage_message =
            crate::common::save_access_key_to_keychain_or_save_to_legacy_keychain(
                network_config,
                credentials_home_dir,
                key_pair_properties_buf,
                public_key_str,
                account_id.as_ref(),
            )?;
        eprintln!("{storage_message}");
        return Ok(());
    }

    let storage_message = crate::common::save_access_key_to_legacy_keychain(
        network_config,
        credentials_home_dir,
        key_pair_properties_buf,
        public_key_str,
        account_id.as_ref(),
    )
    .wrap_err_with(|| format!("Failed to save a file with access key: {public_key_str}"))?;
    eprintln!("{storage_message}");
    Ok(())
}
