use cosmwasm_std::StdResult;
use cw_storage_plus::Item;
use cosmwasm_std::Storage;

// Define the state.
pub const STATE: Item<u32> = Item::new("state");

// Função para inicializar o contador com 0
pub fn initialize_counter(storage: &mut dyn Storage) -> StdResult<()> {
    STATE.save(storage, &0)
}

// function to load the current counter value
pub fn load_counter(storage: &dyn Storage) -> StdResult<u32> {
    STATE.load(storage)
}

// Function to increment counter and to store the value on the blockchain
pub fn increment_counter(storage: &mut dyn Storage) -> StdResult<u32> {
    let mut count = STATE.load(storage)?;
    count += 1;
    STATE.save(storage, &count)?;
    Ok(count)
}