use std::{thread::sleep, time::Duration};
use trace_msg::{
    custom_span::{CustomContext, Interface},
    new_span,
};

pub(crate) fn some_func(arg: i32) -> i32 {
    let mut span = new_span!("", "spme func");
    let ctx = span.get_ctx();
    for idx in 0..arg {
        some_func_next(ctx.clone(), idx);
    }
    span.set_attribute("arg", arg.to_string());
    span.close();
    arg
}

fn some_func_next(ctx: CustomContext, i: i32) {
    let mut span = new_span!("", "some_func_next", ctx);
    sleep(Duration::from_millis(50));
    span.set_attribute("arg", i.to_string());
    span.close();
}
