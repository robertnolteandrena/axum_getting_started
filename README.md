# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befindest du dich im `2-json` Branch. Wir werden uns nun um timeouts kuemmern.

## Was gibts hier zu tun ?

Fangen damit an alle dependencies zu installieren die wir fuer diesen Abschnitt brauchen:

- Produktivcode
  - Das `json` feature von `axum`
    - `cargo add axum -F json`
    - Deserialisation von Request Bodies, die `serde::Deserialize` implementieren
    - Serialisation von Responses, die `serde::Serialize` implementieren
  - `serde` mit feature `serde_derive`
    - `cargo add serde -F derive`
    - Framework fuer Serialisation und Deserialisation
    - Makros durch `derive` feature
- Test Code
  - `mime` crate
    - `cargo add mime`
    - MIME als Typen
  - `serde_json`
    - `cargo add --dev serde_json`
    - JSON support fuer `serde`
  - `spectral`
    - `cargo add --dev spectral`
    - Mehr assertions

### Akzeptanzkriterium

In diesem Abschnitt wollen wir eine Route hinzufuegen, die eine Temperatur in Celsius zu Fahrenheit umrechnet. Wir benoetigen ein `Celsius` und ein `Fahrenheit` struct, wobei das `Fahrenheit` struct das `From<Celsius>` trait implementiert.

```rust
//dependencies
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Fahrenheit {
    pub fahrenheit_value: f32,
}
impl From<Celsius> for Fahrenheit {
    fn from(val: Celsius) -> Self {
        let fahrenheit_value = val.celsius_value * 9f32 / 5f32 + 32f32;
        Fahrenheit { fahrenheit_value }
    }
}
```

Sobald das `Celsius` struct analog kreirt ist koennen wir einen Integrationstest schreiben, ohne dass die IDE uns alles rot einfaerbt.
Um ein struct, dass `serde::Serialize` implementiert in JSON zu ueberfuehren, benutzen wir `serde:json::to_value`

```rust
let celsius_temperature = Celsius {
        celsius_value: 36.8,
};
let json_value = serde_json::to_value(celsius_temperature).unwrap();
let request_body = Body::from(json_value.to_string());
```

Mit dem `request_body` koennen wir dann unseren Request bauen:

```rust
let request = Request::builder()
        .uri("/temperature/fahrenheit")
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(request_body)
        .unwrap();
```

Aus der Response dieses Requests bauen wir ein `Fahrenheit` struct:

```rust
let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
let fahrenheit_temperature: Fahrenheit = serde_json::from_slice(&body_bytes).unwrap();
```

Dann muessen wir nur noch den Assert block schreiben und unser Integrationstest steht:

```rust
//Assert Status Code is Ok
assert_that!(fahrenheit_temperature.fahrenheit_value).is_close_to(98.2, 1e-1f32);
```

### Produktivcode

Das einzige was uebrig bleibt ist schreiben der route.
Der Endpunkt soll unter "/temperature/fahrenheit" leben.
Um diese neue Route zu separieren, koennen wir eine Function schreiben, die uns einen Router fuer die uebergeordnete "/temperature" uri gibt.

```rust
pub fn get_temperature_routes() -> Router {
    Router::new().route(
        "/fahrenheit",
        get(|Json(celsius): Json<Celsius>| async {
            let fahrenheit_temperature: Fahrenheit = celsius.into();
            Json(fahrenheit_temperature)
        }),
    )
}
```

Dieser Router wird dann mit Funktion `nest` eingebunden:

```rust
.nest("/temperature", get_temperature_routes())
```

Damit sollte der Integrationstest gruen werden.

## Wie gehts weiter ?

Langsam aber sicher wird unsere Webapp komplexer
und deine Implementation wird mit Sicherheit im detail anders aussehen als unsere.
Du kannst dich nun entscheiden, ob du mit deiner Implementation weitermachst und lediglich die naechste README auscheckst:

```bash
git checkout 4 -- README.md
```

oder mit unserer Implementation:

```bash
git add .
git commit -m "Add Temperature Routes"
git checkout 4
```
