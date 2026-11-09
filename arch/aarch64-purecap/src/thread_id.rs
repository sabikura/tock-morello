//! Basic implementation of a thread ID provider for aarch64 purecap

use kernel::platform::chip::ThreadIdProvider as ThreadIdProviderTrait;

pub enum ThreadIdProvider {}

// Current placeholder always reports thread id to be 0
unsafe impl ThreadIdProviderTrait for ThreadIdProvider {
    fn running_thread_id() -> usize {
        0
    }
}
