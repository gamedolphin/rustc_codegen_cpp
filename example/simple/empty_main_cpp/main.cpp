#include <array>
#include <bit>
#include <cstdint>
#include <functional>
#include <span>
#include <stdckdint.h>
#include <stdfloat>
#include <string_view>
#include <tuple>
#include <utility>
#include <variant>

#define CHECKED_ADD(a, b)                                                      \
  [&]() {                                                                      \
    using result_type = decltype(a);                                           \
    result_type result;                                                        \
    bool overflow = ckd_add(&result, a, b);                                    \
    return std::make_tuple(result, overflow);                                  \
  }()

#define CHECKED_SUB(a, b)                                                      \
  [&]() {                                                                      \
    using result_type = decltype(a);                                           \
    result_type result;                                                        \
    bool overflow = ckd_sub(&result, a, b);                                    \
    return std::make_tuple(result, overflow);                                  \
  }()

#define CHECKED_MUL(a, b)                                                      \
  [&]() {                                                                      \
    using result_type = decltype(a);                                           \
    result_type result;                                                        \
    bool overflow = ckd_mul(&result, a, b);                                    \
    return std::make_tuple(result, overflow);                                  \
  }()

// TYPES:

// 84169911017540524845353216061896577245
struct struct_3f52890329a70117381258e502e2d0dd {
  int32_t field_0;
  int32_t field_1;
};

// 195083283555941493446795725677288657264
struct struct_92c3ac7a8d03aced5aa7e83b7cca9170 {
  uint8_t field_0;
};

struct enum_e54ef46296e0e1b5002aec6f803a72dd_0 {
  uint16_t field_0;
};
struct enum_e54ef46296e0e1b5002aec6f803a72dd_1 {
  uintptr_t field_0;
};
struct enum_e54ef46296e0e1b5002aec6f803a72dd_2 {};

// 304803166908709285583612564982610948829
using enum_e54ef46296e0e1b5002aec6f803a72dd =
    std::variant<enum_e54ef46296e0e1b5002aec6f803a72dd_0,
                 enum_e54ef46296e0e1b5002aec6f803a72dd_1,
                 enum_e54ef46296e0e1b5002aec6f803a72dd_2>;

// 298299697393357827791159179495502064574
struct struct_e06a6e95018c855298bf4d30b6c92bbe {
  uint32_t field_0;
  uint16_t field_1;
  uint16_t field_2;
};

struct enum_588c4b2913c12d95b4029d1c153a9cea_0 {};
struct enum_588c4b2913c12d95b4029d1c153a9cea_1 {};

// 117700509624456162781467806643877747946
using enum_588c4b2913c12d95b4029d1c153a9cea =
    std::variant<enum_588c4b2913c12d95b4029d1c153a9cea_0,
                 enum_588c4b2913c12d95b4029d1c153a9cea_1>;

using fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 = std::function<void()>;

// 243031744701257153620396438343132985389
struct struct_b6d63621f5a8ee2123d41973f699382d {
  struct_e06a6e95018c855298bf4d30b6c92bbe field_0;
  int *field_1;
};

// 221703861916422404289394136502565181213
struct struct_a6ca9c533cf62603d9e2e5e7a6cd971d {
  uintptr_t field_0;
  uint32_t field_1;
  enum_e54ef46296e0e1b5002aec6f803a72dd field_2;
  enum_e54ef46296e0e1b5002aec6f803a72dd field_3;
};

// 115516678355994952759334352715770204252
struct struct_56e7b3f74beafa9b515b585d8ba7905c {
  struct_92c3ac7a8d03aced5aa7e83b7cca9170 field_0;
};

// 167825326212774001022371638395022866169
struct struct_7e41fb6c1562171f6e859c395f73f6f9 {
  const void *field_0;
};

using closure_89cb9f1b62f39415f2b9d52320e10599 = std::function<int32_t()>;
using fn_ptr_cf46556a41b87d30b5bf92289b804086 =
    std::function<enum_588c4b2913c12d95b4029d1c153a9cea(
        struct_7e41fb6c1562171f6e859c395f73f6f9,
        struct_b6d63621f5a8ee2123d41973f699382d *)>;
using fn_ptr_2bec5f878167b37e1176aaff171ffdb9 =
    std::function<enum_588c4b2913c12d95b4029d1c153a9cea(
        const double *, struct_b6d63621f5a8ee2123d41973f699382d *)>;

struct enum_61dcfd9913b0020b60435675696a78ae_0 {};
struct enum_61dcfd9913b0020b60435675696a78ae_1 {
  std::span<struct_a6ca9c533cf62603d9e2e5e7a6cd971d>
      field_0; /*span of struct_a6ca9c533cf62603d9e2e5e7a6cd971d*/
};

