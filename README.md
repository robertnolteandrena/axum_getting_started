# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment bist du im `2-json` Branch. Hier werden wir eine Route hinzufügen, die sowohl JSON konsumiert als auch zurückgibt.

## Was ist hier zu tun?

Installiere zunächst alle Abhängigkeiten, die wir für diesen Abschnitt benötigen:

- Produktiv-Code
- Das `json` Feature von `axum
  - `cargo add axum -F json`
  - Deserialisierung von Request-Bodies, die `serde::Deserialize` implementieren
  - Serialisierung von Responses, die `serde::Serialize` implementieren
  - `serde` mit Feature `derive`
    - `cargo add serde -F derive`
    - Framework für Serialisierung und Deserialisierung
  - Makros werden durch `derive` bereitgestellt
- Test-Code
  - mime` Crate
    - `cargo add mime`
    - MIME als Typen
  - serde_json
    - `cargo add --dev serde_json`
    - JSON-Unterstützung für `serde`
  - `spectral`
    - `cargo add --dev spectral`
    - Mehr Assertions

### Akzeptanzkriterium

In diesem Abschnitt wollen wir eine Route hinzufügen, die eine Temperatur von Celsius nach Fahrenheit konvertiert.
Wir brauchen ein `Celsius` und ein `Fahrenheit` struct, wobei das `Fahrenheit` struct das `From<Celsius>` trait implementiert.

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

Sobald das `Celsius` struct analog erstellt ist, können wir einen Integrationstest schreiben, ohne dass die IDE uns alles rot einfärbt.
Um ein struct, das `serde::Serialize` implementiert, in JSON zu überführen, benutzen wir `serde:json::to_value`.

```rust
let celsius_temperature = Celsius {
        celsius_value: 36.8,
};
let json_value = serde_json::to_value(celsius_temperature).unwrap();
let request_body = Body::from(json_value.to_string());
```

Mit dem `request_body` können wir dann unsere Anfrage aufbauen:

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

Jetzt müssen wir nur noch den Assert-Block schreiben und unser Integrationstest ist fertig:

```rust
//Assert Status Code is Ok
assert_that!(fahrenheit_temperature.fahrenheit_value).is_close_to(98.2, 1e-1f32);
```

### Produktivcode

Jetzt muss nur noch die Route geschrieben werden.
Der Endpunkt soll unter "/temperature" liegen.
Um diese neue Route zu trennen, können wir eine Funktion schreiben, die uns einen Router für die übergeordnete "/temperature"-URI gibt.

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

Dieser Router wird dann mit der Funktion `nest` eingebunden:

```rust
.nest("/temperature", get_temperature_routes())
```

Damit sollte dew Integrationstest erfolgreich durchlaufen.

## Wie gehts weiter ?

Langsam aber sicher wird unsere Webapp komplexer und deine Implementierung wird im Detail sicher anders aussehen als unsere.
Du kannst nun entscheiden, ob du mit deiner Implementation weitermachen möchtest und einfach die nächste README auscheckst:

```bash
git checkout 4 -- README.md
```

oder mit unserer Implementation:

```bash
git add .
git commit -m "Add Temperature Routes"
git checkout 4
```
