#![feature(impl_trait_in_assoc_type)]
use anyhow::anyhow;
use pilota::FastStr;
use std::collections::HashMap;
use std::sync::Mutex;
use volo_gen::volo::redis::{RedisCommand, RedisResponse};

pub const DEFAULT_ADDR: &str = "127.0.0.1:8080";
pub struct S {
    pub map: Mutex<HashMap<String, String>>,
}

#[volo::async_trait]
impl volo_gen::volo::redis::RedisService for S {
    async fn redis_command(
        &self,
        _req: volo_gen::volo::redis::RedisRequest,
    ) -> ::core::result::Result<volo_gen::volo::redis::RedisResponse, ::volo_thrift::AnyhowError>
    {
        println!("recive redis command:{:?} {:?}", _req.cmd, _req.arguments);
        match _req.cmd {
            RedisCommand::Get => {
                // println! {"get"};
                if let Some(args) = _req.arguments {
                    if args.len() != 1 {
                        return Ok(RedisResponse {
                            ok: false,
                            data: Some(FastStr::from("err,参数数量不等于1")),
                        });
                    } else {
                        let key = args[0].as_str();

                        match self.map.lock().unwrap().get(key) {
                            Some(v) => {
                                // println!("{} -> {}", key, v);
                                Ok(RedisResponse {
                                    ok: true,
                                    data: Some(FastStr::from(v.clone())),
                                })
                            }
                            None => Ok(RedisResponse {
                                ok: false,
                                data: Some(FastStr::from("err,没有找到对应的key")),
                            }),
                        }
                    }
                } else {
                    Err(anyhow!("你提供的参数有问题，我无法正常解析"))
                }
            }
            RedisCommand::Set => {
                // println! {"set"};
                //如果参数数量不等于2 err
                if let Some(args) = _req.arguments {
                    if args.len() != 2 {
                        Ok(RedisResponse {
                            ok: false,
                            data: Some(FastStr::from("err,参数数量不等于2")),
                        })
                    } else {
                        let key = args[0].as_str();
                        let value = args[1].as_str();

                        self.map
                            .lock()
                            .unwrap()
                            .insert(key.to_string(), value.to_string());
                        Ok(RedisResponse {
                            ok: true,
                            data: Some(FastStr::from("set 成功")),
                        })
                    }
                } else {
                    Err(anyhow!("你提供的参数有问题，我无法正常解析"))
                }
            }

            RedisCommand::Del => {
                // println! {"del"};
                if let Some(args) = _req.arguments {
                    if args.len() != 1 {
                        Ok(RedisResponse {
                            ok: false,
                            data: Some(FastStr::from("err,参数数量不等于1")),
                        })
                    } else {
                        let key = args[0].as_str();
                        self.map.lock().unwrap().remove(key);
                        Ok(RedisResponse {
                            ok: true,
                            data: Some(FastStr::from("del 成功")),
                        })
                    }
                } else {
                    Err(anyhow!("你提供的参数有问题，我无法正常解析"))
                }
            }
            RedisCommand::Ping => {
                // println! {"ping"};

                Ok(RedisResponse {
                    ok: true,
                    data: Some(FastStr::from("ping ok")),
                })
            }
            _ => {
                // println! {"not support"};
                Ok(RedisResponse {
                    ok: true,
                    data: Some(FastStr::from("not support")),
                })
            }
        }
    }
}
