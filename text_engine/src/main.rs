mod mod_loader;
mod python_api;
mod dependency_resolver;

use legion::*;

fn main() {
    let mut world = World::default();
    let mut schedule = Schedule::builder().build();

    python_api::inite_module();

    mod_loader::load_all_mods(false);

    schedule.execute(&mut world, &mut Resources::default());
}
