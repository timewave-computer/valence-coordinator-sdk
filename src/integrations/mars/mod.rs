use valence_domain_clients::{clients::neutron::NeutronClient, cosmos::wasm_client::WasmClient};
use valence_lending_utils::mars::{Account, Positions};

// TODO: in the future, see if importing `valence_lending_utils` can be avoided

/// queries the mars credit manager contract for active lending positions
/// for a given `account_id`
pub async fn query_mars_credit_account_positions(
    client: &NeutronClient,
    credit_manager: &str,
    account_id: &str,
) -> anyhow::Result<Positions> {
    // query mars positions owned by the credit account id
    let mars_positions_response: Positions = client
        .query_contract_state(
            credit_manager,
            valence_lending_utils::mars::QueryMsg::Positions {
                account_id: account_id.to_string(),
            },
        )
        .await?;

    Ok(mars_positions_response)
}

/// queries the mars credit manager contract for credit accounts active
/// for a given address
pub async fn query_mars_credit_accounts(
    client: &NeutronClient,
    credit_manager: &str,
    acc_owner: &str,
) -> anyhow::Result<Vec<Account>> {
    // query the mars credit account created and owned by the mars input account
    let mars_credit_accounts: Vec<Account> = client
        .query_contract_state(
            credit_manager,
            valence_lending_utils::mars::QueryMsg::Accounts {
                owner: acc_owner.to_string(),
                start_after: None,
                limit: None,
            },
        )
        .await?;

    Ok(mars_credit_accounts)
}

/// utility query that:
/// 1. queries the specified credit account active lending positions
/// 2. filters the active positions for the specified denom
///   - if denom is found, returns the lending amount
///   - if denom is not actively lent, returns an error
pub async fn query_mars_lending_denom_amount(
    client: &NeutronClient,
    credit_manager: &str,
    acc_id: &str,
    denom: &str,
) -> anyhow::Result<u128> {
    let active_positions =
        query_mars_credit_account_positions(client, credit_manager, acc_id).await?;

    // iterate over the active lending positions and search for the specified denom.
    // if found, return the respective amount.
    // otherwise, return an error.
    active_positions
        .lends
        .into_iter()
        .find(|lend| lend.denom == denom)
        .map(|lend| lend.amount.u128())
        .ok_or_else(|| anyhow::anyhow!("no {denom} active lending positions found"))
}
