use async_trait::async_trait;

// trait AsyncTrait {
//     async fn f() {
//         println!("Couldn't compile")
//     }
// }

#[async_trait]
trait AsyncTrait {
    async fn f() {
        println!("Could compile")
    }
}

#[async_trait]
pub trait AsyncTraitDelegate {
    async fn f(&self);
}

struct Runner {}

#[async_trait]
impl AsyncTraitDelegate for Runner {
    async fn f(&self) {
        println!("Hello, async-trait");
    }
}

fn main() {
    println!("Hello, world!");
}
