# MafijaLife 

MafijaLife (zastarelo: FaksLife) je 2D simulacija študentove izkušnje v kavarni Mafija.

Glavni igralec, Lan, lahko počne tipične stvari, kot so:
- kupi kavo ali tortilijo
- postane anksiozen
- kadi tobačne zvitke, da preneha biti anksiozen
- se pogovarja s prijateljico Emo

ter malo manj tipične stvari, kot so:
- ugotovi, da je izgubil spomin
- umre od prevelike doze kofeina.

Za premikanje ter interakcijo z objekti igralec uporablja tipke s puščicami ali tipke WASD. Za 
pogovor z NPC-ji igralec uporablja miško.

Za začetek igre je potrebno klikniti na gumb "start game" in potem še enkrat klikniti kamorkoli na zaslon.



## Predpriprava za zagon igre

Ta projekt je napisan v Rust-u in se poganja v brskalniku preko WebAssembly (WASM). Za lokalni zagon je potrebno namestiti nekaj dodatnih orodij.

#### 1. Namestitev Rust-a

Najprej moraš imeti nameščen Rust. Če ga še nimaš, si poglej navodila na:
https://www.rust-lang.org/tools/install

Po namestitvi preveri:
```python 
rustc --version
cargo --version
```

#### 2. Dodajanje WASM targeta

Ker se projekt prevaja v WebAssembly, je potrebno dodati wasm32-unknown-unknown target:
```python 
rustup target add wasm32-unknown-unknown
```

Dokumentacija:
https://doc.rust-lang.org/rustup/targets.html

#### 3. Namestitev Trunk

Trunk je orodje za buildanje in serviranje Rust WASM aplikacij.

Namesti se preko cargo:
```python 
cargo install trunk
```

Dokumentacija:
https://trunkrs.dev/

Preveri namestitev:
```python 
trunk --version
```

#### 4. Odvisnosti (dependencies)

Projekt uporablja naslednje glavne Rust knjižnice:

__sauron__ – frontend framework za Rust/WASM
Dokumentacija: https://docs.rs/sauron/latest/sauron/

__wasm-bindgen__ – povezava med Rustom in JavaScriptom
Dokumentacija: https://rustwasm.github.io/docs/wasm-bindgen/

Vse odvisnosti so že definirane v datoteki Cargo.toml, zato jih ni potrebno ročno nameščati.
Cargo jih samodejno prenese ob prvem buildanju.

## Lokalni zagon igre

V root mapi projekta zaženi:
```python 
trunk serve
```

To bo:

prevedlo Rust v WASM,

pognalo lokalni development strežnik,

odprlo aplikacijo v brskalniku.

Privzeto je aplikacija dostopna na:

http://localhost:8080

## Pogoste težave

Trunk lahko ne najde statičnih datotek, zato preveri, da je pot v index.html pravilna (.../FaksLife/dist/index.html)

Preveri, da mapa static/ obstaja

WASM target lahko manjka, zato ponovno zaženi:
```python 
rustup target add wasm32-unknown-unknown
```

#### Težave z buildom

poskusi:
```python 
cargo clean
trunk serve
```
