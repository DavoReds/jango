_default:
    @just --list --justfile {{ justfile() }}

@test *args:
  echo "Testing..."
  mold -run cargo nextest run -j=$(nproc) {{ args }}
  echo "Formatting..."
  cargo fmt --check
  echo "Linting..."
  mold -run cargo clippy -- -D warnings
