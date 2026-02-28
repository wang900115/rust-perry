#![allow(unused)]

/*
    - lower memory consumption
    - no limit on number of threads 
    - good for IO bound computations
*/

async fn g1() -> u32 {
    1
}

async fn g2() -> u32 {
    2 
}

async fn g3() -> u32 {
    3
}

async fn f() {
    let (r1, r2) = tokio::join!(g1(),g2()); // simultaneously
    // let r1 = g1().await;
    // let r2 = g2().await;
    let r3 = g3().await;

}

#[tokio::main]
async fn main() {
    f().await;
}