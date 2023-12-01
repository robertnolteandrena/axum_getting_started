# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befinden wir uns im `4-tower-service` Branch. Hier werden wir uns näher mit dem crate `tower` beschäftigen.
Dazu brauchen wir im ersten Schritt nicht einmal `axum` und schreiben ein paar explorative Tests.
Danach werden wir den `response-time` Header an alle Responses anhängen.

## Was ist hier zu tun?

### Explorative Tests

Viele Funktionen der Rust Standardbibliothek haben Doc-Tests.
Doc-Tests sind Tests innerhalb von Kommentaren zur Dokumentation, die nach drei Backticks anfangen und nach drei Backticks wieder aufhören.
Hier ist zum Beispiel die Implementierung für [`Vec::pop`](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1953):

````rust

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    ///
    /// If you'd like to pop the first element, consider using
    /// [`VecDeque::pop_front`] instead.
    ///
    /// [`VecDeque::pop_front`]: crate::collections::VecDeque::pop_front
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2, 3];
    /// assert_eq!(vec.pop(), Some(3));
    /// assert_eq!(vec, [1, 2]);
    /// ```
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.as_ptr().add(self.len())))
            }
        }
    }

````

Wir sehen direkt, wie eine bestimmte Funktion verwendet werden kann, und wir können ziemlich sicher sein, dass die Code-Dokumentation aktuell ist, da es sich nicht um einen einfachen Kommentar handelt:
Wenn wir `cargo test` aufrufen, werden auch Doc-Tests ausgeführt. Doc-Tests helfen also, die Dokumentation des Codes verständlich und aktuell zu halten.
Obwohl standardisierte Funktionen wie `From::from`, `Clone::clone` oder `Default::default` weniger von Doc-Tests profitieren, können wir einen Doc-Test zur `Fahrenheit::from` Methode hinzufügen:

````rust
impl From<Celsius> for Fahrenheit {
   ///Converts and consumes a Celsius instance to a Fahrenheit instance.
   ///
   ///# Example
   ///```
   ///use hands_on_lib::dto::celsius::Celsius;
   ///use hands_on_lib::dto::fahrenheit::Fahrenheit;
   ///let celsius=Celsius {celsius_value : 0f32};
   ///let fahrenheit= Fahrenheit::from(celsius);
   ///assert_eq!(fahrenheit.fahrenheit_value, 32f32);
   ///```
   fn from(val: Celsius) -> Self {
       let fahrenheit_value = val.celsius_value * 9f32 / 5f32 + 32f32;
       Fahrenheit { fahrenheit_value }
    }
}

````

Wenn wir nun `cargo test` aufrufen, sollte uns dieser Test unter `Doc-tests hands_on_lib` angezeigt werden.

> Okay, und was soll dieser Exkurs in Doc-Tests?

Wenn wir es mit einem neuen, unbekannten Crate zu tun haben, ist ein Ansatz, sich die hoffentlich vorhandenen Doc-Tests anzuschauen und diese dann in ein paar explorative Tests zu übertragen.
Dazu können wir einen neuen Test unter `tests/exploratory_tower_middleware.rs` erstellen. Dann können wir in einem anderen Ordner außerhalb dieses Git Repositories das `tower` Crate clonen und die `tower` Doc-Tests ausführen:

```bash
cd somewhere/outside/this/repo
git clone https://github.com/tower-rs/tower.git
cd tower
cargo test --doc --features full
```

Jedes Code-Segment mit einem Doc-Test ist potentiell interessant für externe Benutzer, wir wollen uns aber auf drei Funktionen beschränken.
Dazu verlassen wir das Repository von `tower`, kehren hierhin zurück und erstellen die Datei `tests/exploratory_tower_middleware.rs` mit folgendem Inhalt:

```rust
use std::{convert::Infallible, fmt::Display};

use spectral::assert_that;
use tower::{ServiceBuilder, ServiceExt};

#[tokio::test]
async fn middleware_with_one_service() {
    let sb = ServiceBuilder::new().service_fn(unimplemented!());
    let response = sb.oneshot("Vanilla request").await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("my_service(Vanilla request)");
}

#[tokio::test]
async fn middleware_with_one_service_and_one_layer() {
    let sb = ServiceBuilder::new()
        .map_request(unimplemented!())
        .service_fn(unimplemented!());
    let response = sb.oneshot("Vanilla request").await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("my_service(map_request(Vanilla request))");
}

#[tokio::test]
async fn middleware_with_different_return_type() {
    //a service consumes a request and returns a response
    let sb = ServiceBuilder::new().service_fn(unimplemented!());
    let response = sb.oneshot(42).await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("my_service(42)");
}

#[tokio::test]
async fn middleware_with_one_service_and_mapresponse() {
    let sb = ServiceBuilder::new()
        .map_response(unimplemented!())
        .service_fn(unimplemented!());
    let response = sb.oneshot("Vanilla request").await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("map_response(my_service(Vanilla request))");
}


```

Dieser Code wird nicht kompilieren, aber unsere IDE sollte nun in der Lage sein, uns Doc-Tests für `service_fn` und `map_request` anzuzeigen.
Mit diesen Doc-Tests und dem Makro `format!()` können wir die ersten 3 Tests implementieren.
Für `map_response` im letzten Test gibt es keine Doc-Tests. Übrigens: Sowohl `&str` als auch `i32` implementieren das [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) Trait.

### Response-Time header

Jetzt, wo unsere explorativen Tests laufen und wir ein wenig mit der Integration von Middleware vertraut sind, verwenden wir `map_respone`, um allen unseren Responses den Header "response-time" hinzuzufügen.
Um Zeitdarstellungen zu handhaben, verwenden wir das `chrono` crate.

```bash
cargo add chrono
```

#### Integrationstest

Zuerst können wir einen Integrationstest schreiben, um zu testen, ob wir einen Header "response-time" auf einer bereits existierenden Route erhalten.
Welche Route wir testen sollte egal sein, da der Header für alle Responses hinzugefügt werden muss.
Um zu überprüfen, ob der "response-time" Header gesetzt ist und in ein DateTime-Objekt geparst werden kann, können wir den folgenden Assert-Block verwenden.

```rust
    //assert that the response-time header is present
    let response_time = response.headers().get("respose-time");
    assert_that!(&response_time).is_some();

    //assert that the response_time header is parseable to datetime
    assert_that!(response_time
        .map(HeaderValue::to_str)
        .and_then(Result::ok)
        .map(DateTime::parse_from_rfc3339)
        .and_then(Result::ok)
        .map(Into::<DateTime<Utc>>::into))
    .is_some();
```

Wenn wir diesen Test formuliert haben, sollte er fehlschlagen, da wir die eigentliche Funktionalität noch implementieren müssen.

#### Implementation

Wir können einen Aufruf von `map_response` im ServiceBuilder verwenden. Dabei sollte `map_response` eine Funktion übergeben werden, die von `mut axum::response::Response` auf `axum::response::Response` mappt. Aus dem Response struct können wir auf den Header zugreifen. Die aktuelle Zeit erhalten wir mit

```rust
let formated_datetime_string = Utc::now().to_rfc3339();
```

Das wäre es für dieses Kapitel, auf zum nächsten:

```bash
git add .
git commit -m "Add Time Header"
git checkout 5-finish
```
