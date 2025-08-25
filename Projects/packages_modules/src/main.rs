use rand::Rng;
// 这个模块是私有的，这意味着没有外部crate 可以访问这个模块。
// 但是，由于它在当前 crate 的根模块下，
// 因此当前 crate 中的任何模块都可以访问该模块中任何公有可见性程序项。
mod crate_helper_module {

    // 这个函数可以被当前 crate 中的任何东西使用
    pub fn crate_helper() {}

    // 此函数*不能*被用于 crate 中的任何其他模块中。它在 `crate_helper_module` 之外不可见，
    // 因此只有当前模块及其后代可以访问它。
    fn implementation_detail() {}
}

// 此函数“对根模块是公有”的，这意味着它可被链接了此crate 的其他crate 使用。
pub fn public_api() {}

// 与 'public_api' 类似，此模块是公有的，因此其他的crate 是能够看到此模块内部的。
pub mod submodule {
    use crate::crate_helper_module;

    pub fn my_method() {
        // 本地crate 中的任何程序项都可以通过上述两个规则的组合来调用辅助模块里的公共接口。
        crate_helper_module::crate_helper();
    }

    // 此函数对任何不是 `submodule` 的后代的模块都是隐藏的
    fn my_implementation() {}

    #[cfg(test)]
    mod test {

        #[test]
        fn test_my_implementation() {
            // 因为此模块是 `submodule` 的后代，因此允许它访问 `submodule` 内部的私有项，而不会侵犯隐私权。
            super::my_implementation();
        }
    }
}

fn main() {}