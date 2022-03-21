use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use log::info;
use serde_json::json;

type ErasedError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, ErasedError>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let make_svc = make_service_fn(move |_| async move {
        Ok::<_, ErasedError>(service_fn(move |req| async move { service(req).await }))
    });

    let addr = ([0, 0, 0, 0], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening http://{}", addr);
    server.await?;

    Ok(())
}

pub async fn service(_request: Request<Body>) -> Result<Response<Body>> {
    let json = json!({
        "response": {
            "text": "Добавила яйца в список покупок. Что-нибудь ещё?",
            "end_session": false
        },
        "session_state": {
            "value": 10
        },
        "user_state_update": {
            "value": 42
        },
        "application_state": {
            "value": 37
        },
        "version": "1.0"
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_vec(&json)?))?)

    // error!("Unsupported request: {:?}", request);

    // let body = hyper::body::aggregate(request).await?;

    // match std::str::from_utf8(body.chunk()) {
    //     Ok(body) if !body.is_empty() => error!("Body {}", body),
    //     _ => (),
    // }

    // let response = Response::builder()
    //     .status(StatusCode::BAD_REQUEST)
    //     .body(Body::from("invalid request"))?;

    // Ok(response)
}
