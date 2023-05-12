use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::Id;
use bevy_egui::{egui, EguiContexts};
use incrustmental::prelude::Quantity;

use crate::resources::StateRes;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    EndGame,
}

pub fn main_menu(
    ctx: bevy_egui::EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    let egui_ctx = ctx.ctx();
    egui::CentralPanel::default().show(egui_ctx, |_| {
        egui::Area::new("main")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(egui_ctx, |ui| {
                if ui.button("Start").clicked() {
                    next_state.set(AppState::Game);
                }

                if ui.button("Exit").clicked() {
                    exit.send(AppExit);
                }
            });
    });
}

pub fn handle_input(input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::E) {
        exit.send(AppExit);
    }
}

pub fn update(
    mut state: ResMut<StateRes>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    state.update(time.delta());

    if state.win() {
        next_state.set(AppState::EndGame);
    }
}

pub fn draw(mut state: ResMut<StateRes>, ctx: EguiContexts) {
    let egui_ctx = ctx.ctx();

    egui::CentralPanel::default().show(egui_ctx, |_| {
        egui::Area::new("main")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(egui_ctx, |ui| {
                ui.label(&format!("Money: ${:.2}", state.money()));

                for material in state.materials().iter() {
                    if !material.active() {
                        continue;
                    }

                    ui.label(&format!(
                        "{}s: {}, price: ${:.2}",
                        material.name(),
                        material.count(),
                        material.price()
                    ));
                }

                for product in state.products().iter() {
                    let price = if let Some(price) = product.price() {
                        format!(" price: ${:.2},", price)
                    } else {
                        "".to_string()
                    };

                    ui.label(&format!(
                        "{}: {}, sold: {},{} interest: {:.4}%",
                        product.name(),
                        product.count(),
                        product.sold(),
                        price,
                        product.interest() * 100.
                    ));
                    ui.label(format!(
                        "{} recipe: {}",
                        product.name(),
                        product.recipe(&state)
                    ));
                }

                for i in 0..state.materials().len() {
                    if state.materials()[i].active() {
                        if ui
                            .button(&format!(
                                "Buy {}s",
                                state.materials()[i].name().to_lowercase()
                            ))
                            .clicked()
                        {
                            state.buy_material(i, 1);
                        }
                    }
                }

                for i in 0..state.products().len() {
                    if state.products()[i].active() {
                        ui.horizontal(|ui| {
                            if ui
                                .button(format!(
                                    "Build {}",
                                    state.products()[i].name().to_lowercase()
                                ))
                                .clicked()
                            {
                                state.construct_product(i);
                            }
                            if ui
                                .button(&format!(
                                    "+0.01 {}'s price",
                                    state.products()[i].name().to_lowercase()
                                ))
                                .clicked()
                            {
                                state.inc_price(i, 0.01);
                            }

                            if ui
                                .button(&format!("-0.01 {:?}'s price", state.products()[i].name()))
                                .clicked()
                            {
                                state.dec_price(i, 0.01);
                            }

                            if ui
                                .button(&format!(
                                    "+0.10 {}'s price",
                                    state.products()[i].name().to_lowercase()
                                ))
                                .clicked()
                            {
                                state.inc_price(i, 0.1);
                            }

                            if ui
                                .button(&format!("-0.10 {:?}'s price", state.products()[i].name()))
                                .clicked()
                            {
                                state.dec_price(i, 0.1);
                            }
                        });
                    }
                }
            });
    });

    egui::SidePanel::left(Id::new("left_panel")).show(egui_ctx, |ui| {
        ui.label("Objectives");
        for obj in state.objective().win_condition().iter() {
            match obj {
                Quantity::Money(p) => {
                    ui.label(&format!("${:.2}", p));
                }
                Quantity::Material(id, cnt) => {
                    ui.label(&format!(
                        "{} {}{}",
                        cnt,
                        state.materials()[*id].name(),
                        if *cnt > 1 { "s" } else { "" }
                    ));
                }
                Quantity::Product(id, cnt) => {
                    ui.label(&format!(
                        "{} {}{}",
                        cnt,
                        state.products()[*id].name(),
                        if *cnt > 1 { "s" } else { "" }
                    ));
                }
            }
            ui.separator();
        }

        let mut buy_automation = None;
        let mut toggle = None;
        ui.label("Automations:");
        for (id, automation) in state.automations().iter().enumerate() {
            if !automation.unlocked() {
                continue;
            }

            if automation.active() {
                if ui
                    .button(format!(
                        "{}{}",
                        automation.name(),
                        if automation.paused() {
                            "(paused)"
                        } else {
                            "(running)"
                        }
                    ))
                    .on_hover_text(automation.description(&state))
                    .clicked()
                {
                    toggle = Some(id);
                }
            } else {
                if ui
                    .button(&format!("Buy automation {}", automation.name()))
                    .on_hover_text(automation.description(&state))
                    .clicked()
                {
                    buy_automation = Some(id);
                }
                ui.label("Price:");
                for price in automation.price().iter() {
                    match price {
                        Quantity::Money(p) => {
                            ui.label(&format!("${:.2}, ", p));
                        }
                        Quantity::Material(id, cnt) => {
                            ui.label(&format!(
                                "{} {}{}",
                                cnt,
                                state.materials()[*id].name(),
                                if *cnt > 1 { "s" } else { "" }
                            ));
                        }
                        Quantity::Product(id, cnt) => {
                            ui.label(&format!(
                                "{} {}{}",
                                cnt,
                                state.products()[*id].name(),
                                if *cnt > 1 { "s" } else { "" }
                            ));
                        }
                    }
                }
            }

            ui.separator();
        }

        if let Some(id) = buy_automation {
            state.buy_automation(id);
        }
        if let Some(id) = toggle {
            state.toggle_automation(id);
        }
    });

    egui::SidePanel::right(Id::new("right_panel")).show(egui_ctx, |ui| {
        ui.label("Perks");

        let mut buy = None;

        for (i, perk) in state.perks().iter().enumerate() {
            if perk.unlocked() {
                if perk.active() {
                    let _ = ui
                        .button(&format!("{}", perk.name()))
                        .on_hover_text(perk.description());
                } else {
                    if ui.button(&format!("Buy perk {}", perk.name())).clicked() {
                        buy = Some(i);
                    }
                    ui.label("Price:");
                    for price in perk.price().iter() {
                        match price {
                            Quantity::Money(p) => {
                                ui.label(&format!("${:.2}, ", p));
                            }
                            Quantity::Material(id, cnt) => {
                                ui.label(&format!(
                                    "{} {}{}",
                                    cnt,
                                    state.materials()[*id].name(),
                                    if *cnt > 1 { "s" } else { "" }
                                ));
                            }
                            Quantity::Product(id, cnt) => {
                                ui.label(&format!(
                                    "{} {}{}",
                                    cnt,
                                    state.products()[*id].name(),
                                    if *cnt > 1 { "s" } else { "" }
                                ));
                            }
                        }
                    }
                };
                ui.separator();
            }
        }

        if let Some(id) = buy {
            state.buy_perk(id);
        }
    });

    egui::TopBottomPanel::bottom(Id::new("bottom_panel")).show(egui_ctx, |ui| {
        ui.label("Badges");
        for badge in state.badges().iter() {
            if badge.unlocked() {
                let _ = ui
                    .button(&format!("{}", badge.name()))
                    .on_hover_text(badge.description());
            }
        }
    });
}

pub fn end_screen(mut exit: EventWriter<AppExit>, ctx: EguiContexts) {
    let egui_ctx = ctx.ctx();

    egui::CentralPanel::default().show(egui_ctx, |_| {
        egui::Area::new("main")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(egui_ctx, |ui| {
                ui.label("You win!");
                if ui.button("Ok").clicked() {
                    exit.send(AppExit);
                }
            });
    });
}
