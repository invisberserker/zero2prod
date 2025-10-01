// using rand_core 0.6.4
use argon2::password_hash::rand_core::{
    CryptoRng as CryptoRng_v0_6, Error as Error_v0_6, RngCore as RngCore_v0_6,
};
use rand::RngCore as RngCore_v0_9;
// using rand_core 0.9.3
use rand_core::TryRngCore as TryRngCore_v0_9;

/// An adapter to make a modern `rand` v0.9+ RNG compatible with
/// the `rand_core` v0.6 traits required by `password-hash`.
pub struct RngAdapter<R>(pub R);

// Implement the old CryptoRng marker trait for our adapter.
impl<R> CryptoRng_v0_6 for RngAdapter<R> where R: RngCore_v0_9 + TryRngCore_v0_9 {}

// Implement the old RngCore trait for our adapter.
impl<R> RngCore_v0_6 for RngAdapter<R>
where
    R: RngCore_v0_9 + TryRngCore_v0_9,
{
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error_v0_6> {
        match self.0.try_fill_bytes(dest) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error_v0_6::new(e.to_string())),
        }
    }
}
