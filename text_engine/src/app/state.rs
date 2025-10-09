use legion::{World, Resources, Schedule};

#[derive(Debug)]
pub struct AppState {
    pub world: World,
    pub schedule: Schedule,
    pub windows: Vec<(String, bool)>,
}

impl AppState {
    pub fn new() -> Self {
        let world = World::default();
        let schedule = Schedule::builder().build();
        
        Self {
            world,
            schedule,
            windows: vec![
                ("Window 1".to_string(), false)
            ],
        }
    }

    pub fn update(&mut self) {
        let mut resources = Resources::default();
        self.schedule.execute(&mut self.world, &mut resources);
    }
}
