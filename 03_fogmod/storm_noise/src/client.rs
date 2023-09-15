use ambient_api::{core::audio, prelude::*};

use ambient_api::audio::AudioPlayer;

use packages::a_foggy_scene::components::coldness;

#[main]
pub fn main() {
    let storm_sound_player = AudioPlayer::new();
    let play_instance =
        storm_sound_player.play(packages::this::assets::url("snowstorm_ambience.ogg"));
    // dbg!(&play_instance);
    // ambient_api::core::messages::Frame::subscribe(move |_| {
    //     let cold: f32 = entity::get_component(player::get_local(), coldness()).unwrap_or(0.0);
    //     storm_sound_player.set_amplitude((0.2 + 1.8 * cold).min(1.));
    // });
}
