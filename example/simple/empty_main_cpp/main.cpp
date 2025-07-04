#include <string_view>
#include <array>
#include <functional>
#include <tuple>
#include <cstdint>
#include <variant>
#include <span>
// TYPES:

// 195083283555941493446795725677288657264
struct struct_92c3ac7a8d03aced5aa7e83b7cca9170 {
uint8_t field_0; 
};

using fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 = std::function<void()>;

struct enum_588c4b2913c12d95b4029d1c153a9cea_0 {

};
struct enum_588c4b2913c12d95b4029d1c153a9cea_1 {

};

// 117700509624456162781467806643877747946
using enum_588c4b2913c12d95b4029d1c153a9cea = std::variant<enum_588c4b2913c12d95b4029d1c153a9cea_0, enum_588c4b2913c12d95b4029d1c153a9cea_1>;


struct enum_e54ef46296e0e1b5002aec6f803a72dd_0 {
    uint16_t field_0; 
};
struct enum_e54ef46296e0e1b5002aec6f803a72dd_1 {
    uintptr_t field_0; 
};
struct enum_e54ef46296e0e1b5002aec6f803a72dd_2 {};

// 304803166908709285583612564982610948829
using enum_e54ef46296e0e1b5002aec6f803a72dd = std::variant<enum_e54ef46296e0e1b5002aec6f803a72dd_0, enum_e54ef46296e0e1b5002aec6f803a72dd_1, enum_e54ef46296e0e1b5002aec6f803a72dd_2>;


// 84169911017540524845353216061896577245
struct struct_3f52890329a70117381258e502e2d0dd {
int32_t field_0; 
int32_t field_1; 
};


// 298299697393357827791159179495502064574
struct struct_e06a6e95018c855298bf4d30b6c92bbe {
uint32_t field_0; 
uint16_t field_1; 
uint16_t field_2; 
};


// 221703861916422404289394136502565181213
struct struct_a6ca9c533cf62603d9e2e5e7a6cd971d {
uintptr_t field_0; 
uint32_t field_1; 
enum_e54ef46296e0e1b5002aec6f803a72dd field_2; 
enum_e54ef46296e0e1b5002aec6f803a72dd field_3; 
};


// 167825326212774001022371638395022866169
struct struct_7e41fb6c1562171f6e859c395f73f6f9 {
void* field_0; 
};


// 115516678355994952759334352715770204252
struct struct_56e7b3f74beafa9b515b585d8ba7905c {
struct_92c3ac7a8d03aced5aa7e83b7cca9170 field_0; 
};


// 243031744701257153620396438343132985389
struct struct_b6d63621f5a8ee2123d41973f699382d {
struct_e06a6e95018c855298bf4d30b6c92bbe field_0; 
int* field_1; 
};

using closure_89cb9f1b62f39415f2b9d52320e10599 = std::function<int32_t()>;

struct enum_61dcfd9913b0020b60435675696a78ae_0 {};
struct enum_61dcfd9913b0020b60435675696a78ae_1 {
    std::span<struct_a6ca9c533cf62603d9e2e5e7a6cd971d> field_0;  /*span of struct_a6ca9c533cf62603d9e2e5e7a6cd971d*/
};

// 130082564477646086090448269010396674222
using enum_61dcfd9913b0020b60435675696a78ae = std::variant<enum_61dcfd9913b0020b60435675696a78ae_0, enum_61dcfd9913b0020b60435675696a78ae_1>;

using fn_ptr_247d8fd692a9b4df9a8c6534257381fd = std::function<enum_588c4b2913c12d95b4029d1c153a9cea(int32_t*, struct_b6d63621f5a8ee2123d41973f699382d*)>;
using fn_ptr_cf46556a41b87d30b5bf92289b804086 = std::function<enum_588c4b2913c12d95b4029d1c153a9cea(struct_7e41fb6c1562171f6e859c395f73f6f9, struct_b6d63621f5a8ee2123d41973f699382d*)>;

