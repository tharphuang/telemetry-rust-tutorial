use std::{sync::Arc, time::Duration};

use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self, RandomIdGenerator, Sampler, Tracer},
    Resource,
};
use tokio::runtime::Runtime;
use tonic::{Request, Status};

#[derive(Debug, Clone)]
pub struct TracerConfig {
    pub name: String,
    pub endpoint: String,
    pub trace_ratio: f64,
    pub time_out: u64,
}

impl TracerConfig {
    pub fn default() -> TracerConfig {
        TracerConfig {
            name: "test".to_owned(),
            endpoint: "http://127.0.0.1:4333".to_owned(),
            trace_ratio: 1.0,
            time_out: 5,
        }
    }
}

fn auth_interceptor(mut request: Request<()>) -> Result<Request<()>, Status> {
    let token = "dev";
    request
        .metadata_mut()
        .append("authorization", token.parse().unwrap());

    Ok(request)
}

pub struct CustomTracer {
    pub tracer: Tracer,
}

pub fn init_tracer(cfg: TracerConfig, multi_runtime: Arc<Runtime>) {
    // 使用tokio run_time 来初始化tracer
    let tracer = multi_runtime.block_on(async {
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(cfg.endpoint.as_str())
                    .with_interceptor(auth_interceptor)
                    .with_timeout(Duration::from_secs(cfg.time_out)),
            )
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::TraceIdRatioBased(cfg.trace_ratio)) // trace 速率控制
                    .with_id_generator(RandomIdGenerator::default())
                    .with_max_events_per_span(64)
                    .with_max_attributes_per_span(16)
                    .with_max_events_per_span(16)
                    .with_resource(Resource::new(vec![KeyValue::new("service.name", cfg.name)])),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .expect("init trace failed");
        tracer
    });

    //全局注册provider
    let provider = tracer.clone().provider().unwrap();
    global::set_tracer_provider(provider);

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new("INFO"))
    //     .with(fmt::layer())
    //     .with(tracing_opentelemetry::layer().with_tracer(tracer))
    //     .init();
}

pub fn close_tracer() {
    // 关闭全局 provider
    global::shutdown_tracer_provider();
}
