#[no_mangle]
extern "C" fn bar_func() -> u32 { func() }

fn func() -> u32 {
    foo::func() + 42
}
