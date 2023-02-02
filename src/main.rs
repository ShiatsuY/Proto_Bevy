use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::WindowMode::BorderlessFullscreen,
    audio::AudioSink,
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Proto".to_string(),
                mode: BorderlessFullscreen,
                //width: 1280.,
                //height: 720.,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Sizes{
            player: 0.,
            star: 0.,
            pickup: 0.,
            orb: 0.,
        })
        .insert_resource(Speed{
            player: 0.,
            star: 0.,
            orb: 0.,
        })
        .insert_resource(Score{
            value: 0,
        })
        .insert_resource(Volume{
            value: 50,
        })
        .insert_resource(OrbsRGB{
            r: 1.,
            g: 0.,
            b: 0.,
        })
        .insert_resource(GameTime{
            value: 0.,
        })
        .insert_resource(IDmin{
            value: -1,
            last: -1,
        })
        .insert_resource(Dist{
            value: f32::INFINITY,
        })
        
        .add_event::<CollisionEvent>()
        .add_event::<PickupCollision>()
        .add_state(GameState::Init)
        .add_system_set(
            SystemSet::on_enter(GameState::Init)
                .with_system(setup_intro)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Init)
                .with_system(delete_intro)
                .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(movement)
                .with_system(orb_movement)
                .with_system(detect_collisions)
                .with_system(manage_collisions.after(detect_collisions))
                .with_system(handle_pickup_collision.after(manage_collisions))
                .with_system(update_time)
                .with_system(update_score)
                .with_system(move_scene)
                .with_system(update_volume)
                .with_system(audio_control)
                .with_system(nearest_pick)
                .with_system(check_win)
        )

        .add_system_set(
            SystemSet::on_enter(GameState::Pause)
                .with_system(toggle_music)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Pause)
                //.with_system(update_volume)
                .with_system(audio_control)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Pause)
                .with_system(toggle_music)
        )

        .add_system_set(
            SystemSet::on_enter(GameState::Dead)
                .with_system(dead_text)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Dead)
                .with_system(move_scene)
                //.with_system(update_volume)
                .with_system(audio_control)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Dead)
                .with_system(delete_dead)
                .with_system(reset_game)
        )

        .add_system_set(
            SystemSet::on_enter(GameState::Victory)
                .with_system(victory_text)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Victory)
                .with_system(move_scene)
                //.with_system(update_volume)
                .with_system(audio_control)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Victory)
                .with_system(delete_victory)
                .with_system(reset_game)
        )
        //.add_system(toggle_cursor)
        .add_system(toggle_state)
        .add_system(bevy::window::close_on_esc)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Init,
    Game,
    Pause,
    Dead,
    Victory
}

struct CollisionEvent(Entity, Entity);

struct PickupCollision(Entity);

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Pickup;
#[derive(Component)]
struct Orb;
#[derive(Component)]
struct OrbBorder;
#[derive(Component)]
struct Star;
#[derive(Component)]
struct Collider(f32);
#[derive(Component, Eq, PartialEq, Ord, PartialOrd)]
enum CollideType {
    Player,
    Pickup,
    Orb,
}
#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct TimeText;
#[derive(Component)]
struct VolumeText;
#[derive(Component)]
struct IntroText;
#[derive(Component)]
struct DeadText;
#[derive(Component)]
struct VictoryText;
#[derive(Component)]
struct PickID {
    number: i32,
}
#[derive(Component)]
struct RootNode;
// TODO: manage text as enum and events


#[derive(Resource)]
struct Sizes {
    player: f32,
    star: f32,
    pickup: f32,
    orb: f32,
}
#[derive(Resource)]
struct Speed {
    player: f32,
    star: f32,
    orb: f32,
}
#[derive(Resource)]
struct Score {
    value: i32,
}
#[derive(Resource)]
struct Volume {
    value: i32,
}

