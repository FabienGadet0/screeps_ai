use screeps::game;

enum Role {
    Harvester,
    Upgrader,
    Builder,
    Fighter,
    Healer,
}

struct Creep {
    pub spawn_id: spawns::SpawnId,
    pub creep_name: creeps::CreepName,
    pub origin_room: rooms::RoomName,
    pub role: Role,
    pub lvl: u8,
    pub task: Option<Task>,
    pub creep_obj: creeps::Creep,
}

impl Creep {
    pub fn new(
        spawn_id: spawns::SpawnId,
        creep_name: creeps::CreepName,
        origin_room: rooms::RoomName,
        role: Role,
        lvl: u8,
        task: Option<Task>,
    ) -> Self {
        if let Some(creep_obj) = game::creeps::get_creep(&creep_name) {
            Self {
                spawn_id,
                creep_name,
                origin_room,
                role,
                lvl,
                task,
                creep_obj,
            }
        }
        debug!("couldn't find creep");
    }

    pub fn set_task(&mut self, task: Task) {
        self.task = Some(task);
    }

    pub fn finish_task(&mut self) {
        self.task = None;
    }

    pub fn run(&mut self) {
        match &self.task {
            Some(task) => match task.action {
                Action::Harvest => run_harvester(self),
                Action::Upgrade => run_upgrader(self),
                Action::Build => todo!(),
                Action::Repair => todo!(),
                Action::Attack => todo!(),
                Action::Heal => todo!(),
            },
            None => {
                debug!("no task");
            }
        }
    }

    pub fn is_spawning(&self) -> bool {
        self.creep_obj.spawning()
    }

    pub fn run_harvester(&mut self) {
        match self
            .creep_obj?
            .store()
            .get_free_capacity(Some(ResourceType::Energy))
        {
            Some(free_capacity) if free_capacity > 0 => {
                if let Some(source) = utils::retrieve_target_obj(self.task?.target_id) {
                    if self.creep_obj.pos().is_near_to(source.pos()) {
                        let r = self.creep_obj?.harvest(&source);
                        if r != ReturnCode::Ok {
                            debug!("couldn't harvest: {:?} -> {:?}", self.creep_obj.name(), r);
                        }
                    } else {
                        self.creep_obj.move_to(&source);
                    }
                }
            }

            Some(free_capacity) if free_capacity == 0 => {
                let spawn = utils::retrieve_target_obj(self.spawn_id)?;
                if self.creep_obj.pos().is_near_to(spawn.pos()) {
                    let r = self.creep_obj?.transfer_all(spawn, ResourceType::Energy);
                    if r != ReturnCode::Ok {
                        debug!("couldn't transfer: {:?} -> {:?}", self.creep_obj.name(), r);
                    }
                } else {
                    self.creep_obj.move_to(&spawn);
                }
            }
            _ => {}
        }

        Ok()
    }

    pub fn run_upgrader(&mut self) {
        self::run_harvester(self);
        Ok()
    }
}

// fn main() {
//     let mut creep_targets = HashMap::new();
//     let mut creeps: Vec<Worker> = Vec::new();
//     let spawn = spawns::Spawn::new("Spawn1");
//     let creep_name = spawn.create_creep(&[Part::Move, Part::Move, Part::Carry, Part::Work]);
//     let creep = creeps::Creep::new(creep_name);
//     let origin_room = rooms::RoomName::new("E33N4");
//     let role = Role::Harvester;
//     let lvl = 1;
//     let worker = Worker::new(Creep::new(
//         spawn.id(),
//         creep_name,
//         origin_room,
//         role,
//         lvl,
//         creep,
//     ));
//     creeps.push(worker);
//     for creep in creeps {
//         creep.run();
//     }
// }
