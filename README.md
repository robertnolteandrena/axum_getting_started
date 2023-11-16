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

Bevor wir unseren produktiven Code anpassen, können wir ein Akzeptanzkriterium als Integrationstest ausdrucken. Wenn wir eine Anfrage an die URI: '/10_seconds_timer' senden, erwarten wir als Antwortstatus 'Internal Server Error'.
Diesen Integrationstest können wir z.B. in eine neue Datei `tests/timeout_error.rs` schreiben. In unserer `cargo watch` Konsole sollten wir nun einen fehlgeschlagenen Test sehen. Um den neuen Test einmalig auszuführen, kann auch `cargo test timeout_error` verwendet werden.

#### Timeout Produktivcode

Zuerst brauchen wir eine neue Route: '/10_seconds_timer'.
Diese soll 10 Sekunden pausieren und dann etwas zurückgeben. Wenn wir nun unseren Test ausführen: `cargo test timeout_error`, sollte dieser noch fehlschlagen. Im naechsten Schritt aendern wir unseren Code so, dass wir nach einer Sekunde einen timeout_error zurueckgeben. Dazu registrieren wir einen Layer in der Funktion `fn construct_app`.
Layer ermöglichen es, zusätzliche Logik für Routen zu definieren. Wir können einen neuen Layer mit dem `ServiceBuilder` erstellen. Dieser gibt uns eine Schnittstelle ähnlich dem Builder Pattern.
Das gewünschte Verhalten dieses Layers ist die Rückgabe eines `Internal Server Errors` nach einer Sekunde.

Zum nächsten Kapitel geht es hier entlang:

```bash
git add .
git commit -m "Timeout Error Handling"
git checkout 2-middleware-errorhandling
```
