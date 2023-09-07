use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        prefab::components::prefab_from_url,
        primitives::components::{cube, quad},
        transform::{
            components::{lookat_target, rotation, scale, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub fn main() {
    // let main_camera_ent =
    // Entity::new()
    //     .with_merge(make_perspective_infinite_reverse_camera())
    //     .with(aspect_ratio_from_window(), EntityId::resources())
    //     .with(main_scene(), ())
    //     .with(translation(), vec3(5., 5., 5.))
    //     .with(lookat_target(), vec3(0., 0., 0.))
    //     .spawn();

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with(quad(), ())
    //     .with(scale(), Vec3::splat(100.))
    //     .spawn();
    spawn_fps_player::init();
    load_scene_and_build_ents::init();
}

mod spawn_fps_player {
    use crate::packages::fps_controller::components::{camera_distance, use_fps_controller};
    use ambient_api::{prelude::*, core::player::components::is_player};
    pub fn init() {
        spawn_query(is_player()).bind(move |players| {
            for (id, _) in players {
                entity::add_components(
                    id,
                    Entity::new()
                        .with(use_fps_controller(), ())
                        .with(camera_distance(), 0.0),
                );
            }
        });
    }
}

mod sceneloader;

mod load_scene_and_build_ents {
    use crate::sceneloader;
    use ambient_api::{
        core::{
            physics::components::cube_collider,
            prefab::components::prefab_from_url,
            primitives::components::cube,
            transform::{
                components::{rotation, scale, translation},
                concepts::make_transformable,
            },
        },
        prelude::*,
    };
    pub fn init() {
        let nodes = sceneloader::test_get_nodes();

        sceneloader::debug_nodes(&nodes);

        for (_key, node) in nodes {
            let node_pos: Option<Vec3> = node.pos;
            let node_rot: Option<Quat> = node.rot;
            let node_siz: Option<Vec3> = node.siz;
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
                        .with(cube_collider(), vec3(1., 1., 1.))
                        .with(translation(), node_pos.unwrap())
                        .with(rotation(), node_rot.unwrap())
                        .with(scale(), node_siz.unwrap())
                        .spawn();
                }
                // "camera" => {
                //     println!(
                //         "Yes: Found camera! @ pos {:?} rot {:?}",
                //         node_pos.unwrap(),
                //         node_rot.unwrap()
                //     );
                //     entity::set_component(main_camera_ent, translation(), node_pos.unwrap());
                //     entity::set_component(main_camera_ent, rotation(), node_rot.unwrap());
                // }
                _ => {
                    if let Some(path) = node.path {
                        Entity::new()
                            .with_merge(make_transformable())
                            // .with_default(cube())
                            .with(translation(), node_pos.unwrap())
                            .with(rotation(), node_rot.unwrap())
                            .with(scale(), node_siz.unwrap())
                            .with(prefab_from_url(), crate::packages::this::assets::url(&path))
                            .spawn();
                    }
                }
            }
        }
    }
}
