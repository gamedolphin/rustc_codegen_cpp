#include <stdfloat>
#include <string_view>
#include <array>
#include <functional>
#include <stdckdint.h>
#include <bit>
#include <tuple>
#include <variant>
#include <cstdint>
#include <span>
#include <utility>

#define CHECKED_ADD(a, b)                                                     \
    [&]() {                                                                   \
        using result_type = decltype(a);                                      \
        result_type result;                                                   \
        bool overflow = ckd_add(&result, a, b);                               \
        return std::make_tuple(result, overflow);                             \
    }()



#define CHECKED_SUB(a, b)                                                     \
    [&]() {                                                                   \
        using result_type = decltype(a);                                      \
        result_type result;                                                   \
        bool overflow = ckd_sub(&result, a, b);                               \
        return std::make_tuple(result, overflow);                             \
    }()



#define CHECKED_MUL(a, b)                                                     \
    [&]() {                                                                   \
        using result_type = decltype(a);                                      \
        result_type result;                                                   \
        bool overflow = ckd_mul(&result, a, b);                               \
        return std::make_tuple(result, overflow);                             \
    }()



struct FatPtr {
    const void* data;
    void* fn;
};

// TYPES:

// Type 261007144137579814786097145423174945660: double, skipped

// Type 40373079694569305626004750781605896368: intptr_t, skipped

// Type 110733052545508572261810273816972735333: std::array<std::string_view, 4>, skipped

// Type 157166153014986124439104429225962660038: uintptr_t, skipped

// Type 105732428401519543651246513221745309749: /*f64*/const double*, skipped

// Type 246629465088513626949685301164813016429: std::string_view, skipped


// 298299697393357827791159179495502064574
class struct_e06a6e95018c855298bf4d30b6c92bbe {
public:
uint32_t field_0; 
uint16_t field_1; 
uint16_t field_2; 
};


// 195083283555941493446795725677288657264
class struct_92c3ac7a8d03aced5aa7e83b7cca9170 {
public:
uint8_t field_0; 
};

// Type 25882202575019293479932656973818029271: uint32_t, skipped

// Type 131578394041067431414637433560278001139: /*u8*/const uint8_t*, skipped

// Type 259515890641039847960718449528024116736: /*()*/const void*, skipped


class enum_588c4b2913c12d95b4029d1c153a9cea_0 {
public:

};
class enum_588c4b2913c12d95b4029d1c153a9cea_1 {
public:

};

// 117700509624456162781467806643877747946
using enum_588c4b2913c12d95b4029d1c153a9cea = std::variant<enum_588c4b2913c12d95b4029d1c153a9cea_0, enum_588c4b2913c12d95b4029d1c153a9cea_1>;

// Type 115387559057565692143404304070439989267: int32_t, skipped

// Type 338307346251064506821929168073718289223: std::tuple</*i32*/const int32_t*, /*i32*/const int32_t*, /*f64*/const double*>, skipped


class enum_e54ef46296e0e1b5002aec6f803a72dd_0 {
public:
    uint16_t field_0; 
};
class enum_e54ef46296e0e1b5002aec6f803a72dd_1 {
public:
    uintptr_t field_0; 
};
class enum_e54ef46296e0e1b5002aec6f803a72dd_2 {};

// 304803166908709285583612564982610948829
using enum_e54ef46296e0e1b5002aec6f803a72dd = std::variant<enum_e54ef46296e0e1b5002aec6f803a72dd_0, enum_e54ef46296e0e1b5002aec6f803a72dd_1, enum_e54ef46296e0e1b5002aec6f803a72dd_2>;

using fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 = void (*) ();
// Type 81227397821202392580868386207259837707: char, skipped

// Type 217836028142283082806331200071340515738: std::tuple<int32_t, bool>, skipped

// Type 7428646492878894209665195255548636123: uint8_t, skipped

// Type 167825326212774001022371638395022866169: /*()*/const void*, skipped


// 84169911017540524845353216061896577245
class struct_3f52890329a70117381258e502e2d0dd {
public:
int32_t field_0; 
int32_t field_1; 
};

// Type 75227227111962583090813812631720034224: /*f64*/const double*, skipped

// Type 261921734845384738759285406578045033687: uint16_t, skipped

// Type 86577360055205003630662857375202502873: void, skipped

// Type 85409593237534839450475128738338522195: /*i32*/const int32_t*, skipped

// Type 327730781684580840597051976726507071179: /*i32*/const int32_t*, skipped

// Type 324675245860759320943513204350442872190: bool, skipped

// Type 52733028066561212956568390239161908445: /**const u8*/const /*u8*/const uint8_t**, skipped


// 221703861916422404289394136502565181213
class struct_a6ca9c533cf62603d9e2e5e7a6cd971d {
public:
uintptr_t field_0; 
uint32_t field_1; 
enum_e54ef46296e0e1b5002aec6f803a72dd field_2; 
enum_e54ef46296e0e1b5002aec6f803a72dd field_3; 
};