struct enum_121128004a13691e63519cc26b34b25a_0 {
    struct_7e41fb6c1562171f6e859c395f73f6f9 field_0; 
    fn_ptr_cf46556a41b87d30b5bf92289b804086 field_1; 
};
struct enum_121128004a13691e63519cc26b34b25a_1 {
    uint16_t field_0; 
};

// 24015184290033080575162413618386219610
using enum_121128004a13691e63519cc26b34b25a = std::variant<enum_121128004a13691e63519cc26b34b25a_0, enum_121128004a13691e63519cc26b34b25a_1>;


// 237314186446701704582585791359993080063
struct struct_b2890cbf12fa2255dd45bbcab73968ff {
enum_121128004a13691e63519cc26b34b25a field_0; 
};


// 338243918424169272477466358438870931970
struct struct_fe7768bac708a98255fe26e1f73c5a02 {
std::span<std::string_view> field_0;  /*span of std::string_view*/
enum_61dcfd9913b0020b60435675696a78ae field_1; 
std::span<struct_b2890cbf12fa2255dd45bbcab73968ff> field_2;  /*span of struct_b2890cbf12fa2255dd45bbcab73968ff*/
};





void  _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0fb956ce54e4ef97E(fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1) {



block0:



block1:



block2:




}

int32_t  _ZN4core3ops8function6FnOnce9call_once17h89ffdac548459e5bE(closure_89cb9f1b62f39415f2b9d52320e10599 _1) {
int32_t _0;


block0:
// Statement: _3 = &mut _1


block1:



block2:



block3:



block4:



return _0;
}

void  _ZN4core3ops8function6FnOnce9call_once17h36be117f97b14d0bE(fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1) {



block0:



block1:




}

struct_b2890cbf12fa2255dd45bbcab73968ff  _ZN4core3fmt2rt8Argument11new_display17h77078538da0a254cE(int32_t* _1) {
struct_b2890cbf12fa2255dd45bbcab73968ff _0;
struct_7e41fb6c1562171f6e859c395f73f6f9 _2;
fn_ptr_cf46556a41b87d30b5bf92289b804086 _3;
fn_ptr_247d8fd692a9b4df9a8c6534257381fd _4;
int32_t* _5;
void* _6;


block0:
// Statement: StorageLive(_2)
// Statement: StorageLive(_3)
// Statement: _6 = &raw const (*_1)
// Statement: StorageLive(_7)
// Statement: _7 = copy _6 as *const () (PtrToPtr)
// Statement: _3 = NonNull::<()> { pointer: move _7 }
// Statement: StorageDead(_7)
// Statement: StorageLive(_4)
// Statement: _5 = <T as std::fmt::Display>::fmt as for<'a, 'b, 'c> fn(&'a T, &'b mut std::fmt::Formatter<'c>) -> std::result::Result<(), std::fmt::Error> (PointerCoercion(ReifyFnPointer, Implicit))
// Statement: _4 = copy _5 as for<'a, 'b> unsafe fn(std::ptr::NonNull<()>, &'a mut std::fmt::Formatter<'b>) -> std::result::Result<(), std::fmt::Error> (Transmute)
// Statement: _2 = core::fmt::rt::ArgumentType::<'_>::Placeholder { value: move _3, formatter: move _4, _lifetime: const PhantomData::<&()> }
// Statement: StorageDead(_4)
// Statement: StorageDead(_3)
// Statement: _0 = core::fmt::rt::Argument::<'_> { ty: move _2 }
// Statement: StorageDead(_2)


return _0;
}

struct_fe7768bac708a98255fe26e1f73c5a02  _ZN4core3fmt2rt38__LT_impl_u20_core__fmt__Arguments_GT_6new_v117hc652ed31e9afa273E(std::array<std::string_view, 3>* _1, std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 2>* _2) {
struct_fe7768bac708a98255fe26e1f73c5a02 _0;


block0:
// Statement: StorageLive(_3)
// Statement: _3 = copy _1 as &[&str] (PointerCoercion(Unsize, Implicit))
// Statement: StorageLive(_4)
// Statement: _4 = copy _2 as &[core::fmt::rt::Argument<'_>] (PointerCoercion(Unsize, Implicit))
// Statement: _0 = Arguments::<'_> { pieces: move _3, fmt: const Option::<&[core::fmt::rt::Placeholder]>::None, args: move _4 }
// Statement: StorageDead(_4)
// Statement: StorageDead(_3)


return _0;
}

