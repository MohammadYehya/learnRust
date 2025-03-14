fn main() {
    // Enums give you a way of saying a value is one of a possible set of values
    {
        enum IPKind {
            V4,
            V6,
        }

        let ip1 = IPKind::V4;
        let ip2 = IPKind::V6;

        fn route(ip_kind: IPKind) -> IPKind {
            ip_kind
        }
        route(ip1);
        route(ip2);
    }

    // Enums can also hold data
    {
        #[derive(Debug)]
        enum IPKind {
            V4(String),
            V6(String),
        }

        let ip1 = IPKind::V4(String::from("127.0.0.1"));
        let ip2 = IPKind::V6(String::from("::1"));

        fn route(ip_kind: IPKind) {
            println!("{:#?}", ip_kind);
        }
        route(ip1);
        route(ip2);
    }

    // Enums are usefull in the sense that each variant can have different types and amounts of associated data which we wouldn’t be able to with a struct
    {
        #[derive(Debug)]
        enum IPKind {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IPKind::V4(127, 0, 0, 1);

        let loopback = IPKind::V6(String::from("::1"));

        fn route(ip_kind: IPKind) {
            println!("{:#?}", ip_kind);
        }
        route(home);
        route(loopback);
    }

    // Enums can also have methods
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {}
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // The Option Enum
    {
        // Rust doesnt have an implementation of Null. Instead it uses an Enum.
        /*
            enum Option<T> {
                None,
                Some(T),
            }
        */

        let some_number = Some(5);
        let some_char = Some('e');

        let absent_number: Option<i32> = None;
    }
}
