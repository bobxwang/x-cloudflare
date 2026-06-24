use worker::*;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from cloud flare Workers!"))
        .get("/api/users", |_req, _| Response::ok("Users endpoint"))
        .run(_req, _env)
        .await
}
