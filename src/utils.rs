//? Objects related functions */
pub(crate) fn retrieve_target_pos(
    target_id: screeps::structures::StructureId,
) -> Option<screeps::RoomPosition> {
    target_id?.resolve()?.pos()
}

pub(crate) fn retrieve_target_obj(
    target_id: screeps::structures::StructureId,
) -> Option<screeps::structures::Structure> {
    target_id?.resolve()
}
//? ======================= */
