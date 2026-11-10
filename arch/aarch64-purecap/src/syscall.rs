//! Kernel-userland system call interface for Morello architecture.

/// Implementation of the `UserspaceKernelBoundary` for the Morello architecture.
pub struct SysCall(());

type Aarch64PurecapRegister = *mut u8;

impl SysCall {
    pub const unsafe fn new() -> SysCall {
        SysCall(())
    }
}

#[repr(C)]
pub struct Aarch64PurecapStoredState {
    regs: [Aarch64PurecapRegister; 31],
    pcc: Aarch64PurecapRegister,
    csp: Aarch64PurecapRegister,
}

impl Default for Aarch64PurecapStoredState {
    fn default() -> Self {
        Self {
            // FIXME: Impl default for Aarch64PurecapRegister (make it a transp struct instead of a
            // type alias mby)
            regs: [const { core::ptr::null_mut() }; 31],
            pcc: core::ptr::null_mut(),
            csp: core::ptr::null_mut(),
        }
    }
}

impl kernel::syscall::UserspaceKernelBoundary for SysCall {
    type StoredState = Aarch64PurecapStoredState;

    fn initial_process_app_brk_size(&self) -> usize {
        // For the first context switch to the process on this arch,
        // memory allocation should not be needed
        0
    }

    unsafe fn initialize_process(
        &self,
        accessible_memory_start: *const u8,
        app_brk: *const u8,
        state: &mut Self::StoredState,
    ) -> Result<(), ()> {
        // FIXME: Implement this
        state
            .regs
            .iter_mut()
            .for_each(|r| *r = core::ptr::null_mut());
        state.pcc = core::ptr::null_mut();
        state.csp = accessible_memory_start as *mut u8;

        Ok(())
    }

    unsafe fn set_syscall_return_value(
        &self,
        accessible_memory_start: *const u8,
        app_brk: *const u8,
        state: &mut Self::StoredState,
        return_value: kernel::syscall::SyscallReturn,
    ) -> Result<(), ()> {
        // FIXME: Implement this
        Ok(())
    }

    unsafe fn set_process_function(
        &self,
        accessible_memory_start: *const u8,
        app_brk: *const u8,
        state: &mut Self::StoredState,
        upcall: kernel::process::FunctionCall,
    ) -> Result<(), ()> {
        // FIXME: Implement this
        Ok(())
    }

    unsafe fn switch_to_process(
        &self,
        accessible_memory_start: *const u8,
        app_brk: *const u8,
        state: &mut Self::StoredState,
    ) -> (kernel::syscall::ContextSwitchReason, Option<*const u8>) {
        // FIXME: Implement this
        todo!()
    }

    unsafe fn print_context(
        &self,
        accessible_memory_start: *const u8,
        app_brk: *const u8,
        state: &Self::StoredState,
        writer: &mut dyn core::fmt::Write,
    ) {
        // FIXME: Implement this
        todo!()
    }

    fn store_context(
        &self,
        state: &Self::StoredState,
        out: &mut [u8],
    ) -> Result<usize, kernel::ErrorCode> {
        // FIXME: Implement this
        todo!()
    }
}
