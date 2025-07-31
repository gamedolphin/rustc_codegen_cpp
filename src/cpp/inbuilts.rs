pub const CHECKED_ADD: &str = r#"
#define CHECKED_ADD(a, b)                                                     \
    [&]() {                                                                   \
        using result_type = decltype(a);                                      \
        result_type result;                                                   \
        bool overflow = ckd_add(&result, a, b);                               \
        return std::make_tuple(result, overflow);                             \
    }()
"#;

pub const CHECKED_SUB: &str = r#"
#define CHECKED_SUB(a, b)                                                     \
    [&]() {                                                                   \
        using result_type = decltype(a);                                      \
        result_type result;                                                   \
        bool overflow = ckd_sub(&result, a, b);                               \
        return std::make_tuple(result, overflow);                             \
    }()
"#;

pub const CHECKED_MUL: &str = r#"
#define CHECKED_MUL(a, b)                                                     \
    [&]() {                                                                   \
        using result_type = decltype(a);                                      \
        result_type result;                                                   \
        bool overflow = ckd_mul(&result, a, b);                               \
        return std::make_tuple(result, overflow);                             \
    }()
"#;

pub const FAT_PTR: &str = r#"
struct FatPtr {
    const void* data;
    void* fn;
};
"#;

pub fn get_inbuilt_functions() -> Vec<String> {
    vec![
        CHECKED_ADD.to_string(),
        CHECKED_SUB.to_string(),
        CHECKED_MUL.to_string(),
        FAT_PTR.to_string(),
    ]
}