#[derive(Resource)]
struct OrbsRGB {
    r: f32,
    g: f32,
    b: f32,
}
#[derive(Resource)]
struct GameTime {
    value: f32,
}
#[derive(Resource)]
struct IDmin {
    value: i32,
    last: i32,
}
#[derive(Resource)]
struct Dist {
    value: f32,
}
#[derive(Resource)]
struct MusicController(Handle<AudioSink>);

fn victory_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    mut query: Query<Entity, With<RootNode>>,
    mut player_query: Query<&mut Visibility, With<Player>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let text_color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.5,
    };

    for node in query.iter_mut(){
        commands.entity(node).with_children(|parent|{
            parent.spawn(
                TextBundle::from_section(
                    "Victory!\nPress Space To Play Again!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: window.width()/20.,
                        color: text_color,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_CENTER)
            )
            .insert(VictoryText);
        });
    }

    for mut player in player_query.iter_mut(){
        player.is_visible = false;
    }
}

fn delete_victory(
    mut commands: Commands, 
    mut query: Query<Entity, With<VictoryText>>,
    mut player_query: Query<&mut Visibility, With<Player>>,
) {
    for text in query.iter_mut(){
        commands.entity(text).despawn_recursive();
    }

    for mut player in player_query.iter_mut(){
        player.is_visible = true;
    }
}

fn check_win(
    orb_query: Query<Entity, With<Orb>>,
    mut state: ResMut<State<GameState>>,
) {
    let mut i = 0;
    for _orb in orb_query.iter() {
        i += 1;
    }
    if i == 0 {
        state.set(GameState::Victory).unwrap();
    }
}

fn setup_intro(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let text_color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.5,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(RootNode)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Press Space To Play!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: window.width()/20.,
                        color: text_color,
                    }
                )
            )
            .insert(IntroText);
        });
}

fn delete_intro(
    mut commands: Commands, 
    mut query: Query<Entity, With<IntroText>>
) {
    for text in query.iter_mut(){
        commands.entity(text).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut size: ResMut<Sizes>,
    mut speed: ResMut<Speed>,
    mut node_query: Query<Entity, With<RootNode>>,
    score: Res<Score>,
    volume: Res<Volume>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    windows.primary_mut().set_cursor_visibility(false);

    let window = windows.get_primary_mut().unwrap();
    let mut rng = rand::thread_rng();

    let text_color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.5,
    };

    let music = asset_server.load("music/p.mp3");
    let handle = audio_sinks.get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume((volume.value as f32)/100.0)));
    commands.insert_resource(MusicController(handle));

    // UI

    for node in node_query.iter_mut(){
        commands.entity(node)
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        score.value.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: window.width()/40.,
                            color: text_color,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(window.height()/64.),
                            left: Val::Px(window.width()/16.),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(ScoreText);
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "0.0",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: window.width()/40.,
                            color: text_color,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(window.height()/64.),
                            //left: Val::Px(window.width()/2.), // hardcoded :<
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(TimeText);
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        volume.value.to_string() + "%",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: window.width()/40.,
                            color: text_color,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(window.height()/64.),
                            right: Val::Px(window.width()/16.),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(VolumeText);
            });
    }

    // Pickups
    size.pickup = window.width() * 0.01;

    for i in 0..10{
        let x = rng.gen_range(size.pickup - window.width()/2. .. -size.pickup + window.width()/2.);
        let y = rng.gen_range(size.pickup - window.height()/2. .. -size.pickup + window.height()/2.);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size.pickup * 0.5).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(x, y, 1.)),
            ..default()
        })
            .insert(Pickup)
            .insert(PickID{number: i})
            .insert(CollideType::Pickup)
            .insert(Collider(size.pickup))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.pickup).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
                ..default()
            });
        });
    }

    // Orbs
    size.orb = window.width() * 0.1;
    speed.orb = window.width()/7.5;

    for i in 0..4{
        let x = (window.width() + size.orb + i as f32 * size.orb * 2. + i as f32 * size.orb) - window.width()/2.;
        let y = rng.gen_range(size.orb - window.height()/2. .. -size.orb + window.height()/2.);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size.orb * 0.95).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_translation(Vec3::new(x, y, 1.)),
            ..default()
        })
            .insert(Orb)
            .insert(CollideType::Orb)
            .insert(Collider(size.orb))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.orb).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                ..default()
            })
            .insert(OrbBorder);
        });
    }

    // Stars
    size.star = window.width()/1000.;
    speed.star = window.width()/2000.;
    let stars = ((window.width()/window.height())*100.).floor();

    for _i in 0 .. stars as u8{
        let x = rng.gen_range(size.star - window.width()/2. .. -size.star + window.width()/2.);
        let y = rng.gen_range(size.star - window.height()/2. .. -size.star + window.height()/2.);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size.star).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..default()
        })
        .insert(Star);
    }
    
    // Player (should be drawn at the end)
    let p_x = 0.;//-window.width()/4.;
    let p_y = 0.;
    size.player = window.width()/50.;
    speed.player = window.width()/3.;
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(size.player).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(p_x, p_y, 2.)),
        ..default()
    })
        .insert(Player)
        .insert(CollideType::Player)
        .insert(Collider(size.player))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.player * 0.95).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                ..default()
            });
        });
}