// Type 157562719757308764846597406676591355488: /*[&'{erased} str; 4_usize]*/const std::array<std::string_view, 4>*, skipped


// 115516678355994952759334352715770204252
class struct_56e7b3f74beafa9b515b585d8ba7905c {
public:
struct_92c3ac7a8d03aced5aa7e83b7cca9170 field_0; 
};


// 243031744701257153620396438343132985389
class struct_b6d63621f5a8ee2123d41973f699382d {
public:
struct_e06a6e95018c855298bf4d30b6c92bbe field_0; 
void * field_1; 
};


class closure_89cb9f1b62f39415f2b9d52320e10599 {
public:
fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 field_0;
};

int32_t fn_3e43e74425cda6b9();

FatPtr* GetClosurePtr_fn_3e43e74425cda6b9(const closure_89cb9f1b62f39415f2b9d52320e10599* closure) {
    return new FatPtr {
        .data = closure,
        .fn = reinterpret_cast<void*>(&fn_3e43e74425cda6b9)
    };
}


class enum_61dcfd9913b0020b60435675696a78ae_0 {};
class enum_61dcfd9913b0020b60435675696a78ae_1 {
public:
    std::span<const struct_a6ca9c533cf62603d9e2e5e7a6cd971d> field_0;  /*span of struct_a6ca9c533cf62603d9e2e5e7a6cd971d*/
};

// 130082564477646086090448269010396674222
using enum_61dcfd9913b0020b60435675696a78ae = std::variant<enum_61dcfd9913b0020b60435675696a78ae_0, enum_61dcfd9913b0020b60435675696a78ae_1>;

using fn_ptr_247d8fd692a9b4df9a8c6534257381fd = enum_588c4b2913c12d95b4029d1c153a9cea (*) (/*i32*/const int32_t*, /*std::fmt::Formatter<'{erased}>*/struct_b6d63621f5a8ee2123d41973f699382d*);
// Type 56324597819588941422001121442096306446: /*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/closure_89cb9f1b62f39415f2b9d52320e10599*, skipped

// Type 48478603669661753618137133807139670675: /*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/closure_89cb9f1b62f39415f2b9d52320e10599*, skipped

// Type 254284346417394979173390165839828138111: /*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/const closure_89cb9f1b62f39415f2b9d52320e10599*, skipped

// Type 307510406270489124676101075503395623806: /*std::fmt::Formatter<'{erased}>*/struct_b6d63621f5a8ee2123d41973f699382d*, skipped

using fn_ptr_cf46556a41b87d30b5bf92289b804086 = enum_588c4b2913c12d95b4029d1c153a9cea (*) (/*()*/const void*, /*std::fmt::Formatter<'{erased}>*/struct_b6d63621f5a8ee2123d41973f699382d*);
using fn_ptr_2bec5f878167b37e1176aaff171ffdb9 = enum_588c4b2913c12d95b4029d1c153a9cea (*) (/*f64*/const double*, /*std::fmt::Formatter<'{erased}>*/struct_b6d63621f5a8ee2123d41973f699382d*);

class enum_121128004a13691e63519cc26b34b25a_0 {
public:
    /*()*/const void* field_0; 
    fn_ptr_cf46556a41b87d30b5bf92289b804086 field_1; 
};
class enum_121128004a13691e63519cc26b34b25a_1 {
public:
    uint16_t field_0; 
};

// 24015184290033080575162413618386219610
using enum_121128004a13691e63519cc26b34b25a = std::variant<enum_121128004a13691e63519cc26b34b25a_0, enum_121128004a13691e63519cc26b34b25a_1>;


// 237314186446701704582585791359993080063
class struct_b2890cbf12fa2255dd45bbcab73968ff {
public:
enum_121128004a13691e63519cc26b34b25a field_0; 
};

// Type 316449497097050733731056948172140677984: std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3>, skipped


// 338243918424169272477466358438870931970
class struct_fe7768bac708a98255fe26e1f73c5a02 {
public:
std::span<const std::string_view> field_0;  /*span of std::string_view*/
enum_61dcfd9913b0020b60435675696a78ae field_1; 
std::span<const struct_b2890cbf12fa2255dd45bbcab73968ff> field_2;  /*span of struct_b2890cbf12fa2255dd45bbcab73968ff*/
};

// Type 168055047979961837458120872777411411953: /*[core::fmt::rt::Argument<'{erased}>; 3_usize]*/const std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3>*, skipped



// CONSTANTS:

constexpr int32_t constant_56ced5e4a15bd89050bb9674fa2df013_25bf8c35ab1a9d99_4 = 1;


constexpr int32_t constant_56ced5e4a15bd89050bb9674fa2df013_52f7aa788c769582_4 = 2;


constexpr int32_t constant_56ced5e4a15bd89050bb9674fa2df013_d09ba78c2a34ad1e_4 = 3;


constexpr double constant_c45c25bfe577a84e0b073a6684adcb7c_adfe931fb778a770_8 = 1.1;



