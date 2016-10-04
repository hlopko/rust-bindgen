/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[derive(Debug)]
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
pub const JSVAL_ALIGNMENT: ::std::os::raw::c_uint = 8;
pub const JSVAL_TAG_SHIFT: ::std::os::raw::c_uint = 47;
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSValueType {
    JSVAL_TYPE_DOUBLE = 0,
    JSVAL_TYPE_INT32 = 1,
    JSVAL_TYPE_UNDEFINED = 2,
    JSVAL_TYPE_BOOLEAN = 3,
    JSVAL_TYPE_MAGIC = 4,
    JSVAL_TYPE_STRING = 5,
    JSVAL_TYPE_SYMBOL = 6,
    JSVAL_TYPE_NULL = 7,
    JSVAL_TYPE_OBJECT = 8,
    JSVAL_TYPE_UNKNOWN = 32,
    JSVAL_TYPE_MISSING = 33,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSValueTag {
    JSVAL_TAG_MAX_DOUBLE = 131056,
    JSVAL_TAG_INT32 = 131057,
    JSVAL_TAG_UNDEFINED = 131058,
    JSVAL_TAG_STRING = 131061,
    JSVAL_TAG_SYMBOL = 131062,
    JSVAL_TAG_BOOLEAN = 131059,
    JSVAL_TAG_MAGIC = 131060,
    JSVAL_TAG_NULL = 131063,
    JSVAL_TAG_OBJECT = 131064,
}
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSValueShiftedTag {
    JSVAL_SHIFTED_TAG_MAX_DOUBLE = 18444492278190833663,
    JSVAL_SHIFTED_TAG_INT32 = 18444633011384221696,
    JSVAL_SHIFTED_TAG_UNDEFINED = 18444773748872577024,
    JSVAL_SHIFTED_TAG_STRING = 18445195961337643008,
    JSVAL_SHIFTED_TAG_SYMBOL = 18445336698825998336,
    JSVAL_SHIFTED_TAG_BOOLEAN = 18444914486360932352,
    JSVAL_SHIFTED_TAG_MAGIC = 18445055223849287680,
    JSVAL_SHIFTED_TAG_NULL = 18445477436314353664,
    JSVAL_SHIFTED_TAG_OBJECT = 18445618173802708992,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSWhyMagic {
    JS_ELEMENTS_HOLE = 0,
    JS_NO_ITER_VALUE = 1,
    JS_GENERATOR_CLOSING = 2,
    JS_NO_CONSTANT = 3,
    JS_THIS_POISON = 4,
    JS_ARG_POISON = 5,
    JS_SERIALIZE_NO_NODE = 6,
    JS_LAZY_ARGUMENTS = 7,
    JS_OPTIMIZED_ARGUMENTS = 8,
    JS_IS_CONSTRUCTING = 9,
    JS_OVERWRITTEN_CALLEE = 10,
    JS_BLOCK_NEEDS_CLONE = 11,
    JS_HASH_KEY_EMPTY = 12,
    JS_ION_ERROR = 13,
    JS_ION_BAILOUT = 14,
    JS_OPTIMIZED_OUT = 15,
    JS_UNINITIALIZED_LEXICAL = 16,
    JS_GENERIC_MAGIC = 17,
    JS_WHY_MAGIC_COUNT = 18,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct jsval_layout {
    pub asBits: __BindgenUnionField<u64>,
    pub debugView: __BindgenUnionField<jsval_layout__bindgen_ty_bindgen_id_90>,
    pub s: __BindgenUnionField<jsval_layout__bindgen_ty_bindgen_id_97>,
    pub asDouble: __BindgenUnionField<f64>,
    pub asPtr: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub asWord: __BindgenUnionField<usize>,
    pub asUIntPtr: __BindgenUnionField<usize>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct jsval_layout__bindgen_ty_bindgen_id_90 {
    pub _bitfield_1: u64,
}
#[test]
fn bindgen_test_layout_jsval_layout__bindgen_ty_bindgen_id_90() {
    assert_eq!(::std::mem::size_of::<jsval_layout__bindgen_ty_bindgen_id_90>()
               , 8usize);
    assert_eq!(::std::mem::align_of::<jsval_layout__bindgen_ty_bindgen_id_90>()
               , 8usize);
}
impl Clone for jsval_layout__bindgen_ty_bindgen_id_90 {
    fn clone(&self) -> Self { *self }
}
impl jsval_layout__bindgen_ty_bindgen_id_90 {
    #[inline]
    pub fn payload47(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 &
                                        (140737488355327usize as u64)) >>
                                       0u32) as u64)
        }
    }
    #[inline]
    pub fn set_payload47(&mut self, val: u64) {
        self._bitfield_1 &= !(140737488355327usize as u64);
        self._bitfield_1 |=
            ((val as u64 as u64) << 0u32) & (140737488355327usize as u64);
    }
    #[inline]
    pub fn tag(&self) -> JSValueTag {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 &
                                        (18446603336221196288usize as u64)) >>
                                       47u32) as u32)
        }
    }
    #[inline]
    pub fn set_tag(&mut self, val: JSValueTag) {
        self._bitfield_1 &= !(18446603336221196288usize as u64);
        self._bitfield_1 |=
            ((val as u32 as u64) << 47u32) &
                (18446603336221196288usize as u64);
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct jsval_layout__bindgen_ty_bindgen_id_97 {
    pub payload: jsval_layout__bindgen_ty_bindgen_id_97__bindgen_ty_bindgen_id_98,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct jsval_layout__bindgen_ty_bindgen_id_97__bindgen_ty_bindgen_id_98 {
    pub i32: __BindgenUnionField<i32>,
    pub u32: __BindgenUnionField<u32>,
    pub why: __BindgenUnionField<JSWhyMagic>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_jsval_layout__bindgen_ty_bindgen_id_97__bindgen_ty_bindgen_id_98() {
    assert_eq!(::std::mem::size_of::<jsval_layout__bindgen_ty_bindgen_id_97__bindgen_ty_bindgen_id_98>()
               , 4usize);
    assert_eq!(::std::mem::align_of::<jsval_layout__bindgen_ty_bindgen_id_97__bindgen_ty_bindgen_id_98>()
               , 4usize);
}
impl Clone for
 jsval_layout__bindgen_ty_bindgen_id_97__bindgen_ty_bindgen_id_98 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_jsval_layout__bindgen_ty_bindgen_id_97() {
    assert_eq!(::std::mem::size_of::<jsval_layout__bindgen_ty_bindgen_id_97>()
               , 4usize);
    assert_eq!(::std::mem::align_of::<jsval_layout__bindgen_ty_bindgen_id_97>()
               , 4usize);
}
impl Clone for jsval_layout__bindgen_ty_bindgen_id_97 {
    fn clone(&self) -> Self { *self }
}
impl Clone for jsval_layout {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Value {
    pub data: jsval_layout,
}
#[test]
fn bindgen_test_layout_Value() {
    assert_eq!(::std::mem::size_of::<Value>() , 8usize);
    assert_eq!(::std::mem::align_of::<Value>() , 8usize);
}
impl Clone for Value {
    fn clone(&self) -> Self { *self }
}
