set dotenv-load := false

default: wasm
  cargo build

wasm:
  #cd frontend && wasm-pack build --target web --out-name package --dev
  # Also create etags for the relevant static files using sha
  cd frontend && sha1sum -z index.html | sed 's/ .*//' > index.html.etag
  cd frontend && sha1sum -z style.css | sed 's/ .*//' > style.css.etag
  #cd frontend && sha1sum -z package.js | sed 's/ .*//' > package.js.etag
  #cd frontend && sha1sum -zb package_bg.wasm | sed 's/ .*//' > package_bg.wasm.etag

release: wasm-release
  cargo build --release

wasm-release:
  #cd frontend && wasm-pack build --target web --out-name package
  # Also create etags for the relevant static files using sha
  cd frontend && sha1sum -z index.html | sed 's/ .*//' > index.html.etag
  cd frontend && sha1sum -z style.css | sed 's/ .*//' > style.css.etag
  #cd frontend && sha1sum -z package.js | sed 's/ .*//' > package.js.etag
  #cd frontend && sha1sum -zb package_bg.wasm | sed 's/ .*//' > package_bg.wasm.etag

serve: default
  cargo run -p backend

serve-release: release
  cargo run -p backend --release

clean:
  rm -r frontend/*.etag
  cargo clean