constexpr std::array<char, 20> constant_768979dbca13b33be3652a1a16a4a660_42321706c74ddd67_20 = {{80,111,105,110,116,32,99,111,111,114,100,105,110,97,116,101,115,58,32,40}};




constexpr std::array<char, 2> constant_768979dbca13b33be3652a1a16a4a660_d4d2e7bca1e46caf_2 = {{44,32}};




constexpr std::array<char, 10> constant_768979dbca13b33be3652a1a16a4a660_d91e800f2de409ff_10 = {{41,44,32,102,108,111,97,116,58,32}};




constexpr std::array<char, 1> constant_768979dbca13b33be3652a1a16a4a660_4ff7a615e1ace5e7_1 = {{10}};




constexpr std::array<std::string_view, 4> constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8_data = {std::string_view(constant_768979dbca13b33be3652a1a16a4a660_42321706c74ddd67_20),std::string_view(constant_768979dbca13b33be3652a1a16a4a660_d4d2e7bca1e46caf_2),std::string_view(constant_768979dbca13b33be3652a1a16a4a660_d91e800f2de409ff_10),std::string_view(constant_768979dbca13b33be3652a1a16a4a660_4ff7a615e1ace5e7_1)};


constexpr const std::array<std::string_view, 4>* constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8 = &constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8_data;



constexpr std::array<uint8_t, 1> constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1_bytes = {0};
constexpr struct_56e7b3f74beafa9b515b585d8ba7905c constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1 = std::bit_cast<struct_56e7b3f74beafa9b515b585d8ba7905c>(constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1_bytes);





// EXTERNAL FUNCTIONS:

extern fn_ptr_247d8fd692a9b4df9a8c6534257381fd _ZN4core3fmt3num3imp52__LT_impl_u20_core__fmt__Display_u20_for_u20_i32_GT_3fmt17h242b909b676029e7E;
extern fn_ptr_2bec5f878167b37e1176aaff171ffdb9 _ZN4core3fmt5float52__LT_impl_u20_core__fmt__Display_u20_for_u20_f64_GT_3fmt17h9bca48ab37033042E;

void  _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0fb956ce54e4ef97E(fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1) {
// _0 ->  T/#1 is void, skipping
// _2 ->  () is void, skipping

bb0:
// Statement: Call: func=<F as FnOnce<()>>::call_once, args=[Spanned { node: move _1, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:152:18: 152:19 (#0) }, Spanned { node: const (), span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:152:18: 152:21 (#0) }], destination=_0, target=Some(bb1), unwind=Continue, call_source=Normal, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:152:18: 152:21 (#0)
// Statement: Call with operand: <F as FnOnce<()>>::call_once
goto bb1;

bb1:
// Statement: Call: func=std::intrinsics::black_box::<()>, args=[Spanned { node: const (), span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/hint.rs:482:34: 482:39 (#0) }], destination=_2, target=Some(bb2), unwind=Unreachable, call_source=Normal, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/hint.rs:482:5: 482:40 (#0)
// Statement: Call with operand: std::intrinsics::black_box::<()>
goto bb2;

bb2:
// Statement: Return


}

int32_t  _ZN4core3ops8function6FnOnce9call_once17h89ffdac548459e5bE(closure_89cb9f1b62f39415f2b9d52320e10599 _1) {
int32_t _0; // Alias(Projection, AliasTy { args: [Self/#0, Args/#1], def_id: DefId(2:3922 ~ core[57fa]::ops::function::FnOnce::Output), .. }) 
/*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/closure_89cb9f1b62f39415f2b9d52320e10599* _3; // &'{erased} mut Self/#0 
// _2 ->  () (at 2) is void, skipping

bb0:
// Statement: Debug: Assign((_3, &mut _1))
_3 /*&'{erased} mut Self/#0*/ = &_1 /*Self/#0*/;
// Statement: Call: func=<Self as FnMut<Args>>::call_mut, args=[Spanned { node: move _3, span: no-location (#0) }, Spanned { node: move _2, span: no-location (#0) }], destination=_0, target=Some(bb1), unwind=Cleanup(bb3), call_source=Misc, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5: 250:71 (#0)
// Statement: Call with operand: <Self as FnMut<Args>>::call_mut
goto bb1;

bb1:
// Statement: Drop: place=_1, target=bb2, unwind=Continue, replace=false, drop=None, async_fut=None

bb2:
// Statement: Return

bb3:
// Statement: Drop: place=_1, target=bb4, unwind=Terminate(InCleanup), replace=false, drop=None, async_fut=None

bb4:
// Statement: UnwindResume

return _0;
}

void  _ZN4core3ops8function6FnOnce9call_once17h36be117f97b14d0bE(fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1) {
// _0 ->  Alias(Projection, AliasTy { args: [Binder { value: fn(), bound_vars: [] }, ()], def_id: DefId(2:3922 ~ core[57fa]::ops::function::FnOnce::Output), .. }) is void, skipping
// _2 ->  () (at 2) is void, skipping

bb0:
// Statement: Call: func=move _1, args=[], destination=_0, target=Some(bb1), unwind=Continue, call_source=Misc, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5: 250:71 (#0)
std::move(_1 /*Binder { value: fn(), bound_vars: [] }*/)();
goto bb1;

bb1:
// Statement: Return


}

