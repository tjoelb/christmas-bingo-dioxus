install-dx:
    rustup target add wasm32-unknown-unknown
    curl -sSL https://dioxus.dev/install.sh | bash

serve:
    dx serve --package web

build:
    dx build --package web

release:
    rm -rf docs && mkdir docs
    dx bundle --release --package web --out-dir docs
    cp -r ./docs/public/ ./docs/
    rm -rf ./docs/public

    # dx bundle --release --package web
    # # Create the docs directory (clear it first if re-deploying)
    # rm -rf docs && mkdir docs

    # #Copy everything from the bundle's public folder into docs/
    # cp -r target/dx/<your-package-name>/release/web/public/* docs/

    # # Copy index.html to 404.html so client-side routing doesn't break on refresh
    # cp docs/index.html docs/404.html

