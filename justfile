build:
  cargo build --color always --message-format human

run: build
  cargo run --color always --message-format human

build-release:
  cargo build --release --color always --message-format human

run-release: build-release
  cargo run --release --color always --message-format human

test:
  cargo test --color always --message-format human

clean:
  cargo clean --color always
