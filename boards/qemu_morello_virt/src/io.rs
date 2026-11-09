use core::panic::PanicInfo;

/// Panic handler
#[cfg(not(test))]
#[panic_handler]
pub unsafe fn panic_fmt(_pi: &PanicInfo) -> ! {
    loop {}
}
