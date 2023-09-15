use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::quad,
        transform::components::{lookat_target, translation},
    },
    prelude::*,
};

use packages::this::components::is_space_sound_emitter;

#[main]
pub fn main() {
    Entity::new()
        .with(translation(), vec3(0., 5., 0.))
        .with(quad(), ())
        .with(is_space_sound_emitter(), ())
        .spawn();

    query(is_space_sound_emitter()).each_frame(|emitters| {
        for (e, _) in emitters {
            entity::set_component(
                e,
                translation(),
                vec3(1., -1., 0.) * game_time().as_secs_f32().sin() * 5.,
            );
        }
    });
}
