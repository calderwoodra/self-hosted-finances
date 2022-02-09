use reqwest::Response;
// use dotenv::dotenv;
// use std::env;

pub enum Operation {
    Get,
    Post,
    Put,
    Delete,
}

// pub async fn request<T: serde::de::Deserialize<'_>>(suffix: &str, operation: Operation) -> T {
//     // dotenv().ok();
//     // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     // let root = "http://127.0.0.1:8000/";
//     // let url = format!(root + "{}", suffix);
//     // let mut res = reqwest::Client::new();
//     // let res = match operation {
//     //     Operation::Get => res.get(url),
//     //     Operation::Post => res.post(url),
//     //     Operation::Put => res.put(url),
//     //     Operation::Delete => res.delete(url),
//     // };
//     // let text = res.header("Accept", "application/json").send().await?.text().await?;
//     // let t: T = serde_json::from_str(&text).unwrap();
//     // return t;
// }