fn nearest_pick(
    mut commands: Commands,
    mut pick_q: Query<(Entity, &Transform, &PickID), With<Pickup>>,
    player_q: Query<&Transform, With<Player>>,
    mut min_id: ResMut<IDmin>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    size: Res<Sizes>,
    mut dist: ResMut<Dist>
) {
    dist.value = f32::INFINITY;
    let mut vs = Vec::new();
    for player in player_q.iter(){
        for (_entity, pick, pick_id) in pick_q.iter_mut() {
            dist.value = player.translation.distance(pick.translation);
            vs.push((dist.value, pick_id.number));
        }
    }
    let mut minid = -1;
    let mut minvalue = f32::INFINITY;
    for v in vs {
        let (a, b) = v;
        if a < minvalue {
            minvalue = a;
            minid = b;
        }
    }
    min_id.value = minid;

    if min_id.value != min_id.last {
        for (entity, _pick, pick_id) in pick_q.iter_mut() {
            if pick_id.number == min_id.value {
                commands.entity(entity).despawn_descendants();
                    commands.entity(entity)
                        .with_children(|parent| {
                            parent.spawn(MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::new(size.pickup).into()).into(),
                                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                                ..default()
                            });
                        });
            }
            if pick_id.number == min_id.last {
                commands.entity(entity).despawn_descendants();
                    commands.entity(entity)
                        .with_children(|parent| {
                            parent.spawn(MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::new(size.pickup).into()).into(),
                                material: materials.add(ColorMaterial::from(Color::BLUE)),
                                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                                ..default()
                            });
                        });
                
            }
        }
    }
    min_id.last = min_id.value;
}

fn dead_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    mut query: Query<Entity, With<RootNode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let text_color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.5,
    };

    for node in query.iter_mut(){
        commands.entity(node).with_children(|parent|{
            parent.spawn(
                TextBundle::from_section(
                    "Defeat!\nPress Space To Play Again!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: window.width()/20.,
                        color: text_color,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_CENTER)
            )
            .insert(DeadText);
        });
    }
}

fn delete_dead(
    mut commands: Commands, 
    mut query: Query<Entity, With<DeadText>>
) {
    for text in query.iter_mut(){
        commands.entity(text).despawn_recursive();
    }
}