// 130082564477646086090448269010396674222
using enum_61dcfd9913b0020b60435675696a78ae =
    std::variant<enum_61dcfd9913b0020b60435675696a78ae_0,
                 enum_61dcfd9913b0020b60435675696a78ae_1>;

using fn_ptr_247d8fd692a9b4df9a8c6534257381fd =
    std::function<enum_588c4b2913c12d95b4029d1c153a9cea(
        const int32_t *, struct_b6d63621f5a8ee2123d41973f699382d *)>;

struct enum_121128004a13691e63519cc26b34b25a_0 {
  struct_7e41fb6c1562171f6e859c395f73f6f9 field_0;
  fn_ptr_cf46556a41b87d30b5bf92289b804086 field_1;
};
struct enum_121128004a13691e63519cc26b34b25a_1 {
  uint16_t field_0;
};

// 24015184290033080575162413618386219610
using enum_121128004a13691e63519cc26b34b25a =
    std::variant<enum_121128004a13691e63519cc26b34b25a_0,
                 enum_121128004a13691e63519cc26b34b25a_1>;

// 237314186446701704582585791359993080063
struct struct_b2890cbf12fa2255dd45bbcab73968ff {
  enum_121128004a13691e63519cc26b34b25a field_0;
};

// 338243918424169272477466358438870931970
struct struct_fe7768bac708a98255fe26e1f73c5a02 {
  std::span<std::string_view> field_0; /*span of std::string_view*/
  enum_61dcfd9913b0020b60435675696a78ae field_1;
  std::span<struct_b2890cbf12fa2255dd45bbcab73968ff>
      field_2; /*span of struct_b2890cbf12fa2255dd45bbcab73968ff*/
};

constexpr int32_t constant_56ced5e4a15bd89050bb9674fa2df013_25bf8c35ab1a9d99_4 =
    1;

constexpr int32_t constant_56ced5e4a15bd89050bb9674fa2df013_52f7aa788c769582_4 =
    2;

constexpr double constant_c45c25bfe577a84e0b073a6684adcb7c_adfe931fb778a770_8 =
    1.1;

constexpr std::array<char, 20>
    constant_768979dbca13b33be3652a1a16a4a660_42321706c74ddd67_20 = {
        {80,  111, 105, 110, 116, 32,  99,  111, 111, 114,
         100, 105, 110, 97,  116, 101, 115, 58,  32,  40}};

constexpr std::array<char, 2>
    constant_768979dbca13b33be3652a1a16a4a660_d4d2e7bca1e46caf_2 = {{44, 32}};

constexpr std::array<char, 10>
    constant_768979dbca13b33be3652a1a16a4a660_d91e800f2de409ff_10 = {
        {41, 44, 32, 102, 108, 111, 97, 116, 58, 32}};

constexpr std::array<char, 1>
    constant_768979dbca13b33be3652a1a16a4a660_4ff7a615e1ace5e7_1 = {{10}};

constexpr std::array<std::string_view, 4>
    constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8_data = {
        std::string_view(
            constant_768979dbca13b33be3652a1a16a4a660_42321706c74ddd67_20),
        std::string_view(
            constant_768979dbca13b33be3652a1a16a4a660_d4d2e7bca1e46caf_2),
        std::string_view(
            constant_768979dbca13b33be3652a1a16a4a660_d91e800f2de409ff_10),
        std::string_view(
            constant_768979dbca13b33be3652a1a16a4a660_4ff7a615e1ace5e7_1)};

constexpr const std::array<std::string_view, 4>
    *constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8 =
        &constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8_data;

constexpr std::array<uint8_t, 1>
    constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1_bytes = {0};
constexpr struct_56e7b3f74beafa9b515b585d8ba7905c
    constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1 =
        std::bit_cast<struct_56e7b3f74beafa9b515b585d8ba7905c>(
            constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1_bytes);

void _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0fb956ce54e4ef97E(
    fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1){

  bb0 :
      // Statement: Terminator { source_info: SourceInfo { span:
      // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:152:18:
      // 152:21 (#0), scope: scope[0] }, kind: _0 = <F as
      // FnOnce<()>>::call_once(move _1, const ()) -> [return: bb1, unwind
      // continue] }

  bb1 :
      // Statement: Terminator { source_info: SourceInfo { span:
      // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/hint.rs:482:5:
      // 482:40 (#0), scope: scope[2] }, kind: _2 =
      // std::intrinsics::black_box::<()>(const ()) -> [return: bb2, unwind
      // unreachable] }

  bb2 :
      // Statement: Terminator { source_info: SourceInfo { span:
      // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:158:2:
      // 158:2 (#0), scope: scope[0] }, kind: return }

}

