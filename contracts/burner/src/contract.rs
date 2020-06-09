use cosmwasm_std::{
    generic_err, log, Api, BankMsg, Binary, Env, Extern, HandleResponse, InitResponse,
    MigrateResponse, Order, Querier, StdResult, Storage,
};

use crate::msg::{EmptyMsg, MigrateMsg};

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: EmptyMsg,
) -> StdResult<InitResponse> {
    Err(generic_err("You can only use this contract for migrations"))
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: EmptyMsg,
) -> StdResult<HandleResponse> {
    Err(generic_err("You can only use this contract for migrations"))
}

pub fn migrate<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: MigrateMsg,
) -> StdResult<MigrateResponse> {
    // delete all state
    let keys: Vec<_> = deps
        .storage
        .range(None, None, Order::Ascending)?
        .map(|(k, _)| k)
        .collect();
    for k in keys {
        deps.storage.remove(&k);
    }

    // get balance and send all to recipient
    let from_addr = deps.api.human_address(&env.contract.address)?;
    let balance = deps.querier.query_all_balances(&from_addr)?;
    let send = BankMsg::Send {
        from_address: from_addr,
        to_address: msg.payout.clone(),
        amount: balance,
    };

    Ok(MigrateResponse {
        messages: vec![send.into()],
        log: vec![log("action", "burn"), log("payout", msg.payout)],
        data: Some(Binary::from(b"burnt".to_vec())),
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: EmptyMsg,
) -> StdResult<Binary> {
    Err(generic_err("You can only use this contract for migrations"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, StdError};

    #[test]
    fn init_fails() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = EmptyMsg {};
        let env = mock_env(&deps.api, "creator", &coins(1000, "earth"));
        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg);
        match res.unwrap_err() {
            StdError::GenericErr { msg, .. } => {
                assert_eq!(msg, "You can only use this contract for migrations")
            }
            _ => panic!("expected migrate error message"),
        }
    }

    // #[test]
    // fn migrate_cleans_up_data() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));
    //
    //     let msg = InitMsg { count: 17 };
    //     let env = mock_env(&deps.api, "creator", &coins(2, "token"));
    //     let _res = init(&mut deps, env, msg).unwrap();
    //
    //     // beneficiary can release it
    //     let unauth_env = mock_env(&deps.api, "anyone", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let res = handle(&mut deps, unauth_env, msg);
    //     match res {
    //         Err(StdError::Unauthorized { .. }) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }
    //
    //     // only the original creator can reset the counter
    //     let auth_env = mock_env(&deps.api, "creator", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let _res = handle(&mut deps, auth_env, msg).unwrap();
    //
    //     // should now be 5
    //     let res = query(&deps, QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
