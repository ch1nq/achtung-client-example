//! Sample gRPC agent (`achtung.agent`).
//!
//! Deliberately dumb: it ignores the game state entirely and picks a random
//! direction each tick (heavily weighted toward going straight so the game
//! still lasts more than a couple of ticks). This is a pipeline test agent, not
//! a competitor — the point is to exercise Initialize/Play and let the
//! engine resolve a placement order, not to play well.

use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod agentpb {
    tonic::include_proto!("achtung.agent");
}

use agentpb::agent_server::{Agent, AgentServer};
use agentpb::{
    AgentAction, Direction, InitializeRequest, InitializeResponse, PlayRequest, PlayResponse,
};

struct SampleAgent;

/// Really dumb: random walk. Mostly straight, occasional random turn.
fn random_action() -> AgentAction {
    let direction = match rand::random_range(0..10) {
        0 => Direction::TurnLeft,
        1 => Direction::TurnRight,
        _ => Direction::Staight,
    };
    AgentAction {
        direction: direction as i32,
    }
}

#[tonic::async_trait]
impl Agent for SampleAgent {
    async fn initialize(
        &self,
        request: Request<InitializeRequest>,
    ) -> Result<Response<InitializeResponse>, Status> {
        let req = request.into_inner();
        tracing::info!(player_id = req.player_id, "initialized");
        Ok(Response::new(InitializeResponse {}))
    }

    type PlayStream = ReceiverStream<Result<PlayResponse, Status>>;

    async fn play(
        &self,
        request: Request<Streaming<PlayRequest>>,
    ) -> Result<Response<Self::PlayStream>, Status> {
        let mut inbound = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        tokio::spawn(async move {
            // Lockstep echo: one reply per request, tick echoed back so the
            // host can match replies to ticks.
            while let Ok(Some(req)) = inbound.message().await {
                let resp = PlayResponse {
                    tick: req.tick,
                    action: Some(random_action()),
                };
                if tx.send(Ok(resp)).await.is_err() {
                    break;
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sample_agent=info,info".into()),
        )
        .init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(50052);
    let addr = format!("0.0.0.0:{port}").parse()?;

    let svc = SampleAgent;

    tracing::info!(%addr, "sample-agent listening");
    Server::builder()
        .add_service(AgentServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
