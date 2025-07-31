use cosmwasm_std::Binary;
use valence_authorization_utils::msg::ProcessorMessage;
use valence_domain_clients::{
    clients::neutron::NeutronClient,
    cosmos::{base_client::BaseClient, wasm_client::WasmClient},
};

/// enqueues a set of messages to a given authorizations module,
/// under the specified authorization label.
/// messages are expected to be passed in as json bytes.
pub async fn enqueue(
    client: &NeutronClient,
    authorizations: &str,
    label: &str,
    messages: Vec<Binary>,
) -> anyhow::Result<()> {
    // wrap the json-encoded messages into processor message format
    let encoded_messages = messages
        .into_iter()
        .map(|msg| ProcessorMessage::CosmwasmExecuteMsg { msg })
        .collect();

    let tx_resp = client
        .execute_wasm(
            authorizations,
            valence_authorization_utils::msg::ExecuteMsg::PermissionlessAction(
                valence_authorization_utils::msg::PermissionlessMsg::SendMsgs {
                    label: label.to_string(),
                    messages: encoded_messages,
                    ttl: None,
                },
            ),
            vec![],
            None,
        )
        .await?;

    // poll for inclusion to avoid account sequence mismatch errors
    client.poll_for_tx(&tx_resp.hash).await?;

    Ok(())
}

/// ticks the processor
pub async fn tick(client: &NeutronClient, processor: &str) -> anyhow::Result<()> {
    let tx_resp = client
        .execute_wasm(
            processor,
            valence_processor_utils::msg::ExecuteMsg::PermissionlessAction(
                valence_processor_utils::msg::PermissionlessMsg::Tick {},
            ),
            vec![],
            None,
        )
        .await?;

    // poll for inclusion to avoid account sequence mismatch errors
    client.poll_for_tx(&tx_resp.hash).await?;

    Ok(())
}

/// constructs the zk authorization execution message and executes it.
/// authorizations module will perform the zk verification and, if
/// successful, push it to the processor for execution
pub async fn post_zkp_on_chain(
    client: &NeutronClient,
    authorizations: &str,
    authorization_label: &str,
    (proof_program, inputs_program): (Vec<u8>, Vec<u8>),
    (proof_domain, inputs_domain): (Vec<u8>, Vec<u8>),
) -> anyhow::Result<()> {
    // construct the zk authorization registration message
    let execute_zk_authorization_msg =
        valence_authorization_utils::msg::PermissionlessMsg::ExecuteZkAuthorization {
            label: authorization_label.to_string(),
            message: Binary::from(inputs_program),
            proof: Binary::from(proof_program),
            domain_message: Binary::from(inputs_domain),
            domain_proof: Binary::from(proof_domain),
        };

    let tx_resp = client
        .execute_wasm(
            authorizations,
            valence_authorization_utils::msg::ExecuteMsg::PermissionlessAction(
                execute_zk_authorization_msg,
            ),
            vec![],
            None,
        )
        .await?;

    // poll for inclusion to avoid account sequence mismatch errors
    client.poll_for_tx(&tx_resp.hash).await?;

    Ok(())
}
