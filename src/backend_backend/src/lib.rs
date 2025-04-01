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
#[derive(Deserialize, Clone, Debug, Serialize, CandidType)]
struct UserProfile {
    user_id: Principal,
    email: Option<String>,
    pfp: Option<String>,
    last_login: u64,
}

#[derive(Deserialize, Clone, Debug, Serialize, CandidType)]
struct GameState {
    active_aliens: [usize; 16],
    inventory_aliens: Vec<usize>,
    power_ups: [Option<PowerUpKind>; 3],
    king_lvl: usize,
    total_merged_aliens: usize,
}

#[derive(Deserialize, Clone, Debug, Serialize, CandidType)]
struct Progress {
    iq: usize,
    social_score: usize,
    product: usize,
    all_task_done: bool,
    akai_balance: usize,
    total_task_completed: usize,
    streak: usize,
    badges: Vec<BadgesKind>,
}

#[derive(Deserialize, Clone, Debug, Serialize, CandidType)]
struct SocialData {
    players_referred: usize,
    referal_code: String,
}

#[derive(Deserialize, Clone, Debug, Serialize, CandidType)]
struct UserData {
    profile: UserProfile,
    game_state: GameState,
    progress: Progress,
    social: SocialData,
    league: LeagueType,
}

// Define enums for PowerUpKind and BadgesKind
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Copy, CandidType)]
enum PowerUpKind {
    RowPowerUp,
    ColumnPowerUp,
    NearestSquarePowerUp,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, CandidType)]
enum BadgesKind {
    TenTaskBadge,
    TwentyTaskBadge,
    ThirtyTaskBadge,
}

#[derive(Deserialize, Clone, Debug, Serialize, PartialEq, CandidType)]
enum LeagueType {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    GrandMaster,
    Challenger,
}

#[derive(Deserialize, Clone, Debug, Serialize, CandidType)]
struct LeaderboardData {
    league: usize,
    global: usize,
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