struct_b2890cbf12fa2255dd45bbcab73968ff  _ZN4core3fmt2rt8Argument11new_display17h77078538da0a254cE(/*i32*/const int32_t* _1) {
struct_b2890cbf12fa2255dd45bbcab73968ff _0; // core::fmt::rt::Argument<'{erased}> 
enum_121128004a13691e63519cc26b34b25a _2; // core::fmt::rt::ArgumentType<'{erased}> 
/*()*/const void* _3; // std::ptr::NonNull<()> 
fn_ptr_cf46556a41b87d30b5bf92289b804086 _4; // Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] } 
fn_ptr_247d8fd692a9b4df9a8c6534257381fd _5; // Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon), Region(BrAnon)] } 
/*i32*/const int32_t* _6; // *const T/#1 
/*()*/const void* _7; // *const () 

bb0:
// Statement: Debug: Assign((_6, &raw const (*_1)))
_6 /**const T/#1*/ = &*(_1);
// Statement: Debug: Assign((_7, copy _6 as *const () (PtrToPtr)))
_7 /**const ()*/ = static_cast</*()*/const void*>(std::bit_cast</*i32*/const int32_t*>(_6 /**const T/#1*/));
// Statement: Debug: Assign((_3, NonNull::<()> { pointer: move _7 }))
_3 /*std::ptr::NonNull<()>*/ = std::move(_7 /**const ()*/);
// Statement: Debug: Assign((_5, <T as std::fmt::Display>::fmt as for<'a, 'b, 'c> fn(&'a T, &'b mut std::fmt::Formatter<'c>) -> std::result::Result<(), std::fmt::Error> (PointerCoercion(ReifyFnPointer, Implicit))))
_5 /*Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon), Region(BrAnon)] }*/ = _ZN4core3fmt3num3imp52__LT_impl_u20_core__fmt__Display_u20_for_u20_i32_GT_3fmt17h242b909b676029e7E;
// Statement: Debug: Assign((_4, copy _5 as for<'a, 'b> unsafe fn(std::ptr::NonNull<()>, &'a mut std::fmt::Formatter<'b>) -> std::result::Result<(), std::fmt::Error> (Transmute)))
_4 /*Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] }*/ = reinterpret_cast<fn_ptr_cf46556a41b87d30b5bf92289b804086>(_5 /*Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon), Region(BrAnon)] }*/);
// Statement: Debug: Assign((_2, core::fmt::rt::ArgumentType::<'_>::Placeholder { value: move _3, formatter: move _4, _lifetime: const PhantomData::<&()> }))
_2 /*core::fmt::rt::ArgumentType<'{erased}>*/ = enum_121128004a13691e63519cc26b34b25a_0{.field_0 = std::move(_3 /*std::ptr::NonNull<()>*/), .field_1 = std::move(_4 /*Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] }*/), /* field_2 - Unsupported constant value: ZeroSized for type std::marker::PhantomData<&'{erased} ()> */};
// Statement: Debug: Assign((_0, core::fmt::rt::Argument::<'_> { ty: move _2 }))
_0 /*core::fmt::rt::Argument<'{erased}>*/ = struct_b2890cbf12fa2255dd45bbcab73968ff{.field_0 = std::move(_2 /*core::fmt::rt::ArgumentType<'{erased}>*/)};
// Statement: Return

return _0;
}

struct_fe7768bac708a98255fe26e1f73c5a02  _ZN4core3fmt2rt38__LT_impl_u20_core__fmt__Arguments_GT_6new_v117h44cfd91a5dd25e61E(/*[&'{erased} str; 4_usize]*/const std::array<std::string_view, 4>* _1, /*[core::fmt::rt::Argument<'{erased}>; 3_usize]*/const std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3>* _2) {
struct_fe7768bac708a98255fe26e1f73c5a02 _0; // std::fmt::Arguments<'{erased}> 
std::span<const std::string_view> _3; // &'{erased} [&'{erased} str] 
std::span<const struct_b2890cbf12fa2255dd45bbcab73968ff> _4; // &'{erased} [core::fmt::rt::Argument<'{erased}>] 

bb0:
// Statement: Debug: Assign((_3, copy _1 as &[&str] (PointerCoercion(Unsize, Implicit))))
_3 /*&'{erased} [&'{erased} str]*/ = *std::bit_cast</*[&'{erased} str; 4_usize]*/const std::array<std::string_view, 4>*>(_1 /*&'{erased} [&'{erased} str; P/#1]*/);
// Statement: Debug: Assign((_4, copy _2 as &[core::fmt::rt::Argument<'_>] (PointerCoercion(Unsize, Implicit))))
_4 /*&'{erased} [core::fmt::rt::Argument<'{erased}>]*/ = *std::bit_cast</*[core::fmt::rt::Argument<'{erased}>; 3_usize]*/const std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3>*>(_2 /*&'{erased} [core::fmt::rt::Argument<'{erased}>; A/#2]*/);
// Statement: Debug: Assign((_0, Arguments::<'_> { pieces: move _3, fmt: const Option::<&[core::fmt::rt::Placeholder]>::None, args: move _4 }))
_0 /*std::fmt::Arguments<'{erased}>*/ = struct_fe7768bac708a98255fe26e1f73c5a02{.field_0 = std::move(_3 /*&'{erased} [&'{erased} str]*/), /* field_1 - Unsupported constant value: Indirect { alloc_id: alloc11, offset: Size(0 bytes) } for type std::option::Option<&'{erased} [core::fmt::rt::Placeholder]> */.field_2 = std::move(_4 /*&'{erased} [core::fmt::rt::Argument<'{erased}>]*/)};
// Statement: Return

return _0;
}

