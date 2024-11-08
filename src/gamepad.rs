use bevy::{input::gamepad::{AxisSettings, ButtonSettings, GamepadConnection, GamepadEvent, GamepadSettings}, prelude::*};

pub struct MyGamepadPlugin;

impl Plugin for MyGamepadPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            configure_gamepads.run_if(resource_exists_and_changed::<MyGamepad>)
        ).add_systems(Update, gamepad_connections);
    }
}

/// Simple resource to store the ID of the first connected gamepad.
/// We can use it to know which gamepad to use for player input.
#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

fn configure_gamepads(
    my_gamepad: Option<Res<MyGamepad>>,
    mut settings: ResMut<GamepadSettings>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    // add a larger default dead-zone to all axes (ignore small inputs, round to zero)
    settings.default_axis_settings.set_deadzone_lowerbound(-0.1);
    settings.default_axis_settings.set_deadzone_upperbound(0.1);

    // make the right stick "binary", squash higher values to 1.0 and lower values to 0.0
    let mut right_stick_settings = AxisSettings::default();
    right_stick_settings.set_deadzone_lowerbound(-0.5);
    right_stick_settings.set_deadzone_upperbound(0.5);
    right_stick_settings.set_livezone_lowerbound(-0.5);
    right_stick_settings.set_livezone_upperbound(0.5);
    // the raw value should change by at least this much,
    // for Bevy to register an input event:
    right_stick_settings.set_threshold(0.01);

    // make the triggers work in big/coarse steps, to get fewer events
    // reduces noise and precision
    let mut trigger_settings = AxisSettings::default();
    trigger_settings.set_threshold(0.25);

    // set these settings for the gamepad we use for our player
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickX },
        right_stick_settings.clone()
    );
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickY },
        right_stick_settings.clone()
    );
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftZ },
        trigger_settings.clone()
    );
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::RightZ },
        trigger_settings.clone()
    );

    // for buttons (or axes treated as buttons):
    let mut button_settings = ButtonSettings::default();
    // require them to be pressed almost all the way, to count
    button_settings.set_press_threshold(0.9);
    // require them to be released almost all the way, to count
    button_settings.set_release_threshold(0.1);

    settings.default_button_settings = button_settings;
}

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}
