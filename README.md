# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment bist du im `1-timeout-errorhandling` Branch. Hier werden wir ein wenig Tooling aufsetzen, um den Entwicklungsprozess angenehmer zu gestalten.
Danach implementieren wir einen Server seitigen Timeout.

## Was gibt es hier zu tun?

### Schnelles Feedback

Wir haben nun einen Unit-Test und einen Integrationstest, die wir im vorherigen Abschnitt mit `cargo test` ausgeführt haben.
Um die Tests immer dann auszuführen, wenn sich etwas geändert hat, können wir `cargo-watch` verwenden:

```bash
cargo install cargo-watch
cargo watch -q -c -x "test -- --show-output"
```

Wir können Cargo auch sagen, dass der Befehl nur ausgeführt werden soll, wenn sich Dateien in einem bestimmten Ordner ändern:

```bash
cargo watch -q -w tests -c -x "test -- --show-output"
```

Nachdem wir das einmal ausprobiert habe, muss ich zugeben, dass der Unit-Test im Moment überflüssig ist und gelöscht werden kann.
Der Integrationstest reicht für den Anfang.

### Middleware Fehlerbehandlung

Axum hat keine eigene Middleware, sondern verwendet `[tower](https://crates.io/crates/tower)`.
Tower ist eine Bibliothek zum Erstellen von Netzwerk-Clients und -Servern.
Wir können uns nun in Tower einhängen, um eine Timeout Error Response zurückzugeben, falls unser "blazingly fast" Server doch mal länger braucht, um eine Anfrage zu bearbeiten.
Im Moment verwenden wir tower nur als dev-dependency, jetzt brauchen wir tower in unserem produktiven Code mit dem 'timeout' Feature:

```bash
cargo add tower -F timeout
```

#### Timeout Integrationstest

Bevor wir unseren produktiven Code anpassen, können wir das Akzeptanzkriterium als Integrationstest (`tests/timeout_error.rs`) ausdrücken.
Wenn wir eine Anfrage an die URI: '/10_seconds_timer' senden, erwarten wir als Antwortstatus 'Internal Server Error'.
In `lib.rs` können wir eine Konstante hinzufuegen, die wir dann auch in den Tests verwenden werden.

```rust
pub const TIMER_URI:&str ="/10_seconds_timer";
```

Damit wird unser 'Arrange' Part zu:

```rust
let app = construct_app();
let request=Request::builder()
                    .uri(TIMER_URI)
                    .body(Body::empty())
                    .unwrap()
```

Der 'Act' Block bleibt so wie im vorherigen Test (`tests/hello_world.rs`).
Im 'Assert' Block überpruefen wir, ob der Server auch wirklich einen Internal Server Status zurückgibt:

```rust
assert_eq!(response.status(),StatusCode::INERNAL_SERVER_ERROR)
```

Dieser Test kann mit `cargo test timeout_error` ausgeführt werden, und sollte fehlschlagen: Wir sollten einen `404` bekommen.
Im nächsten Schritt passen wir den Produktivcode an, um unseren Test grün zu bekommen.

#### Timeout Produktivcode

##### User Story Teil 1

> Als API Konsument möchte ich einen Endpunkt, der mich 10 Sekunden warten lässt.

```rust
pub fn construct_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            TIMER_URI,
            get(|| async {
                tokio::time::sleep(Duration::from_secs(10)).await;
                "10 seconds have passed: you may continue with other task now"
            }),
        )
}
```

Der erste Teil der User Story ist nun abgeschlossen, aber unser Akzeptanztest läuft noch nicht durch, denn
der Request hat keinen Timeout ausgelöst.

##### User Story Teil 2

> Als Person die die Server Rechnung bezahlt, möchte ich ein Sicherheitsnetz haben.
> Requests müssen innerhalb von einer Sekunde abgeschlossen sein. Falls das nicht der Fall ist muss der Request 'abgelehnt' werden.

Dazu registrieren wir einen Layer in der Funktion `fn construct_app`.

> Layer ermöglichen es, zusätzliche Logik für Routen zu definieren.

Wir können einen neuen Layer mit dem ServiceBuilder erstellen.
Dieser gibt uns eine Schnittstelle ähnlich dem Builder Pattern.

```rust
pub fn construct_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            TIMER_URI,
            get(|| async {
                tokio::time::sleep(Duration::from_secs(10)).await;
                "10 seconds have passed: you may continue with other task now"
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(1)),
        )
}
```

Unsere IDE sollte uns einen Fehler anzeigen, denn wir haben noch nicht `handle_timeout_error` definiert.
Dies ist eine Funktion, die im Falle der Timeout Überschreitung den INTERNAL SERVER ERROR zurückgeben soll:

```rust
pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, &'static str) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Handler has taken too long",
        )
    } else {
        (StatusCode::OK, "")
    }
}
```

Damit sollte der Integrationstest grün werden und wir können uns im nächsten Abschnitt mit JSON beschäftigen:

```bash
git add .
git commit -m "Timeout Error"
git checkout 2-json
```