fn reset_game(
    mut windows: ResMut<Windows>,
    mut color: ResMut<OrbsRGB>,
    mut size: ResMut<Sizes>,
    mut speed: ResMut<Speed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut time: ResMut<GameTime>,
    mut player_query: Query<(Entity, &mut Transform), (With<Player>, Without<Orb>)>,
    mut orb_query: Query<(Entity, &mut Transform), (With<Orb>, Without<Player>)>
) {
    let mut rng = rand::thread_rng();
    let window = windows.get_primary_mut().unwrap();

    score.value = 0;
    time.value = 0.;
    color.r = 1.;
    color.g = 0.;
    color.b = 0.;
    speed.orb = window.width()/8.;
    
    for (p, mut transform) in player_query.iter_mut(){
        let p_x = -window.width()/4.;
        let p_y = 0.;

        transform.translation.x = p_x;
        transform.translation.y = p_y;
        commands.entity(p).despawn_descendants();
        commands
            .entity(p)
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(size.player * 0.95).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    transform: Transform::from_translation(Vec3::new(0.,0.,4.)),
                    ..default()
                });
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(size.player * 0.0095 * score.value as f32).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(0.,0.,5.)),
                    ..default()
                });
            });
    }
    let mut i = 0;
    for (o, mut transform) in orb_query.iter_mut(){
        let o_x = (window.width() + size.orb + i as f32 * size.orb * 2. + i as f32 * size.orb) - window.width()/2.;
        let o_y = rng.gen_range(size.orb - window.height()/2. .. -size.orb + window.height()/2.);
        transform.translation.x = o_x;
        transform.translation.y = o_y;
        i += 1;
        commands.entity(o).despawn_descendants();
        commands
            .entity(o)
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(size.orb).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(0.,0.,-1.)),
                    ..default()
                }).insert(OrbBorder);
            });
    }
    if i == 0 {
        size.orb = window.width() * 0.1;

        for i in 0..4{
            let x = (window.width() + size.orb + i as f32 * size.orb * 2. + i as f32 * size.orb) - window.width()/2.;
            let y = rng.gen_range(size.orb - window.height()/2. .. -size.orb + window.height()/2.);

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.orb * 0.95).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                ..default()
            })
                .insert(Orb)
                .insert(CollideType::Orb)
                .insert(Collider(size.orb))
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(size.orb).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                    ..default()
                })
                .insert(OrbBorder);
            });
        }

        size.pickup = window.width() * 0.01;

        for i in 0..10{
            let x = rng.gen_range(size.pickup - window.width()/2. .. -size.pickup + window.width()/2.);
            let y = rng.gen_range(size.pickup - window.height()/2. .. -size.pickup + window.height()/2.);

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.pickup * 0.5).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                ..default()
            })
                .insert(Pickup)
                .insert(PickID{number: i})
                .insert(CollideType::Pickup)
                .insert(Collider(size.pickup))
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(size.pickup).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
                    ..default()
                });
            });
        }

    }
}

fn toggle_music(
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
) {
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        sink.toggle();
    }
}

fn update_time(
    time: Res<Time>, 
    mut time_counter: ResMut<GameTime>, 
    mut query: Query<&mut Text, With<TimeText>>
) {
    for mut text in &mut query {
        time_counter.value = time_counter.value + time.delta_seconds();

        text.sections[0].value = format!("{:.1}", time_counter.value);
    }
}

fn update_score(
    score: Res<Score>, 
    mut query: Query<&mut Text, With<ScoreText>>
) {
    for mut text in &mut query {
        text.sections[0].value = score.value.to_string();
    }
}

fn toggle_state(
    input: Res<Input<KeyCode>>, 
    mut state: ResMut<State<GameState>>, 
) {
    if input.just_pressed(KeyCode::Space) {
        match state.current() {
            GameState::Init => {
                state.set(GameState::Game).unwrap();
            }
            GameState::Game => {
                state.set(GameState::Pause).unwrap();
            }
            GameState::Pause => {
                state.set(GameState::Game).unwrap();
            }
            GameState::Dead => {
                state.set(GameState::Game).unwrap();
            }
            GameState::Victory => {
                state.set(GameState::Game).unwrap();
            }
        }
    }
}

fn detect_collisions(
    collision_query: Query<(Entity, &Collider, &CollideType, &Transform)>,
    mut event_writer: EventWriter<CollisionEvent>,
) {
    for (entity_a, collider_a, collide_type_a, transform_a) in collision_query.iter() {
        for (entity_b, collider_b, collide_type_b, transform_b) in collision_query.iter() {
            //to avoid duplicate calculations and events
            if entity_a < entity_b {
                let distance = transform_a.translation - transform_b.translation;
                
                if distance.length() <= collider_a.0 + collider_b.0 {
                    //to enforce order as player < pickup < orb for easier handling
                    //println!{"{}", distance};
                    if collide_type_a < collide_type_b {
                        event_writer.send(CollisionEvent(entity_a, entity_b));
                    } else {
                        event_writer.send(CollisionEvent(entity_b, entity_a));
                    }
                }
            }
        }
    }
}