int32_t _ZN4core3ops8function6FnOnce9call_once17h89ffdac548459e5bE(
    closure_89cb9f1b62f39415f2b9d52320e10599 _1) {
  int32_t
      _0; // Alias(Projection, AliasTy { args: [Self/#0, Args/#1], def_id:
          // DefId(2:3922 ~ core[57fa]::ops::function::FnOnce::Output), .. })
  closure_89cb9f1b62f39415f2b9d52320e10599 *_2; // &'{erased} mut Self/#0

bb0:
  // Statement: Debug: Assign((_3, &mut _1))
  // _3 /*&'{erased} mut Self/#0*/ = &mut _1;
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
  // 250:71 (#0), scope: scope[0] }, kind: _0 = <Self as
  // FnMut<Args>>::call_mut(move _3, move _2) -> [return: bb1, unwind: bb3] }

bb1:
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
  // 250:71 (#0), scope: scope[0] }, kind: drop(_1) -> [return: bb2, unwind
  // continue] }

bb2:
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
  // 250:71 (#0), scope: scope[0] }, kind: return }

bb3:
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
  // 250:71 (#0), scope: scope[0] }, kind: drop(_1) -> [return: bb4, unwind
  // terminate(cleanup)] }

bb4:
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
  // 250:71 (#0), scope: scope[0] }, kind: resume }

  return _0;
}

void _ZN4core3ops8function6FnOnce9call_once17h36be117f97b14d0bE(
    fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1){

  bb0 :
      // Statement: Terminator { source_info: SourceInfo { span:
      // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
      // 250:71 (#0), scope: scope[0] }, kind: _0 = move _1() -> [return: bb1,
      // unwind continue] }

  bb1 :
      // Statement: Terminator { source_info: SourceInfo { span:
      // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
      // 250:71 (#0), scope: scope[0] }, kind: return }

}

struct_b2890cbf12fa2255dd45bbcab73968ff
    _ZN4core3fmt2rt8Argument11new_display17h77078538da0a254cE(
        const int32_t *_1) {
  struct_b2890cbf12fa2255dd45bbcab73968ff
      _0; // core::fmt::rt::Argument<'{erased}>
  enum_121128004a13691e63519cc26b34b25a
      _2; // core::fmt::rt::ArgumentType<'{erased}>
  struct_7e41fb6c1562171f6e859c395f73f6f9 _3; // std::ptr::NonNull<()>
  fn_ptr_cf46556a41b87d30b5bf92289b804086
      _4; // Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut
          // std::fmt::Formatter<'^1>) -> std::result::Result<(),
          // std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] }
  fn_ptr_247d8fd692a9b4df9a8c6534257381fd
      _5; // Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) ->
          // std::result::Result<(), std::fmt::Error>, bound_vars:
          // [Region(BrAnon), Region(BrAnon), Region(BrAnon)] }
  const int32_t *_6; // *const T/#1
  const void *_7;    // *const ()

bb0:
  // Statement: Debug: StorageLive(_2)
  // Statement: StorageLive(_2)
  // Statement: Debug: StorageLive(_3)
  // Statement: StorageLive(_3)
  // Statement: Debug: Assign((_6, &raw const (*_1)))
  // _6 /**const T/#1*/ = &raw const (*_1);
  // Statement: Debug: StorageLive(_7)
  // Statement: StorageLive(_7)
  // Statement: Debug: Assign((_7, copy _6 as *const () (PtrToPtr)))
  // _7 /**const ()*/ = copy _6 as *const () (PtrToPtr);
  // Statement: Debug: Assign((_3, NonNull::<()> { pointer: move _7 }))
  // _3 /*std::ptr::NonNull<()>*/ = NonNull::<()> { pointer: move _7 };
  // Statement: Debug: StorageDead(_7)
  // Statement: StorageDead(_7)
  // Statement: Debug: StorageLive(_4)
  // Statement: StorageLive(_4)
  // Statement: Debug: Assign((_5, <T as std::fmt::Display>::fmt as for<'a, 'b,
  // 'c> fn(&'a T, &'b mut std::fmt::Formatter<'c>) -> std::result::Result<(),
  // std::fmt::Error> (PointerCoercion(ReifyFnPointer, Implicit)))) _5 /*Binder
  // { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) ->
  // std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon),
  // Region(BrAnon), Region(BrAnon)] }*/ = <T as std::fmt::Display>::fmt as
  // for<'a, 'b, 'c> fn(&'a T, &'b mut std::fmt::Formatter<'c>) ->
  // std::result::Result<(), std::fmt::Error> (PointerCoercion(ReifyFnPointer,
  // Implicit)); Statement: Debug: Assign((_4, copy _5 as for<'a, 'b> unsafe
  // fn(std::ptr::NonNull<()>, &'a mut std::fmt::Formatter<'b>) ->
  // std::result::Result<(), std::fmt::Error> (Transmute))) _4 /*Binder { value:
  // unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) ->
  // std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon),
  // Region(BrAnon)] }*/ = copy _5 as for<'a, 'b> unsafe
  // fn(std::ptr::NonNull<()>, &'a mut std::fmt::Formatter<'b>) ->
  // std::result::Result<(), std::fmt::Error> (Transmute); Statement: Debug:
  // Assign((_2, core::fmt::rt::ArgumentType::<'_>::Placeholder { value: move
  // _3, formatter: move _4, _lifetime: const PhantomData::<&()> })) _2
  // /*core::fmt::rt::ArgumentType<'{erased}>*/ =
  // core::fmt::rt::ArgumentType::<'_>::Placeholder { value: move _3, formatter:
  // move _4, _lifetime: const PhantomData::<&()> }; Statement: Debug:
  // StorageDead(_4) Statement: StorageDead(_4) Statement: Debug:
  // StorageDead(_3) Statement: StorageDead(_3) Statement: Debug: Assign((_0,
  // core::fmt::rt::Argument::<'_> { ty: move _2 })) _0
  // /*core::fmt::rt::Argument<'{erased}>*/ = core::fmt::rt::Argument::<'_> {
  // ty: move _2 }; Statement: Debug: StorageDead(_2) Statement: StorageDead(_2)
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:112:6:
  // 112:6 (#0), scope: scope[0] }, kind: return }

  return _0;
}

