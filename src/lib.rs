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

    pub struct UserBotPair(User, Bot);

    impl UserBotPair {
        pub fn map(users: Vec<User>, bots: Vec<Bot>) -> Vec<UserBotPair> {
            // todo
            vec![]
        }
    }
}

use domain::*;

pub fn main() {
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
    let new_bot = Bot {
        full_name: "bot 2".into(),
    };

    let new_bots = vec![
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

    let new_users = vec![User {
        first_name: "dan".into(),
        last_name: "person".into(),
    }];

    let mutation = mutation! {
        insert!(&new_bot),
        insert_many!(&new_bots),
        insert_many!(&new_users),
        invalidate_key("entites_with_names")
        and_then(query)
    };

    let res = infra::run(mutation);

    // [
    //   {
    //      user: {
    //          first_name: "Dan",
    //          last_name: "Person",
    //      },
    //      bot: {
    //          full_name: "bot 1"
    //      },
    //   },
    //   {
    //      user: {
    //          first_name: "Dan",
    //          last_name: "Person",
    //      },
    //      bot: {
    //          full_name: "bot 2"
    //      },
    //   },
    // ]
}
