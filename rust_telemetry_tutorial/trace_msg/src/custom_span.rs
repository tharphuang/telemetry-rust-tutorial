use opentelemetry::Context;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub trait SpanInterface {
    fn get_ctx(&self) -> Context;
    fn with_ctx(&self, ctx: Context) -> Self;
    fn enter(&self);
    fn set_event<T: ?Sized + serde::Serialize>(&self, msg_type: &str, value: &T) -> Self;
    fn close(self);
}

#[derive(Clone)]
pub struct CustomSpan {
    pub span: Span,
}

/// Creating a new_custom_span:
/// ```
/// # use trace_msg::new_custom_span;
/// # fn main() {
/// //trace_level(trace,debug,info,warn,error)
/// let span = new_custom_span!("info", "my span");
/// // or default span trace_level = info
/// let span = snew_custom_span!("my span");
/// // do work inside the span...
/// # }
/// ```
#[macro_export]
macro_rules! new_custom_span {
    ($lvl:expr, $name:expr) => {{
        match $lvl {
            "trace" => trace_msg::custom_span::CustomSpan {
                span: tracing::span!(target: module_path!(), Level::TRACE, $name),
            },
            "debug" => trace_msg::custom_span::CustomSpan {
                span: tracing::span!(target: module_path!(), Level::DEBUG, $name),
            },
            "warn" => trace_msg::custom_span::CustomSpan {
                span: tracing::span!(target: module_path!(), Level::WARN, $name),
            },
            "error" => trace_msg::custom_span::CustomSpan {
                span: tracing::span!(target: module_path!(), Level::ERROR, $name),
            },
            _ => trace_msg::custom_span::CustomSpan {
                span: tracing::span!(target: module_path!(), Level::INFO, $name),
            },
        }
    }};
    ($name:expr) => {
        trace_msg::custom_span::CustomSpan {
            span: tracing::span!(target: module_path!(), Level::INFO, $name),
        }
    };
}

impl SpanInterface for CustomSpan {
    fn with_ctx(&self, ctx: Context) -> Self {
        self.span.set_parent(ctx);
        self.clone()
    }

    fn get_ctx(&self) -> Context {
        self.span.context()
    }

    fn enter(&self) {
        let _ = self.span.enter();
    }

    fn set_event<T: ?Sized + serde::Serialize>(&self, msg_type: &str, value: &T) -> Self {
        let msg = serde_json::to_string(value).expect("failed to encode msg");
        self.span.set_attribute("msg_type", msg_type.to_owned());
        self.span.set_attribute("msg", msg);
        self.clone()
    }

    fn close(self) {
        self.span.entered().exit();
    }
}