void  _ZN10empty_main4main17hd0b442567fa79b17E() {
int32_t _1;
int32_t _2;
int32_t _3;
std::tuple<int32_t, bool> _4;
struct_3f52890329a70117381258e502e2d0dd _5;
std::tuple<int32_t, bool> _6;
struct_fe7768bac708a98255fe26e1f73c5a02 _8;
std::tuple<int32_t*, int32_t*> _9;
int32_t* _10;
int32_t* _11;
std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 2> _12;
struct_b2890cbf12fa2255dd45bbcab73968ff _13;
struct_b2890cbf12fa2255dd45bbcab73968ff _14;
std::array<std::string_view, 3>* _15;
std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 2>* _16;
int32_t* _17;
int32_t* _18;


block0:
// Statement: _2 = const 1_i32
// Statement: _3 = const 2_i32
// Statement: _4 = AddWithOverflow(copy _2, copy _3)


block1:
// Statement: _1 = move (_4.0: i32)
// Statement: _5 = Point { x: copy _1, y: const 3_i32 }
// Statement: _6 = AddWithOverflow(copy (_5.0: i32), copy _1)


block2:
// Statement: (_5.0: i32) = move (_6.0: i32)
// Statement: _10 = &(_5.0: i32)
// Statement: _11 = &(_5.1: i32)
// Statement: _9 = (move _10, move _11)
// Statement: _17 = deref_copy (_9.0: &i32)


block3:
// Statement: _18 = deref_copy (_9.1: &i32)


block4:
// Statement: _12 = [move _13, move _14]
// Statement: _15 = const main::promoted[0]
// Statement: _16 = &_12


block5:



block6:




}

int32_t  _ZN4core3ops8function6FnOnce40call_once_u7b__u7b_vtable_shim_u7d__u7d_17hd25a61e515fdb9a3E(closure_89cb9f1b62f39415f2b9d52320e10599* _1) {
int32_t _0;


block0:



block1:



return _0;
}

int32_t  _ZN3std2rt10lang_start28__u7b__u7b_closure_u7d__u7d_17h083d4027b6fa8260E(closure_89cb9f1b62f39415f2b9d52320e10599* _1) {
int32_t _0;
fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _3;
uint8_t _4;


block0:
// Statement: StorageLive(_2)
// Statement: StorageLive(_3)
// Statement: StorageLive(_4)
// Statement: _4 = copy ((*_1).0: fn() -> T)


block1:
// Statement: StorageDead(_4)


block2:
// Statement: StorageDead(_3)
// Statement: StorageLive(_5)
// Statement: _5 = copy ((_2.0: std::sys::process::unix::common::ExitCode).0: u8)
// Statement: _0 = move _5 as i32 (IntToInt)
// Statement: StorageDead(_5)
// Statement: StorageDead(_2)


return _0;
}

intptr_t  _ZN3std2rt10lang_start17habb8eed6ce6072afE(fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1, intptr_t _2, uint8_t** _3, uint8_t _4) {
intptr_t _0;


block0:
// Statement: StorageLive(_5)
// Statement: StorageLive(_7)
// Statement: _7 = {closure@/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:10: 206:17} { 0: copy _1 }
// Statement: _6 = &_7
// Statement: _5 = copy _6 as &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe (PointerCoercion(Unsize, Implicit))


block1:
// Statement: StorageDead(_7)
// Statement: StorageDead(_5)


return _0;
}

struct_56e7b3f74beafa9b515b585d8ba7905c  _ZN54__LT__LP__RP__u20_as_u20_std__process__Termination_GT_6report17h44442036e6046436E() {
struct_56e7b3f74beafa9b515b585d8ba7905c _0;


block0:
// Statement: _0 = const ExitCode(std::sys::process::unix::common::ExitCode(0_u8))


return _0;
}
int main() {
 _ZN10empty_main4main17hd0b442567fa79b17E(); 
}