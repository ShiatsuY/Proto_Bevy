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
        .add_startup_system(setup)
        .add_system(toggle_cursor)
        .add_system(bevy::window::close_on_esc)
        .add_system(movement)
        .add_system(increase_size)
        .run();
}

#[derive(Component)]
struct Player(f32, f32, f32, f32);

#[derive(Component)]
struct Movable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    windows.primary_mut().set_cursor_visibility(false);
    commands.spawn(Camera2dBundle::default());
    let window = windows.get_primary_mut().unwrap();
    let p_size = 15.;
    let p_speed = 300.;
    let p_x = -window.width()/4.;
    let p_y = 0.;

    let star_size = 5.;
    
    commands.spawn(Player(p_size, p_speed, p_x, p_y));

    // Player
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(p_size).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(p_x, p_y, 0.)),
        ..default()
    })
    .insert(Movable);

    // Stars
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(star_size).into()).into(),
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut mesh_query: Query<Entity, With<Movable>>,
    transform_query: Query<&Transform, With<Movable>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut p = player_query.single_mut();
        let m = mesh_query.single_mut();
        let transform = transform_query.single();
        p.0 += 50.;
        p.2 = transform.translation.x;
        p.3 = transform.translation.y;


        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(p.0 * 3.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_translation(Vec3::new(p.2, p.3, 0.)),
            ..default()
        })
        .insert(Movable);

        commands.entity(m).despawn_recursive();

        //let size = player.0;

        //let mesh_2d = pos_query.single_mut();
        //let mesh_handle = &mesh_2d.0;

        // look for query bundles


    }
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut Transform, With<Movable>>,
    mut player_query: Query<&Player>,
) {
    let player = player_query.single_mut();
    let size = player.0;
    let speed = player.1;
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;
    let window = windows.get_primary_mut().unwrap();

    //println!("{},{}", player.0, player.1);

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
