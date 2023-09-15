use ambient_api::prelude::*;

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
    let my_camera = PerspectiveInfiniteReverseCamera {
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

    spawn_query(is_space_sound_emitter()).bind(move |emitters| {
        for (e, _) in emitters {
            let spatial_audio_player = audio::SpatialAudioPlayer::new();
            spatial_audio_player.set_amplitude(6.0);
            spatial_audio_player.set_looping(true);
            spatial_audio_player.set_listener(my_camera);
            spatial_audio_player.play_sound_on_entity(
                packages::this::assets::url("4211__dobroide__firecrackling.ogg"),
                e,
            );
        }
    });
}
