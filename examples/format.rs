use std::fmt::ArgumentV1;

async fn foo(_: String) {}

fn bar() -> impl Send {
    async move {
        foo(format!("")).await;
    }
}

fn main() {}