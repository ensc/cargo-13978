#[no_mangle]
extern "C" fn foo_func() -> u32 { func() }

pub fn func() -> u32 {
    23
}