struct_fe7768bac708a98255fe26e1f73c5a02
_ZN4core3fmt2rt38__LT_impl_u20_core__fmt__Arguments_GT_6new_v117h44cfd91a5dd25e61E(
    const std::array<std::string_view, 4> *_1,
    const std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3> *_2) {
  struct_fe7768bac708a98255fe26e1f73c5a02 _0; // std::fmt::Arguments<'{erased}>
  std::span<std::string_view> _3;             // &'{erased} [&'{erased} str]
  std::span<struct_b2890cbf12fa2255dd45bbcab73968ff>
      _4; // &'{erased} [core::fmt::rt::Argument<'{erased}>]

bb0:
  // Statement: Debug: StorageLive(_3)
  // Statement: StorageLive(_3)
  // Statement: Debug: Assign((_3, copy _1 as &[&str] (PointerCoercion(Unsize,
  // Implicit)))) _3 /*&'{erased} [&'{erased} str]*/ = copy _1 as &[&str]
  // (PointerCoercion(Unsize, Implicit)); Statement: Debug: StorageLive(_4)
  // Statement: StorageLive(_4)
  // Statement: Debug: Assign((_4, copy _2 as &[core::fmt::rt::Argument<'_>]
  // (PointerCoercion(Unsize, Implicit)))) _4 /*&'{erased}
  // [core::fmt::rt::Argument<'{erased}>]*/ = copy _2 as
  // &[core::fmt::rt::Argument<'_>] (PointerCoercion(Unsize, Implicit));
  // Statement: Debug: Assign((_0, Arguments::<'_> { pieces: move _3, fmt: const
  // Option::<&[core::fmt::rt::Placeholder]>::None, args: move _4 })) _0
  // /*std::fmt::Arguments<'{erased}>*/ = Arguments::<'_> { pieces: move _3,
  // fmt: const Option::<&[core::fmt::rt::Placeholder]>::None, args: move _4 };
  // Statement: Debug: StorageDead(_4)
  // Statement: StorageDead(_4)
  // Statement: Debug: StorageDead(_3)
  // Statement: StorageDead(_3)
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:215:6:
  // 215:6 (#0), scope: scope[0] }, kind: return }

  return _0;
}

