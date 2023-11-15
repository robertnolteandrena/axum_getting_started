# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befindest du dich im `1-fast-feedback` Branch. Hier werden wir ein bischen Tooling aufsetzen und verwenden um den Entwicklungsprozess angenehmer zu machen.

## Was gibts hier zu tun ?

Es gab einen Zusatz im code:

```rust
#[cfg(test)]
mod tests{
    // Some Dependencies
    #[tokio::test]
    async fn hello_world(){
        // Setup
        // Run
        // Assert
    }
}
```

Wir haben Tests (yay 😊).
Diese tests koennen wir einmalig ausfueren mit:

```bash
cargo test
```

oder besser noch: ausfueren nach jeder Aenderung:

```bash
cargo install cargo-watch
cargo watch -q -c -x  "test -- --show-output"
```

