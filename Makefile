all: build

.PHONY: build
build:
	cargo build
	cp target/debug/webots-rs sample_project/controllers/rust_controller/rust_controller

	@echo
	@echo Done. Now you can run the simulation in Webots.

.PHONY: clean
clean:
	cargo clean
	rm -f sample_project/controllers/rust_controller/rust_controller