void _ZN10empty_main4main17hd0b442567fa79b17E() {
  int32_t _1;                                  // i32
  int32_t _2;                                  // i32
  int32_t _3;                                  // i32
  std::tuple<int32_t, bool> _4;                // (i32, bool)
  struct_3f52890329a70117381258e502e2d0dd _5;  // Point
  std::tuple<int32_t, bool> _6;                // (i32, bool)
  double _7;                                   // f64
  double _8;                                   // f64
  struct_fe7768bac708a98255fe26e1f73c5a02 _10; // std::fmt::Arguments<'{erased}>
  std::tuple<const int32_t *, const int32_t *, const double *>
      _11;            // (&'{erased} i32, &'{erased} i32, &'{erased} f64)
  const int32_t *_12; // &'{erased} i32
  const int32_t *_13; // &'{erased} i32
  const double *_14;  // &'{erased} f64
  std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3>
      _15; // [core::fmt::rt::Argument<'{erased}>; 3_usize]
  struct_b2890cbf12fa2255dd45bbcab73968ff
      _16; // core::fmt::rt::Argument<'{erased}>
  struct_b2890cbf12fa2255dd45bbcab73968ff
      _17; // core::fmt::rt::Argument<'{erased}>
  struct_b2890cbf12fa2255dd45bbcab73968ff
      _18; // core::fmt::rt::Argument<'{erased}>
  const std::array<std::string_view, 4>
      *_19; // &'{erased} [&'{erased} str; 4_usize]
  const std::array<struct_b2890cbf12fa2255dd45bbcab73968ff, 3>
      *_20; // &'{erased} [core::fmt::rt::Argument<'{erased}>; 3_usize]
  const int32_t *_21; // &'{erased} i32
  const int32_t *_22; // &'{erased} i32
  const double *_23;  // &'{erased} f64

bb0:
  // Statement: Debug: Assign((_2, const 1_i32))
  _2 /*i32*/ = constant_56ced5e4a15bd89050bb9674fa2df013_25bf8c35ab1a9d99_4;
  // Statement: Debug: Assign((_3, const 2_i32))
  _3 /*i32*/ = constant_56ced5e4a15bd89050bb9674fa2df013_52f7aa788c769582_4;
  // Statement: Debug: Assign((_4, AddWithOverflow(copy _2, copy _3)))
  _4 /*(i32, bool)*/ = CHECKED_ADD(std::bit_cast<int32_t>(_2 /*i32*/),
                                   std::bit_cast<int32_t>(_3 /*i32*/));
  // Statement: Terminator { source_info: SourceInfo { span: src/main.rs:10:14:
  // 10:21 (#0), scope: scope[3] }, kind: assert(!move (_4.1: bool), "attempt to
  // compute `{} + {}`, which would overflow", move _2, move _3) -> [success:
  // bb1, unwind continue] }

bb1:
  // Statement: Debug: Assign((_1, move (_4.0: i32)))
  _1 /*i32*/ = std::move(std::get<0>(_4));
  // Statement: Debug: Assign((_5, Point { x: copy _1, y: const 3_i32 }))
  // _5 /*Point*/ = Point { x: copy _1, y: const 3_i32 };
  // Statement: Debug: Assign((_6, AddWithOverflow(copy (_5.0: i32), copy _1)))
  _6 /*(i32, bool)*/ = CHECKED_ADD(std::bit_cast<int32_t>(_5.field_0),
                                   std::bit_cast<int32_t>(_1 /*i32*/));
  // Statement: Terminator { source_info: SourceInfo { span: src/main.rs:12:5:
  // 12:15 (#0), scope: scope[5] }, kind: assert(!move (_6.1: bool), "attempt to
  // compute `{} + {}`, which would overflow", copy (_5.0: i32), copy _1) ->
  // [success: bb2, unwind continue] }

bb2:
  // Statement: Debug: Assign(((_5.0: i32), move (_6.0: i32)))
  _5.field_0 = std::move(std::get<0>(_6));
  // Statement: Debug: Assign((_8, const 1.1000000000000001f64))
  _8 /*f64*/ = constant_c45c25bfe577a84e0b073a6684adcb7c_adfe931fb778a770_8;
  // Statement: Debug: Assign((_7, Add(move _8, const 1.1000000000000001f64)))
  _7 /*f64*/ = std::move(_8 /*f64*/) +
               constant_c45c25bfe577a84e0b073a6684adcb7c_adfe931fb778a770_8;
  // Statement: Debug: Assign((_12, &(_5.0: i32)))
  // _12 /*&'{erased} i32*/ = &(_5.0: i32);
  // Statement: Debug: Assign((_13, &(_5.1: i32)))
  // _13 /*&'{erased} i32*/ = &(_5.1: i32);
  // Statement: Debug: Assign((_14, &_7))
  // _14 /*&'{erased} f64*/ = &_7;
  // Statement: Debug: Assign((_11, (move _12, move _13, move _14)))
  // _11 /*(&'{erased} i32, &'{erased} i32, &'{erased} f64)*/ = (move _12, move
  // _13, move _14); Statement: Debug: Assign((_21, deref_copy (_11.0: &i32)))
  // _21 /*&'{erased} i32*/ = deref_copy (_11.0: &i32);
  // Statement: Terminator { source_info: SourceInfo { span: src/main.rs:15:35:
  // 15:37 (#7), scope: scope[7] }, kind: _16 =
  // core::fmt::rt::Argument::<'_>::new_display::<i32>(copy _21) -> [return:
  // bb3, unwind continue] }

bb3:
  // Statement: Debug: Assign((_22, deref_copy (_11.1: &i32)))
  // _22 /*&'{erased} i32*/ = deref_copy (_11.1: &i32);
  // Statement: Terminator { source_info: SourceInfo { span: src/main.rs:15:39:
  // 15:41 (#7), scope: scope[7] }, kind: _17 =
  // core::fmt::rt::Argument::<'_>::new_display::<i32>(copy _22) -> [return:
  // bb4, unwind continue] }

bb4:
  // Statement: Debug: Assign((_23, deref_copy (_11.2: &f64)))
  // _23 /*&'{erased} f64*/ = deref_copy (_11.2: &f64);
  // Statement: Terminator { source_info: SourceInfo { span: src/main.rs:15:51:
  // 15:53 (#7), scope: scope[7] }, kind: _18 =
  // core::fmt::rt::Argument::<'_>::new_display::<f64>(copy _23) -> [return:
  // bb5, unwind continue] }

bb5:
  // Statement: Debug: Assign((_15, [move _16, move _17, move _18]))
  // _15 /*[core::fmt::rt::Argument<'{erased}>; 3_usize]*/ = [move _16, move
  // _17, move _18]; Statement: Debug: Assign((_19, const main::promoted[0]))
  _19 /*&'{erased} [&'{erased} str; 4_usize]*/ =
      constant_768979dbca13b33be3652a1a16a4a660_bd60acb658c79e45_8;
  // Statement: Debug: Assign((_20, &_15))
  // _20 /*&'{erased} [core::fmt::rt::Argument<'{erased}>; 3_usize]*/ = &_15;
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:28:
  // 143:61 (#7), scope: scope[8] }, kind: _10 = core::fmt::rt::<impl
  // Arguments<'_>>::new_v1::<4, 3>(copy _19, copy _20) -> [return: bb6, unwind
  // continue] }

bb6:
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:9:
  // 143:62 (#4), scope: scope[6] }, kind: _9 = _print(move _10) -> [return:
  // bb7, unwind continue] }

bb7:
  // Statement: Terminator { source_info: SourceInfo { span: src/main.rs:16:2:
  // 16:2 (#0), scope: scope[0] }, kind: return }
}

