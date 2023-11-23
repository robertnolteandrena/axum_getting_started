# Rust Seminar

Dieses Repo begleitet das Seminar "Die Programmiersprache Rust".
Hier findest du alle Zwischenschritte der Hands On Übungen.

## Wo bin ich?

Im Moment befinden wir uns im `4-tower-service ` Branch. Hier werden wir uns n\aeher mit `tower` besch\aeftigen.
Daf\uer brauchen wir im ersten Schritt nicht einmal `axum` und schreiben ein paar explorative Tests.
Danach werden wir allen responses den `response-timestamp` header hinzuf\uegen.

## Was gibts hier zu tun ?

### Explorative Tests

Explorative tests sind nicht daf\uer gedacht die Korrektheit eines Programms sicherzustellen, sondern dienen uns um die Funktionalit\aet eine Programms zu lernen oder auch zu dokumentieren.
Viele Funktionen der Rust standardbibliothek haben zum Beispiel Doc-Tests. Hier ist die implementation f\uer [`Vec::pop`](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1953):

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
Wenn wir `cargo test` aufrufen, werden Doc-Tests auch ausgef\uehrt. Doc-Tests helfen also die Dokumentation meines Codes verst\aendlich und aktuell zu halten.

### Response-Time header

Wir werden nun mit Repr\aesentationen von Zeit arbeiten.
Daf\uer verwenden wir das `chrono` crate, das nach eigener Beschreibung darauf abzielt alle Funktionalit\aet f\uer Datums und Zeit operationen zu implementieren.

```bash
cargo add chrono
```
