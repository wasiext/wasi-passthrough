use crate::bindings::exports;
use crate::bindings::wasi::clocks::monotonic_clock;
use crate::bindings::wasi::clocks::monotonic_clock::Duration;
use crate::bindings::wasi::clocks::monotonic_clock::Instant;

impl exports::wasi::clocks::monotonic_clock::Guest for () {
    fn now() -> Instant {
        monotonic_clock::now()
    }

    fn resolution() -> Duration {
        monotonic_clock::resolution()
    }

    fn subscribe_instant(when: Instant) -> exports::wasi::io::poll::Pollable {
        monotonic_clock::subscribe_instant(when).into()
    }

    fn subscribe_duration(when: Duration) -> exports::wasi::io::poll::Pollable {
        monotonic_clock::subscribe_duration(when).into()
    }
}
