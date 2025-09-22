mod mod_loader;
mod python_api;
mod dependency_resolver;

use legion::*;

fn main() {
    // 1. Инициализируем ECS
    let mut world = World::default();
    let mut schedule = Schedule::builder().build();

    python_api::inite_module();

    // 2. Загружаем мод Core
    mod_loader::load_all_mods(false);

    // 3. Пробуем выполнить ECS цикл (пока пустой)
    schedule.execute(&mut world, &mut Resources::default());
}
