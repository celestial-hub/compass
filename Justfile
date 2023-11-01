alias r := run

run *args:
  cargo run --release -- {{args}}

test:
  cargo nextest run || cargo insta review