fn manage_collisions(
    mut score: ResMut<Score>,
    mut speed: ResMut<Speed>,
    mut rgb: ResMut<OrbsRGB>,
    mut state: ResMut<State<GameState>>, 
    size: Res<Sizes>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut pickup_event_writer: EventWriter<PickupCollision>,
    mut query: Query<(Entity, &CollideType)>,
    mut o_query: Query<Entity, With<Orb>>,
    mut pick_q: Query<Entity, With<Pickup>>
) {
    for event in collision_event_reader.iter() {
        match event {
            CollisionEvent(entity_a, entity_b) => {
                let mut collide_a = None;
                let mut collide_b = None;
                for (entity, collide_type) in query.iter_mut() {
                    if &entity == entity_a {
                        collide_a = Some(collide_type);
                    } else if &entity == entity_b {
                        collide_b = Some(collide_type);
                    }
                }
                match (collide_a, collide_b) {
                    (Some(CollideType::Player), Some(CollideType::Pickup)) => {
                        //player hit pickup
                        pickup_event_writer.send(PickupCollision(*entity_b));
                        score.value += 1;

                        if score.value <= 100 {
                            commands
                                .entity(*entity_a)
                                .with_children(|parent| {
                                    parent.spawn(MaterialMesh2dBundle {
                                        mesh: meshes.add(shape::Circle::new(size.player * 0.95).into()).into(),
                                        material: materials.add(ColorMaterial::from(Color::BLUE)),
                                        transform: Transform::from_translation(Vec3::new(0.,0.,4.)),
                                        ..default()
                                    });
                                    parent.spawn(MaterialMesh2dBundle {
                                        mesh: meshes.add(shape::Circle::new(size.player * score.value as f32 / 100. as f32).into()).into(),
                                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                                        transform: Transform::from_translation(Vec3::new(0.,0.,5.)),
                                        ..default()
                                    });
                                });
                        
                            let window = windows.get_primary().unwrap();
                            // window.width()/8.
                            speed.orb += window.width()/700.;
                            //println!("{}", speed.orb);
                            rgb.r -= 0.01;
                            rgb.b += 0.01;

                            let new_rgb = Color::Rgba {
                                red: rgb.r,
                                green: rgb.g,
                                blue: rgb.b,
                                alpha: 1.0,
                            };

                            for o in o_query.iter_mut(){
                                commands.entity(o).despawn_descendants();

                                commands
                                    .entity(o)
                                    .with_children(|parent| {
                                        parent.spawn(MaterialMesh2dBundle {
                                            mesh: meshes.add(shape::Circle::new(size.orb).into()).into(),
                                            material: materials.add(ColorMaterial::from(new_rgb)),
                                            transform: Transform::from_translation(Vec3::new(0.,0.,-1.)),
                                            ..default()
                                        }).insert(OrbBorder);
                                    });
                            }
                        }
                        if score.value >= 100 {
                            // delete all pickups
                            for pick in pick_q.iter_mut(){
                                commands.entity(pick).despawn_recursive();
                            }
                        }
                    },
                    (Some(CollideType::Player), Some(CollideType::Orb)) => {
                        //player hit orb
                        if score.value < 100 {
                            state.set(GameState::Dead).unwrap();
                        } else {
                            commands.entity(*entity_b).despawn_recursive();
                        }
                    },
                    (Some(CollideType::Pickup), Some(CollideType::Orb)) => {
                        //pickup hit orb
                        pickup_event_writer.send(PickupCollision(*entity_a));
                    },
                    (Some(CollideType::Pickup), Some(CollideType::Pickup)) => {
                        //pickup hit pickup
                        //maybe relocate pickup
                        
                    }
                    _ => {
                        println!("unknown collision")
                    }
                }

            }
        }
    }
}

