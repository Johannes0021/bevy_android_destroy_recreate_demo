use bevy::prelude::*;

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_quad_to_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
    commands.spawn(Sprite {
        color: Color::srgb(0.0, 1.0, 0.0),
        custom_size: Some(Vec2 { x: 21.0, y: 21.0 }),
        ..default()
    });
}

fn move_quad_to_input(
    touches: Res<Touches>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut sprite_transform: Single<&mut Transform, With<Sprite>>,
) {
    let (camera, camera_transform) = *camera;

    if let Some(touch) = touches.iter().next()
        && let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, touch.position())
    {
        sprite_transform.translation = world_position.extend(0.0);
    } else if let Some(cursor_pos) = window.cursor_position()
        && let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
    {
        sprite_transform.translation = world_position.extend(0.0);
    }
}
