use opentelemetry::sdk::Resource;
use opentelemetry::{sdk::trace as sdktrace, trace::TraceError};
use opentelemetry_semantic_conventions as semcov;

pub fn identity(v: opentelemetry_jaeger::PipelineBuilder) -> opentelemetry_jaeger::PipelineBuilder {
    v
}

// https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/sdk-environment-variables.md#jaeger-exporter
pub fn init_tracer<F>(resource: Resource, transform: F) -> Result<sdktrace::Tracer, TraceError>
where
    F: FnOnce(opentelemetry_jaeger::PipelineBuilder) -> opentelemetry_jaeger::PipelineBuilder,
{
    opentelemetry::global::set_text_map_propagator(
        opentelemetry::sdk::propagation::TraceContextPropagator::new(),
    );

    let mut pipeline = opentelemetry_jaeger::new_pipeline();
    if let Some(name) = resource.get(semcov::resource::SERVICE_NAME) {
        pipeline = pipeline.with_service_name(name.to_string());
    }
    pipeline = pipeline.with_trace_config(
        sdktrace::config()
            .with_resource(resource)
            .with_sampler(sdktrace::Sampler::AlwaysOn),
    );
    pipeline = transform(pipeline);
    pipeline.install_batch(opentelemetry::runtime::Tokio)
}
