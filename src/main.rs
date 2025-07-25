use serde_json::json;
use tokio::select;
#[tokio::main]
async fn main() {
    let k = "key";
    let v = "value";
    let data = json!({
   k: "value"
  });

    {
        select! {
                            v = nothing() => {

                            println!("{v}");
            }
        }
    }
}


async fn nothing() -> i32 {
    123
}
