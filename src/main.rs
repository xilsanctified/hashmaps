mod chapter8 {
    pub mod game_score {

        use std::collections::HashMap;

        pub fn create_game() -> HashMap<String, i32> {
            let mut t = HashMap::new();
            t.insert(String::from("Team Yellow"), 0);
            t.insert(String::from("Team Blue"), 0);
            t
        }

        pub fn show_score(t: &mut HashMap<String, i32>) {
            for (key, value) in t {
                println!("{}: {}", key, value);
            }
        }

        pub fn increment_point(k: String, t: &mut HashMap<String, i32>) {
            let score = t.entry(k).or_insert(0);
            *score += 1 // dereference score so I can apply a new value to it.
        }
    }
}

use self::chapter8::game_score::{create_game, increment_point, show_score};
fn main() {
    let mut x = create_game();
    show_score(&mut x);
    increment_point(String::from("Team Blue"), &mut x);
    show_score(&mut x);
}
