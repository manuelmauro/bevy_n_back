use bevy::{prelude::*, render::pass::ClearColor};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::VecDeque;

const SIZE: f32 = 60.0;
const SPACING: f32 = 20.0;

#[derive(Clone, Debug, PartialEq)]
enum Vertex {
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

impl Vertex {
    fn translation(&self) -> Vec3 {
        Vec3::new(
            &self.column() * (SIZE + SPACING),
            &self.row() * (SIZE + SPACING),
            0.0,
        )
    }

    fn row(&self) -> f32 {
        match &self {
            Vertex::TopLeft => 1.0,
            Vertex::TopCenter => 1.0,
            Vertex::TopRight => 1.0,
            Vertex::CenterLeft => 0.0,
            Vertex::Center => 0.0,
            Vertex::CenterRight => 0.0,
            Vertex::BottomLeft => -1.0,
            Vertex::BottomCenter => -1.0,
            Vertex::BottomRight => -1.0,
            Vertex::None => 0.0,
        }
    }

    fn column(&self) -> f32 {
        match &self {
            Vertex::TopLeft => -1.0,
            Vertex::TopCenter => 0.0,
            Vertex::TopRight => 1.0,
            Vertex::CenterLeft => -1.0,
            Vertex::Center => 0.0,
            Vertex::CenterRight => 1.0,
            Vertex::BottomLeft => -1.0,
            Vertex::BottomCenter => 0.0,
            Vertex::BottomRight => 1.0,
            Vertex::None => 0.0,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex::None
    }
}

impl Distribution<Vertex> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vertex {
        match rng.gen_range(0..=9) {
            0 => Vertex::TopLeft,
            1 => Vertex::TopCenter,
            2 => Vertex::TopRight,
            3 => Vertex::CenterLeft,
            4 => Vertex::Center,
            5 => Vertex::CenterRight,
            6 => Vertex::BottomLeft,
            7 => Vertex::BottomCenter,
            _ => Vertex::BottomRight,
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
        self.true_pos as f32
            / (self.true_pos as f32 + 0.5 * (self.false_pos as f32 + self.false_neg as f32))
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
    score: Score,
    answer: bool,
    cues: CueChain<Vertex>,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            score: Default::default(),
            answer: false,
            cues: CueChain::new(3),
        }
    }
}

struct CueChain<T> {
    short_memory: VecDeque<T>,
}

impl<T: Default> CueChain<T> {
    fn new(span: usize) -> Self {
        let mut cc = CueChain {
            short_memory: VecDeque::new(),
        };

        for _ in 0..span {
            cc.short_memory.push_front(Default::default())
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

impl<T: PartialEq> CueChain<T> {
    fn is_match(&self) -> bool {
        self.short_memory.back() == self.short_memory.front()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MyLabels {
    ScoreCheck,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalState::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(timer_system.system())
        .add_system(score_system.system().label(MyLabels::ScoreCheck))
        .add_system(cue_system.system().after(MyLabels::ScoreCheck))
        .add_system(answer_system.system())
        .add_system(scoreboard_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

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
    let cell = Vertex::None;
    let cell_material = materials.add(Color::rgb(0.46, 0.64, 0.0).into());
    commands
        .spawn_bundle(SpriteBundle {
            material: cell_material.clone(),
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

fn scoreboard_system(scoreboard: Res<GlobalState>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!(
        "Correct: {}, Wrong: {}, Score: {}",
        scoreboard.score.correct(),
        scoreboard.score.wrong(),
        scoreboard.score.f1_score()
    );
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
    mut board_query: Query<(&Vertex, &mut Transform, &Timer)>,
) {
    if let Ok((_vertex, mut transform, timer)) = board_query.single_mut() {
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
