use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy_mod_picking::PickingCameraBundle;

pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

// change input mapping for orbit and panning here
const ORBIT_BUTTON: MouseButton = MouseButton::Middle;
const PAN_BUTTON: MouseButton = MouseButton::Right;

pub fn pan_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
) {
    if !input_mouse.pressed(PAN_BUTTON) {
        return;
    }
    let mut pan = Vec2::ZERO;
    for ev in ev_motion.iter() {
        pan += ev.delta;
    }
    if pan.length_squared() <= 0.0 {
        return;
    }
    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        // make panning distance independent of resolution and FOV,
        let window = get_primary_window_size(&windows);
        pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
        // translate by local axes
        let right = transform.rotation * Vec3::X * -pan.x;
        let up = transform.rotation * Vec3::Y * pan.y;
        // make panning proportional to distance away from focus point
        let translation = (right + up) * pan_orbit.radius;
        pan_orbit.focus += translation;

        transform.translation =
            get_translation(transform.rotation, pan_orbit.focus, pan_orbit.radius);
    }
}

pub fn orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if !input_mouse.pressed(ORBIT_BUTTON) {
        return;
    }
    let mut rotation_move = Vec2::ZERO;
    for ev in ev_motion.iter() {
        rotation_move += ev.delta;
    }

    for (mut pan_orbit, mut transform) in query.iter_mut() {
        if input_mouse.just_pressed(ORBIT_BUTTON) || input_mouse.just_released(ORBIT_BUTTON) {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        if rotation_move.length_squared() <= 0.0 {
            return;
        }
        let window = get_primary_window_size(&windows);
        let delta_x = {
            let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
            if pan_orbit.upside_down {
                -delta
            } else {
                delta
            }
        };
        let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        transform.rotation = yaw * transform.rotation; // rotate around global y axis
        transform.rotation *= pitch; // rotate around local x axis

        transform.translation =
            get_translation(transform.rotation, pan_orbit.focus, pan_orbit.radius);
    }
}

pub fn scroll_camera(
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let mut scroll = 0.0;
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    if scroll.abs() <= 0.0 {
        return;
    }
    for (mut pan_orbit, mut transform) in query.iter_mut() {
        pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
        // dont allow zoom to reach zero or you get stuck
        pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);

        transform.translation =
            get_translation(transform.rotation, pan_orbit.focus, pan_orbit.radius);
    }
}

fn get_translation(rotation: Quat, focus: Vec3, radius: f32) -> Vec3 {
    // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
    // parent = x and y rotation
    // child = z-offset
    let rot_matrix = Mat3::from_quat(rotation);

    focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, radius))
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}

pub fn spawn_camera(commands: &mut Commands, position: Vec3, look_at: Vec3) {
    let radius = position.length();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(position).looking_at(look_at, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());
}
