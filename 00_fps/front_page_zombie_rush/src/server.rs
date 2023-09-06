use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        physics::components::plane_collider,
        prefab::components::prefab_from_url,
        primitives::components::{cube, quad},
        transform::{
            components::{lookat_target, rotation, scale, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

mod sceneloader;

#[main]
pub fn main() {
    let main_camera_ent = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), vec3(9.5, -65.0, 18.0))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with(quad(), ())
    //     .with(scale(), Vec3::splat(100.))
    //     .spawn();
    Entity::new()
        .with(quad(), ())
        .with(plane_collider(), ())
        .with(scale(), Vec3::ONE * 1000.)
        .spawn();

    let nodes = sceneloader::test_get_nodes();

    sceneloader::debug_nodes(&nodes);

    for (_key, node) in nodes {
        let node_pos: Option<Vec3> = node.pos;
        let node_rot: Option<Quat> = node.rot;
        match node.name.as_str() {
            "cube1" | "cube2" => {
                println!(
                    "Spawn one cube @ pos {:?} rot {:?}",
                    node_pos.unwrap(),
                    node_rot.unwrap()
                );
                Entity::new()
                    .with_merge(make_transformable())
                    .with(cube(), ())
                    .with(translation(), node_pos.unwrap())
                    .with(rotation(), node_rot.unwrap())
                    .spawn();
            }
            "camera" => {
                println!(
                    "Yes: Found camera! @ pos {:?} rot {:?}",
                    node_pos.unwrap(),
                    node_rot.unwrap()
                );
                entity::set_component(main_camera_ent, translation(), node_pos.unwrap());
                entity::set_component(main_camera_ent, rotation(), node_rot.unwrap());
            }
            _ => {
                if let Some(path) = node.path {
                    // spawn no buildings

                    // Entity::new()
                    //     .with_merge(make_transformable())
                    //     // .with_default(cube())
                    //     .with(translation(), node_pos.unwrap())
                    //     .with(rotation(), node_rot.unwrap())
                    //     .with(prefab_from_url(), packages::this::assets::url(&path))
                    //     .spawn();
                }
            }
        }
    }
}