struct_b2890cbf12fa2255dd45bbcab73968ff
_ZN4core3fmt2rt8Argument11new_display17h3a95d3486de58403E(const double *_1) {
  struct_b2890cbf12fa2255dd45bbcab73968ff
      _0; // core::fmt::rt::Argument<'{erased}>
  enum_121128004a13691e63519cc26b34b25a
      _2; // core::fmt::rt::ArgumentType<'{erased}>
  struct_7e41fb6c1562171f6e859c395f73f6f9 _3; // std::ptr::NonNull<()>
  fn_ptr_cf46556a41b87d30b5bf92289b804086
      _4; // Binder { value: unsafe fn(std::ptr::NonNull<()>, &'^0 mut
          // std::fmt::Formatter<'^1>) -> std::result::Result<(),
          // std::fmt::Error>, bound_vars: [Region(BrAnon), Region(BrAnon)] }
  fn_ptr_2bec5f878167b37e1176aaff171ffdb9
      _5; // Binder { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) ->
          // std::result::Result<(), std::fmt::Error>, bound_vars:
          // [Region(BrAnon), Region(BrAnon), Region(BrAnon)] }
  const double *_6; // *const T/#1
  const void *_7;   // *const ()

bb0:
  // Statement: Debug: StorageLive(_2)
  // Statement: StorageLive(_2)
  // Statement: Debug: StorageLive(_3)
  // Statement: StorageLive(_3)
  // Statement: Debug: Assign((_6, &raw const (*_1)))
  // _6 /**const T/#1*/ = &raw const (*_1);
  // Statement: Debug: StorageLive(_7)
  // Statement: StorageLive(_7)
  // Statement: Debug: Assign((_7, copy _6 as *const () (PtrToPtr)))
  // _7 /**const ()*/ = copy _6 as *const () (PtrToPtr);
  // Statement: Debug: Assign((_3, NonNull::<()> { pointer: move _7 }))
  // _3 /*std::ptr::NonNull<()>*/ = NonNull::<()> { pointer: move _7 };
  // Statement: Debug: StorageDead(_7)
  // Statement: StorageDead(_7)
  // Statement: Debug: StorageLive(_4)
  // Statement: StorageLive(_4)
  // Statement: Debug: Assign((_5, <T as std::fmt::Display>::fmt as for<'a, 'b,
  // 'c> fn(&'a T, &'b mut std::fmt::Formatter<'c>) -> std::result::Result<(),
  // std::fmt::Error> (PointerCoercion(ReifyFnPointer, Implicit)))) _5 /*Binder
  // { value: fn(&'^0 T/#1, &'^1 mut std::fmt::Formatter<'^2>) ->
  // std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon),
  // Region(BrAnon), Region(BrAnon)] }*/ = <T as std::fmt::Display>::fmt as
  // for<'a, 'b, 'c> fn(&'a T, &'b mut std::fmt::Formatter<'c>) ->
  // std::result::Result<(), std::fmt::Error> (PointerCoercion(ReifyFnPointer,
  // Implicit)); Statement: Debug: Assign((_4, copy _5 as for<'a, 'b> unsafe
  // fn(std::ptr::NonNull<()>, &'a mut std::fmt::Formatter<'b>) ->
  // std::result::Result<(), std::fmt::Error> (Transmute))) _4 /*Binder { value:
  // unsafe fn(std::ptr::NonNull<()>, &'^0 mut std::fmt::Formatter<'^1>) ->
  // std::result::Result<(), std::fmt::Error>, bound_vars: [Region(BrAnon),
  // Region(BrAnon)] }*/ = copy _5 as for<'a, 'b> unsafe
  // fn(std::ptr::NonNull<()>, &'a mut std::fmt::Formatter<'b>) ->
  // std::result::Result<(), std::fmt::Error> (Transmute); Statement: Debug:
  // Assign((_2, core::fmt::rt::ArgumentType::<'_>::Placeholder { value: move
  // _3, formatter: move _4, _lifetime: const PhantomData::<&()> })) _2
  // /*core::fmt::rt::ArgumentType<'{erased}>*/ =
  // core::fmt::rt::ArgumentType::<'_>::Placeholder { value: move _3, formatter:
  // move _4, _lifetime: const PhantomData::<&()> }; Statement: Debug:
  // StorageDead(_4) Statement: StorageDead(_4) Statement: Debug:
  // StorageDead(_3) Statement: StorageDead(_3) Statement: Debug: Assign((_0,
  // core::fmt::rt::Argument::<'_> { ty: move _2 })) _0
  // /*core::fmt::rt::Argument<'{erased}>*/ = core::fmt::rt::Argument::<'_> {
  // ty: move _2 }; Statement: Debug: StorageDead(_2) Statement: StorageDead(_2)
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:112:6:
  // 112:6 (#0), scope: scope[0] }, kind: return }

  return _0;
}

