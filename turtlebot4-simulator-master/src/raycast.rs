use bevy::prelude::*;
// use bevy_mod_picking::{PickableBundle, Raycast};

#[derive(Component)]
struct RaycastLine {
    pub start: Vec3,
    pub end: Vec3,
    pub color: Color,
}

fn cast_ray_system(
    mut query: Query<(&mut Transform, &mut RaycastLine)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut direction = Vec3::X; 

    if keyboard_input.pressed(KeyCode::Right) {
        direction = Vec3::new(direction.x.cos() - direction.y.sin(), 
                             direction.x.sin() + direction.y.cos(), 
                             0.0); 
    }

    if keyboard_input.pressed(KeyCode::Left) {
        direction = Vec3::new(direction.x.cos() + direction.y.sin(), 
                             direction.x.sin() - direction.y.cos(), 
                             0.0); 
    }

    let origin = Vec3::ZERO; 
    let length = 5.0; 

    for (mut transform, mut line) in query.iter_mut() {
        line.end = origin + direction * length;

        // Update the line's visual representation
        transform.scale = Vec3::new((line.end - line.start).length(), 1.0, 1.0);
        transform.rotation = Quat::from_rotation_axis(Vec3::Z, (line.end - line.start).normalize().cross(Vec3::Z).angle_between(Vec3::X));
    }
}

fn setup_raycast_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Create a line mesh
    let line = Line::new(Vec2::ZERO, Vec2::X * 5.0); 
    let mesh = meshes.add(line.into()); 

    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0), 
        ..default()
    })
    .insert(RaycastLine { 
        start: Vec3::ZERO, 
        end: Vec3::X * 5.0, 
        color: Color::RED 
    });
}

// sontinue------->copy everything to new files and workspace