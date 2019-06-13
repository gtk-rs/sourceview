GIR = gir/target/bin/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/GtkSource-3.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs
	cargo fmt
	cd sourceview-sys && cargo fmt

not_bound: $(GIR) $(GIR_FILES)
	$(GIR) -m not_bound -c Gir.toml

regen_check: $(GIR) $(GIR_FILES)
	rm src/auto/*
	rm sourceview-sys/tests/*
	$(GIR) -c Gir.toml
	$(GIR) -c sourceview-sys/Gir.toml
	cargo fmt
	cd sourceview-sys && cargo fmt
	git diff -R --exit-code

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

gir-sys : sourceview-sys/src/lib.rs

sourceview-sys/src/lib.rs : sourceview-sys/Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c sourceview-sys/Gir.toml

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
