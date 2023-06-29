use std::fmt::Display;
use std::time::Duration;

pub trait BPM: Clone + Copy + Eq + Ord + Sized + Display {
    fn as_u64(&self) -> u64;

    #[inline]
    fn get_bar_time(&self) -> Duration {
        Duration::from_millis(240000 / self.as_u64())
    }

    #[inline]
    fn get_tempo(&self) -> u64 {
        60000000 / self.as_u64()
    }
}

macro_rules! impl_bpm {
    ($num_type:ty) => {
        impl BPM for $num_type {
            #[inline]
            fn as_u64(&self) -> u64 {
                *self as u64
            }
        }
    };
}

impl BPM for u64 {
    #[inline]
    fn as_u64(&self) -> u64 {
        *self
    }
}

impl_bpm!(i64);
impl_bpm!(u32);
impl_bpm!(i32);
impl_bpm!(u16);
impl_bpm!(i16);
impl_bpm!(u8);
impl_bpm!(i8);
