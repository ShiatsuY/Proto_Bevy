use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Proto".to_string(),
                width: 1280.,
                height: 720.,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Size{
            player: 0.,
            star: 0.,
            pickup: 0.,
            orb: 0.,
        })
        .insert_resource(Speed{
            player: 0.,
            star: 0.,
            orb: 0.,
            pickup: 0.,
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
        
        .add_event::<CollisionEvent>()
        .add_event::<PickupCollision>()
        .add_startup_system(setup)
        .add_state(GameState::Init)
        .add_system(play_music)
        .add_system_set(
            SystemSet::on_enter(GameState::Init)
                .with_system(ui_invis)
                .with_system(setup_intro)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Init)
                .with_system(delete_intro)
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
                .with_system(play_music)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Dead)
                .with_system(dead_text)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Dead)
                .with_system(delete_dead)
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
    Dead
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

#[derive(Resource)]
struct Size {
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
    pickup: f32,
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

#[derive(Default)]
struct GameTime {
    value: f32,
}
impl GameTime {
    pub fn default() -> GameTime {
        GameTime {
            value: 0.0,
        }
    }
}

fn reset_game(
    mut windows: ResMut<Windows>,
    mut size: ResMut<Size>,
    mut speed: ResMut<Speed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut time: Local<GameTime>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Orb>)>,
    mut orb_query: Query<&mut Transform, (With<Orb>, Without<Player>)>
) {
    score.value = 0;
    time.value = 0.;
    let mut rng = rand::thread_rng();
    let window = windows.get_primary_mut().unwrap();

    for mut transform in player_query.iter_mut(){
        let p_x = -window.width()/4.;
        let p_y = 0.;
        transform.translation.x = p_x;
        transform.translation.y = p_y;
    }
    let mut i = 0;
    for mut transform in orb_query.iter_mut(){
        let o_x = (window.width() + size.orb + i as f32 * size.orb * 2. + i as f32 * size.orb) - window.width()/2.;
        let o_y = rng.gen_range(size.orb - window.height()/2. .. -size.orb + window.height()/2.);
        transform.translation.x = o_x;
        transform.translation.y = o_y;
        i += 1;
    }
}

fn dead_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    let text_color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.5,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Defeat!\nPress Space To Play Again!", 
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: window.width()/20.,
                    color: text_color,
                })
                .with_alignment(TextAlignment::CENTER),
            ..default()
        },
        DeadText,
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut size: ResMut<Size>,
    mut speed: ResMut<Speed>,
    score: Res<Score>,
    volume: Res<Volume>,
    asset_server: Res<AssetServer>,
) {
    windows.primary_mut().set_cursor_visibility(false);
    
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let mut rng = rand::thread_rng();

    let text_color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.5,
    };

    // UI

    commands.spawn((
        TextBundle::from_section(
            score.value.to_string(),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: text_color,
            },
        )
        .with_text_alignment(TextAlignment::CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(window.height()/64.),
                left: Val::Px(window.width()/16.),
                ..default()
            },
            ..default()
        }),
        ScoreText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "0.0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: text_color,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(window.height()/64.),
                left: Val::Px(window.width()/2. - window.width()/25.), // hardcoded :<
                ..default()
            },
            ..default()
        }),
        TimeText,
    ));

    commands.spawn((
        TextBundle::from_section(
            volume.value.to_string() + "%",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: text_color,
            },
        )
        .with_text_alignment(TextAlignment::CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(window.height()/64.),
                right: Val::Px(window.width()/16.),
                ..default()
            },
            ..default()
        }),
        VolumeText,
    ));

    // Pickups
    size.pickup = window.width() * 0.01;

    for _i in 0..9{
        let x = rng.gen_range(size.pickup - window.width()/2. .. -size.pickup + window.width()/2.);
        let y = rng.gen_range(size.pickup - window.height()/2. .. -size.pickup + window.height()/2.);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size.pickup * 0.5).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(x, y, 1.)),
            visibility: Visibility {
                is_visible: false,
            },
            ..default()
        })
            .insert(Pickup)
            .insert(CollideType::Pickup)
            .insert(Collider(size.pickup))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.pickup).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                ..default()
            });
        });
    }

    // Orbs
    size.orb = window.width() * 0.1;
    speed.orb = window.width()/8.;

    for i in 0..4{
        let x = (window.width() + size.orb + i as f32 * size.orb * 2. + i as f32 * size.orb) - window.width()/2.;
        let y = rng.gen_range(size.orb - window.height()/2. .. -size.orb + window.height()/2.);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size.orb * 0.975).into()).into(),
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
            });
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
            visibility: Visibility {
                is_visible: false,
            },
            ..default()
        })
        .insert(Star);
    }
    
    // Player (should be drawn at the end)
    let p_x = -window.width()/4.;
    let p_y = 0.;
    size.player = window.width()/50.;
    speed.player = window.width()/3.;
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(size.player).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(p_x, p_y, 0.)),
        visibility: Visibility {
            is_visible: false,
        },
        ..default()
    })
        .insert(Player)
        .insert(CollideType::Player)
        .insert(Collider(size.player))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.player * 0.95).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            });
        });
}

