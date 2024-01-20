mod greeting;

use greeting::greet;

// #[tokio::main]
// async fn main() {
//     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//     println!("{}", greet("chennai geeks"));
// }

fn main() {
    let body = async {
        println!("task started");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("{}", greet("chennai geeks"));
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime");
        let handle = rt.spawn(body);
        println!("This should print");
        rt.block_on(handle).expect("Failed running the Runtime");
    }
}


