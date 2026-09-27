# Blanket Implementation в Rust

## Что такое blanket implementation

**Blanket implementation** (покрывающая реализация) — это **реализация trait** для **всех типов**, которые **удовлетворяют** некоторому **ограничению** (condition).

```rust
impl<T: Condition> Trait for T { ... }
```

**Читается:** «Реализуй `Trait` для **любого** `T`, который **реализует** `Condition`».

## Пример 1: `Clone` для `Copy`

```rust
impl<T: Copy> Clone for T {
    fn clone(&self) -> Self {
        *self
    }
}
```

- **`T: Copy`** — **ограничение**.
- **`Clone for T`** — реализация для **всех** `T: Copy`.
- **`*self`** — **побитовое** копирование.

**Логика:** если тип **`Copy`**, то `clone` — **тривиален** (`*self`).

**Следствие:** любой тип, реализующий `Copy`, **автоматически** получает `Clone`.

```rust
#[derive(Copy)]
struct Point { x: i32, y: i32 }
// Point: Copy → Point: Clone (автоматически)
```

## Пример 2: `Clone` для `&T`

```rust
impl<T: ?Sized> Clone for &T {
    fn clone(&self) -> Self {
        *self
    }
}
```

- **`T: ?Sized`** — `T` может быть **DST**.
- **`Clone for &T`** — реализация для **всех** ссылок.
- **`*self`** — **разыменование** `&&T` → `&T`.

**Логика:** ссылка **`&T`** — **`Copy`** (копируется указатель). `clone` — **тривиален**.

## Пример 3: `ToString` для `Display`

```rust
impl<T: Display + ?Sized> ToString for T {
    fn to_string(&self) -> String {
        // использует Display
    }
}
```

- **`T: Display`** — ограничение.
- **`ToString for T`** — для **всех** `Display`.
- **`.to_string()`** — **автоматически** доступен для **всех** `Display`.

```rust
let x = 42;
println!("{}", x.to_string());   // ✅ 42: Display → ToString
```

## Пример 4: `From` для `Into`

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

- **`U: From<T>`** — ограничение.
- **`Into<U> for T`** — для **всех** пар.
- **`.into()`** — **автоматически** доступен, если есть `From`.

**В вашем примере:**

```rust
impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        Self(value.0 * 1.8 + 32.0)
    }
}

let fah: Fahrenheit = Celsius(10.0).into();   // ✅ через blanket impl
let fah2 = Fahrenheit::from(Celsius(10.0));   // ✅ напрямую
```

**`Celsius(10.0).into()`** работает, потому что **`Fahrenheit: From<Celsius>`** → **`Celsius: Into<Fahrenheit>`** (blanket impl).

## Где применяется в стандартной библиотеке

### 1. `Clone` для `Copy`

```rust
impl<T: Copy> Clone for T { ... }
```

**Следствие:** любой `Copy` — `Clone`.

### 2. `Into` для `From`

```rust
impl<T, U> Into<U> for T where U: From<T> { ... }
```

**Следствие:** реализуете `From` → получаете `Into` **бесплатно**.

### 3. `ToString` для `Display`

```rust
impl<T: Display + ?Sized> ToString for T { ... }
```

**Следствие:** любой `Display` — `ToString`.

### 4. `FromIterator` для `Vec`

```rust
impl<T> FromIterator<T> for Vec<T> { ... }
```

**Следствие:** `.collect::<Vec<_>>()` работает для любого итератора.

### 5. `Borrow` для `T`

```rust
impl<T> Borrow<T> for T { ... }
```

**Следствие:** любой тип **заимствует** себя.

### 6. `AsRef` для `&T`

```rust
impl<T: ?Sized, U: ?Sized> AsRef<U> for &T where T: AsRef<U> { ... }
```

**Следствие:** `&T` — `AsRef`, если `T` — `AsRef`.

### 7. `PartialEq` для `&T`

```rust
impl<A: ?Sized, B: ?Sized> PartialEq<&B> for &A where A: PartialEq<B> { ... }
```

**Следствие:** `&T == &T`, если `T == T`.

## Сводная таблица

| Blanket impl | Ограничение | Следствие |
|---|---|---|
| `Clone for T` | `T: Copy` | `Copy → Clone` |
| `Into<U> for T` | `U: From<T>` | `From → Into` |
| `ToString for T` | `T: Display` | `Display → ToString` |
| `FromIterator<T> for Vec<T>` | — | `.collect()` |
| `Borrow<T> for T` | — | Самозаимствование |
| `AsRef<U> for &T` | `T: AsRef<U>` | Транзитивность |
| `PartialEq<&B> for &A` | `A: PartialEq<B>` | Сравнение ссылок |

## Разбор вашего примера

### `From<Celsius> for Fahrenheit`

```rust
impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        Self(value.0 * 1.8 + 32.0)
    }
}
```

- **`From<Celsius>`** — конверсия Celsius → Fahrenheit.

### Использование `.into()`

```rust
let fah: Fahrenheit = Celsius(10.0).into();
```

**`.into()`** — из **blanket impl**:

```rust
impl<T, U> Into<U> for T where U: From<T> { ... }
```

- **`T = Celsius`**.
- **`U = Fahrenheit`**.
- **`Fahrenheit: From<Celsius>`** → **`Celsius: Into<Fahrenheit>`**.

### Использование `From::from`

```rust
let fah2 = Fahrenheit::from(Celsius(10.0));
```

**Прямой** вызов `From::from`.

**Результат:** **одинаковый** — `Fahrenheit(50.0)`.

## Сводная таблица

| Аспект | Описание |
|---|---|
| **Blanket impl** | Реализация для **всех** `T`, удовлетворяющих условию |
| **Синтаксис** | `impl<T: Condition> Trait for T` |
| **Применение** | `Clone`, `Into`, `ToString`, `Borrow`, `AsRef` |
| **Следствие** | **Автоматическая** реализация |
| **Пример** | `Copy → Clone`, `From → Into`, `Display → ToString` |

## Итог

- **Blanket implementation** — реализация trait для **всех** типов, **удовлетворяющих** условию.
- **Синтаксис:** `impl<T: Condition> Trait for T`.
- **Стандартная библиотека** использует **активно:**
  - `Clone for T: Copy`;
  - `Into<U> for T: From`;
  - `ToString for T: Display`;
  - `Borrow<T> for T`;
  - `AsRef<U> for &T`.
- **Следствие:** **автоматическая** реализация — **`Copy → Clone`**, **`From → Into`**, **`Display → ToString`**.
- **В вашем примере:**
  - `From<Celsius> for Fahrenheit` → **`Celsius: Into<Fahrenheit>`** (blanket impl);
  - `.into()` и `Fahrenheit::from(...)` — **эквивалентны**.
- **Правило:** реализуете `From` → получаете `Into` **бесплатно**.
