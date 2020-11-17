mod infra {
    use super::domain::*;
    pub fn run(query: Query) -> ExecutionResult {
        //let (filter, selection) = query;

        // Generate SQL query with filters hmm...
        let sql = r#"
        SELECT {selection} FROM entities
        {where clause}
        "#;

        let result = execute(sql);

        ExecutionResult(query.clone(), result)
    }

    fn execute(_query: &str) -> Payload {
        Payload(vec![])
    }
}

mod domain {
    use serde_json::Value;

    #[derive(Clone)]
    pub struct Entity(Value);

    #[derive(Clone)]
    pub struct Selection(pub Vec<Value>);

    #[derive(Clone)]
    pub struct Payload(pub Vec<Entity>);

    #[derive(Clone)]
    pub struct Query(pub Box<Filter>, pub Selection);

    #[derive(Clone)]
    pub struct ExecutionResult(pub Query, pub Payload);

    #[derive(Clone)]
    pub enum Filter {
        Equal(Value, Value),
        And(Box<Filter>, Box<Filter>),
    }

    impl Filter {
        pub fn by_owner_id(owner_id: String) -> Box<Filter> {
            Box::new(Filter::Equal(
                Value::String("owner_id".to_string()),
                Value::String(owner_id),
            ))
        }
        pub fn by_type(type_: String) -> Box<Filter> {
            Box::new(Filter::Equal(
                Value::String("type".to_string()),
                Value::String(type_),
            ))
        }
        pub fn and(self, other: Box<Filter>) -> Box<Filter> {
            Box::new(Filter::And(Box::new(self), other))
        }
    }

    #[derive(Loadable, Creatable)]
    pub struct User {
        first_name: String,
        last_name: String,
    }

    #[derive(Loadable, Creatable)]
    pub struct Bot {
        full_name: String,
    }

    #[derive(Loadable)]
    #[from(User, Bot)]
    pub struct UserBotPair(User, Bot);

    impl User {
        pub fn new(first_name: String, last_name: String) -> User {
            User {
                first_name,
                last_name,
            }
        }

        selection! {
            first_name: String,
            last_name: String
        }

        filter! {
            kind "user"
        }
    }
}

use domain::*;
use serde_json::Value;

pub fn main() {
    let filter = Filter::by_owner_id("dan".into()).and(Filter::by_type("user".into()));
    let selection = Selection(vec![Value::String("owner_id".to_string())]);

    // This just feels like a bad idea all around.
    // Load values
    let query = query! {
        query! {
            filter {
                by_owner_id("dan"),
            }
            extends User::query
        },
        query! {
            extends Bot::query
        },
        map(UserBotPairs::new),
        cache_key(UserBotPairs::cache_key)
    };

    let _ = infra::run(query);

    // Create new values
    let value = Bot {
        full_name: "bot 2".into(),
    };

    let other_values = vec![
        Bot {
            full_name: "bot 3".into(),
        },
        Bot {
            full_name: "bot 4".into(),
        },
        Bot {
            full_name: "bot 5".into(),
        },
    ];

    let mutation = mutation! {
        insert!(&value),
        insert_many!(&other_values),
        invalidate_key("entites_with_names")
        and_then(query)
    };

    let _ = infra::run(mutation);
}
