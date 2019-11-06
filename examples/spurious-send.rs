use std::future::Future;
use std::pin::Pin;

fn f<T>(_: &T) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    unimplemented!()
}

pub fn g<T: Sync>(x: &'static T) -> impl Future<Output = ()> + Send {
    async move { f(x).await }
}

fn main() {}