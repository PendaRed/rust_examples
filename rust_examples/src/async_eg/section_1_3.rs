// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;
use async_std::task;

async fn hello_world() {
    let mut i = 1;
    for n in 1..10000 {
        i = n;
    }
    println!("hello, world!");
}

async fn slow_hello_world(name: &str, with_yield:bool) {
    let mut i = 1;
    for n in 1..100000 {
        i = n;
        if n%10000==0 {
            println!("...{}", name);
            if with_yield {
                // @TODO
            }
        }
    }
    println!("Slow hello {}", name);
}

pub fn main_section_1_3() {
    let future1 = hello_world(); // Nothing is printed
    block_on(future1); // `future` is run and "hello, world!" is printed

    block_on(async {
        let f1 = slow_hello_world("fut2", true);
        let f2 = slow_hello_world("fut3", true);

        futures::join!(f1, f2);
    }
    );
}