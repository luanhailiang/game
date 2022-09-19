use std::sync::Arc;
use std::sync::Mutex;
use bevy_ecs::{ prelude::*};

include!("system.rs");
include!("resource.rs");
include!("component.rs");

pub struct EcsState{
    world: World,
    schedule: Schedule,
}

impl EcsState{
    pub fn get_instance() -> Arc<Mutex<EcsState>> {
        static mut ECS_STATE: Option<Arc<Mutex<EcsState>>> = None;
        unsafe {
            ECS_STATE.get_or_insert_with(|| {
                Arc::new(Mutex::new(EcsState {world:World::new(),schedule:Schedule::default()}))
            }).clone()
        }
    }

    pub fn init(&mut self) {
        self.world.init_resource::<GameState>();
        self.schedule.add_stage("update", SystemStage::parallel()
            .with_system(print_message_system)
            .with_system(new_round_system.after(print_message_system))
            .with_system(score_check_system.after(new_round_system))
            .with_system(game_over_system.after(score_check_system))
        );
    }

    pub fn run(&mut self){
        self.schedule.run(&mut self.world);
    }
}
