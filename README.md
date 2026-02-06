# deep-space-logistics

## commands

### ./
#### run WITHOUT tauri v2 to dev
```bash
npm --prefix tauri-app run dev
```
#### build final zip for itchio
```bash
sh build_archive.sh
```
#### tests for ./game_core
```bash
cargo test -p game_core
```

### ./game_core
#### build zip in ./tauri-app/src/pkg
```bash
wasm-pack build --target web --out-dir ../tauri-app/src/pkg --release
```

### ./tauri-app
#### start tauri app
```bash
GDK_BACKEND=x11 cargo tauri dev
```
