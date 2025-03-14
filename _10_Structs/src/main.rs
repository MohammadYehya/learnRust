struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(name: &str, email: &str) -> User {
    User {
        active: true,
        username: String::from(name),
        email: String::from(email),
        sign_in_count: 1,
    }
}
fn main() {
    {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user1.username = String::from("MohammadYehya");

        println!(
            "{},{},{},{}",
            user1.active, user1.username, user1.email, user1.sign_in_count
        );
    }

    {
        let mut user2 = build_user("Someone", "abc@example.com");
        user2.sign_in_count += 1;

        println!(
            "{},{},{},{}",
            user2.active, user2.username, user2.email, user2.sign_in_count
        );
    }

    {
        let user3 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
        let user4 = User {
            email: String::from("xyz@example.net"),
            ..user3 // This allows to auto fill the remaining entries from user3.
        };
        // However, calling User {} actually moves data. Therefore, we can not use user3 anymore.
        println!(
            "{},{},{},{}",
            user4.active, user4.username, user4.email, user4.sign_in_count
        );
    }

    {
        // Tuple Structs
        struct Color(i8, i8, i8);
        struct Point(i8, i8, i8);

        let black = Color(0, 0, 0);
        let point = Point(0, 0, 0);

        println!("{},{},{}", black.0, black.1, black.2);
        println!("{},{},{}", point.0, point.1, point.2);

        // let (x,y,z) = point;     //Doesnt Work
        let Point(x, y, z) = point;
        println!("{},{},{}", x, y, z);
    }

    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // To create a method we use the impl statement
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let r = Rectangle {
            width: 50,
            height: 30,
        };
        println!("The area is: {}", r.area());

        impl Rectangle {
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }
        }
        // Here the function square is not a method. It is an associated function with no self parameter. They are similar to static functions in C++

        let sq = Rectangle::square(3);
        println!("Width: {} Height: {}", sq.width, sq.height);
        
    }
}
