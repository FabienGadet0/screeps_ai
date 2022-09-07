use crate::utils::retrieve_target_pos;
use screeps::game;

enum Action {
    Harvest,
    Upgrade,
    Build,
    Repair,
    Attack,
    Heal,
}

struct Task {
    target_id: Option<screeps::structures::StructureId>,
    target_pos: Option<screeps::RoomPosition>,
    action: Action,
}

impl Task {
    pub fn new(target_id: Option<screeps::structures::StructureId>, action: Action) -> Self {
        Self {
            target_id,
            target_pos: retrieve_target_pos(target_id),
            action,
        }
    }
}

// pub fn do(&self, creep_obj: game::creeps::Creep) {
//     match self.action {
//         Action::Harvest => run_harvester(creep_obj),
//         Action::Upgrade => run_upgrader(creep_obj),
//         Action::Build => todo!(),
//         Action::Repair => todo!(),
//         Action::Attack => todo!(),
//         Action::Defend => todo!(),
//     }
// }

//    ReturnCode::Ok => {
//    ReturnCode::NotInRange => {
//    ReturnCode::NotEnoughEnergy => {
//    ReturnCode::Tired => {
//    ReturnCode::NotOwner => {
//    ReturnCode::Busy => {
//    ReturnCode::InvalidTarget => {
//    ReturnCode::NotFound => {
//    ReturnCode::NotAllowed => {
//    ReturnCode::RangeTooFar => {
