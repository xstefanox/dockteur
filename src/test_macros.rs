#[macro_export]
macro_rules! map {
    {$($k: expr => $v: expr),* $(,)?} => {
        {
            let map: std::collections::HashMap<String, String> = vec! [
                $(
                    ($k.to_string(), $v.to_string()),
                )*
            ].iter().cloned().collect();

            map
        }
    };
}

#[macro_export]
macro_rules! u16nz {
    ($a:expr) => {
        NonZeroU16::new($a).unwrap()
    };
}
