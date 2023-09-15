use ambient_api::{core::transform::components::translation, prelude::*};

use packages::a_foggy_scene::components::coldness;

#[main]
pub fn main() {
    query((coldness(), translation())).each_frame(|coldplayers| {
        dbg!(&coldplayers);
        for i in 0..coldplayers.len() - 1 {
            for j in i + 1..coldplayers.len() {
                // for each pair of coldplayers...
                let ipos: Vec3 = coldplayers[i].1 .1;
                let jpos: Vec3 = coldplayers[j].1 .1;
                if ipos.distance_squared(jpos) < 5. * 5. {
                    // make us warmer
                    entity::mutate_component(coldplayers[i].0, coldness(), |cold| {
                        *cold = (*cold - 0.04 * delta_time()).max(0.);
                    });
                    entity::mutate_component(coldplayers[j].0, coldness(), |cold| {
                        *cold = (*cold - 0.04 * delta_time()).max(0.);
                    });
                }
            }
        }
    });
}
