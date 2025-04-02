build:
	cd server && cargo build --release
	cd client && cargo build --release

run:
	start cmd /K "cd server && cargo run --release"
	start cmd /K "cd client && cargo run --release"
	start cmd /K "cd client && cargo run --release"
	start cmd /K "cd client && cargo run --release"

test:
	start cmd /K "cd server && cargo run --release"
	start cmd /K "cd client && cargo run --release"

clean:
	cd server && cargo clean
	cd client && cargo clean