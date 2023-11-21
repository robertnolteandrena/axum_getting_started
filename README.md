# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befinden wir uns im `3-passing-state` Branch. Hier werden wir uns in die Middleware einklinken und den User-Agent-Header auslesen.

## Was gibts hier zu tun ?

### Akzeptanztest

Beginnen wir erneut mit einem Akzeptanztest. Wir senden eine Anfrage an `/header/which_user_agent`, in der wir den Header `User-Agent` setzen:

```rust
let request = Request::builder()
        .uri("/header/which_user_agent")
        .header(USER_AGENT, "Some User Agent")
        .body(Body::empty())
        .unwrap();
```

Die Antwort, die wir erhalten, sollte `OK` sein und den User Agent im Body enthalten.
Wie immer sollte dieser Test fehlschlagen, da wir die Route noch nicht implementiert haben.

### Produktivcode

Die Routen werden wieder in eine eigene Datei ausgelagert, um später mit `.nest(...,...)` eingebunden zu werden.

```rust
pub fn get_header_routes() -> Router {
    Router::new()
        .route("/which_user_agent", get(handler))
        .route_layer(middleware::from_fn(extract_user_agent))
}
```

Wir müssen nun zwei Funktionen definieren: `handler` und `extract_user_agent`.Außerdem benötigen wir eine struct `UserAgentValue`:

```rust
#[derive(Clone)]
struct UserAgentValue(String);
```

#### Funktion `extract_user_agent`

```rust
async fn extract_user_agent<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let user_agent_header = req.headers().get(USER_AGENT);
    if let Some(user_agent) = user_agent_header
        .and_then(|user_agent_header: &HeaderValue| user_agent_header.to_str().ok())
        .map(String::from)
        .map(UserAgentValue)
    {
        req.extensions_mut().insert(user_agent);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
```

#### Funktion `handler`

```rust
async fn handler(Extension(UserAgentValue(user_agent)): Extension<UserAgentValue>) -> Response {
    user_agent.into_response()
}
```

Danach muss der Router aus `get_header_routes` in `lib.rs` eingebunden werden und unser Test sollte laufen.
Übrigens: Möglicherweise müssen nun vorherige Tests angepasst werden.

Das wäre es für dieses Kapitel, auf zum nächsten:

```bash
git add .
git commit -m "Extracting User Agent in Middleware"
git checkout 4-no-name
```
