# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befinden wir uns im `4-tower-service ` Branch. Hier werden wir uns n\aeher mit `tower` besch\aeftigen.
Daf\uer brauchen wir im ersten Schritt nicht einmal `axum` und schreiben ein paar explorative Tests.
Danach werden wir allen responses den `response-timestamp` header hinzuf\uegen.

## Was gibts hier zu tun ?

### Explorative Tests

Viele Funktionen der Rust standardbibliothek haben Doc-Tests. Doc-Tests sind tests die direkt in der Dokumentation eingebettet ist. Hier ist zum Beispiel die Implementation f\uer [`Vec::pop`](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1953):

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

Wir sehen direkt wie eine bestimmte Funktion verwendet werden kann, und wir k\oennen uns recht sicher sein, dass die Code Dokumentation aktuell ist, denn es handelt sich nicht um ein reines Kommentar:
Wenn wir `cargo test` aufrufen, werden Doc-Tests auch ausgef\uehrt. Doc-Tests helfen also die Dokumentation des Codes verst\aendlich und aktuell zu halten.
Obwohl standardisierte Funktionen wie `From::from`, `Clone::clone` oder `Default::default` weniger von Doc-Tests profitieren, k\oennen wir einen Doc-Test zur `Fahrenheit::from` methode hinzuf\uegen:

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

Wenn wir nun `cargo test` aufrufen, sollte dieser Test uns unter `Doc-tests hands_on_lib` angezeigt werden.

> Okay, und was sollte dieser Exkurs in Doc-Tests?

Haben wir es mit einem neuen, unbekannten crate zu tun, ist eine herangehensweise sich hoffentlich existierenden Doc-Tests anzuschauen und diese dann in ein paar explorative Tests zu \uebertragen.
Daf\uer k\oennen wir einen neuen test anlegen unter `tests/exploratory_tower_middleware.rs`. Dann koennen wir in einem anderen Ordner ausserhalb dieses git repos das tower crate clonen und die doc-tests von tower laufen lassen:

````bash
cd somewhere/outside/this/repo
git clone https://github.com/tower-rs/tower.git
cd tower
cargo test --doc --features full
```

Jedes Segment mit einem Doc-Test ist potentiell interessant f\uer Benutzer:innen des crates.
Eine IDE sollte in der Lage sein uns Doc-Tests f\uer eine Funktion anzuzeigen, sodass wir das tower repository wieder verlassen k\oennen und in dieses repository zur\ueckkehren.
Wir gucken uns jetzt 3 Funktionen von `tower` an. Daf\uer kannst du den folgenden Code in `tests/exploratory_tower_middleware.rs` kopieren:
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
Dieser Code wird nicht compilen, aber unsere IDE sollte nun in der lage sein uns doc-tests f\uer `service_fn` und `map_request` anzuzeigen.
Mit diesen Doc-tests und dem `format!()` macro k\oennen wir die ersten 3 tests implementieren.
F\uer `map_response` im letzten Test existieren keine Doc-Tests. \Uebrigens: Sowohl `&str` als auch `i32` implementieren das [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait.


### Response-Time header

Jetzt wo unsere explorativen Tests laufen und wir ein wenig vertraut mit der einbindung von middleware sind, benutzen wir `map_respone` um allen unseren responses den header "response-time" hinzuzuf\uegen.
Um mit Repr\aesentationen von Zeit umzugehen verwenden wir das `chrono` crate.

```bash
cargo add chrono
````

#### Integrationstest

Zuerst k\oennen wir einen Integrationstest schreiben, der testen soll, ob wir einen "response-time" header auf einer bereits existenten route geliefert bekommen.
Welche Route wir testen sollte egal sein, denn der header soll f\uer alle Responses hinzugef\uegt werden.
Um zu \ueberpr\uefen ob der "response-time" header gesetzt ist und in ein DateTime objekt geparsed werden kann, k\oennen wir den folgenden assert block verwenden.

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

Wenn wir diesen test ausformuliert haben, sollte er fehlschlagen, denn wir m\uessen die eigentliche Funktionalit\aet noch implementieren.

#### Implementation

Wir k\oennen einen aufruf zu `map_response` im ServiceBuilder verwenden. Dabei sollte `map_response` eine function \uebergeben bekommen, die von `mut axum::response::Response` auf `axum::response::Response` abbildet. Von dem Response struct k\oennen wir auf die header zugreifen. Die aktuelle Zeit k\oennen wir erhalten mit:

```rust

let formated_datetime_string = Utc::now().to_rfc3339();
```
