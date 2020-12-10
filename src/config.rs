#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub processes: Cow<'static, [_Config__processes]>,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct _Config__processes {
    pub capabilities: Cow<'static, [Cow<'static, str>]>,
    pub gid: i64,
    pub id: Cow<'static, str>,
    pub ipcs: Cow<'static, [Cow<'static, str>]>,
    pub params: Cow<'static, [Cow<'static, str>]>,
    pub path: Cow<'static, str>,
    pub uid: i64,
}

pub const CONFIG: Config = Config {
    processes: Cow::Borrowed(&[_Config__processes {
            capabilities: Cow::Borrowed(&[Cow::Borrowed("RAW_ETH")]),
            gid: 1000,
            id: Cow::Borrowed("PURPLE"),
            ipcs: Cow::Borrowed(&[Cow::Borrowed("/tmp/purple2red")]),
            params: Cow::Borrowed(&[Cow::Borrowed("2")]),
            path: Cow::Borrowed("sleep"),
            uid: 1000,
        }, _Config__processes {
            capabilities: Cow::Borrowed(&[Cow::Borrowed("RAW_ETH")]),
            gid: 1000,
            id: Cow::Borrowed("PURPLE"),
            ipcs: Cow::Borrowed(&[Cow::Borrowed("/tmp/purple2red")]),
            params: Cow::Borrowed(&[]),
            path: Cow::Borrowed("ls"),
            uid: 1000,
        }]),
};
