use colored::*;
use lazy_static::lazy_static;
use std::net::SocketAddr;
lazy_static! {
    static ref CLIENT: volo_gen::volo::redis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::redis::RedisServiceClientBuilder::new("volo-redis")
            .address(addr)
            .build()
    };
}
#[volo::main]

async fn main() {
    tracing_subscriber::fmt::init();
    println!("welocome to volo redis client");
    println!("Usage:");
    println!("type `get key` to get value");
    println!("type `set key value` to set value");
    println!("type `del key` to del value");
    println!("type `ping` to ping server");
    println!("type `exit` to exit\n");

    loop {
        //读入 一行
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let args: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let op = &args[0];
        let resp = match op.to_lowercase().as_str() {
            "get" => {
                let key = args[1].clone();
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Get,
                    arguments: Some(vec![key.into()]),
                };
                CLIENT.redis_command(req).await.unwrap()
            }
            "set" => {
                let key = args[1].clone();
                let value = args[2].clone();
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Set,
                    arguments: Some(vec![key.into(), value.into()]),
                };
                CLIENT.redis_command(req).await.unwrap()
            }
            "del" => {
                let key = args[1].clone();
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Del,
                    arguments: Some(vec![key.into()]),
                };
                CLIENT.redis_command(req).await.unwrap()
            }
            "ping" => {
                let req = volo_gen::volo::redis::RedisRequest {
                    cmd: volo_gen::volo::redis::RedisCommand::Ping,
                    arguments: None,
                };
                CLIENT.redis_command(req).await.unwrap()
            }
            "exit" => {
                println!("bye");
                return;
            }
            _ => {
                println!("not support command");
                continue;
            }
        };
        if resp.ok {
            print!("{}", "SUCCESS: ".green());
            println!("{}\n", resp.data.unwrap());
        } else {
            print!("{}", "ERROR: ".red());
            println!("{}\n", resp.data.unwrap());
        }
    }
}
