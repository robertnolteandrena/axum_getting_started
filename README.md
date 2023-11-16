# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befindest du dich im `1-fast-feedback` Branch. Hier werden wir ein bischen Tooling aufsetzen und verwenden um den Entwicklungsprozess angenehmer zu machen.

## Was gibts hier zu tun ?

### Fast Feedback

Wir haben nun einen Unit test und einen Integrationstest, welche wir im vorherigen Abschnitt mit `cargo test` ausgefuehrt haben.
Um die tests immer auszufuehren sobald etwas geandert wurde, konnen wir `cargo-watch` benutzen:

```bash
cargo install cargo-watch
cargo watch -q -c -x  "test -- --show-output"
```

Wir koennen cargo auch sagen dass der Command nur ausgefuehrt werden soll, wenn sich dateien in einem bestimmten Ordner aendern:

```bash
cargo install cargo-watch
cargo watch -q -w tests -c -x  "test -- --show-output"
```

Nachdem wir das einmal durchexerziert haben, muss ich zugeben dass der Unit Test gerade unnoetig ist und geloescht werden kann. Der Integrationstest reicht fuers erste.

### Middleware Errorhandling

Axum hat keine eigene Middleware sondern benutzt `[tower](https://crates.io/crates/tower)`.
Tower ist eine library um netzwerk Clients und Servers zu kreiren.
Wir koennen uns nun in tower reinhaengen um eine timeout error response zurueckzugeben, falls unser "blazingly fast" server doch mal laenger braucht um einen Request abzuarbeiten.
Gerade benutzen wir tower lediglich als dev-dependency in unserem Integrationstest.
Wir brauchen tower nun in unserem Produktivcode mit dem 'timeout' feature:

```bash
cargo add tower -F timeout
```

#### Timeout Integrationstest

Befor wir unseren Produktivcode anpassen, koennen wir ein Akzeptanzkriterium als Integrationstest ausdruecken. Wenn wir einen Request an die uri : '10_seconds_timer' schicken, erwarten wir einen 'Internal Server Error' als response status.
Diesen Integrationstest koennen wir zum beispiel in eine neue datei `tests/timeout_error.rs` schreiben. In unserer `cargo watch` Konsole muessten wir nun einen fehlgeschlagenen Test sehen. Um den neuen test einmalig auszufuehren, kann auch `cargo test timeout_error` benutzt
werden.

#### Timeout Produktivcode
