use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};
use rand::Rng;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Proto".to_string(),
                width: 640.,
                height: 360.,
                ..default()
            },
            ..default()
        }))
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
        })
        .add_startup_system(setup)
        .add_system(toggle_cursor)
        .add_system(bevy::window::close_on_esc)
        .add_system(movement)
        .add_system(orb_movement)
        .add_system(increase_size)
        .run();
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Orb;
#[derive(Component)]
struct OrbBorder;
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
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut size: ResMut<Size>,
    mut speed: ResMut<Speed>,
) {
    windows.primary_mut().set_cursor_visibility(false);
    
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();

    // Orbs
    size.orb = window.width() * 0.1;
    speed.orb = window.width()/8.; // window.width() / 512.;

    for i in 0..4{
        let mut rng = rand::thread_rng();

        let x = (window.width() + size.orb + i as f32 * size.orb * 2. + i as f32 * size.orb) - window.width()/2.;
        let y = rng.gen_range(size.orb - window.height()/2. .. -size.orb + window.height()/2.);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size.orb * 0.975).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_translation(Vec3::new(x, y, 2.)),
            ..default()
        })
        .insert(Orb)
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size.orb).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0.,0.,-1.)),
                ..default()
            });
        });
    }

    // Stars
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(size.star).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    // Player (should be drawn at the end)
    let p_x = -window.width()/4.;
    let p_y = 0.;
    size.player = window.width()/80.;
    speed.player = window.width()/3.;
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(size.player).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(p_x, p_y, 3.)),
        ..default()
    }).insert(Player);

}

fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

fn increase_size(
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
            } else {
                speed.orb = 0.0;
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