use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, Storable, StableBTreeMap};
use serde::Serialize;
use std::borrow::Cow;
use std::cell::RefCell;

mod crud;
// Define the Memory type using VirtualMemory with DefaultMemoryImpl
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Initialize thread-local storage for MEMORY_MANAGER and CANISTER_DATA
thread_local! {
    // Memory Manager to handle multiple memories
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // StableBTreeMap to store user data with String keys and UserData values
    pub static CANISTER_DATA: RefCell<StableBTreeMap<String, UserData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            // Obtain a mutable reference to the memory
            MEMORY_MANAGER.with(|m| m.borrow_mut().get(MemoryId::new(0))),
        )
    );
}

// Define the UserData struct with necessary fields
#[derive(Deserialize, CandidType, Clone, Debug, Serialize)]
struct UserData {
    #[serde(default)]
    name: Option<String>,
    wallet_address: String,
    clicks: usize,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    twitter: Option<String>,
    #[serde(default)]
    instagram: Option<String>,
    exp: usize,
    rating: usize,
    streak_count: usize,
    last_login: usize,
    #[serde(default)]
    aliens: Vec<usize>,
    #[serde(default)]
    power_ups: Vec<PowerUpKind>,
    #[serde(default)]
    badges: Vec<BadgesKind>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            name: None,
            wallet_address: String::new(),
            clicks: 0,
            email: None,
            twitter: None,
            instagram: None,
            exp: 0,
            rating: 0,
            streak_count: 0,
            last_login: 0,
            aliens: vec![],
            power_ups: vec![],
            badges: vec![],
        }
    }
}

// Define enums for PowerUpKind and BadgesKind
#[derive(Clone, Debug, Deserialize, CandidType, Serialize)]
enum PowerUpKind {
    Spawner,
    ClickMultiplier,
    AutoFiller,
    AlienMultiplier,
}

#[derive(Clone, Debug, Deserialize, CandidType, Serialize)]
enum BadgesKind {
    LoginStreak { lvl: usize },
}

// Implement the Storable trait for UserData
impl Storable for UserData {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        ciborium::ser::into_writer(self, &mut bytes).unwrap();
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let namespace: Self = ciborium::de::from_reader(bytes.as_ref()).unwrap();
        namespace
    }
}

// Define an update method to synchronize update
// Additional CRUD operations can be implemented similarly

ic_cdk::export_candid!();