use crate::FnPtr;
use force_send_sync::Sync;
pub mod _0;
pub mod _1;
lazy_static::lazy_static! {
    pub static ref FUNCTIONS: Sync<*const FnPtr> = unsafe {Sync::new({
        use std::alloc::Layout;
        use std::mem::{size_of, align_of};
        
        let l = Layout::from_size_align(size_of::<FnPtr>() * u8::max_value() as usize, align_of::<FnPtr>()).unwrap();
        let m = std::alloc::alloc_zeroed(l) as *mut FnPtr;

        *m.add(0) = &_0::part1 as FnPtr;

        *m.add(((1 as u8) << 3 | 0) as usize) = &_1::part1 as FnPtr;
        *m.add(((1 as u8) << 3 | 1) as usize) = &_1::part2 as FnPtr;

        m
    })};
}