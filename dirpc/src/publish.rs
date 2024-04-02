#[macro_export]
macro_rules! publish {
    (%GET_FUNCTION_NAME% -> $function_split:ident; $($function_path:ident;)+) => {
        $crate::publish!(%GET_FUNCTION_NAME% -> $($function_path;)+)
    };
    (%GET_FUNCTION_NAME% -> $function_split:ident;) => {
        stringify!($function_split)
    };
    ($server_var:ident => {include $module_path:path; $($other:tt)*}) => {
        $module_path(&mut $server_var);
        $crate::publish!($server_var => {$($other)*})
    };
    ($server_var:ident => {mod $name:ident = $module_path:path; $($other:tt)*}) => {
        $module_path($crate::server::ServerAddFunctionality::add_namespace(&mut $server_var, stringify!($name)));
        $crate::publish!($server_var => {$($other)*})
    };
    ($server_var:ident => {mod $name:ident { $($content:tt)* } $($other:tt)*}) => {
        {
            let mut $server_var = $crate::server::ServerAddFunctionality::add_namespace(&mut $server_var, stringify!($name));
            $crate::publish!($server_var => {$($content)*});
        }
        $crate::publish!($server_var => {$($other)*})
    };
    ($server_var:ident => {$function_name:ident ($($function_arg:ident),*) = $function_path:path; $($other:tt)*}) => {
        $crate::server::ServerAddFunctionality::add_function(&mut $server_var, stringify!($function_name), ($(stringify!($function_arg),)*), $function_path);
        $crate::publish!($server_var => {$($other)*})
    };
    ($server_var:ident => {$function_name:ident ($($function_arg:ident),*); $($other:tt)*}) => {
        $crate::server::ServerAddFunctionality::add_function(&mut $server_var, stringify!($function_name), ($(stringify!($function_arg),)*), $function_name);
        $crate::publish!($server_var => {$($other)*})
    };
    ($server_var:ident => {$($path_first_arg:ident)? $(::$function_path:ident)* ($($function_arg:ident),*); $($other:tt)*}) => {
        $crate::server::ServerAddFunctionality::add_function(&mut $server_var, $crate::publish!(%GET_FUNCTION_NAME% -> $($function_path;)*), ($(stringify!($function_arg),)*), $($path_first_arg)? $(::$function_path)*);
        $crate::publish!($server_var => {$($other)*})
    };
    ($server_var:ident => {}) => {};
}

#[macro_export]
macro_rules! namespace {
    ($name:ident<$ctx:ty, $req:ty> {$($other:tt)*}) => {
        pub fn $name(mut server: impl $crate::server::ServerAddFunctionality<$ctx, $req>) {
            let mut server = $crate::server::ServerAddFunctionality::add_namespace(&mut server, stringify!($name));
            $crate::publish!(server => {$($other)*});
        }
    };
    (($fn_name:ident <= $name:literal)<$ctx:ty, $req:ty> {$($other:tt)*}) => {
        pub fn $fn_name(mut server: impl $crate::server::ServerAddFunctionality<$ctx, $req>) {
            let mut server = $crate::server::ServerAddFunctionality::add_namespace(&mut server, $name);
            $crate::publish!(server => {$($other)*});
        }
    };
}