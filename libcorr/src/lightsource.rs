use derive_new::new;
use getset::{Getters, MutGetters};

use crate::vector::Vector4F;

#[derive(Getters, MutGetters, new)]
pub struct LightSource {
    #[getset(get = "pub", get_mut = "pub")]
    direction: Vector4F,
    #[getset(get = "pub", get_mut = "pub")]
    color: [u32; 4],
}
