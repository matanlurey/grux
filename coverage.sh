CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/cargo-test-%p-%m.profraw' cargo test --tests
grcov target/coverage --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../**' --ignore "/*" --ignore "**test*" -o target/coverage/html
open target/coverage/html/index.html
