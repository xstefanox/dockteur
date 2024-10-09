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
macro_rules! assert_ok {
    ( $x:expr ) => {{
        assertables::assert_ok!($x);
        $x.unwrap()
    }};
}

#[macro_export]
macro_rules! assert_err {
    ( $x:expr ) => {{
        assertables::assert_err!($x);
        $x.unwrap_err()
    }};
}
