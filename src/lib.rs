use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{RenderCreation, WgpuLimits, WgpuSettings, WgpuSettingsPriority},
    },
    window::{AppLifecycle, PresentMode},
    winit::WinitSettings,
};
use std::time::Duration;

// Solve LogPlugin error logging on recreation.
//
//use bevy::log::LogPlugin;
//use std::sync::atomic::{AtomicBool, Ordering};
//static FIRST_BUILD: AtomicBool = AtomicBool::new(true);

#[bevy_main]
pub fn main() {
    let plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,

                #[cfg(target_os = "android")]
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    bevy::window::MonitorSelection::Current,
                ),

                ..default()
            }),
            ..default()
        })
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                priority: WgpuSettingsPriority::WebGL2,
                limits: WgpuLimits::downlevel_webgl2_defaults()
                    .using_resolution(WgpuLimits::default()),
                ..default()
            })),
            ..default()
        });

    // Solve LogPlugin error logging on recreation.
    //
    //if !FIRST_BUILD.swap(false, Ordering::Relaxed) {
    //    plugins = plugins.disable::<LogPlugin>();
    //}

    App::new()
        .add_plugins(plugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_winit_settings_on_lifecycle, move_quad_to_input),
        )
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

fn update_winit_settings_on_lifecycle(
    mut winit_settings: ResMut<WinitSettings>,
    mut lifecycle_messages: MessageReader<AppLifecycle>,
) {
    for msg in lifecycle_messages.read() {
        match msg {
            AppLifecycle::Suspended => {
                *winit_settings = WinitSettings {
                    unfocused_mode: bevy::winit::UpdateMode::Reactive {
                        wait: Duration::from_secs(1),
                        react_to_device_events: false,
                        react_to_user_events: false,
                        react_to_window_events: false,
                    },
                    ..WinitSettings::game()
                };
            }

            AppLifecycle::Running => *winit_settings = WinitSettings::game(),

            // WillSuspend and WillResume are never called.
            // Idle might also not be called but I'm not sure.
            AppLifecycle::Idle | AppLifecycle::WillSuspend | AppLifecycle::WillResume => (),
        }
    }
}
