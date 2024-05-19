macro_rules! register_commands {
    ( $( $cmd:ident ),* ) => {
        $(
            mod $cmd;
            pub use $cmd::$cmd;
        )*
    };
}

register_commands!(user_info, about, avatar, balance, give, register, xkcd);
