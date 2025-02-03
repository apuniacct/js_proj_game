# Projekt Javascript - program gry testowej WASM

## Cel projektu
Ogólny cel projektu jest opisany w readme [repozytorium głównego](https://github.com/apuniacct/js_proj):

## Założenie programu
Program zawarty w tym repozytorium ma na celu być przykładowym programem .wasm serwowanym przez program główny projektu. Jest on prostym szkieletem gry napisanym w języku Rust za pomocą silnika [Bevy](https://bevyengine.org/) i biblioteki/pluginu symulacji fizyki [Avian](https://github.com/Jondolf/avian).

## Przygotowanie programu WASM
### Instalacja wymaganych programów
#### Instalacja języka Rust - `rustup`
Instrukcje instalacji języka rust w zależności od platformy są opisane na [oficjalnej stronie języka](https://www.rust-lang.org/tools/install) (w przypadku wielu dystrybucji systemu Linux program rustup może także znajdować się w oficjalnych repozytoriach). Dla części platform strona wspomina także o dodaniu ścieżki cargo do zmiennej `$PATH` - jest to wskazane w celu wykonania jednego z późniejszych kroków kompilacji.

Innymi przydatnymi stronami wprowadzającymi do podstaw uywania języka są strony [Getting Started](https://www.rust-lang.org/learn/get-started) oraz [The Rust Book](https://doc.rust-lang.org/book/).

#### Przygotowanie do kompilacji programów WebAssembly
Rustup pozwala na krzyżową kompilację na platformę wasm przez instalację platformy docelowej `wasm32-unknown-unknown` (Więcej informacji na stronie [nieoficjalnej książki Bevy](https://bevy-cheatbook.github.io/platforms/wasm.html#quick-start)):
`rustup target install wasm32-unknown-unknown`

Dodatkowym narzędziem potrzebnym do wygenerowania interfejsu między Javascriptem a skompilowanym plikiem .wasm jest [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen). Detaliczne instrukcje i dokumentacja są dostępne w podanym repozytorium, ale najprostszą opcją jest użycie komendy: 
`cargo install wasm-bindgen-cli`

Pobrane narzędzie zostanie zainstalowane wtedy w folderze uprzednio dodanym do `$PATH` podczas instalacji rustup.

###Kompilacja programu
Do kompilacji programu można użyć załączonego w repozytorium skryptu `prepare_wasm.sh`. Jeśli wszystkie wymagane programy zostały poprawnie zainstalowane skompilowany program .wasm i jego wygenerowane bindingi w formie pliku .js zostaną umieszczone w folderze `./bindgen_output`.
