use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology}; 
use bevy_render::render_asset::RenderAssetUsages; 
use bevy_mod_raycast::prelude::*; 

#[derive(Resource)]
struct DistanceResource {
    distance: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()),
            DeferredRaycastingPlugin::<()>::default(),
        ))
        .insert_resource(RaycastPluginState::<()>::default().with_debug_cursor())
        .insert_resource(DistanceResource { distance: 0.0 })
        .add_systems(Startup, setup)
        .add_systems(Update, (calculate_distance, move_camera)) // Systems to update distance and control camera
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a camera with a RaycastSource using the cursor
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0), // Start camera 5 units away
            ..default()
        },
        RaycastSource::<()>::new_cursor(),
    ));
    println!("Camera with RaycastSource added.");

    // Create a simple cube
    let vertices = vec![
        ([0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),
        ([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),
        ([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
        ([0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
        ([0.5, 0.5, -0.5], [0.0, 0.0, -1.0]),
        ([-0.5, 0.5, -0.5], [0.0, 0.0, -1.0]),
        ([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
        ([0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
    ];
    let indices = vec![
        0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 1, 5, 6, 6, 2, 1, 0, 4, 7, 7, 3, 0, 0, 1, 5, 5, 4, 0, 3, 2, 6, 6, 7, 3,
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::empty());
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.iter().map(|(pos, _)| *pos).collect::<Vec<[f32; 3]>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vertices.iter().map(|(_, norm)| *norm).collect::<Vec<[f32; 3]>>(),
    );
    mesh.insert_indices(Indices::U32(indices));

    // Spawn the cube in the scene
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, -5.0), // Place cube at -5 units on z-axis
            ..default()
        },
        RaycastMesh::<()>::default(),
    ));
    println!("Cube added to the scene at position (0.0, 0.0, -5.0).");
}

fn calculate_distance(
    mut raycast: Raycast,
    camera_query: Query<&Transform, With<Camera3d>>, // Query the camera transform
    mut distance_res: ResMut<DistanceResource>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        // Create a Ray3d from the camera's position and forward direction
        let ray_origin = camera_transform.translation;
        let ray_direction = *camera_transform.forward(); // Dereference to get Vec3
        let ray = Ray3d::new(ray_origin, ray_direction);

        // Check for intersections along this ray
        if let Some((_, intersection)) = raycast.cast_ray(ray, &RaycastSettings::default()).first() {
            let distance = ray_origin.distance(intersection.position());
            distance_res.distance = distance;
            println!("Distance to object: {:.2} units", distance);
        } else {
            distance_res.distance = 0.0;
            println!("No intersection detected.");
        }
    }
}
fn move_camera(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Camera3d>>) {
    for mut transform in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.z -= 0.1; // Move camera closer
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.z += 0.1; // Move camera further away
        }
    }
}
