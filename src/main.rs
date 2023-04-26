use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use incremental::{State, ProductMaterial, MaterialKind, Product, Stage, build_lemonade};
use systems::{handle_input, update, draw, setup};

mod incremental;
mod systems;

fn main() -> std::io::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "IncRustMental".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .insert_resource(State {
            money: 0f64,
            materials: [
                ProductMaterial {
                    bought: 0,
                    kind: MaterialKind::Lemonstand,
                    count: 1,
                    base_price: 1f64,
                    growth: 1.07f64,
                    active: false,
                },
                ProductMaterial {
                    bought: 0,
                    count: 100,
                    base_price: 0.1f64,
                    growth: 1.07f64,
                    kind: MaterialKind::Lemon,
                    active: false,
                },
            ],
            products: [
                Product {
                    count: 0,
                    price: 1.0,
                    sold: 0,
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    name: "Lemonade".to_string(),
                    dependencies: vec![0, 1],
                    active: false,
                    build_fn: build_lemonade,
                    unlocks: Vec::new(),
                },
            ],
            stage: Stage::Normal,
            exit: false,
        })
        .add_startup_system(setup)
        .add_system(handle_input.before(update))
        .add_system(update)
        .add_system(draw.after(update))
        .run();

    Ok(())
}
