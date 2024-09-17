use crate::bindings::exports;
use crate::bindings::wasi::random::{insecure, insecure_seed, random};

impl exports::wasi::random::random::Guest for () {
    fn get_random_bytes(len: u64) -> Vec<u8> {
        random::get_random_bytes(len)
    }

    fn get_random_u64() -> u64 {
        random::get_random_u64()
    }
}

impl exports::wasi::random::insecure::Guest for () {
    fn get_insecure_random_bytes(len: u64) -> Vec<u8> {
        insecure::get_insecure_random_bytes(len)
    }

    fn get_insecure_random_u64() -> u64 {
        insecure::get_insecure_random_u64()
    }
}

impl exports::wasi::random::insecure_seed::Guest for () {
    fn insecure_seed() -> (u64, u64) {
        insecure_seed::insecure_seed()
    }
}
