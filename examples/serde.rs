use anyhow::Result;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Builder, Serialize, Deserialize)]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
    #[builder(default = "32")]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

impl User {
    fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

fn main() -> Result<()> {
    let user = User::build()
        .name("Alice")
        .email("yu@answesome.com")
        .age(30)
        .skill("programming")
        .skill("debugging")
        .build()?;

    let json = serde_json::to_string(&user)?;
    let user1 = serde_json::from_str(&json)?;
    assert_eq!(user, user1);

    Ok(())
}