int32_t
_ZN4core3ops8function6FnOnce40call_once_u7b__u7b_vtable_shim_u7d__u7d_17hd25a61e515fdb9a3E(
    closure_89cb9f1b62f39415f2b9d52320e10599 *_1) {
  int32_t
      _0; // Alias(Projection, AliasTy { args: [Self/#0, Args/#1], def_id:
          // DefId(2:3922 ~ core[57fa]::ops::function::FnOnce::Output), .. })

bb0:
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
  // 250:71 (#0), scope: scope[0] }, kind: _0 = <Self as
  // FnOnce<Args>>::call_once(move (*_1), move _2) -> [return: bb1, unwind
  // continue] }

bb1:
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5:
  // 250:71 (#0), scope: scope[0] }, kind: return }

  return _0;
}

intptr_t _ZN3std2rt10lang_start17habb8eed6ce6072afE(
    fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6 _1, intptr_t _2,
    const const uint8_t **_3, uint8_t _4) {
  intptr_t _0;   // isize
  const int *_5; // &'{erased} dyn [Binder { value: Trait(std::ops::Fn<()>),
                 // bound_vars: [] }, Binder { value:
                 // Projection(ExistentialProjection { def_id: DefId(2:3922 ~
                 // core[57fa]::ops::function::FnOnce::Output), args: [()],
                 // term: Term::Ty(i32), use_existential_projection_new_instead:
                 // () }), bound_vars: [] }, Binder { value:
                 // AutoTrait(DefId(2:3647 ~ core[57fa]::marker::Sync)),
                 // bound_vars: [] }, Binder { value: AutoTrait(DefId(2:49650 ~
                 // core[57fa]::panic::unwind_safe::RefUnwindSafe)), bound_vars:
                 // [] }] + '{erased}
  const closure_89cb9f1b62f39415f2b9d52320e10599
      *_6; // &'{erased} Closure(DefId(1:47 ~
           // std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder {
           // value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder
           // { value: fn() -> T/#0, bound_vars: [] },)])
  closure_89cb9f1b62f39415f2b9d52320e10599
      _7; // Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}),
          // [T/#0, i8, Binder { value: extern "RustCall" fn(()) -> i32,
          // bound_vars: [] }, (Binder { value: fn() -> T/#0, bound_vars: []
          // },)])

bb0:
  // Statement: Debug: StorageLive(_5)
  // Statement: StorageLive(_5)
  // Statement: Debug: StorageLive(_7)
  // Statement: StorageLive(_7)
  // Statement: Debug: Assign((_7,
  // {closure@/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:10:
  // 206:17} { 0: copy _1 })) _7 /*Closure(DefId(1:47 ~
  // std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8, Binder { value: extern
  // "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder { value: fn() -> T/#0,
  // bound_vars: [] },)])*/ =
  // {closure@/home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:10:
  // 206:17} { 0: copy _1 }; Statement: Debug: Assign((_6, &_7)) _6 /*&'{erased}
  // Closure(DefId(1:47 ~ std[c81d]::rt::lang_start::{closure#0}), [T/#0, i8,
  // Binder { value: extern "RustCall" fn(()) -> i32, bound_vars: [] }, (Binder
  // { value: fn() -> T/#0, bound_vars: [] },)])*/ = &_7; Statement: Debug:
  // Assign((_5, copy _6 as &dyn std::ops::Fn() -> i32 + std::marker::Sync +
  // std::panic::RefUnwindSafe (PointerCoercion(Unsize, Implicit)))) _5
  // /*&'{erased} dyn [Binder { value: Trait(std::ops::Fn<()>), bound_vars: []
  // }, Binder { value: Projection(ExistentialProjection { def_id: DefId(2:3922
  // ~ core[57fa]::ops::function::FnOnce::Output), args: [()], term:
  // Term::Ty(i32), use_existential_projection_new_instead: () }), bound_vars:
  // [] }, Binder { value: AutoTrait(DefId(2:3647 ~ core[57fa]::marker::Sync)),
  // bound_vars: [] }, Binder { value: AutoTrait(DefId(2:49650 ~
  // core[57fa]::panic::unwind_safe::RefUnwindSafe)), bound_vars: [] }] +
  // '{erased}*/ = copy _6 as &dyn std::ops::Fn() -> i32 + std::marker::Sync +
  // std::panic::RefUnwindSafe (PointerCoercion(Unsize, Implicit)); Statement:
  // Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:205:5:
  // 210:6 (#0), scope: scope[0] }, kind: _0 = rt::lang_start_internal(move _5,
  // move _2, move _3, move _4) -> [return: bb1, unwind continue] }

bb1:
  // Statement: Debug: StorageDead(_7)
  // Statement: StorageDead(_7)
  // Statement: Debug: StorageDead(_5)
  // Statement: StorageDead(_5)
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:211:2:
  // 211:2 (#0), scope: scope[0] }, kind: return }

  return _0;
}

