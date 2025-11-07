use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    components::submit_transaction,
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet, Wallet},
};

const COMPUTE_UNIT_LIMIT: u32 = 200_000;

#[component]
pub fn Home() -> Element {
    let mut destination = use_signal(|| String::new());
    let mut amount = use_signal(|| String::new());
    let mut address_error = use_signal(|| None::<String>);
    let mut amount_error = use_signal(|| None::<String>);
    let wallet = use_wallet();

    let mut balance = use_signal(|| Err(GatewayError::Unknown));
    let gateway = use_gateway();

    // Fetch balance when wallet connects
    use_effect(move || {
        if let Wallet::Connected(pubkey) = wallet.cloned() {
            spawn(async move {
                match gateway.read().rpc.get_balance(&pubkey).await {
                    Ok(bal) => {
                        let sol_balance = bal as f64 / 1_000_000_000.0;
                        balance.set(Ok(sol_balance));
                    }
                    Err(e) => balance.set(Err(e)),
                }
            });
        } else {
            balance.set(Err(GatewayError::WalletDisconnected));
        }
    });

    let validate_address = move |addr: &str| -> Option<String> {
        if addr.is_empty() {
            None
        } else if Pubkey::try_from(addr).is_err() {
            Some("Invalid Solana address".to_string())
        } else {
            None
        }
    };

    let validate_amount = move |amt: &str, bal: f64| -> Option<String> {
        if amt.is_empty() {
            None
        } else if let Ok(amt_f64) = amt.parse::<f64>() {
            if amt_f64 <= 0.0 {
                Some("Amount must be greater than 0".to_string())
            } else if amt_f64 > bal {
                Some("Insufficient balance".to_string())
            } else {
                None
            }
        } else {
            Some("Invalid amount".to_string())
        }
    };

    let build_transaction = move || -> Option<VersionedTransaction> {
        let Wallet::Connected(authority) = wallet.cloned() else {
            return None;
        };

        let destination_str = destination.read().clone();
        let amount_str = amount.read().clone();

        if destination_str.is_empty() || amount_str.is_empty() {
            return None;
        }

        let dest_pubkey = Pubkey::try_from(destination_str.as_str()).ok()?;
        let amount_f64 = amount_str.parse::<f64>().ok()?;
        let amount_lamports = (amount_f64 * 1_000_000_000.0) as u64;

        let mut ixs = vec![];
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(COMPUTE_UNIT_LIMIT));
        ixs.push(transfer(&authority, &dest_pubkey, amount_lamports));

        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Some(tx)
    };

    let can_transfer = use_memo(move || {
        if let Wallet::Disconnected = wallet.cloned() {
            return false;
        }
        address_error.read().is_none()
            && amount_error.read().is_none()
            && !destination.read().is_empty()
            && !amount.read().is_empty()
            && build_transaction().is_some()
    });

    rsx! {
        div {
            class: "max-w-2xl mx-auto",
            h1 {
                class: "text-4xl font-bold mb-8 text-elements-highEmphasis",
                "Transfer SOL"
            }

            if let Wallet::Connected(_) = wallet.cloned() {
                if let Ok(bal) = balance.cloned() {
                    div {
                        class: "mb-6 p-4 bg-surface-elevated rounded-lg border border-gray-800",
                        p {
                            class: "text-elements-midEmphasis",
                            "Balance: "
                            span {
                                class: "text-elements-highEmphasis font-semibold",
                                "{bal:.9} SOL"
                            }
                        }
                    }
                }
            }

            div {
                class: "bg-surface-elevated rounded-xl border border-gray-800 p-6",
                div {
                    class: "mb-6",
                    label {
                        class: "block text-sm font-semibold mb-2 text-elements-highEmphasis",
                        "Recipient Address"
                    }
                    input {
                        class: "w-full h-12 px-4 rounded-lg bg-base-canvas border border-gray-800 text-elements-highEmphasis focus:outline-none focus:border-elements-purple",
                        placeholder: "Enter Solana wallet address",
                        value: destination.clone(),
                        oninput: move |e: FormEvent| {
                            let new_value = e.value();
                            address_error.set(validate_address(&new_value));
                            destination.set(new_value);
                        },
                    }
                    if let Some(err) = address_error.cloned() {
                        p {
                            class: "mt-2 text-sm text-elements-red",
                            "{err}"
                        }
                    }
                }

                div {
                    class: "mb-6",
                    label {
                        class: "block text-sm font-semibold mb-2 text-elements-highEmphasis",
                        "Amount (SOL)"
                    }
                    input {
                        class: "w-full h-12 px-4 rounded-lg bg-base-canvas border border-gray-800 text-elements-highEmphasis focus:outline-none focus:border-elements-purple",
                        r#type: "number",
                        step: "0.000000001",
                        placeholder: "0.0",
                        value: amount.clone(),
                        oninput: move |e: FormEvent| {
                            let new_value = e.value();
                            let bal = balance.read().clone().unwrap_or(0.0);
                            amount_error.set(validate_amount(&new_value, bal));
                            amount.set(new_value);
                        },
                    }
                    if let Some(err) = amount_error.cloned() {
                        p {
                            class: "mt-2 text-sm text-elements-red",
                            "{err}"
                        }
                    }
                }

                button {
                    class: "w-full h-12 rounded-full controls-primary disabled:opacity-50 disabled:cursor-not-allowed",
                    disabled: !can_transfer.cloned(),
                    onclick: move |_| {
                        if let Some(tx) = build_transaction() {
                            submit_transaction(tx);
                            // Clear form after submission
                            destination.set(String::new());
                            amount.set(String::new());
                            address_error.set(None);
                            amount_error.set(None);
                        }
                    },
                    "Transfer"
                }
            }

            if let Wallet::Disconnected = wallet.cloned() {
                div {
                    class: "mt-6 p-4 bg-surface-elevated rounded-lg border border-gray-800 text-center",
                    p {
                        class: "text-elements-midEmphasis",
                        "Please connect your wallet to transfer SOL"
                    }
                }
            }
        }
    }
}
