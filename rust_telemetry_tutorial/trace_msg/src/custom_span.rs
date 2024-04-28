use opentelemetry::{
    global::{self, BoxedSpan},
    trace::{Span, SpanBuilder, TraceContextExt, Tracer},
    Context, KeyValue,
};

#[derive(Clone, Default)]
pub struct CustomContext(Context);

pub trait Interface {
    fn get_ctx(&self) -> CustomContext;
    fn set_custom_attribute<T: ?Sized + serde::Serialize>(&mut self, msg_type: &str, value: &T);
    fn set_attribute(&mut self, key: &'static str, value: String);
    fn close(&mut self);
}

pub struct CustomSpan(&'static str, BoxedSpan);

pub fn new(tracer: &'static str, name: &'static str, code_line: String) -> CustomSpan {
    let tracer_box = global::tracer(tracer);
    let span_builder =
        SpanBuilder::from_name(name).with_attributes(vec![KeyValue::new("code_line", code_line)]);
    let span = tracer_box.build(span_builder);
    CustomSpan(tracer, span)
}

pub fn new_with_ctx(
    tracer: &'static str,
    name: &'static str,
    ctx: CustomContext,
    code_line: String,
) -> CustomSpan {
    let tracer_box = global::tracer(tracer);
    let span_builder =
        SpanBuilder::from_name(name).with_attributes(vec![KeyValue::new("code_line", code_line)]);
    let span = tracer_box.build_with_context(span_builder, &ctx.0);

    CustomSpan(tracer, span)
}

impl Interface for CustomSpan {
    fn get_ctx(&self) -> CustomContext {
        let ctx = opentelemetry::trace::Span::span_context(&self.1);
        CustomContext(Context::current().with_remote_span_context(ctx.clone()))
    }

    fn set_custom_attribute<T: ?Sized + serde::Serialize>(&mut self, msg_type: &str, value: &T) {
        let msg = serde_json::to_string(value).expect("failed to encode msg");
        self.1.set_attributes(vec![
            KeyValue::new("msg", msg),
            KeyValue::new("msg_type", msg_type.to_string()),
        ]);
    }

    fn close(&mut self) {
        opentelemetry::trace::Span::end(&mut self.1)
    }

    fn set_attribute(&mut self, key: &'static str, value: String) {
        self.1.set_attribute(KeyValue::new(key, value))
    }
}