struct_56e7b3f74beafa9b515b585d8ba7905c
_ZN54__LT__LP__RP__u20_as_u20_std__process__Termination_GT_6report17h44442036e6046436E() {
  struct_56e7b3f74beafa9b515b585d8ba7905c _0; // std::process::ExitCode

bb0:
  // Statement: Debug: Assign((_0, const
  // ExitCode(std::sys::process::unix::common::ExitCode(0_u8))))
  _0 /*std::process::ExitCode*/ =
      constant_56e7b3f74beafa9b515b585d8ba7905c_dc58fdc2e5c5babe_1;
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/process.rs:2549:6:
  // 2549:6 (#0), scope: scope[0] }, kind: return }

  return _0;
}

int32_t
_ZN3std2rt10lang_start28__u7b__u7b_closure_u7d__u7d_17h083d4027b6fa8260E(
    const closure_89cb9f1b62f39415f2b9d52320e10599 *_1) {
  int32_t _0;                                 // i32
  struct_56e7b3f74beafa9b515b585d8ba7905c _2; // std::process::ExitCode
  fn_ptr_4ef64bc9380831fe7304bc1cc3a8ebe6
      _4;     // Binder { value: fn() -> T/#0, bound_vars: [] }
  uint8_t _5; // u8

bb0:
  // Statement: Debug: StorageLive(_2)
  // Statement: StorageLive(_2)
  // Statement: Debug: StorageLive(_3)
  // Statement: StorageLive(_3)
  // Statement: Debug: StorageLive(_4)
  // Statement: StorageLive(_4)
  // Statement: Debug: Assign((_4, copy ((*_1).0: fn() -> T)))
  //_4 /*Binder { value: fn() -> T/#0, bound_vars: [] }*/ =
  //std::bit_cast<int32_t>((*(_1))(/* TODO: function args */));
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18:
  // 206:75 (#0), scope: scope[0] }, kind: _3 =
  // std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> T, T>(move _4)
  // -> [return: bb1, unwind continue] }

bb1:
  // Statement: Debug: StorageDead(_4)
  // Statement: StorageDead(_4)
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18:
  // 206:84 (#0), scope: scope[0] }, kind: _2 = <T as Termination>::report(move
  // _3) -> [return: bb2, unwind continue] }

bb2:
  // Statement: Debug: StorageDead(_3)
  // Statement: StorageDead(_3)
  // Statement: Debug: StorageLive(_5)
  // Statement: StorageLive(_5)
  // Statement: Debug: Assign((_5, copy ((_2.0:
  // std::sys::process::unix::common::ExitCode).0: u8)))
  _5 /*u8*/ = std::bit_cast<uint8_t>(_2.field_0.field_0);
  // Statement: Debug: Assign((_0, move _5 as i32 (IntToInt)))
  // _0 /*i32*/ = move _5 as i32 (IntToInt);
  // Statement: Debug: StorageDead(_5)
  // Statement: StorageDead(_5)
  // Statement: Debug: StorageDead(_2)
  // Statement: StorageDead(_2)
  // Statement: Terminator { source_info: SourceInfo { span:
  // /home/nambiar/.rustup/toolchains/nightly-2025-06-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:93:
  // 206:93 (#0), scope: scope[0] }, kind: return }

  return _0;
}
int main() { _ZN10empty_main4main17hd0b442567fa79b17E(); }
