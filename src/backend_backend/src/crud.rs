use candid::Principal;
use ic_cdk_macros::{query, update};
use crate::{UserData, CANISTER_DATA};


#[update]
fn sync_updates(wallet_address: String, data: String) -> Result<(), String> {
    // Deserialize the incoming data into UserData
    let user_data: UserData = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    // Insert or update the UserData in the map
    CANISTER_DATA.with(|cans_data| {
        let mut map = cans_data.borrow_mut();
        map.insert(wallet_address, user_data);
    });

    Ok(())
}

#[update]
fn register_principal(wallet_address: String) -> Result<(), String> {
    // Access the CANISTER_DATA using thread-local storage or appropriate concurrency control
    CANISTER_DATA.with(|cans_data| {
        // Borrow the mutable map safely
        let mut map = cans_data.borrow_mut();

        // Check if the wallet_address is already registered
        if map.contains_key(&wallet_address) {
            Ok(())
        } else {
            // Wallet address is not registered; proceed to register
            let user_data = UserData {
                wallet_address: wallet_address.clone(),
                // Initialize other fields with default values
                ..Default::default()
            };

            // Insert the new UserData into the map
            map.insert(wallet_address.clone(), user_data);


            Ok(())
        }
    })
}

#[query]
fn get_data() -> String{
    CANISTER_DATA.with(|data|{
        let mut res = vec![];
        for i in data.borrow().iter(){
            res.push(i);
        }
        return serde_json::to_string(&res).unwrap();
    })
}

#[query]
fn get_registered_principals() -> Vec<String>{
    CANISTER_DATA.with(|data|{
        let mut res = vec![];
        for (i, _) in data.borrow().iter(){
            res.push(i);
        }
        res
    })
}