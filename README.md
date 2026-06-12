# Doom (raycaster) — Rust

Prosty silnik 3D typu "raycaster" w Rust — mini remake klasycznego Wolfensteina. Renderuje scenę do bufora pikseli i wyświetla okno przy pomocy biblioteki `minifb`. Główny plik: `src/main.rs`.

## Wymagania
- Rust (stable) i Cargo
- Dodaj zależność w `Cargo.toml`:
```
[dependencies]
minifb = "0.22"
```

## Uruchomienie
- Tryb debug:
```
cargo run
```
- Tryb wydaniowy (szybsze, polecane):
```
cargo run --release
```

## Sterowanie
- W — do przodu
- S — do tyłu
- A — strafe w lewo
- D — strafe w prawo
- , (Comma) — obrót w lewo
- . (Period) — obrót w prawo
- Esc — wyjście

## Krótkie wyjaśnienie działania (odniesienia do `src/main.rs`)
- Mapa: stała tablica 24×24 `MAP` (1 = ściana, 0 = przestrzeń).
- Rozdzielczość: `WIDTH = 640`, `HEIGHT = 480`. Bufor pikseli: `Vec<u32>` o rozmiarze WIDTH*HEIGHT.
- Kamera: `pos_x`, `pos_y` (pozycja), `dir_x`, `dir_y` (kierunek), `plane_x`, `plane_y` (płaszczyzna kamery / FOV).
- Render: dla każdej kolumny ekranu liczony jest promień, wykonuje się DDA by znaleźć przecięcie ze ścianą, obliczana jest odległość i wysokość linii do narysowania. Brak tekstur — ściany mają jednolite kolory, różne w zależności od boku (x/y).
- Kolizje: przy ruchu sprawdzane jest czy docelowa komórka mapy jest pusta (0).

## Szybkie modyfikacje
- Zmiana rozdzielczości: edytuj `WIDTH` i `HEIGHT`.
- Zmiana mapy: edytuj tablicę `MAP`.
- FOV: zmień `plane_y`.
- Prędkość ruchu/obracania: zmienne `move_speed` i `rot_speed` (są zależne od czasu klatki).