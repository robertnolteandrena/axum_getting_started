# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befindest du dich im `0-get-started` Branch. Dies ist der Anfang unserer Hands On Übung.

## Was ist hier?

Neben der Datei README.md gibt es ein Verzeichnis `src` und eine Datei `Cargo.toml`.
In der `Cargo.toml` findest du unter `[dependencies]` die Abhängigkeiten dieses Projektes: `axum` und `tokio`.
Zwei Dateien: `bin.rs`, `lib.rs` befinden sich im `src` Ordner. `lib.rs` ist noch leer und in `bin.rs` befindet sich der Code für einen lokalen Webserver auf Port 3000. Dieser Webserver hat momentan nur eine Route.

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

Eine gute Frage: Um Integrationstests schreiben!
In Rust können wir zwei verschiedene Arten von Tests schreiben: Unit Tests und Integrationstests.
Unit Tests sind Teil unseres Projekts und haben privilegierten Zugriff auf unseren Code.
Integrationstests hingegen sind eigenständige ausführbare Programme, die unseren Code nur als Crate importieren.

Zu diesem Zweck kann in der `lib.rs` eine öffentliche Funktion erstellt werden, die uns unsere Routen zurückgibt.
Diese Funktion muss dann in `bin.rs` importiert und benutzt werden.
Als nächstes können wir einen Unit-Test in `lib.rs` schreiben.
Um ganz sicher zu gehen, können wir noch einen Integrationstest hinzufügen.
Integrationstests leben normalerweise in einem `/tests` Ordner.
Wir können also die Datei `/tests/integration_test.rs` erstellen. Dort können wir unseren Unit-Test hineinkopieren und müssen nur die Form etwas ändern.

```rust
//dependencies #[tokio::test]
async fn hello_world(){
//Test Code
}

```

Mit `cargo test` werden sowohl Unit- als auch Integrationstests durchgeführt .
Das waere es fuer dieses Kapitel auf zum naechsten:

```bash
git add .
git commit -m "Adding Tests"
git checkout 1-fast-feedback
```
