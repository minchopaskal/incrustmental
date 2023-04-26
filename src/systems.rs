use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::incremental::{State, Stage, LEMONADE, LEMONS, SHOPS};

fn choose_multiplier(title: &str, ctx: &mut EguiContexts,) -> (u32, bool) {
    let mut multiplier = 1;
    let mut clicked = false;
    egui::Window::new(title).show(ctx.ctx_mut(), |ui| {
        if ui.button("1").clicked() {
            clicked = true;
        }

        if ui.button("10").clicked() {
            multiplier = 10;
            clicked = true;
        }
        
        if ui.button("100").clicked() {
            multiplier = 100;
            clicked = true;
        }

        if ui.button("Cancel").clicked() {
            multiplier = 0;
            clicked = true;
        }
    });

    (multiplier, clicked)
}

pub fn setup(mut state: ResMut<State>) {
    state.activate_product(LEMONADE);
}

pub fn handle_input(
    mut state: ResMut<State>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::D) {
        state.stage = Stage::Buy(SHOPS);
    }

    if input.just_pressed(KeyCode::F) {
        state.stage = Stage::Buy(LEMONS);
    }

    if input.just_pressed(KeyCode::E) {
        state.exit();
    }

    if input.just_pressed(KeyCode::K) {
        state.inc_price(LEMONADE);
    }

    if input.just_pressed(KeyCode::J) {
        state.dec_price(LEMONADE);
    }
}

pub fn update(mut state: ResMut<State>, time: Res<Time>) {
    state.update(time.delta());
}

pub fn draw(mut state: ResMut<State>, mut ctx: EguiContexts) {
    egui::Window::new("Lemonstand").show(ctx.ctx_mut(), |ui| {
        ui.label(&format!("Money: ${:.2}", state.money));

        for material in state.materials.iter() {
            ui.label(&format!("{:?}: {}, price: ${:.2}", material.kind, material.count, material.price()));
        }

        for product in state.products.iter() {
            ui.label(&format!("{}: {}, sold: {}, price: ${:.2}, interest: {:.4}%", product.name, product.count, product.sold, product.price, product.interest() * 100.));
        }

        ui.label("[d] - Open new stand");
        ui.label("[f] - Buy a lemon");
        ui.label("[k/j] - Increase/decrease lemonade price");
        ui.label("[e] - Exit");
    });

    match state.stage {
    Stage::Buy(id) => {
        let (cnt, click) = choose_multiplier(&format!("How many {:?}s to buy?", state.materials[id].kind), &mut ctx);
        if click {
            state.buy_material(id, cnt);
            state.stage = Stage::Normal;
        }
    },
    _ => {},
    }
}