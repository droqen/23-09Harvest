use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::quad,
        text::components::text,
        transform::{
            components::{lookat_target, translation},
            concepts::{Transformable, TransformableOptional},
        },
    },
    prelude::*,
};

use packages::orbit_camera::{
    components::camera_angle,
    concepts::{OrbitCamera, OrbitCameraOptional},
};

#[main]
pub fn main() {
    OrbitCamera {
        is_orbit_camera: (),
        optional: OrbitCameraOptional {
            camera_angle: Some(vec2(0., 0.5)),
            camera_distance: Some(10.0),
            lookat_target: Some(Vec3::ZERO),
        },
    }
    .make()
    .spawn();
    query(camera_angle()).each_frame(|cams| {
        for (cam, angle) in cams {
            entity::mutate_component(cam, camera_angle(), |ang| ang.x = ang.x + delta_time());
        }
    });

    Entity::new()
        .with_merge(
            Transformable {
                local_to_world: Mat4::IDENTITY,
                optional: TransformableOptional {
                    translation: None,
                    rotation: None,
                    scale: None,
                },
            }
            .make(),
        )
        .with(text(), "Hello".into())
        .spawn();

    // PerspectiveInfiniteReverseCamera {
    //     optional: PerspectiveInfiniteReverseCameraOptional {
    //         aspect_ratio_from_window: Some(entity::resources()),
    //         main_scene: Some(()),
    //         translation: Some(Vec3::ONE * 5.),
    //         ..default()
    //     },
    //     ..PerspectiveInfiniteReverseCamera::suggested()
    // }
    // .make()
    // .with(lookat_target(), vec3(0., 0., 0.))
    // .spawn();

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();

    println!("Hello, Ambient!");
}
