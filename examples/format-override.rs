extern crate alloc;

macro_rules! format {
    ($($arg:tt)*) => {{
        let res = alloc::fmt::format(alloc::__export::format_args!($($arg)*));
        res
    }}
}

async fn foo(_: String) {}

fn bar() -> impl Send {
    async move {
        foo(format!("")).await;
    }
}

fn main() {}