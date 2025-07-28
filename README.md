# Binlay 🧰

**Binlay** is a lightweight, terminal-based ELF binary analyzer written in Rust. It parses ELF files (executables, shared objects, object files), extracts important sections and symbol data, and presents a clear, visual breakdown of per-symbol sizes to help you understand binary footprint and performance.

<img width="963" height="472" alt="image" src="https://github.com/user-attachments/assets/7d365b38-b194-4a42-ae6b-815453d559e9" />


## 📋 Features

- 📦 Supports ELF binaries: executables (`ET_EXEC`), shared libraries (`ET_DYN`), and object files (`ET_REL`)
- 📏 Computes and displays:
  - Symbol sizes (`st_size`)
  - Percentage contribution of each symbol relative to total symbol size
- 🧮 Pretty-printed ASCII table with visual bar graph
- 🔀 Works on **unlinked `.o` object files** — ideal for pre-link analysis

---

## 🚀 Quick Start

### Prerequisites
- Rust ≥ 1.70
- Git

```bash
git clone https://github.com/sidtohan/binlay.git
cd binlay
cargo build --release
```

## 🔧 Usage

```bash
# Build the binary
cargo build --release

# Analyze an ELF binary
./target/release/binlay ./your_binary

# Or analyze an object file
./target/release/binlay ./foo.o
```
### Optional CLI flags 
- --sort: Sort by size
- --min-size: Filter out tiny symbols
- --top N: Show top-N symbols only
- --sort-ascending: Sort the symbols in ascending order of size

## 📜 License

MIT © 2025 @sidtohan
