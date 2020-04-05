mod address;
pub use address::*;

mod message;
pub use message::*;

mod message_field;
pub use message_field::*;

mod message_fields;
pub use message_fields::*;

mod connection;
pub use connection::*;

mod utils;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use zvariant::{Array, FromVariant};
    use zvariant::{Variant, VariantValue};

    #[test]
    fn basic_connection() {
        crate::Connection::new_session()
            .map_err(|e| {
                println!("error: {}", e);

                e
            })
            .unwrap();
    }

    #[test]
    fn freedesktop_api() {
        let mut connection = crate::Connection::new_session()
            .map_err(|e| {
                println!("error: {}", e);

                e
            })
            .unwrap();

        if std::env::var("GET_MACHINE_ID").unwrap_or(String::from("1")) == "1" {
            let reply = connection
                .call_method::<()>(
                    Some("org.freedesktop.DBus"),
                    "/org/freedesktop/DBus",
                    Some("org.freedesktop.DBus.Peer"),
                    "GetMachineId",
                    None,
                )
                .unwrap();

            assert!(reply
                .body_signature()
                .map(|s| s == <&str>::signature())
                .unwrap());
            let id: &str = reply.body().unwrap().unwrap();
            println!("Machine ID: {}", id);
        }

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "NameHasOwner",
                Some(&"org.freedesktop.DBus"),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s == bool::signature())
            .unwrap());
        assert!(reply.body::<bool>().unwrap().unwrap());

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetNameOwner",
                Some(&"org.freedesktop.DBus"),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s == <&str>::signature())
            .unwrap());
        let owner: &str = reply.body().unwrap().unwrap();
        println!("Owner of 'org.freedesktop.DBus' is: {}", owner);

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus.Properties"),
                "GetAll",
                Some(&"org.freedesktop.DBus"),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s.as_str() == "a{sv}")
            .unwrap());
        let hashmap: HashMap<&str, Variant> = reply.body().unwrap().unwrap();

        // "Features" property
        let features = Array::from_variant_ref(&hashmap["Features"]).unwrap();
        println!("org.freedesktop.DBus.Features on /org/freedesktop/DBus:");
        for feature in features.get() {
            print!(" {}", <&str>::from_variant_ref(feature).unwrap());
        }
        println!();

        // "Interfaces" property
        let interfaces = Array::from_variant_ref(&hashmap["Interfaces"]).unwrap();
        println!("org.freedesktop.DBus.Interfaces on /org/freedesktop/DBus:");
        for interface in interfaces.get() {
            print!(" {}", <&str>::from_variant_ref(interface).unwrap());
        }
        println!();
    }
}