void  _ZN10empty_main4main17hd0b442567fa79b17E() {
// _0 ->  () is void, skipping
int32_t _1; // i32 
int32_t _2; // i32 
int32_t _3; // i32 
std::tuple<int32_t, bool> _4; // (i32, bool) 
struct_3f52890329a70117381258e502e2d0dd _5; // Point 
std::tuple<int32_t, bool> _6; // (i32, bool) 
double _7; // f64 
double _8; // f64 
// _9 ->  () is void, skipping
struct_fe7768bac708a98255fe26e1f73c5a02 _10; // std::fmt::Arguments<'{erased}> 
std::tuple</*i32*/const int32_t*, /*i32*/const int32_t*, /*f64*/const double*> _11; // (&'{erased} i32, &'{erased} i32, &'{erased} f64) 
/*i32*/const int32_t* _12; // &'{erased} i32 
/*i32*/const int32_t* _13; // &'{erased} i32 
/*f64*/const double* _14; // &'{erased} f64 
std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3> _15; // [core::fmt::rt::Argument<'{erased}>; 3_usize] 
struct_b2890cbf12fa2255dd45bbcab73968ff _16; // core::fmt::rt::Argument<'{erased}> 
struct_b2890cbf12fa2255dd45bbcab73968ff _17; // core::fmt::rt::Argument<'{erased}> 
struct_b2890cbf12fa2255dd45bbcab73968ff _18; // core::fmt::rt::Argument<'{erased}> 
/*[&'{erased} str; 4_usize]*/const std::array<std::string_view, 4>* _19; // &'{erased} [&'{erased} str; 4_usize] 
/*[core::fmt::rt::Argument<'{erased}>; 3_usize]*/const std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3>* _20; // &'{erased} [core::fmt::rt::Argument<'{erased}>; 3_usize] 
/*i32*/const int32_t* _21; // &'{erased} i32 
/*i32*/const int32_t* _22; // &'{erased} i32 
/*f64*/const double* _23; // &'{erased} f64 

bb0:
// Statement: Debug: Assign((_2, const 1_i32))
_2 /*i32*/ = constant_56ced5e4a15bd89050bb9674fa2df013_25bf8c35ab1a9d99_4;
// Statement: Debug: Assign((_3, const 2_i32))
_3 /*i32*/ = constant_56ced5e4a15bd89050bb9674fa2df013_52f7aa788c769582_4;
// Statement: Debug: Assign((_4, AddWithOverflow(copy _2, copy _3)))
_4 /*(i32, bool)*/ = CHECKED_ADD(std::bit_cast<int32_t>(_2 /*i32*/), std::bit_cast<int32_t>(_3 /*i32*/));
// Statement: Assert: cond=move (_4.1: bool), expected=false, msg=Overflow(Add, move _2, move _3), target=bb1, unwind=Continue

bb1:
// Statement: Debug: Assign((_1, move (_4.0: i32)))
_1 /*i32*/ = std::move(std::get<0>(_4));
// Statement: Debug: Assign((_5, Point { x: copy _1, y: const 3_i32 }))
_5 /*Point*/ = struct_3f52890329a70117381258e502e2d0dd{.field_0 = std::bit_cast<int32_t>(_1 /*i32*/), .field_1 = constant_56ced5e4a15bd89050bb9674fa2df013_d09ba78c2a34ad1e_4};
// Statement: Debug: Assign((_6, AddWithOverflow(copy (_5.0: i32), copy _1)))
_6 /*(i32, bool)*/ = CHECKED_ADD(std::bit_cast<int32_t>((_5).field_0), std::bit_cast<int32_t>(_1 /*i32*/));
// Statement: Assert: cond=move (_6.1: bool), expected=false, msg=Overflow(Add, copy (_5.0: i32), copy _1), target=bb2, unwind=Continue

bb2:
// Statement: Debug: Assign(((_5.0: i32), move (_6.0: i32)))
(_5).field_0 = std::move(std::get<0>(_6));
// Statement: Debug: Assign((_8, const 1.1000000000000001f64))
_8 /*f64*/ = constant_c45c25bfe577a84e0b073a6684adcb7c_adfe931fb778a770_8;
// Statement: Debug: Assign((_7, Add(move _8, const 1.1000000000000001f64)))
_7 /*f64*/ = std::move(_8 /*f64*/) + constant_c45c25bfe577a84e0b073a6684adcb7c_adfe931fb778a770_8;
// Statement: Debug: Assign((_12, &(_5.0: i32)))
_12 /*&'{erased} i32*/ = &(_5).field_0;
// Statement: Debug: Assign((_13, &(_5.1: i32)))
_13 /*&'{erased} i32*/ = &(_5).field_1;
// Statement: Debug: Assign((_14, &_7))
_14 /*&'{erased} f64*/ = &_7 /*f64*/;
// Statement: Debug: Assign((_11, (move _12, move _13, move _14)))
_11 /*(&'{erased} i32, &'{erased} i32, &'{erased} f64)*/ = std::make_tuple(std::move(_12 /*&'{erased} i32*/), std::move(_13 /*&'{erased} i32*/), std::move(_14 /*&'{erased} f64*/));
// Statement: Debug: Assign((_21, deref_copy (_11.0: &i32)))
_21 /*&'{erased} i32*/ = std::bit_cast</*i32*/const int32_t*>(std::get<0>(_11));
// Statement: Call: func=core::fmt::rt::Argument::<'_>::new_display::<i32>, args=[Spanned { node: copy _21, span: src/main.rs:15:56: 15:60 (#7) }], destination=_16, target=Some(bb3), unwind=Continue, call_source=Normal, fn_span=src/main.rs:15:35: 15:37 (#7)
// Statement: Call with operand: core::fmt::rt::Argument::<'_>::new_display::<i32>
goto bb3;

bb3:
// Statement: Debug: Assign((_22, deref_copy (_11.1: &i32)))
_22 /*&'{erased} i32*/ = std::bit_cast</*i32*/const int32_t*>(std::get<1>(_11));
// Statement: Call: func=core::fmt::rt::Argument::<'_>::new_display::<i32>, args=[Spanned { node: copy _22, span: src/main.rs:15:62: 15:66 (#7) }], destination=_17, target=Some(bb4), unwind=Continue, call_source=Normal, fn_span=src/main.rs:15:39: 15:41 (#7)
// Statement: Call with operand: core::fmt::rt::Argument::<'_>::new_display::<i32>
goto bb4;

bb4:
// Statement: Debug: Assign((_23, deref_copy (_11.2: &f64)))
_23 /*&'{erased} f64*/ = std::bit_cast</*f64*/const double*>(std::get<2>(_11));
// Statement: Call: func=core::fmt::rt::Argument::<'_>::new_display::<f64>, args=[Spanned { node: copy _23, span: src/main.rs:15:68: 15:70 (#7) }], destination=_18, target=Some(bb5), unwind=Continue, call_source=Normal, fn_span=src/main.rs:15:51: 15:53 (#7)
// Statement: Call with operand: core::fmt::rt::Argument::<'_>::new_display::<f64>
goto bb5;

bb5:
// Statement: Debug: Assign((_15, [move _16, move _17, move _18]))
_15 /*[core::fmt::rt::Argument<'{erased}>; 3_usize]*/ = {std::move(_16 /*core::fmt::rt::Argument<'{erased}>*/), std::move(_17 /*core::fmt::rt::Argument<'{erased}>*/), std::move(_18 /*core::fmt::rt::Argument<'{erased}>*/)};
// Statement: Debug: Assign((_19, const main::promoted[0]))
_19 /*&'{erased} [&'{erased} str; 4_usize]*/ = constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8;
// Statement: Debug: Assign((_20, &_15))
_20 /*&'{erased} [core::fmt::rt::Argument<'{erased}>; 3_usize]*/ = &_15 /*[core::fmt::rt::Argument<'{erased}>; 3_usize]*/;
// Statement: Call: func=core::fmt::rt::<impl Arguments<'_>>::new_v1::<4, 3>, args=[Spanned { node: copy _19, span: src/main.rs:15:14: 15:54 (#0) }, Spanned { node: copy _20, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:28: 143:61 (#7) }], destination=_10, target=Some(bb6), unwind=Continue, call_source=Normal, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:28: 143:61 (#7)
// Statement: Call with operand: core::fmt::rt::<impl Arguments<'_>>::new_v1::<4, 3>
goto bb6;

bb6:
// Statement: Call: func=_print, args=[Spanned { node: move _10, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:28: 143:61 (#5) }], destination=_9, target=Some(bb7), unwind=Continue, call_source=Normal, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:9: 143:62 (#4)
// Statement: Call with operand: _print
goto bb7;

bb7:
// Statement: Return


}

