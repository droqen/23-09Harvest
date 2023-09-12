use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .spawn();

    println!("Hello, Ambient!");

    add_models();
}

pub fn add_models() {
    use ambient_api::core::model::components::model_from_url;
    Entity::new()
        .with_merge(make_transformable())
        .with(
            model_from_url(),
            packages::this::assets::url("really_tall_lumpy_wall.glb"),
        )
        .spawn();
}
