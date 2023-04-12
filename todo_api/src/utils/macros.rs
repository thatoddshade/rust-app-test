macro_rules! map {
    ($($key:expr => $v:expr),* $(,)?) => {{
        let mut m = ::std::collections::HashMap::new();
        $(m.insert($key, $v);)+
        m
    }};
}

pub(crate) use map;