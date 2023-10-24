# winkhouse3
version of winkhouse made with rust and dioxus framework


sudo sysctl -w kernel.yama.ptrace_scope=0

dx build --features web
cargo run --features ssr

npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch