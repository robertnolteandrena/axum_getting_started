# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befindest du dich im `0-run-app-write-tests` Branch. Dies ist der Anfang unserer Hands On Übung.

## Was ist hier?

Neben der Datei README.md gibt es ein Verzeichnis `src` und eine Datei `Cargo.toml`.
In der `Cargo.toml` findest du unter `[dependencies]` die Abhängigkeiten dieses Projektes: `axum` und `tokio`.
Zwei Dateien: `bin.rs`, `lib.rs` befinden sich im `src` Ordner. `lib.rs` ist noch leer und in `bin.rs` befindet sich der Code um einen Webserver auf Port 3000 zu starten. Dieser Webserver hat momentan nur eine Route: `/`.

## Was nun?

### Rust Installation

Auf der [Installationsseite](https://www.rust-lang.org/tools/install) findest du die empfohlene Methode, Rust für dein Betriebssystem zu installieren.
Wenn du **Linux** benutzt, kannst du `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` verwenden.
Danach solltest du über die notwendigen Werkzeuge verfügen, um den Quellcode in ein ausführbares Programm umzuwandeln.

Du kannst den Webserver mit dem Befehl `cargo run` kompilieren und starten.
Um zu überprüfen, ob alles funktioniert, kannst du in einem anderen Terminal `curl http://127.0.0.1:3000/` ausführen. Dieser sollte folgendes ausgeben:

```bash
Hello, World!
```

Willkommen in deinem eigenen World Wide Web!

### Warum existiert `lib.rs` ?

Eine gute Frage: Um Integrationstests zu schreiben!
In Rust können wir zwei verschiedene Arten von Tests schreiben:

- Unit Tests
- Integrationstests

Unit Tests sind Teil unseres Projekts und haben **privilegierten** Zugriff auf unseren Code.
Integrationstests hingegen sind **eigenständige** ausführbare Programme, die unseren Code nur als Crate importieren.

Zu diesem Zweck kann in der `lib.rs` eine öffentliche Funktion erstellt werden, die uns unsere Routen zurückgibt.
Diese Funktion muss dann in `bin.rs` importiert und benutzt werden.
Diese Funktion können wir dann auch in Integrationstests importieren und mit ihr unser Programm testen, ohne einen richtigen Server zu starten.
Als nächstes können wir einen Unit-Test in `lib.rs` schreiben und einen Integrationstest in `tests/hello_world.rs`.

#### Unit test

Unit Tests werden am Ende der Datei hinzugefügt (`lib.rs`).
Normalerweise kommen sie in ein Modul mit dem Namen "tests", welches nur kompiliert wird, falls ein Test-Build ausgeführt wird:

```rust
//productive code

#[cfg(test)]
mod tests{
    //dependencies

    #[tokio::test]
    async fn hello_world(){
        //Arrange
        //Act
        //Assert
    }
}
```

Wir werden noch zwei weitere Crates brauchen um unseren Test zu schreiben.
Da wir diese allerdings nur für die Tests brauchen, können wir uns darauf beschränken sie als dev-dependency hinzuzufügen:

```bash
cargo add --dev tower hyper
```

Die dependencies werden hoffentlich von unserer IDE aufgelöst und wir können uns direkt dem Dreigestirn aus Arrange, Act und Assert zuwenden.

##### Arrange

Hier müssen wir auf unsere Webapp zugreifen. Dafür haben wir die Funktion `fn construct_app`:

```rust
let app=construct_app();
```

##### Act

Nun schicken wir einen Request an unsere App. Den Request können wir mit einem Builder-ähnlichen Interface konstruieren:

```rust
let request=Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap();
```

Diesen Request werfen wir nun unserer App zu, welche uns eine Response geben wird:

```rust
let response=app.oneshot(request).await.unwrap();
```

##### Assert

Wir extrahieren den Response Body und den Status Code:

```rust
let status_code=response.status();
let body=hyper::body::to_bytes(response.into_body()).await.unwrap();
```

und erwarten den Status Code `OK` sowie den Response Body "Hello World".

```rust
assert_eq!(status_code,StatusCode::OK);
assert_eq!(&body[..],b"Hello, World!");
```

#### Integration test

Der Integrationstest muss nicht in ein eigenes Modul geschrieben, denn er wird sowieso zu einer eigenständigen Binary.

```rust
//dependencies

#[tokio::test]
async fn hello_world(){
    //Arrange
    //Act
    //Assert
}
```

Mit `cargo test` werden sowohl Unit- als auch Integrationstests durchgeführt.
Das wäre es für dieses Kapitel, auf zum nächsten:

```bash
git add .
git commit -m "Adding Tests"
git checkout 1-timeout-errorhandling
```
