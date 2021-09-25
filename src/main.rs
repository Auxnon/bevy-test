use bevy::{
    app::AppExit, input::keyboard::KeyboardInput, pbr::AmbientLight, prelude::*, scene::InstanceId,
};
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin, PixelSpriteQuad};

fn main() {
    //bevy::app::App::build()
    App::build()
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::rgb(0.1, 0.1, 0.1),
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        //.add_system(move_scene_entities.system())
        .add_system(print_keyboard_event_system.system())
        .add_system(key_system.system())
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
    /*
    let sprite_handle = materials.add(asset_server.load("my-pixel-art-sprite.png").into());
    commands.spawn_bundle(SpriteBundle {
        material: sprite_handle,
        mesh: quad.clone().into(),
        ..Default::default()
    });*/

    commands
        .spawn_bundle((
            {
                let mut t = Transform::from_xyz(0., 0., 0.);
                t.scale = Vec3::new(0.05, 0.05, 0.05);
                t
            },
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("console.glb#Scene0"));
        });

    let instance_id = scene_spawner.spawn(asset_server.load("console.glb"));
    //scene_instance.0 = Some(instance_id);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: {
            let mut t =
                Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y);
            t.scale = Vec3::new(0.1, 0.3, 0.5);
            t
        },
        ..Default::default()
    });

    //commands.spawn_bundle(PixelCameraBundle::from_resolution(320, 240));

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

        /*let x = 100. * f32::cos(time.delta_seconds());
        let y = 100. * f32::cos(time.delta_seconds());
        let z = 100. * f32::cos(time.delta_seconds());
        *transform = Transform::from_xyz(x, y, z) * *transform;*/
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
        //println!("{}", transform.scale);
        direction *= -1.;
        scale += 0.5;
    }
}

fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        //info!("{:?}", event);
        info!("{:?}", event.key_code);
    }
}

pub fn key_system(
    keys: Res<Input<KeyCode>>,
    btns: Res<Input<MouseButton>>,
    mut exit: EventWriter<AppExit>,
) {
    // Keyboard input
    if keys.pressed(KeyCode::Space) {
        eprintln!("space is being held down");
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    // Mouse buttons
    if btns.just_pressed(MouseButton::Left) {
        eprintln!("a left click just happened");
    }
}
