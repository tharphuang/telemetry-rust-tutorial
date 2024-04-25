use std::sync::Arc;
use trace_msg::custom_span::SpanInterface;
use trace_msg::custom_tracer::{close_tracer, init_tracer, TracerConfig};
use trace_msg::new_custom_span;
use tracing::Level;
mod msg;
use msg::MessageType;
mod third_part;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let multi_runtime = Arc::new(rt);

    let trace_cfg = TracerConfig::default();
    init_tracer(trace_cfg, multi_runtime.clone());

    write();
    //third_part::some_func(2);

    close_tracer();
}

async fn async_put_callback(ctx: opentelemetry::Context) {
    let span = new_custom_span!("async_put_callback").with_ctx(ctx);
    //thread::sleep(Duration::from_millis(500));
    let mut total = 0;
    for i in 0..100 {
        total += i
    }
    println!("{}", total);
    let yig_call_back_info = msg::BackendPutData {
        cost_time: 12,
        host: "http://localhost:8190".to_owned(),
        fs_id: "test".to_owned(),
        obj_key: "a".to_owned(),
    };
    span.set_event(
        MessageType::BackendPutDataMsg.as_str_name(),
        &yig_call_back_info,
    )
    .close();
}

async fn backend_put(ctx: opentelemetry::Context) {
    let span = new_custom_span!("backend_put");
    span.with_ctx(ctx);
    let new_ctx = span.get_ctx();
    async_put_callback(new_ctx.clone()).await;
    async_put_callback(new_ctx.clone()).await;
    async_put_callback(new_ctx.clone()).await;
    let yig_put_info = msg::BackendPutData {
        cost_time: 12,
        host: "http://localhost:8190".to_owned(),
        fs_id: "test".to_owned(),
        obj_key: "a".to_owned(),
    };
    span.set_event(&MessageType::BackendPutDataMsg.as_str_name(), &yig_put_info)
        .close();
}

fn meta_put(ctx: opentelemetry::Context) {
    let span = new_custom_span!("warn", "meta_put").with_ctx(ctx);
    let mete_put_info = msg::CreateIno {
        name: "update_seg".to_string(),
        cost_time: 12,
        host: "http://localhost:8190".to_owned(),
        fs_id: "test".to_owned(),
        parent_ino: "123".to_string(),
    };
    span.set_event(&MessageType::CreateInoMsg.as_str_name(), &mete_put_info)
        .close();
}

fn write() {
    let span = new_custom_span!("info", "write");
    let ctx = span.get_ctx();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime current thread new successfully.");
    rt.block_on(async { backend_put(ctx.clone()).await });
    println!("yig put completed");

    meta_put(ctx);
    println!("meta put completed");
    span.close();
}
