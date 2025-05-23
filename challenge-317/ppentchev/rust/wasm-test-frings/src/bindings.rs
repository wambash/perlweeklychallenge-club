// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod ringlet {
    pub mod pwc317_t2_frings {
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod frings {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn frings(str1: &str, str2: &str) -> bool {
                unsafe {
                    let vec0 = str1;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let vec1 = str2;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "ringlet:pwc317-t2-frings/frings@0.1.0")]
                    unsafe extern "C" {
                        #[link_name = "frings"]
                        fn wit_import2(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                        ) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    unsafe extern "C" fn wit_import2(
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                    ) -> i32 {
                        unreachable!()
                    }
                    let ret = unsafe {
                        wit_import2(ptr0.cast_mut(), len0, ptr1.cast_mut(), len1)
                    };
                    _rt::bool_lift(ret as u8)
                }
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #![allow(dead_code, clippy::all)]
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[unsafe(
    link_section = "component-type:wit-bindgen:0.41.0:ringlet:pwc317-t2-test-frings@0.1.0:test-frings:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 261] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x83\x01\x01A\x02\x01\
A\x02\x01B\x02\x01@\x02\x04str1s\x04str2s\0\x7f\x04\0\x06frings\x01\0\x03\0%ring\
let:pwc317-t2-frings/frings@0.1.0\x05\0\x04\0/ringlet:pwc317-t2-test-frings/test\
-frings@0.1.0\x04\0\x0b\x11\x01\0\x0btest-frings\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.227.1\x10wit-bindgen-rust\x060.41.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
