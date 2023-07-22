use std::{fmt::Display, time::Duration};

/// BPM trait to count tempo and bar time.
/// Implementor is required to be a number
/// in order to be converted to u64 to calculate all measures

pub trait BPM: Clone + Copy + Eq + Ord + Sized + Display {
    fn as_u64(&self) -> u64;

    #[inline]
    fn bar_time(&self) -> Duration {
        Duration::from_millis((self.tempo() as f64 / 250.0).round() as u64)
    }

    #[inline]
    fn tempo(&self) -> u64 {
        (60_000_000.0 / self.as_u64() as f64).round() as u64
    }
}

impl BPM for u64 {
    #[inline]
    fn as_u64(&self) -> u64 {
        *self
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

impl_bpm!(i64);
impl_bpm!(u32);
impl_bpm!(i32);
impl_bpm!(u16);
impl_bpm!(i16);
impl_bpm!(u8);
impl_bpm!(i8);