struct_b2890cbf12fa2255dd45bbcab73968ff  _ZN4core3fmt2rt8Argument11new_display17h3a95d3486de58403E(/*f64*/const double* _1) {
struct_b2890cbf12fa2255dd45bbcab73968ff _0; // core::fmt::rt::Argument<'{erased}> 
enum_121128004a13691e63519cc26b34b25a _2; // core::fmt::rt::ArgumentType<'{erased}> 
/*()*/const void* _3; // std::ptr::NonNull<()> 
fn_ptr_cf46556a41b87d30b5bf92289b804086 _4; // Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] } 
fn_ptr_2bec5f878167b37e1176aaff171ffdb9 _5; // Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon), Region(BrAnon)] } 
/*f64*/const double* _6; // *const T/#1 
/*()*/const void* _7; // *const () 

bb0:
// Statement: Debug: Assign((_6, &raw const (*_1)))
_6 /**const T/#1*/ = &*(_1);
// Statement: Debug: Assign((_7, copy _6 as *const () (PtrToPtr)))
_7 /**const ()*/ = static_cast</*()*/const void*>(std::bit_cast</*f64*/const double*>(_6 /**const T/#1*/));
// Statement: Debug: Assign((_3, NonNull::<()> { pointer: move _7 }))
_3 /*std::ptr::NonNull<()>*/ = std::move(_7 /**const ()*/);
// Statement: Debug: Assign((_5, <T as std::fmt::Display>::fmt as for<'a, 'b, 'c> fn(&'a T, &'b mut std::fmt::Formatter<'c>) -> std::result::Result<(), std::fmt::Error> (PointerCoercion(ReifyFnPointer, Implicit))))
_5 /*Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon), Region(BrAnon)] }*/ = _ZN4core3fmt5float52__LT_impl_u20_core__fmt__Display_u20_for_u20_f64_GT_3fmt17h9bca48ab37033042E;
// Statement: Debug: Assign((_4, copy _5 as for<'a, 'b> unsafe fn(std::ptr::NonNull<()>, &'a mut std::fmt::Formatter<'b>) -> std::result::Result<(), std::fmt::Error> (Transmute)))
_4 /*Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] }*/ = reinterpret_cast<fn_ptr_cf46556a41b87d30b5bf92289b804086>(_5 /*Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon), Region(BrAnon)] }*/);
// Statement: Debug: Assign((_2, core::fmt::rt::ArgumentType::<'_>::Placeholder { value: move _3, formatter: move _4, _lifetime: const PhantomData::<&()> }))
_2 /*core::fmt::rt::ArgumentType<'{erased}>*/ = enum_121128004a13691e63519cc26b34b25a_0{.field_0 = std::move(_3 /*std::ptr::NonNull<()>*/), .field_1 = std::move(_4 /*Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) -> std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] }*/), /* field_2 - Unsupported constant value: ZeroSized for type std::marker::PhantomData<&'{erased} ()> */};
// Statement: Debug: Assign((_0, core::fmt::rt::Argument::<'_> { ty: move _2 }))
_0 /*core::fmt::rt::Argument<'{erased}>*/ = struct_b2890cbf12fa2255dd45bbcab73968ff{.field_0 = std::move(_2 /*core::fmt::rt::ArgumentType<'{erased}>*/)};
// Statement: Return

return _0;
}

