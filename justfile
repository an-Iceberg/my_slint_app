build-dev:
  cargo build --color always --message-format human

run-dev: build-dev
  cargo run --color always --message-format human

build:
  cargo build --release --color always --message-format human

run: build
  cargo run --release --color always --message-format human

test:
  cargo test --color always --message-format human

clean:
  cargo clean --color always
