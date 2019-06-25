use core::sync::atomic;

#[doc(hidden)]
#[allow(dead_code)]
pub unsafe fn unsafe_marker() {
    // This marker is used to make the macro unsafe
    atomic::compiler_fence(atomic::Ordering::Relaxed);
}

macro_rules! assume_unreachable {
    () => {
        if cfg!(debug_assertions) {
            unreachable!()
        } else {
            crate::macros::unsafe_marker();
            unreachable!()
            // core::hint::unreachable_unchecked()
        }
    };
}
