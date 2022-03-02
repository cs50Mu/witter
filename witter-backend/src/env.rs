#[derive(Debug, Eq, PartialEq)]
pub enum Env {
    Production,
    Development,
    Test,
}

pub fn current() -> Env {
    match std::env::var("APP_ENV").unwrap().as_str() {
        "production" => Env::Production,
        "development" => Env::Development,
        "test" => Env::Test,
        _ => panic!("unknown environment"),
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::tests::test_helper::test_setup;

    #[async_std::test]
    async fn in_test_env_during_tests() {
        test_setup().await;

        let env = current();
        assert_eq!(env, Env::Test);
    }
}