fn handle_pickup_collision(
    windows: Res<Windows>,
    size: Res<Sizes>,
    mut event_reader: EventReader<PickupCollision>,
    mut query: Query<(Entity, &mut Transform), (With<Pickup>, Without<Orb>)>,
) {
    let window = windows.get_primary().unwrap();
    let mut rng = rand::thread_rng();
    for PickupCollision(event_entity) in event_reader.iter() {
        for (query_entity, mut transform) in query.iter_mut() {
            if event_entity == &query_entity {
                transform.translation.x =
                    rng.gen_range(
                        size.pickup - window.width()/2. .. -size.pickup + window.width()/2.
                    );
                transform.translation.y =
                    rng.gen_range(
                        size.pickup - window.height()/2. .. -size.pickup + window.height()/2.
                    );
            }
         }
    }
}

fn move_scene(
    size: Res<Sizes>,
    speed: Res<Speed>,
    mut windows: ResMut<Windows>,
    mut s_query: Query<&mut Transform, With<Star>>,
) {
    let window = windows.get_primary_mut().unwrap();
    for mut transform in s_query.iter_mut() {
        if transform.translation.x + size.star > -window.width()/2. {
            transform.translation.x -= speed.star;
        } else {
            transform.translation.x = window.width()/2. + size.star;
        }
    }
}

fn audio_control(
    input: Res<Input<KeyCode>>,
    mut volume: ResMut<Volume>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Local<Handle<AudioSink>>,
    mut query: Query<&mut Text, With<VolumeText>>
) {
    if input.pressed(KeyCode::M) {
        if let Some(sink) = audio_sinks.get(&*music_controller) {
            println!("M pressed");
            volume.value = 0;
            sink.set_volume(0.0);
            for mut text in &mut query {
                text.sections[0].value = volume.value.to_string();
            }
        }
    }
}

fn update_volume(
    volume: Res<Volume>,
    mut query: Query<&mut Text, With<VolumeText>>
) {}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    size: Res<Sizes>,
    speed: Res<Speed>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let window = windows.get_primary_mut().unwrap();
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::W) && transform.translation.y + size.player < window.height()/2.  {
            direction.y += 1.;
        }
        if input.pressed(KeyCode::S) && transform.translation.y - size.player > -window.height()/2.  {
            direction.y -= 1.;
        }
        if input.pressed(KeyCode::D) && transform.translation.x + size.player < window.width()/2.  {
            direction.x += 1.;
        }
        if input.pressed(KeyCode::A) && transform.translation.x - size.player > -window.width()/2. {
            direction.x -= 1.;
        }

        transform.translation += speed.player * time.delta_seconds() * direction.normalize_or_zero();

        if transform.translation.x < -window.width()/2. + size.player {
            transform.translation.x = -window.width()/2. + size.player;
        }
        if transform.translation.x > window.width()/2. - size.player {
            transform.translation.x = window.width()/2. - size.player;
        }
        if transform.translation.y < -window.height()/2. + size.player {
            transform.translation.y = -window.height()/2. + size.player;
        }
        if transform.translation.y > window.height()/2. - size.player {
            transform.translation.y = window.height()/2. - size.player;
        }
    }
}

fn orb_movement(
    time: Res<Time>,
    speed: Res<Speed>,
    size: Res<Sizes>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<Orb>>,
) {
    let mut direction = Vec3::ZERO;
    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();
    direction.x = -1.;

    for mut transform in query.iter_mut(){
        if transform.translation.x <= -window.width()/2. - size.orb{
            transform.translation.x = window.width()/2. + size.orb;
            transform.translation.y = rng.gen_range(size.orb - window.height()/2. .. -size.orb + window.height()/2.);
        } else {
            transform.translation += speed.orb * time.delta_seconds() * direction.normalize_or_zero();
        }   
    }
}