int32_t  _ZN4core3ops8function6FnOnce40call_once_u7b__u7b_vtable_shim_u7d__u7d_17hd25a61e515fdb9a3E(/*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/closure_89cb9f1b62f39415f2b9d52320e10599* _1) {
int32_t _0; // Alias(Projection, AliasTy { args: [Self/#0, Args/#1], def_id: DefId(2:3922 ~ core[57fa]::ops::function::FnOnce::Output), .. }) 
// _2 ->  () (at 2) is void, skipping

bb0:
// Statement: Call: func=<Self as FnOnce<Args>>::call_once, args=[Spanned { node: move (*_1), span: no-location (#0) }, Spanned { node: move _2, span: no-location (#0) }], destination=_0, target=Some(bb1), unwind=Continue, call_source=Misc, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5: 250:71 (#0)
// Statement: Call with operand: <Self as FnOnce<Args>>::call_once
goto bb1;

bb1:
// Statement: Return

return _0;
}

intptr_t  _ZN3std2rt10lang_start17habb8eed6ce6072afE(fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1, intptr_t _2, /**const u8*/const /*u8*/const uint8_t** _3, uint8_t _4) {
intptr_t _0; // isize 
void * _5; // &'{erased} dyn [Binder { value: Trait(std::ops::Fn<()>), bound_vars: [] }, Binder { value: Projection(ExistentialProjection { def_id: DefId(2:3922 ~ core[57fa]::ops::function::FnOnce::Output), args: [()], term: Term::Ty(i32), use_existential_projection_new_instead: () }), bound_vars: [] }, Binder { value: AutoTrait(DefId(2:3647 ~ core[57fa]::marker::Sync)), bound_vars: [] }, Binder { value: AutoTrait(DefId(2:49650 ~ core[57fa]::panic::unwind_safe::RefUnwindSafe)), bound_vars: [] }] + '{erased} 
/*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/const closure_89cb9f1b62f39415f2b9d52320e10599* _6; // &'{erased} Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn() -> T/#0, bound_vars: [] },)]) 
closure_89cb9f1b62f39415f2b9d52320e10599 _7; // Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn() -> T/#0, bound_vars: [] },)]) 

bb0:
// Statement: Debug: Assign((_7, {closure@/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:10: 206:17} { 0: copy _1 }))
_7 /*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn() -> T/#0, bound_vars: [] },)])*/ = closure_89cb9f1b62f39415f2b9d52320e10599{.field_0 = _1 /*Binder { value: fn() -> T/#0, bound_vars: [] }*/};
// Statement: Debug: Assign((_6, &_7))
_6 /*&'{erased} Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn() -> T/#0, bound_vars: [] },)])*/ = &_7 /*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn() -> T/#0, bound_vars: [] },)])*/;
// Statement: Debug: Assign((_5, copy _6 as &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe (PointerCoercion(Unsize, Implicit))))
_5 /*&'{erased} dyn [Binder { value: Trait(std::ops::Fn<()>), bound_vars: [] }, Binder { value: Projection(ExistentialProjection { def_id: DefId(2:3922 ~ core[57fa]::ops::function::FnOnce::Output), args: [()], term: Term::Ty(i32), use_existential_projection_new_instead: () }), bound_vars: [] }, Binder { value: AutoTrait(DefId(2:3647 ~ core[57fa]::marker::Sync)), bound_vars: [] }, Binder { value: AutoTrait(DefId(2:49650 ~ core[57fa]::panic::unwind_safe::RefUnwindSafe)), bound_vars: [] }] + '{erased}*/ = GetClosurePtr_fn_3e43e74425cda6b9(std::bit_cast</*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/const closure_89cb9f1b62f39415f2b9d52320e10599*>(_6 /*&'{erased} Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn() -> T/#0, bound_vars: [] },)])*/));
// Statement: Call: func=rt::lang_start_internal, args=[Spanned { node: move _5, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:9: 206:93 (#0) }, Spanned { node: move _2, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:207:9: 207:13 (#0) }, Spanned { node: move _3, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:208:9: 208:13 (#0) }, Spanned { node: move _4, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:209:9: 209:16 (#0) }], destination=_0, target=Some(bb1), unwind=Continue, call_source=Normal, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:205:5: 210:6 (#0)
// Statement: Call with operand: rt::lang_start_internal
goto bb1;

