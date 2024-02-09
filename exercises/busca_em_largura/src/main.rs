use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone)]
struct Person {
    name: String,
    sell_mango: bool,
}

fn main() {
    let mut persons: VecDeque<Person> = VecDeque::from([
        Person {
            name: String::from("Alice"),
            sell_mango: false,
        },
        Person {
            name: String::from("Bob"),
            sell_mango: false,
        },
        Person {
            name: String::from("Charlie"),
            sell_mango: false,
        },
    ]);

    let person_friends: HashMap<String, Vec<Person>> = HashMap::from([
        (
            "Alice".to_string(),
            vec![
                Person {
                    name: String::from("Peggy"),
                    sell_mango: false,
                },
                Person {
                    name: String::from("Romeo"),
                    sell_mango: false,
                },
            ],
        ),
        (
            "Bob".to_string(),
            vec![
                Person {
                    name: String::from("Peggy"),
                    sell_mango: false,
                },
                Person {
                    name: String::from("Clarie"),
                    sell_mango: true,
                },
            ],
        ),
        (
            "Charlie".to_string(),
            vec![
                Person {
                    name: String::from("Tom"),
                    sell_mango: false,
                },
                Person {
                    name: String::from("Jhonny"),
                    sell_mango: true,
                },
            ],
        ),
    ]);

    let mut persons_verified: Vec<String> = vec![];

    while !persons.is_empty() {
        let person: Person = persons.pop_front().unwrap();

        if !persons_verified.contains(&person.name.to_string()) {
            if person.sell_mango {
                println!("{} vende manga!", person.name);
                break;
            } else {
                let friends: Vec<Person> = match person_friends.get(person.name.as_str()) {
                    Some(friends) => friends.clone(),
                    None => vec![],
                };

                for friend in friends {
                    persons.push_back(friend);
                }
            }

            persons_verified.push(person.name.to_string());
        }
    }
}
