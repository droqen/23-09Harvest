use ambient_api::prelude::*;

#[main]
pub fn main() {
    spawn_big_ground_quad();
    spawn_fog_enabled_camera();
    spawn_sun_and_sky_with_fog();
}

pub fn spawn_big_ground_quad() {
    use ambient_api::core::{
        primitives::components::quad,
        transform::{components::scale, concepts::make_transformable},
    };
    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .with(scale(), vec3(100., 100., 100.))
        .spawn();
}

pub fn spawn_fog_enabled_camera() {
    use ambient_api::core::{
        app::components::main_scene,
        camera::{
            components::{aspect_ratio_from_window, fog},
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    };
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(fog(), ())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    println!("Hello, Ambient!");
}

pub fn spawn_sun_and_sky_with_fog() {
    use ambient_api::core::{
        app::components::main_scene,
        rendering::components::{
            fog_color, fog_density, fog_height_falloff, light_diffuse, sky, sun,
        },
        transform::{components::rotation, concepts::make_transformable},
    };
    Entity::new()
        .with_merge(make_transformable())
        .with(sun(), 0.0)
        .with(rotation(), Quat::from_rotation_y(-1.))
        .with(main_scene(), ())
        .with(light_diffuse(), Vec3::ONE)
        .with(fog_color(), vec3(1., 1., 1.))
        .with(fog_density(), 0.1)
        .with(fog_height_falloff(), 0.01)
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(sky(), ())
        .spawn();
}
