use bevy::{gltf::GltfPrimitive, prelude::*};

fn main() {
    App::new()
        .init_resource::<PrimitiveHandle>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, check_load_status)
        .run();
}

#[derive(Resource, Default)]
pub struct PrimitiveHandle(Option<Handle<GltfPrimitive>>);

/// check load status of asset at different stages.
pub fn check_load_status(asset_server: Res<AssetServer>, mut primitive: ResMut<PrimitiveHandle>) {
    if let Some(handle) = &primitive.0 {
        let path = handle.path().unwrap();
        match asset_server.load_state(handle) {
            bevy::asset::LoadState::NotLoaded => println!("not loaded {:#}", path),
            // perpetually stuck laoding here, but no error from the asset loader about being an invalid asset?
            bevy::asset::LoadState::Loading => {
                println!("loading {:#?}", path)
            }
            bevy::asset::LoadState::Loaded => {
                println!("loaded {:#}", path);
                primitive.0 = None;
            }
            bevy::asset::LoadState::Failed(asset_load_error) => {
                println!("failed to load {:#}. REASON: {:#}", path, asset_load_error)
            }
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut primitive: ResMut<PrimitiveHandle>,
    asset_server: Res<AssetServer>,
) {
    // example primitive.
    primitive.0 = Some(asset_server.load("cube.glb#Mesh0/Primitive0"));

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
