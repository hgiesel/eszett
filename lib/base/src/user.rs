#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
}

pub fn create_user(
    name: &str,
) -> User {
    User { name: name.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let username = "alice";
        let user = create_user(username);

        assert_eq!(user, User { name: "alice".to_string() });
    }
}