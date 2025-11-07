use dioxus::prelude::*;
use crate::hooks::{use_wallet, Wallet, sign_and_send_transaction};
use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    message::Message,
};
use std::str::FromStr;

#[component]
pub fn Home() -> Element {
    let wallet = use_wallet();
    let mut recipient = use_signal(|| String::new());
    let mut amount = use_signal(|| String::new());
    let mut status_message = use_signal(|| String::new());
    let mut is_processing = use_signal(|| false);

    let handle_transfer = move |_| {
        let wallet_clone = wallet.cloned();
        let recipient_str = recipient.cloned();
        let amount_str = amount.cloned();

        spawn(async move {
            is_processing.set(true);
            status_message.set("Processing transfer...".to_string());

            // Validate wallet is connected
            let sender_pubkey = match wallet_clone {
                Wallet::Connected(pubkey) => pubkey,
                Wallet::Disconnected => {
                    status_message.set("Error: Please connect your wallet first".to_string());
                    is_processing.set(false);
                    return;
                }
            };

            // Validate recipient address
            let recipient_pubkey = match Pubkey::from_str(&recipient_str) {
                Ok(pubkey) => pubkey,
                Err(_) => {
                    status_message.set("Error: Invalid recipient address".to_string());
                    is_processing.set(false);
                    return;
                }
            };

            // Parse amount
            let lamports = match amount_str.parse::<f64>() {
                Ok(sol) => (sol * 1_000_000_000.0) as u64,
                Err(_) => {
                    status_message.set("Error: Invalid amount".to_string());
                    is_processing.set(false);
                    return;
                }
            };

            if lamports == 0 {
                status_message.set("Error: Amount must be greater than 0".to_string());
                is_processing.set(false);
                return;
            }

            #[cfg(feature = "web")]
            {
                use solana_client_wasm::WasmClient;
                use solana_extra_wasm::transaction_status::TransactionConfirmationStatus;

                // Create the transfer instruction
                let instruction = system_instruction::transfer(
                    &sender_pubkey,
                    &recipient_pubkey,
                    lamports,
                );

                // Get recent blockhash from Solana
                let client = WasmClient::new("https://api.mainnet-beta.solana.com");
                
                let blockhash = match client.get_latest_blockhash().await {
                    Ok(hash) => hash,
                    Err(e) => {
                        status_message.set(format!("Error: Failed to get blockhash: {:?}", e));
                        is_processing.set(false);
                        return;
                    }
                };

                // Create transaction
                let message = Message::new(&[instruction], Some(&sender_pubkey));
                let mut transaction = Transaction::new_unsigned(message);
                transaction.message.recent_blockhash = blockhash;

                // Serialize transaction to base64
                let serialized = bincode::serialize(&transaction).unwrap();
                let tx_base64 = base64::encode(&serialized);

                // Sign transaction via wallet adapter
                let signed_tx_base64 = match sign_and_send_transaction(tx_base64).await {
                    Ok(signed) => signed,
                    Err(e) => {
                        status_message.set(format!("Error signing transaction: {}", e));
                        is_processing.set(false);
                        return;
                    }
                };

                // Deserialize signed transaction
                let signed_tx_bytes = match base64::decode(&signed_tx_base64) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        status_message.set(format!("Error decoding signed transaction: {}", e));
                        is_processing.set(false);
                        return;
                    }
                };

                let signed_transaction: Transaction = match bincode::deserialize(&signed_tx_bytes) {
                    Ok(tx) => tx,
                    Err(e) => {
                        status_message.set(format!("Error deserializing transaction: {}", e));
                        is_processing.set(false);
                        return;
                    }
                };

                // Send transaction
                match client.send_transaction(&signed_transaction).await {
                    Ok(signature) => {
                        status_message.set(format!("Success! Transaction signature: {}", signature));
                        recipient.set(String::new());
                        amount.set(String::new());
                    }
                    Err(e) => {
                        status_message.set(format!("Error sending transaction: {:?}", e));
                    }
                }
            }

            #[cfg(not(feature = "web"))]
            {
                status_message.set("Transfer functionality only available in web build".to_string());
            }

            is_processing.set(false);
        });
    };

    rsx! {
        div {
            class: "max-w-2xl mx-auto",
            
            // Header
            div {
                class: "text-center mb-12",
                h1 {
                    class: "text-5xl font-bold text-white mb-4",
                    "SOL Transfer"
                }
                p {
                    class: "text-gray-300 text-lg",
                    "Send SOL tokens to any Solana address"
                }
            }

            // Transfer form
            div {
                class: "bg-gray-800/50 backdrop-blur-md rounded-2xl p-8 border border-gray-700 shadow-2xl",
                
                // Wallet status indicator
                div {
                    class: "mb-6 p-4 rounded-lg",
                    class: if matches!(wallet.cloned(), Wallet::Connected(_)) {
                        "bg-green-900/30 border border-green-500"
                    } else {
                        "bg-yellow-900/30 border border-yellow-500"
                    },
                    p {
                        class: "text-center font-medium",
                        class: if matches!(wallet.cloned(), Wallet::Connected(_)) {
                            "text-green-400"
                        } else {
                            "text-yellow-400"
                        },
                        if let Wallet::Connected(_) = wallet.cloned() {
                            "✓ Wallet Connected"
                        } else {
                            "⚠ Please connect your wallet to continue"
                        }
                    }
                }

                // Form fields
                div {
                    class: "space-y-6",
                    
                    // Recipient address
                    div {
                        label {
                            class: "block text-gray-300 font-medium mb-2",
                            "Recipient Address"
                        }
                        input {
                            class: "w-full bg-gray-900/50 border border-gray-600 rounded-lg px-4 py-3 text-white placeholder-gray-500 focus:outline-none focus:border-purple-500 focus:ring-2 focus:ring-purple-500/50 transition-all",
                            r#type: "text",
                            placeholder: "Enter Solana address (e.g., 7xKX...)",
                            value: "{recipient}",
                            disabled: is_processing.cloned() || !matches!(wallet.cloned(), Wallet::Connected(_)),
                            oninput: move |evt| recipient.set(evt.value())
                        }
                    }

                    // Amount
                    div {
                        label {
                            class: "block text-gray-300 font-medium mb-2",
                            "Amount (SOL)"
                        }
                        input {
                            class: "w-full bg-gray-900/50 border border-gray-600 rounded-lg px-4 py-3 text-white placeholder-gray-500 focus:outline-none focus:border-purple-500 focus:ring-2 focus:ring-purple-500/50 transition-all",
                            r#type: "number",
                            step: "0.000000001",
                            min: "0",
                            placeholder: "0.0",
                            value: "{amount}",
                            disabled: is_processing.cloned() || !matches!(wallet.cloned(), Wallet::Connected(_)),
                            oninput: move |evt| amount.set(evt.value())
                        }
                    }

                    // Transfer button
                    button {
                        class: "w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-bold py-4 rounded-lg transition-all duration-200 transform hover:scale-105 disabled:hover:scale-100",
                        disabled: is_processing.cloned() || !matches!(wallet.cloned(), Wallet::Connected(_)),
                        onclick: handle_transfer,
                        if is_processing.cloned() {
                            "Processing..."
                        } else {
                            "Transfer SOL"
                        }
                    }
                }

                // Status message
                if !status_message.cloned().is_empty() {
                    div {
                        class: "mt-6 p-4 rounded-lg",
                        class: if status_message.cloned().starts_with("Success") {
                            "bg-green-900/30 border border-green-500 text-green-400"
                        } else if status_message.cloned().starts_with("Error") {
                            "bg-red-900/30 border border-red-500 text-red-400"
                        } else {
                            "bg-blue-900/30 border border-blue-500 text-blue-400"
                        },
                        p {
                            class: "text-sm break-words",
                            "{status_message}"
                        }
                    }
                }
            }

            // Information section
            div {
                class: "mt-8 bg-gray-800/30 rounded-lg p-6 border border-gray-700",
                h3 {
                    class: "text-white font-semibold text-lg mb-3",
                    "ℹ️ How it works"
                }
                ul {
                    class: "text-gray-300 space-y-2 text-sm",
                    li { "1. Connect your Solana wallet using the button in the navigation bar" }
                    li { "2. Enter the recipient's Solana address" }
                    li { "3. Specify the amount of SOL to transfer" }
                    li { "4. Click 'Transfer SOL' and approve the transaction in your wallet" }
                    li { "5. Wait for confirmation on the Solana network" }
                }
            }
        }
    }
}
