use axum::{
    Json,
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use color_eyre::eyre::Result;
use noise::audio::write_wav_to_bytes;
use noise::factory::create_component;
use noise::parser::parse_components;
use noise::pipeline::Pipeline;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
struct GenerateRequest {
    pipeline: String,
    #[serde(default = "default_duration")]
    duration: f64,
    #[serde(default = "default_sample_rate")]
    sample_rate: f64,
}

fn default_duration() -> f64 {
    1.0
}

fn default_sample_rate() -> f64 {
    44100.0
}

#[derive(Debug, Serialize)]
struct GenerateResponse {
    samples: usize,
    duration: f64,
    sample_rate: f64,
    pipeline: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Clone)]
struct AppState {}

#[instrument(skip(_state, req))]
async fn generate_audio(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!(
        "Generating audio: pipeline={}, duration={}s, sample_rate={}Hz",
        req.pipeline, req.duration, req.sample_rate
    );

    let components = parse_components(&req.pipeline);

    if components.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Pipeline must have at least one component".to_string(),
            }),
        ));
    }

    let mut pipeline = Pipeline::new();

    for (i, spec) in components.iter().enumerate() {
        let comp = create_component(spec).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Failed to create component {}: {}", i, e),
                }),
            )
        })?;
        pipeline.add_component(comp).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Failed to add component {}: {}", i, e),
                }),
            )
        })?;
    }

    let samples = pipeline.run(req.duration, req.sample_rate).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Pipeline execution failed: {}", e),
            }),
        )
    })?;

    info!("Generated {} samples", samples.len());

    Ok(Json(GenerateResponse {
        samples: samples.len(),
        duration: req.duration,
        sample_rate: req.sample_rate,
        pipeline: req.pipeline,
    }))
}

#[instrument(skip(_state, req))]
async fn generate_audio_wav(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<GenerateRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    info!(
        "Generating WAV: pipeline={}, duration={}s, sample_rate={}Hz",
        req.pipeline, req.duration, req.sample_rate
    );

    let components = parse_components(&req.pipeline);

    if components.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Pipeline must have at least one component".to_string(),
            }),
        ));
    }

    let mut pipeline = Pipeline::new();

    for (i, spec) in components.iter().enumerate() {
        let comp = create_component(spec).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Failed to create component {}: {}", i, e),
                }),
            )
        })?;
        pipeline.add_component(comp).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Failed to add component {}: {}", i, e),
                }),
            )
        })?;
    }

    let samples = pipeline.run(req.duration, req.sample_rate).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Pipeline execution failed: {}", e),
            }),
        )
    })?;

    let wav_bytes = write_wav_to_bytes(&samples, req.sample_rate).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("WAV generation failed: {}", e),
            }),
        )
    })?;

    info!("Generated WAV file with {} bytes", wav_bytes.len());

    Ok((
        StatusCode::OK,
        [("Content-Type", "audio/wav")],
        wav_bytes,
    ))
}

async fn health_check() -> &'static str {
    "OK"
}

fn init_tracing() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("noise=info"));

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing();

    info!("Starting Noise audio server");

    let state = Arc::new(AppState {});

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/generate", post(generate_audio))
        .route("/generate/wav", post(generate_audio_wav))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .expect("Failed to bind to port 42069");

    info!("Server listening on http://0.0.0.0:42069");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
