use crate::{domain_id, RRefable, TypeIdentifiable, HEAP};
use core::alloc::Layout;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct RRef<T>
where
    T: 'static + RRefable,
{
    domain_id_pointer: *mut u64,
    pub(crate) borrow_count_pointer: *mut u64,
    pub(crate) value_pointer: *mut T,
}

unsafe impl<T: RRefable> RRefable for RRef<T> {}
unsafe impl<T: RRefable> Send for RRef<T> where T: Send {}

impl<T: RRefable> RRef<T>
where
    T: TypeIdentifiable,
{
    pub(crate) unsafe fn new_with_layout(value: T, layout: Layout) -> RRef<T> {
        let type_id = T::type_id();
        let allocation = match unsafe {
            HEAP.get()
                .expect("Shared heap not initialized")
                .alloc(layout, type_id)
        } {
            Some(allocation) => allocation,
            None => panic!("Shared heap allocation failed"),
        };
        let value_pointer = allocation.value_pointer as *mut T;
        *allocation.domain_id_pointer = domain_id();
        *allocation.borrow_count_pointer = 0;
        core::ptr::write(value_pointer, value);
        RRef {
            domain_id_pointer: allocation.domain_id_pointer,
            borrow_count_pointer: allocation.borrow_count_pointer,
            value_pointer,
        }
    }
    pub fn new(value: T) -> RRef<T> {
        let layout = Layout::new::<T>();
        unsafe { Self::new_with_layout(value, layout) }
    }
    pub fn new_aligned(value: T, align: usize) -> RRef<T> {
        let size = core::mem::size_of::<T>();
        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
        unsafe { Self::new_with_layout(value, layout) }
    }

    pub fn move_to(&self, new_domain_id: u64) {
        unsafe {
            *self.domain_id_pointer = new_domain_id;
        }
    }
}

impl<T: RRefable> Deref for RRef<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.value_pointer }
    }
}

impl<T: RRefable> DerefMut for RRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value_pointer }
    }
}

impl<T: RRefable> Drop for RRef<T> {
    fn drop(&mut self) {
        unsafe { HEAP.get().unwrap().dealloc(self.value_pointer as _) }
    }
}