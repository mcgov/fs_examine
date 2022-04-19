#[macro_export]
macro_rules! define_flags {
    ($sz:ty,$a:ident,$b:literal) => {
        pub const $a: $sz = $b;
    };
    ($sz:ty,$($a:ident,literal),+) => {
        crate::define_flags!($sz, $a);
    };
}

#[macro_export]
macro_rules! declare_bitfield {
    ($sz:ty,$struct:ident, $name:ident, $($a:ident+literal),+ ) => {
        pub struct $struct {
            bitfield: $sz,
        }

        impl $struct {
            pub fn match_flag(&self, flag: $sz) -> bool {
                (self.bitfield & flag) == flag
            }
        }
        pub mod $name {
            crate::define_flags!($sz, $($a),+);
        }
    };
}

#[macro_export]
macro_rules! declare_enum {
    ($sz:ty,$struct:ident, $name:ident, $($a:ident,$b:literal),+ ) => {
        pub struct $struct {
            type_flag: $sz,
        }

        impl $struct {
            pub fn match_flag(&self, flag: $sz) -> bool {
                self.type_flag == flag
            }
        }
        pub mod $name {
            define_flags!($sz, $($a,$b),+)
        }
    };
}
