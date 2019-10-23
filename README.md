# Type Parser

Il problema che mi piacerebbe risolvere e' il seguente: parsare semplici stringhe tornando 
variabili tipate correttamente.

Ad esempio vorrei scrivere un codice simile al seguente:

```rust
let (a, b) = "1 2.4".parse().unwrap();

assert_eq!(1u32, a);
assert_eq!(2.4f64, b);
```

Ovviamente la sintassi cosi' com'e' non puo' essere valida dato che

1. Non sto indicando in nessuna maniera che il mio separatore e' uno spazio
2. Non posso definire la trait generica `FromStr` per un tipo che non e' mio (la tupla)

Potremmo quindi provare con 

```rust
let Sp(a, b) = "1 2.4".parse().unwrap();

assert_eq!(1u32, a);
assert_eq!(2.4f64, b);
```

Ovviamente vorremo poi comporre queste direttive pre avere qualcosa del tipo

```rust
let Sp(a, Sp(b, c)) = "1 2.4 c".parse().unwrap();

assert_eq!(1u32, a);
assert_eq!(2.4f64, b);
assert_eq!('c', c);
```

A questo punto il passo naturale sarebbe

```rust
let sp!(a, b, c) = "1 2.4 c".parse().unwrap();

assert_eq!(1u32, a);
assert_eq!(2.4f64, b);
assert_eq!('c', c);
```

Ovviamente poi si potrebbero generalizzare i separatori e arrivare a una macro che costruise il parser
con una sorta di dsl da definire (Fuori dallo scopo di questa sera).