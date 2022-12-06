extern = $(shell find . -type f -name '*.rs' -o -name '*.toml')

.PHONY: day-06-vis
aoc2022-06.wasm: $(extern)
	cargo build -p day-06-vis --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/day_06_vis.wasm aoc2022-06.wasm