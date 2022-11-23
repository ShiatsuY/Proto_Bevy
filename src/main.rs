use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};




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
        .insert_resource(Size{
            player: 15.,
            star: 5.,
            pickup: 0.,
            orb: 0.,
        })
        .insert_resource(Speed{
            player: 300.,
            star: 0.,
            orb: 0.,
        })
        .add_startup_system(setup)
        .add_system(toggle_cursor)
        .add_system(bevy::window::close_on_esc)
        .add_system(movement)
        .add_system(increase_size)
        .run();
}

#[derive(Component)]
struct Player;
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
    size: Res<Size>,
) {
    windows.primary_mut().set_cursor_visibility(false);
    commands.spawn(Camera2dBundle::default());
    let window = windows.get_primary_mut().unwrap();

    let p_x = -window.width()/4.;
    let p_y = 0.;

    // Player
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(size.player).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(p_x, p_y, 0.)),
        ..default()
    })
        .insert(Player);

    // Stars
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(size.star).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

}

fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

fn increase_size(
    input: Res<Input<KeyCode>>,
    mut size: ResMut<Size>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = query.get_single_mut() {
        if input.just_pressed(KeyCode::Space) {
            let tmp_size = size.player + 50.;
            size.player = tmp_size;
            commands
                .entity(entity)
                .insert(Into::<bevy::sprite::Mesh2dHandle>::into(meshes
                    .add(shape::Circle::new(tmp_size * 3.)
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
        let size = size.player;
        let speed = speed.player;

        if input.pressed(KeyCode::W) && transform.translation.y + size < window.height()/2.  {
            direction.y += 1.;
        }
        if input.pressed(KeyCode::S) && transform.translation.y - size > -window.height()/2.  {
            direction.y -= 1.;
        }
        if input.pressed(KeyCode::D) && transform.translation.x + size < window.width()/2.  {
            direction.x += 1.;
        }
        if input.pressed(KeyCode::A) && transform.translation.x - size > -window.width()/2. {
            direction.x -= 1.;
        }
        transform.translation += speed * time.delta_seconds() * direction.normalize_or_zero();
    }
}