set dotenv-load := false

default: wasm
  cargo build

wasm:
  # Create etags for the relevant static files using sha
  cd frontend && sha1sum -z index.html | sed 's/ .*//' > index.html.etag
  cd frontend && sha1sum -z style.css | sed 's/ .*//' > style.css.etag
  #cd frontend && sha1sum -z package.js | sed 's/ .*//' > package.js.etag

release: wasm-release
  cargo build --release

wasm-release:
  # Create etags for the relevant static files using sha
  cd frontend && sha1sum -z index.html | sed 's/ .*//' > index.html.etag
  cd frontend && sha1sum -z style.css | sed 's/ .*//' > style.css.etag
  #cd frontend && sha1sum -z package.js | sed 's/ .*//' > package.js.etag

serve: default
  cargo run -p backend

serve-release: release
  cargo run -p backend --release

clean:
  rm -r frontend/*.etag
  cargo clean
