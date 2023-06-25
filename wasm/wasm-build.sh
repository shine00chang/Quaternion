if [ "$1" = "--dev" ]
then
	wasm-pack build --debug --target web --no-typescript -- --no-default-features
else
	wasm-pack build --target web --no-typescript -- --no-default-features
fi 

# wasm-opt optimization
wasm-opt -O3 pkg/quaternion_wasm_bg.wasm -o pkg/quaternion_wasm_bg_opt.wasm
