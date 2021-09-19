use bevy::{pbr::AmbientLight, prelude::*, scene::InstanceId};

fn main() {
    //bevy::app::App::build()
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .add_system(move_scene_entities.system())
        .add_system(print_keyboard_event_system.system())
        .run();
}

#[derive(Default)]
struct SceneInstance(Option<InstanceId>);

struct EntityInMyScene;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 0.0, -1.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("console.glb"));
        });

    let instance_id = scene_spawner.spawn(asset_server.load("console.glb"));
    //scene_instance.0 = Some(instance_id);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    });
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..Default::default()
        })
        .insert(Rotates);
}

/// this component indicates what entities should rotate
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}

// This system will move all entities with component `EntityInMyScene`, so all
// entities from the second scene
fn move_scene_entities(
    time: Res<Time>,
    mut scene_entities: Query<&mut Transform>, //With<EntityInMyScene>
) {
    let mut direction = 1.;
    let mut scale = 1.;
    for mut transform in scene_entities.iter_mut() {
        transform.translation = Vec3::new(
            scale * direction * time.seconds_since_startup().sin() as f32 / 20.,
            0.,
            time.seconds_since_startup().cos() as f32 / 20.,
        );
        println!("{}", transform.scale);
        direction *= -1.;
        scale += 0.5;
    }
}

fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}
