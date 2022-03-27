#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{GetStudentAccResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{StudentAccreditation, STUDENTACC};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let studentacc = StudentAccreditation {
        admin: msg.sender,
        studentname: msg.studentname,
        studentid: msg.studentid,
        universitites: msg.universitites,
        degrees: msg.degrees,
    };
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STUDENTACC.save(deps.storage, &studentacc)?; 

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddUniversity { studentid, university } => try_add_uni(deps, info, studentid, university),
        ExecuteMsg::RemoveUniversity { studentid, university } => try_remove_uni(deps, info, studentid, university),
        ExecuteMsg::AddDegree { studentid, degree } => try_add_degree(deps, info, studentid, degree),
        ExecuteMsg::RemoveDegree { studentid, degree } => try_remove_degree(deps, info, studentid, degree),
    }
}

pub fn try_add_uni(deps: DepsMut, info: MessageInfo, studentid: i32, university: String) -> Result<Response, ContractError> {
    STUDENTACC.update(deps.storage, |mut studentacc| -> Result<_, ContractError> {
        if info.sender != studentacc.admin {
            return Err(ContractError::Unauthorized {});
        }
        studentacc.universitites.insert(university, studentacc.universitites.len());
        Ok(studentacc)
    })?;
    Ok(Response::new().add_attribute("method", "add_uni"))
}

pub fn try_remove_uni(deps: DepsMut, info: MessageInfo, studentid: i32, university: String) -> Result<Response, ContractError> {
    STUDENTACC.update(deps.storage, |mut studentacc| -> Result<_, ContractError> {
        if info.sender != studentacc.admin {
            return Err(ContractError::Unauthorized {});
        }
        let index = studentacc.universitites
        .iter()
        .position(|&x| x == university)
        .unwrap();

        studentacc.universitites.remove(index);;
        Ok(studentacc)
    })?;
    Ok(Response::new().add_attribute("method", "remove_university"))
}

pub fn try_add_degree(deps: DepsMut, info: MessageInfo, studentid: i32, degree: String) -> Result<Response, ContractError> {
    STUDENTACC.update(deps.storage, |mut studentacc| -> Result<_, ContractError> {
        if info.sender != studentacc.admin {
            return Err(ContractError::Unauthorized {});
        }
        studentacc.degrees.insert(degree, studentacc.degrees.len());
        Ok(studentacc)
    })?;
    Ok(Response::new().add_attribute("method", "add_degree"))
}

pub fn try_remove_degree(deps: DepsMut, info: MessageInfo, studentid: i32, degree: String) -> Result<Response, ContractError> {
    STUDENTACC.update(deps.storage, |mut studentacc| -> Result<_, ContractError> {
        if info.sender != studentacc.admin {
            return Err(ContractError::Unauthorized {});
        }
        let index = studentacc.degrees
        .iter()
        .position(|&x| x == degree)
        .unwrap();

        studentacc.degrees.remove(index);;
        Ok(studentacc)
    })?;
    Ok(Response::new().add_attribute("method", "remove_degree"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStudentAcc { studentid } => to_binary(&get_student_acc(deps, studentid)?),
    }
}

fn get_student_acc(deps: Deps, studentid: i32) -> StdResult<GetStudentAccResponse> {
    let studentacc = STUDENTACC.load(deps.storage)?;
    Ok(GetStudentAccResponse { studentname: studentacc.studentname, studentid: studentacc.studentid, universitites: studentacc.universitites, degrees: studentacc.degrees })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
