use bevy::{prelude::*, render::pass::ClearColor};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::VecDeque;

const SIZE: f32 = 60.0;
const SPACING: f32 = 20.0;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    None,
}

impl Cell {
    fn translation(&self) -> Vec3 {
        Vec3::new(
            self.column() * (SIZE + SPACING),
            self.row() * (SIZE + SPACING),
            0.0,
        )
    }

    fn row(&self) -> f32 {
        match &self {
            Cell::TopLeft => 1.0,
            Cell::TopCenter => 1.0,
            Cell::TopRight => 1.0,
            Cell::CenterLeft => 0.0,
            Cell::Center => 0.0,
            Cell::CenterRight => 0.0,
            Cell::BottomLeft => -1.0,
            Cell::BottomCenter => -1.0,
            Cell::BottomRight => -1.0,
            Cell::None => 0.0,
        }
    }

    fn column(&self) -> f32 {
        match &self {
            Cell::TopLeft => -1.0,
            Cell::TopCenter => 0.0,
            Cell::TopRight => 1.0,
            Cell::CenterLeft => -1.0,
            Cell::Center => 0.0,
            Cell::CenterRight => 1.0,
            Cell::BottomLeft => -1.0,
            Cell::BottomCenter => 0.0,
            Cell::BottomRight => 1.0,
            Cell::None => 0.0,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::None
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=9) {
            0 => Cell::TopLeft,
            1 => Cell::TopCenter,
            2 => Cell::TopRight,
            3 => Cell::CenterLeft,
            4 => Cell::Center,
            5 => Cell::CenterRight,
            6 => Cell::BottomLeft,
            7 => Cell::BottomCenter,
            _ => Cell::BottomRight,
        }
    }
}

struct Score {
    false_pos: usize,
    true_pos: usize,
    false_neg: usize,
    true_neg: usize,
}

impl Score {
    fn correct(&self) -> usize {
        self.true_pos + self.true_neg
    }

    fn wrong(&self) -> usize {
        self.false_pos + self.false_neg
    }

    fn f1_score(&self) -> f32 {
        if self.true_pos + self.false_pos == 0 {
            1.0
        } else {
            self.true_pos as f32
                / (self.true_pos as f32 + 0.5 * (self.false_pos as f32 + self.false_neg as f32))
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Score {
            false_pos: 0,
            true_pos: 0,
            false_neg: 0,
            true_neg: 0,
        }
    }
}

struct GlobalState {
    answer: bool,
    score: Score,
    cues: CueChain<Cell>,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            answer: false,
            score: Default::default(),
            cues: CueChain::with_n_back(2),
        }
    }
}

/// Memorization and generation of new cues.
struct CueChain<T> {
    short_memory: VecDeque<T>,
}

impl<T: Default> CueChain<T> {
    fn with_n_back(n: usize) -> Self {
        let mut cc = CueChain {
            short_memory: VecDeque::new(),
        };

        for _ in 0..n + 1 {
            cc.short_memory.push_front(Default::default());
        }

        cc
    }
}

impl<T> CueChain<T>
where
    Standard: Distribution<T>,
    T: Clone + PartialEq + Default,
{
    fn gen(&mut self) -> T {
        let mut rng = rand::thread_rng();
        let y = rng.gen::<f64>();

        let cue = if y < 0.25 && *self.short_memory.front().unwrap() != Default::default() {
            self.short_memory.front().unwrap().clone()
        } else {
            rand::random()
        };

        self.short_memory.push_back(cue);
        self.short_memory.pop_front();

        (*self.short_memory.back().unwrap()).clone()
    }
}

impl<T: PartialEq + Default> CueChain<T> {
    fn is_match(&self) -> bool {
        if self.short_memory.front() != Some(&Default::default()) {
            self.short_memory.back() == self.short_memory.front()
        } else {
            false
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum SystemLabel {
    ScoreCheck,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(GlobalState::default())
        .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
        .add_startup_system(setup.system())
        .add_system(timer_system.system())
        .add_system(score_system.system().label(SystemLabel::ScoreCheck))
        .add_system(cue_system.system().after(SystemLabel::ScoreCheck))
        .add_system(answer_system.system())
        .add_system(ui_example.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(240.0, 240.0);

    // left
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });
    // bottom
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        material: wall_material,
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });

    // Add cell
    let cell = Cell::None;
    let cell_material = materials.add(Color::rgb(0.66, 0.76, 0.0).into());
    commands
        .spawn_bundle(SpriteBundle {
            material: cell_material,
            sprite: Sprite::new(Vec2::new(
                (bounds.x - SPACING) / 3.0,
                (bounds.x - SPACING) / 3.0,
            )),
            transform: Transform::from_translation(cell.translation()),
            ..Default::default()
        })
        .insert(cell)
        .insert(Timer::from_seconds(2.0, true));
}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut query: Query<&mut Timer>) {
    for mut timer in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            info!("tick!")
        }
    }
}

fn cue_system(
    mut scoreboard: ResMut<GlobalState>,
    mut board_query: Query<(&Cell, &mut Transform, &Timer)>,
) {
    if let Ok((_, mut transform, timer)) = board_query.single_mut() {
        if timer.just_finished() {
            let new_cue = scoreboard.cues.gen();

            info!("cue: {:?}", new_cue);
            transform.translation = new_cue.translation();
        }
    }
}

fn answer_system(mut scoreboard: ResMut<GlobalState>, keyboard_input: Res<Input<KeyCode>>) {
    if !scoreboard.answer {
        if keyboard_input.pressed(KeyCode::Left) {
            info!("matching!");
            scoreboard.answer = true;
        }
    }
}

fn score_system(mut scoreboard: ResMut<GlobalState>, mut query: Query<&Timer>) {
    if let Ok(timer) = query.single_mut() {
        if timer.just_finished() {
            if scoreboard.answer {
                if scoreboard.cues.is_match() {
                    scoreboard.score.true_pos += 1;
                    info!("true_positive");
                } else {
                    scoreboard.score.false_pos += 1;
                    info!("false_positive");
                }
            } else {
                if scoreboard.cues.is_match() {
                    scoreboard.score.false_neg += 1;
                    info!("false_neg");
                } else {
                    scoreboard.score.true_neg += 1;
                    info!("true_neg");
                }
            }

            scoreboard.answer = false;
        }
    }
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
fn ui_example(egui_context: ResMut<EguiContext>, global: Res<GlobalState>) {
    egui::Window::new("Debug").show(egui_context.ctx(), |ui| {
        ui.label(format!("Correct: {}", global.score.correct()));
        ui.label(format!("Wrong: {}", global.score.wrong()));
        ui.label(format!("F1 Score: {}", global.score.f1_score()));
    });
}
