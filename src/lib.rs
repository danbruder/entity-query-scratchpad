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
}

use domain::*;
use serde_json::Value;

pub fn main() {
    let filter = Filter::by_owner_id("dan".into()).and(Filter::by_type("user".into()));
    let selection = Selection(vec![Value::String("owner_id".to_string())]);

    struct User {
        first_name: String,
        last_name: String,
    }
    struct Bot {
        full_name: String,
    }

    let user_map_fn = |first_name: String, last_name: String| -> User {
        User {
            first_name,
            last_name,
        }
    };

    let bot_map_fn = |full_name: String| -> Bot { Bot { full_name } };
    let both_map_fn = |users: Vec<User>, bots: Vec<Bot>| -> Vec<(User, Bot)> { vec![] };

    // This just feels like a bad idea all around.
    let query = query! {
        sub_query! {
            filter {
                by_owner_id("dan"),
                by_type("user"),
            }
            select {
                first_name: String
                last_name: String
            }
            map(map_fn)
        },
        sub_query! {
            filter {
                by_type("bot"),
            }
            select {
                full_name: String
            }
            map(bot_map_fn)
        },
        map(both_map_fn)
    };

    let _ = infra::run(query);
}
