#![allow(dead_code)]

use zvariant::Type;
use zvariant_derive::Type;

#[test]
fn derive_unit_struct() {
    #[derive(Type)]
    struct FooF(f64);

    assert_eq!(FooF::signature(), "d")
}

#[test]
fn derive_struct() {
    #[derive(Type)]
    struct TestStruct {
        name: String,
        age: u8,
        blob: Vec<u8>,
    }

    assert_eq!(TestStruct::signature(), "(syay)")
}

#[test]
fn derive_enum() {
    #[repr(u32)]
    #[derive(Type)]
    enum RequestNameFlags {
        AllowReplacement = 0x01,
        ReplaceExisting = 0x02,
        DoNotQueue = 0x04,
    }

    assert_eq!(RequestNameFlags::signature(), "u")
}
