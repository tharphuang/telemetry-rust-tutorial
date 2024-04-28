#[macro_export]
macro_rules! new_span {
    ($tracer:expr, $name:expr) => {{
        let code_line = module_path!();
        $crate::custom_span::new($tracer, $name, format!("{}:{}", file!(), line!()))
    }};
    ($tracer:expr, $name:expr, $ctx:expr) => {
        $crate::custom_span::new_with_ctx($tracer, $name, $ctx, format!("{}:{}", file!(), line!()))
    };
}
