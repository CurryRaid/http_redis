use reqwest::{header::CONTENT_TYPE, StatusCode};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let pong = client
        .get("http://localhost:3000/ping")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("pong: {}", pong);
    let body = "foo=bar";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(set.status(), 200);
    println!("set success");
    let get = client
        .get("http://localhost:3000/get/foo")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(get, "found");
    println!("get success");

    let body = "key=foo";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(del.status(), 200);
    println!("del success");

    let get = client
        .get("http://localhost:3000/get/foo")
        .send()
        .await
        .unwrap();
    // println!("get: {:?}", get);
    assert_eq!(get.status(), StatusCode::NOT_FOUND);
    assert_eq!(get.text().await.unwrap(), "not found");
    println!("test success");
}
