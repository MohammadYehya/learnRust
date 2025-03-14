#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    // Matching allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
    // Similar to switch case in C++

    {
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {state:?}!");
                    25
                }
            }
        }

        let coin = Coin::Quarter(UsState::Alaska);
        println!("{}", value_in_cents(coin));
    }

    // Matching is often used with Option<T>
    {
        fn plus_one(x: Option<i8>) -> Option<i8> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        // Remember that matches are always exhaustive meaning that they need to go through all possible values
        let five = Some(5);
        let _six = plus_one(five);
        let _none = plus_one(None);
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(_num_spaces: u8) {}
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }

    // The match operator can also be used in the form of an if let statement.
    // This is done to reduce boiler plate code
    {
        let config_max = Some(3i8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }

        // If we add an else block, it would act as - => ()
        let coin = Coin::Quarter(UsState::Alaska);
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {state:?}!");
        } else {
            count += 1;
        }
        println!("{count}")
    }

    // We can shorten it further with the let else statement
    {
        let coin = Coin::Quarter(UsState::Alaska);
        fn describe_state_quarter(coin: Coin) -> Option<String> {
            let Coin::Quarter(state) = coin else {
                return None;
            };
            Some(format!("{state:?}"))
        }
        if let Some(s) = describe_state_quarter(coin) {
            println!("{s}");
        } else {
            println!("None");
        }
    }
}
