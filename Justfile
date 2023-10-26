alias r := run

run *args:
  cargo run --release -- {{args}}
