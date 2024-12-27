use avian3d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, (hello_world, setup_scene, setup_camera))
        .run();
}

fn hello_world() {
    println!("hello world");
}

fn get_weapon() -> WeaponType {
    WeaponType::SigMpx
}

fn spawn_weapon(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
    let weapon = get_weapon();

    let weapon_handle =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(get_weapon_asset_location(&weapon)));

    commands
        .spawn((
            SceneRoot(weapon_handle),
            Transform::from_xyz(0.0, 0.2, 0.0).looking_at(Vec3::X, Vec3::Y),
            //MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        ))
        .id()
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    //commands.spawn((
    //    Mesh3d(meshes.add(Mesh::from(Cuboid {
    //        half_size: Vec3::new(1.0, 1.0, 1.0),
    //    }))),
    //    MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
    //    Transform::from_xyz(0.0, 0.5, 0.0),
    //));

    // plane
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d {
            half_size: Vec2::new(2.5, 2.5),
            ..Default::default()
        }))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub enum WeaponModifications {
    Suppressor,
}

pub enum WeaponType {
    SigMpx,
}

pub struct WeaponConfiguration(WeaponType, Vec<WeaponModifications>);

pub fn get_weapon_asset_location(weapon: &WeaponType) -> String {
    match weapon {
        WeaponType::SigMpx => "sig_mpx_one.glb".to_owned(),
    }
}

#[derive(Component)]
struct WeaponMarker(WeaponType);

#[derive(Event)]
pub struct SpawnWeapon(WeaponType);

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Camera3d { ..default() },
            Transform::from_xyz(-1.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            MyCameraMarker,
        ))
        .with_children(|parent| {
            let weapon = get_weapon();

            let weapon_handle = asset_server
                .load(GltfAssetLabel::Scene(0).from_asset(get_weapon_asset_location(&weapon)));

            parent.spawn((
                WeaponMarker(weapon),
                SceneRoot(weapon_handle),
                Transform::from_xyz(0.1, -0.25, -0.4).looking_to(Vec3::Z, Vec3::Y),
                //MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            ));
        });
}
