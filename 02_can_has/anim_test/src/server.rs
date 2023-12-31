use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        model::components::model_from_url,
        primitives::components::quad,
        transform::components::{lookat_target, translation},
    },
    prelude::*,
};

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 5.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 0.))
    .spawn();

    // Entity::new()
    //     .with(
    //         model_from_url(),
    //         packages::this::assets::url("material_chicken.glb"),
    //     )
    //     .spawn();

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();

    println!("Hello, Ambient!");
}
