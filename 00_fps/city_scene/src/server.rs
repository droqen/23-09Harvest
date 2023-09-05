use ambient_api::{
    core::{
        app::components::{main_scene, name},
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        model::components::model_from_url,
        prefab::components::prefab_from_url,
        primitives::components::quad,
        rendering::components::pbr_material_from_url,
        transform::{
            components::{lookat_target, rotation, scale, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use std::f32::consts::PI;

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), vec3(5., 5., 1.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .with(scale(), Vec3::splat(10.))
        .spawn();

    Entity::new()
        .with(name(), "Water Tower Test".into())
        .with_merge(make_transformable())
        .with(
            model_from_url(),
            packages::this::assets::url("Offworld_Western_Water_tower_01_repack.fbx".into()),
        )
        // doesn't work - removed from assets
        // .with(
        //     pbr_material_from_url(),
        //     packages::this::assets::url("Offworld_Western_Water_tower_BaseColor.png".into()),
        // )
        .with(translation(), vec3(2., 2., 1.00))
        .with(rotation(), Quat::from_rotation_z(0.1))
        .spawn();

    println!("Hello, Ambient!");
}
