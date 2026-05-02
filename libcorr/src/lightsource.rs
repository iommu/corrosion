use derive_new::new;
use getset::{Getters, MutGetters};

use crate::vector::Vector4F;

#[derive(Getters, MutGetters, new)]
pub struct LightSource {
    #[getset(get = "pub", get_mut = "pub")]
    direction: Vector4F,
    #[getset(get = "pub", get_mut = "pub")]
    color: [u8; 4],
    intensity: u8,
}

impl LightSource {
    pub fn intensity(&self) -> u8 {
        self.intensity
    }

    pub fn intensity_mut(&mut self) -> &mut u8 {
        &mut self.intensity
    }
}