bb1:
// Statement: Return

return _0;
}

struct_56e7b3f74beafa9b515b585d8ba7905c  _ZN54__LT__LP__RP__u20_as_u20_std__process__Termination_GT_6report17h44442036e6046436E() {
struct_56e7b3f74beafa9b515b585d8ba7905c _0; // std::process::ExitCode 

bb0:
// Statement: Debug: Assign((_0, const ExitCode(std::sys::process::unix::common::ExitCode(0_u8))))
_0 /*std::process::ExitCode*/ = constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1;
// Statement: Return

return _0;
}

int32_t  _ZN3std2rt10lang_start28__u7b__u7b_closure_u7d__u7d_17h083d4027b6fa8260E(/*Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [(), i8, Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn(), bound_vars: [] },)])*/const closure_89cb9f1b62f39415f2b9d52320e10599* _1) {
int32_t _0; // i32 
struct_56e7b3f74beafa9b515b585d8ba7905c _2; // std::process::ExitCode 
// _3 ->  T/#0 is void, skipping
fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _4; // Binder { value: fn() -> T/#0, bound_vars: [] } 
uint8_t _5; // u8 

bb0:
// Statement: Debug: Assign((_4, copy ((*_1).0: fn() -> T)))
_4 /*Binder { value: fn() -> T/#0, bound_vars: [] }*/ = (*(_1)).field_0;
// Statement: Call: func=std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> T, T>, args=[Spanned { node: move _4, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:70: 206:74 (#0) }], destination=_3, target=Some(bb1), unwind=Continue, call_source=Normal, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18: 206:75 (#0)
// Statement: Call with operand: std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> T, T>
goto bb1;

bb1:
// Statement: Call: func=<T as Termination>::report, args=[Spanned { node: move _3, span: /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18: 206:75 (#0) }], destination=_2, target=Some(bb2), unwind=Continue, call_source=Normal, fn_span=/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:76: 206:84 (#0)
// Statement: Call with operand: <T as Termination>::report
goto bb2;

bb2:
// Statement: Debug: Assign((_5, copy ((_2.0: std::sys::process::unix::common::ExitCode).0: u8)))
_5 /*u8*/ = std::bit_cast<uint8_t>(((_2).field_0).field_0);
// Statement: Debug: Assign((_0, move _5 as i32 (IntToInt)))
_0 /*i32*/ = static_cast<int32_t>(std::move(_5 /*u8*/));
// Statement: Return

return _0;
}
int main() {
 _ZN10empty_main4main17hd0b442567fa79b17E(); 
}