//! VRAM memory allocator.

use std::alloc::{AllocError, Allocator, Layout};
use std::ptr::NonNull;

// Implementing an `std::alloc::Allocator` type is the best way to handle this case, since it gives
// us full control over the normal `std` implementations (like `Box`). The only issue is that this is another unstable feature to add.
// Sadly the vram memory allocator included in `libctru` doesn't implement `vramRealloc` at the time of these additions,
// but the default fallback of the `std` will take care of that for us.

/// [`Allocator`] struct for VRAM memory.
///
/// To use this struct the main crate must activate the `allocator_api` unstable feature.
#[derive(Copy, Clone, Default, Debug)]
pub struct VramAllocator;

impl VramAllocator {
    /// Returns the amount of free space left in the LINEAR memory sector.
    #[doc(alias = "vramSpaceFree")]
    pub fn free_space() -> u32 {
        unsafe { ctru_sys::vramSpaceFree() }
    }
}

unsafe impl Allocator for VramAllocator {
    #[doc(alias = "vramAlloc", alias = "vramMemAlign")]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let pointer = unsafe { ctru_sys::vramMemAlign(layout.size(), layout.align()) };

        NonNull::new(pointer.cast())
            .map(|ptr| NonNull::slice_from_raw_parts(ptr, layout.size()))
            .ok_or(AllocError)
    }

    #[doc(alias = "vramFree")]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        unsafe {
            ctru_sys::vramFree(ptr.as_ptr().cast());
        }
    }
}
