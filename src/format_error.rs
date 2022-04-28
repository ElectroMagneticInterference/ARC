#[macro_export]
macro_rules! log {
    ( $function:expr ) => {{
        let error = ::windows::core::Error::from(windows::core::HRESULT(unsafe {
            $function.try_into().unwrap()
        }));

        if error != ::windows::core::Error::OK {
            println!(
                "{} failed with error code {:?}: {}",
                &stringify!($function)[0..stringify!($function)
                    .find("(")
                    .unwrap_or(stringify!($function).len())],
                error.code(),
                error.message()
            );
        }
    }};
    ($function:expr, $extra_info:expr) => {{
        let error = ::windows::core::Error::from(windows::core::HRESULT(unsafe {
            $function.try_into().unwrap()
        }));

        if error != ::windows::core::Error::OK {
            println!(
                "{} failed with error code {:?}: {} {}",
                &stringify!($function)[0..stringify!($function)
                    .find("(")
                    .unwrap_or(stringify!($function).len())],
                error.code(),
                error.message(),
                $extra_info
            );
        }
    }};
    ($function:expr, $extra_info:expr,*) => {{
        let error = ::windows::core::Error::from(windows::core::HRESULT(unsafe {
            $function.try_into().unwrap()
        }));

        if error != ::windows::core::Error::OK {
            println!(
                "{} failed with error code {:?}: {} {}" + (" {}",)* $extra_info.len(),
                &stringify!($function)[0..stringify!($function)
                    .find("(")
                    .unwrap_or(stringify!($function).len())],
                error.code(),
                error.message()
                $(, $extra_info)*
            );
        }
    }};
}
