use rocket::tokio;
use rust_backend;
// use rocket::{self, launch};

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     rust_backend::rocket().launch().await
// }

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     rust_backend::rocket()
//         .launch()
//         .await?;
//     Ok(())
// }
// #[launch]
// async fn main() -> _ {
//     rust_backend::rocket()
// }
  

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let _ = rust_backend::rocket().launch().await;
    });
}