fn setup_intro(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    let text_color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.5,
    };
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(
                    "Press Space To Play!", 
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: window.width()/20.,
                        color: text_color,
                    })
                    .with_alignment(TextAlignment::CENTER),
                ..default()
            },
            IntroText,
        ));
        
}

fn play_music(){

}

fn delete_intro(mut commands: Commands, mut query: Query<Entity, With<IntroText>>){
    for mut text in query.iter_mut(){
        commands.entity(text).despawn();
    }
}

fn delete_dead(mut commands: Commands, mut query: Query<Entity, With<DeadText>>){
    for mut text in query.iter_mut(){
        commands.entity(text).despawn();
    }
}

fn ui_invis(mut query: Query<&mut Visibility, (With<Text>, Without<IntroText>)>){
    for mut vis in query.iter_mut(){
        vis.is_visible = false;
    }
}

fn update_time(time: Res<Time>, mut time_counter: Local<GameTime>, mut query: Query<&mut Text, With<TimeText>>){
    for mut text in &mut query {
        time_counter.value = time_counter.value + time.delta_seconds();

        text.sections[0].value = format!("{:.1}", time_counter.value);
    }
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>){
    for mut text in &mut query {
        text.sections[0].value = score.value.to_string();
    }
}

fn toggle_state(
    input: Res<Input<KeyCode>>, 
    mut state: ResMut<State<GameState>>, 
    mut query: Query<&mut Visibility>,
) {
    if input.just_pressed(KeyCode::Space) {
        match state.current() {
            GameState::Init => {
                state.set(GameState::Game).unwrap();
                    for mut vis in query.iter_mut(){
                        vis.is_visible = true;
                    }
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
        }
    }
}

fn _toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

fn _collision(
    windows: Res<Windows>,
    mut score: ResMut<Score>,
    mut speed: ResMut<Speed>,
    size: Res<Size>,
    player_q: Query<&Transform, (With<Player>, Without<Orb>, Without<Pickup>)>,
    orb_q: Query<&Transform, (With<Orb>, Without<Pickup>)>,
    mut pick_q: Query<&mut Transform, With<Pickup>>,
) {
    // There can ever be only 1 Player
    if let Ok(transform_p) = player_q.get_single() {

        // collision with orb
        for transform_o in orb_q.iter(){
            let a = size.orb + size.player;
            let x = transform_o.translation.x - transform_p.translation.x;
            let y = transform_o.translation.y - transform_p.translation.y;

            if a > (((x*x) + (y*y)) as f32).sqrt(){
                speed.orb = 0.0;
                speed.player = 0.0;
                // die or lose 1 live
            }
            // collision orb - pickup
            for mut transform_pick in pick_q.iter_mut(){
                let a2 = size.pickup + size.orb;
                let x2 = transform_pick.translation.x - transform_o.translation.x;
                let y2 = transform_pick.translation.y - transform_o.translation.y;
            
                if a2 > (((x2*x2) + (y2*y2)) as f32).sqrt(){
                    let window = windows.get_primary().unwrap();
                    let mut rng = rand::thread_rng();
                    transform_pick.translation.x = rng.gen_range(size.pickup - window.width()/2. .. -size.pickup + window.width()/2.);
                    transform_pick.translation.y = rng.gen_range(size.pickup - window.height()/2. .. -size.pickup + window.height()/2.);
                }
            }

        }

        // collision with pickup
        for mut transform_pick in pick_q.iter_mut(){
            let a = size.pickup + size.player;
            let x = transform_pick.translation.x - transform_p.translation.x;
            let y = transform_pick.translation.y - transform_p.translation.y;
        
            if a > (((x*x) + (y*y)) as f32).sqrt(){
                let window = windows.get_primary().unwrap();
                let mut rng = rand::thread_rng();
                transform_pick.translation.x = rng.gen_range(size.pickup - window.width()/2. .. -size.pickup + window.width()/2.);
                transform_pick.translation.y = rng.gen_range(size.pickup - window.height()/2. .. -size.pickup + window.height()/2.);
                // increment counter
                score.value += 1;
            }
        }
    }
}

fn detect_collisions(
    collision_query: Query<(Entity, &Collider, &CollideType, &Transform)>,
    mut event_writer: EventWriter<CollisionEvent>,
)
{
    for (entity_a, collider_a, collide_type_a, transform_a) in collision_query.iter() {
        for (entity_b, collider_b, collide_type_b, transform_b) in collision_query.iter() {
            //to avoid duplicate calculations and events
            if entity_a < entity_b {
                let distance = transform_a.translation - transform_b.translation;
                if distance.length() <= collider_a.0 + collider_b.0 {
                    //to enforce order as player < pickup < orb for easier handling
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
    size: Res<Size>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut pickup_event_writer: EventWriter<PickupCollision>,
    mut query: Query<(Entity, &CollideType)>,
    mut o_query: Query<Entity, With<Orb>>,
    mut pick_q: Query<Entity, With<Pickup>>
)
{
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

                        if score.value >= 100 {
                            // delete all pickups
                            for mut pick in pick_q.iter_mut(){
                                commands.entity(pick).despawn_recursive();
                            }
                        } else {
                        
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
                                    mesh: meshes.add(shape::Circle::new(size.player * 0.0095 * score.value as f32).into()).into(),
                                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                                    transform: Transform::from_translation(Vec3::new(0.,0.,5.)),
                                    ..default()
                                });
                            });
                        
                        let window = windows.get_primary().unwrap();
                        // window.width()/8.
                        speed.orb += window.width()/800.;
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
                            commands
                                .entity(o)
                                .with_children(|parent| {
                                    parent.spawn(MaterialMesh2dBundle {
                                        mesh: meshes.add(shape::Circle::new(size.orb).into()).into(),
                                        material: materials.add(ColorMaterial::from(new_rgb)),
                                        transform: Transform::from_translation(Vec3::new(0.,0.,-1.)),
                                        ..default()
                                    });
                                });
                        }}
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
    size: Res<Size>,
    mut speed: ResMut<Speed>,
    mut event_reader: EventReader<PickupCollision>,
    mut query: Query<(Entity, &mut Transform), (With<Pickup>, Without<Orb>)>,
)
{
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

fn _increase_size(
    input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut size: ResMut<Size>,
    mut speed: ResMut<Speed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut query: Query<Entity, With<Player>>,
) {
    let window = windows.get_primary().unwrap();

    if let Ok(entity) = query.get_single_mut() {
        if input.just_pressed(KeyCode::Space) {
            if speed.orb == 0.0 {
                speed.orb = window.width()/8.;
                speed.player = window.width()/3.;
                speed.star = window.width()/2000.;
            } else {
                speed.orb = 0.0;
                speed.player = 0.0;
                speed.star = 0.0;
            }
            size.player = size.player + window.height()/4800.;
            commands
                .entity(entity)
                .insert(Into::<bevy::sprite::Mesh2dHandle>::into(meshes
                    .add(shape::Circle::new(size.player)
                        .into())));
        }
    }
}

fn move_scene(
    size: Res<Size>,
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

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    size: Res<Size>,
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
    }
}

fn orb_movement(
    time: Res<Time>,
    speed: Res<Speed>,
    size: Res<Size>,
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