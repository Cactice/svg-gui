use glam::{DMat4, DVec2, Mat4};
use natura::Spring;
use std::sync::mpsc::{channel, Sender};
use std::{
    f64::consts::PI,
    hash::{BuildHasher, Hasher},
    iter::zip,
};
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;

struct LifeGame {
    dollars: [i32; 4],
    position: [usize; 4],
    current_player: usize,
    position_to_dollar: Vec<i32>,
}

struct LifeGameView {
    player_avatar_matrices: [SpringMat4; 4],
    tip_matrix: SpringMat4,
    players_text: [String; 4],
    position_to_coordinates: Vec<DVec2>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct LifeGameViewGpuBytes {
    players_avatar_matrices: Mat4,
    tip_matrix: Mat4,
}

impl LifeGameView {
    async fn roulette_clicked(&mut self, life_game: &mut LifeGame) {
        if self.tip_matrix.complete_animation.is_some()
            || self
                .player_avatar_matrices
                .iter()
                .any(|spring| spring.complete_animation.is_some())
        {
            return;
        }

        let one_sixths_spins = LifeGame::spin_roulette();
        self.tip_matrix
            .spring_to(DMat4::from_rotation_z(one_sixths_spins as f64 * PI / 3.))
            .await;

        life_game.proceed(one_sixths_spins);
        self.player_avatar_matrices[life_game.current_player]
            .spring_to(DMat4::from_translation(
                (
                    self.position_to_coordinates[life_game.position[life_game.current_player]],
                    0.0,
                )
                    .into(),
            ))
            .await;
        life_game.finish_turn()
    }
}

struct SpringMat4 {
    spring: Spring,
    target: DMat4,
    current: DMat4,
    velocity: DMat4,
    complete_animation: Option<Sender<()>>,
}

impl SpringMat4 {
    async fn spring_to(&mut self, target: DMat4) {
        self.target = target;
        let (sender, receiver) = channel::<()>();
        self.complete_animation = Some(sender);
        let is_err = receiver.recv().is_err();
        self.complete_animation = None;
        if is_err { /* TODO: How to handle this...?*/ }
    }
    fn update(&mut self) -> bool {
        zip(
            zip(self.current.to_cols_array(), self.velocity.to_cols_array()),
            self.target.to_cols_array(),
        )
        .for_each(|((mut current_position, mut vel), target)| {
            (current_position, vel) = self.spring.update(current_position, vel, target);
        });
        let animating_complete = self.current.abs_diff_eq(self.target, 0.1)
            && self.velocity.abs_diff_eq(DMat4::ZERO, 0.01);
        if let Some(animating_completed) = self.complete_animation.clone() {
            animating_completed.send(()).unwrap();
        }
        self.complete_animation = None;
        animating_complete
    }
}

pub(crate) fn rand_u64() -> u64 {
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
        % u64::MAX
        / u64::MAX
}
const RANDOM_VARIANCE: u64 = 6;
const RANDOM_BASE: u64 = 10;
const ROULETTE_MAX: u64 = 6;

impl LifeGame {
    fn spin_roulette() -> u64 {
        RANDOM_BASE + (rand_u64() % RANDOM_VARIANCE)
    }
    fn proceed(&mut self, steps: u64) {
        let proceed = steps % ROULETTE_MAX;
        self.position[self.current_player] =
            (self.position[self.current_player] + proceed as usize).min(self.position.len() - 1);
    }
    fn finish_turn(&mut self) {
        let dollar_delta = self
            .position_to_dollar
            .get(self.current_player)
            .expect("current_player is invalid");
        self.dollars[self.current_player] += dollar_delta;
        for n in 1..4 {
            if n == 4 {
                todo!("game finished")
            } else {
                self.position[self.current_player] = self.current_player + n;
                break;
            }
        }
    }
}

fn main() {
    let package = parser::parse(include_str!("../../svg/life.svg")).expect("failed to parse XML");
    let document = package.as_document();
    let x = evaluate_xpath(&document, "//g[@id='Route']/path[matches(@id,'\\d\\.')]")
        .expect("Xpath parsing error");
}