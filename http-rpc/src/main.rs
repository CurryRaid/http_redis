use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
// use serde::Deserialize;
use volo_gen::volo::redis::{RedisServiceClient, RedisServiceClientBuilder};

use volo_redis::DEFAULT_ADDR;

type RpcClient = RedisServiceClient;
type RpcClientBuilder = RedisServiceClientBuilder;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = DEFAULT_ADDR.parse().unwrap();
    let rpc_cli = RpcClientBuilder::new("volo_redis").address(addr).build();

    // build the application with router
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/get/:keys", get(get_key).with_state(rpc_cli.clone()))
        .route(
            "/set",
            get(show_set_form).post(set_key).with_state(rpc_cli.clone()),
        )
        .route("/del", get(show_del_form).post(del_key).with_state(rpc_cli));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> (StatusCode, &'static str) {
    (StatusCode::OK, "pong")
}

/// Get a key
async fn get_key(Path(key): Path<String>, State(rpc_cli): State<RpcClient>) -> Response {
    let req = volo_gen::volo::redis::RedisRequest {
        cmd: volo_gen::volo::redis::RedisCommand::Get,
        arguments: Some(vec![key.into()]),
    };
    if rpc_cli.redis_command(req).await.unwrap().ok {
        (StatusCode::OK, "found").into_response()
    } else {
        (StatusCode::NOT_FOUND, "not NOT_FOUND").into_response()
    }
}

// #[derive(Deserialize, Debug)]
// // struct FormKey {
// //     key: String,
// // }

/// Show the form for set a key
async fn show_set_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/set" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

/// Set a key
async fn set_key(State(rpc_cli): State<RpcClient>, setkey: String) -> Response {
    //以等号分割FormKey.key，前面是key，后面是value
    let args: Vec<String> = setkey.split('=').map(|s| s.to_string()).collect();
    let key = args[0].clone();
    let value = args[1].clone();

    let req = volo_gen::volo::redis::RedisRequest {
        cmd: volo_gen::volo::redis::RedisCommand::Set,
        arguments: Some(vec![key.into(), value.into()]),
    };
    if rpc_cli.redis_command(req).await.unwrap().ok {
        (StatusCode::OK, "set ok").into_response()
    } else {
        (StatusCode::NOT_FOUND, "set err").into_response()
    }
}

async fn show_del_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/del" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn del_key(State(rpc_cli): State<RpcClient>, key: String) -> (StatusCode, &'static str) {
    let args: Vec<String> = key.split('=').map(|s| s.to_string()).collect();

    let key = args[1].clone();
    // println!("removing {}", key);
    let req = volo_gen::volo::redis::RedisRequest {
        cmd: volo_gen::volo::redis::RedisCommand::Del,
        arguments: Some(vec![key.into()]),
    };
    if rpc_cli.redis_command(req).await.unwrap().ok {
        // println!("del ok");
        (StatusCode::OK, "del ok")
    } else {
        (StatusCode::NOT_FOUND, "del err")
    }
}
