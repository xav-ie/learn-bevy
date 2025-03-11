default:
    @just run

# run with dynamically linked bevy for faster compile of just the game code
run:
  cargo run --features bevy/dynamic_linking

build:
  cargo build

release:
  cargo build --release
