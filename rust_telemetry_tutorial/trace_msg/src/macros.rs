
/// # Examples
///
/// Creating a new span:
/// ```
/// # use trace_msg::new_span;
/// # fn main() {
/// let span = new_span!("my tracer", "my span");
/// // or with context, children of the context
/// let ctx = span.get_ctx();
/// let span2 = new_span!("my tracer", "my span 2", ctx);
/// # }
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
