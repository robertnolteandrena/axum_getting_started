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

Eine gute Frage: Um Integrationstests zu schreiben!
In Rust koennen wir zwei verschiedene Arten von tests schreiben: Unit Tests und Integration Tests.
Unit Tests sind Bestandteil unseres Projektes und haben priviligierten Zugang zu unserem Code.
Integrationstests auf der anderen Hand, sind eigenstaendige executables, die unseren Code lediglich als Crate importieren.
