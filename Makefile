all: 
	wasm-pack build --release --target web --out-dir public/wasm
	rm public/wasm/.gitignore

serve:
	python3 -m http.server