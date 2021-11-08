use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model::book;

#[derive(Serialize, Deserialize)]
pub struct FeedRequest {
    UserId: i64,
    ClientIp: String,
    UrlPath: String,
    RequestUrl: String,
    RequestUri: String,
    Action: String,
    DeviceId: i64,
    AppId: i32,
    AppName: String,
    AppVersionName: String,
    AppVersion: String,
    VersionCode: String,
    Host: String,
    ClientParams: HashMap<String, Vec<String>>,
    Authors: Vec<book::Author>,
    Header: HashMap<String, Vec<String>>,
    PostParams: HashMap<String, Vec<String>>,
    FeedBiz: i64,
    BizParams: String,
    // unused fields # 19 to 254
    Base: Base,
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    LogID: String,
    Caller: String,
    Addr: String,
    Client: String,
    //TrafficEnv: TrafficEnv,
    Extra: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct TrafficEnv {
    Open: bool,
    Env: String,
}
