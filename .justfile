install-dx:
    rustup target add wasm32-unknown-unknown
    curl -sSL https://dioxus.dev/install.sh | bash

serve:
    dx serve --package web

build:
    dx build --package web

release:
    dx build --release --package web