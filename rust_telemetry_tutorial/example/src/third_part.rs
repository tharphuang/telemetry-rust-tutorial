#[tracing::instrument]
pub(crate) fn some_func(arg: i32) -> i32 {
    for idx in 0..arg {
        tracing::info!(
            name = "third_part::some_func",
            func_step = idx + 1,
            "third_part_func running in step :{}",
            idx+1
        );
        some_func_next();
    }
    arg
}

#[tracing::instrument]
fn some_func_next() {
    tracing::warn!(name = "third_part::some_func_next")
}
