use msg::MessageType;
use std::sync::Arc;
use std::time::Instant;
use trace_msg::{
    custom_span::{CustomContext, Interface},
    custom_tracer::{close_tracer, init_tracer, TracerConfig},
    new_span,
};

mod msg;
mod third_part;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let multi_runtime = Arc::new(rt);

    let tracer_cfg = TracerConfig::default();
    init_tracer(tracer_cfg, multi_runtime.clone());

    test_func();
    third_part::some_func(2);

    close_tracer();
}

async fn async_callback(ctx: CustomContext) {
    let mut span = new_span!("test", "async_callback", ctx);
    let now = Instant::now();
    let mut total = 0;
    for i in 0..100 {
        total += i
    }
    println!("{}", total);
    let callback_msg = msg::Callback {
        cost_time: now.elapsed().as_micros() as i64,
        host: "http://localhost:80".to_owned(),
        calllback: "async_callback".to_owned(),
        arg: "None".to_owned(),
    };
    span.set_custom_attribute(MessageType::CallbackMsg.as_str_name(), &callback_msg);
    span.close()
}

async fn async_f1(ctx: CustomContext) {
    let mut span = new_span!("test", "async_f1", ctx);
    let now = Instant::now();
    let n_ctx = span.get_ctx();
    async_callback(n_ctx.clone()).await;
    async_callback(n_ctx.clone()).await;
    async_callback(n_ctx.clone()).await;
    let func_msg = msg::Func {
        cost_time: now.elapsed().as_micros() as i64,
        host: "http://localhost:80".to_owned(),
        func_name: "async_f1".to_owned(),
        arg: "None".to_owned(),
    };
    span.set_custom_attribute(&MessageType::FuncMsg.as_str_name(), &func_msg);
    span.close()
}

fn f2(ctx: CustomContext) {
    let mut span = new_span!("test", "f2", ctx);
    let now = Instant::now();
    let func_msg = msg::Func {
        cost_time: now.elapsed().as_micros() as i64,
        host: "http://localhost:8190".to_owned(),
        func_name: "f2".to_owned(),
        arg: "None".to_owned(),
    };
    span.set_custom_attribute(&MessageType::FuncMsg.as_str_name(), &func_msg);
    span.close()
}

fn test_func() {
    let mut span = new_span!("test", "test_func");
    let now = Instant::now();
    let ctx = span.get_ctx();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime current thread new successfully.");
    rt.block_on(async { async_f1(ctx.clone()).await });
    println!("async_f1 completed");

    f2(ctx);
    println!("f2 completed");
    span.set_attribute("cost_time", now.elapsed().as_micros().to_string());
    span.close();
}
