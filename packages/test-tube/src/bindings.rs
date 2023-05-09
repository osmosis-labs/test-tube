#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GoString_ {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}
#[test]
fn bindgen_test_layout__GoString_() {
    assert_eq!(
        ::std::mem::size_of::<_GoString_>(),
        16usize,
        concat!("Size of: ", stringify!(_GoString_))
    );
    assert_eq!(
        ::std::mem::align_of::<_GoString_>(),
        8usize,
        concat!("Alignment of ", stringify!(_GoString_))
    );
    fn test_field_p() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_GoString_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).p) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_GoString_),
                "::",
                stringify!(p)
            )
        );
    }
    test_field_p();
    fn test_field_n() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_GoString_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_GoString_),
                "::",
                stringify!(n)
            )
        );
    }
    test_field_n();
}
pub type GoInt8 = ::std::os::raw::c_schar;
pub type GoUint8 = ::std::os::raw::c_uchar;
pub type GoInt16 = ::std::os::raw::c_short;
pub type GoUint16 = ::std::os::raw::c_ushort;
pub type GoInt32 = ::std::os::raw::c_int;
pub type GoUint32 = ::std::os::raw::c_uint;
pub type GoInt64 = ::std::os::raw::c_longlong;
pub type GoUint64 = ::std::os::raw::c_ulonglong;
pub type GoInt = GoInt64;
pub type GoUint = GoUint64;
pub type GoUintptr = size_t;
pub type GoFloat32 = f32;
pub type GoFloat64 = f64;
pub type GoComplex64 = __BindgenComplex<f32>;
pub type GoComplex128 = __BindgenComplex<f64>;
pub type _check_for_64_bit_pointer_matching_GoInt = [::std::os::raw::c_char; 1usize];
pub type GoString = _GoString_;
pub type GoMap = *mut ::std::os::raw::c_void;
pub type GoChan = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoInterface {
    pub t: *mut ::std::os::raw::c_void,
    pub v: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_GoInterface() {
    assert_eq!(
        ::std::mem::size_of::<GoInterface>(),
        16usize,
        concat!("Size of: ", stringify!(GoInterface))
    );
    assert_eq!(
        ::std::mem::align_of::<GoInterface>(),
        8usize,
        concat!("Alignment of ", stringify!(GoInterface))
    );
    fn test_field_t() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<GoInterface>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).t) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(GoInterface),
                "::",
                stringify!(t)
            )
        );
    }
    test_field_t();
    fn test_field_v() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<GoInterface>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(GoInterface),
                "::",
                stringify!(v)
            )
        );
    }
    test_field_v();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoSlice {
    pub data: *mut ::std::os::raw::c_void,
    pub len: GoInt,
    pub cap: GoInt,
}
#[test]
fn bindgen_test_layout_GoSlice() {
    assert_eq!(
        ::std::mem::size_of::<GoSlice>(),
        24usize,
        concat!("Size of: ", stringify!(GoSlice))
    );
    assert_eq!(
        ::std::mem::align_of::<GoSlice>(),
        8usize,
        concat!("Alignment of ", stringify!(GoSlice))
    );
    fn test_field_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<GoSlice>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(GoSlice),
                "::",
                stringify!(data)
            )
        );
    }
    test_field_data();
    fn test_field_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<GoSlice>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(GoSlice),
                "::",
                stringify!(len)
            )
        );
    }
    test_field_len();
    fn test_field_cap() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<GoSlice>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cap) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(GoSlice),
                "::",
                stringify!(cap)
            )
        );
    }
    test_field_cap();
}
extern "C" {
    pub fn InitTestEnv() -> GoUint64;
}
extern "C" {
    pub fn InitAccount(envId: GoUint64, coinsJson: GoString) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn BeginBlock(envId: GoUint64);
}
extern "C" {
    pub fn EndBlock(envId: GoUint64);
}
extern "C" {
    pub fn IncreaseTime(envId: GoUint64, seconds: GoInt64);
}
extern "C" {
    pub fn Execute(envId: GoUint64, base64ReqDeliverTx: GoString) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Query(
        envId: GoUint64,
        path: GoString,
        base64QueryMsgBytes: GoString,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn AccountSequence(envId: GoUint64, bech32Address: GoString) -> GoUint64;
}
extern "C" {
    pub fn AccountNumber(envId: GoUint64, bech32Address: GoString) -> GoUint64;
}
extern "C" {
    pub fn Simulate(envId: GoUint64, base64TxBytes: GoString) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetParamSet(
        envId: GoUint64,
        subspaceName: GoString,
        base64ParamSetBytes: GoString,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetParamSet(
        envId: GoUint64,
        subspaceName: GoString,
        typeUrl: GoString,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetValidatorAddress(envId: GoUint64, n: GoInt32) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetValidatorPrivateKey(envId: GoUint64) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetBlockTime(envId: GoUint64) -> GoInt64;
}
extern "C" {
    pub fn GetBlockHeight(envId: GoUint64) -> GoInt64;
}
