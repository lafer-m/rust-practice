use std::{future::Future, pin::Pin, task::{Context, Poll}};

struct MyFuture {}

impl Future for MyFuture {
    type Output = u16;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>  {
        Poll::Ready(18)
    } 
}

#[tokio::test]
async fn test_futrue() {
    let f = MyFuture {}.await;
    println!("result is {f}");
}