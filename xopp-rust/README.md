# Xournal++ Modern (Rust Edition)

A high-performance, lightweight, cross-platform rewrite of **Xournal++** built entirely in **Rust** using `egui` / `eframe`.

---

## Highlights & Features

- **Blazing Fast Native Performance**:
  - Pure Rust architecture with zero heavy C/GTK dependency overhead.
  - Hardware-accelerated 2D vector canvas rendering via GPU.
- **Full `.xopp` File Format Compatibility**:
  - Reads and writes official gzip-compressed XML `.xopp` and `.xoj` notebook documents.
  - Bidirectional interoperability with upstream Xournal++.
- **Modern UI Design System**:
  - Contemporary rounded geometry (8px radius) for toolbars and buttons.
  - Instant runtime **Dark Mode** toggle with pure AMOLED black (`#000000`) canvas surrounding the paper.
  - Smooth infinite pan and zoom with mouse wheel and drag gestures.
- **Authentic Notebook Paper Engine**:
  - Ruled paper format with classic header divider and left vertical pink margin.
  - Lined, 5mm Graph/Grid, Plain White, and Dotted matrix ruling formats.
- **Inking & Tool Suite**:
  - Smooth freehand vector pen with dynamic widths.
  - Translucent highlighter with non-destructive blending.
  - Stroke-based and segment eraser.
  - Geometric shape tools: Line, Arrow, Rectangle, Ellipse/Circle.
  - Full Undo/Redo command stack (`Ctrl+Z` / `Ctrl+Y`).
- **Page Management**:
  - Sidebar with sequential page indexing (`Page X of Y`), dynamic page additions, deletions, and paper background selection.

---

## Architecture Overview

```
xopp-rust/
├── Cargo.toml            # Project manifest and dependencies
├── src/
│   ├── main.rs           # Native application entry point
│   ├── lib.rs            # Library interface
│   ├── app.rs            # Main event loop and UI layout
│   ├── theme.rs          # Modern Dark / Light visual styling
│   ├── model/            # Core document and inking data structures
│   │   ├── point.rs      # 2D points with pressure sampling
│   │   ├── stroke.rs     # Vector stroke paths and color representation
│   │   ├── layer.rs      # Multi-layer canvas abstraction
│   │   ├── page.rs       # Page sizing and background ruling setup
│   │   └── document.rs   # Document container with page management
│   ├── io/               # Document serialization
│   │   └── xopp.rs       # Gzip-compressed XML (.xopp/.xoj) reader and writer
│   ├── tools/            # Interactive drawing handlers
│   │   ├── pen.rs        # Pen stroke builder with jitter filtering
│   │   ├── eraser.rs     # Hit detection and stroke eraser
│   │   └── shapes.rs     # Geometric shape construction
│   ├── render/           # Vector rendering pipeline
│   │   ├── paper.rs      # Notebook paper rulings and shadow rendering
│   │   └── canvas.rs     # Viewport transformation and stroke painter
│   └── history/          # Undo / Redo action history stack
└── tests/                # Automated unit and integration tests
```

---

## How to Build and Run

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (version 1.75 or later).

### Build & Run
```bash
# Clone the branch
git clone -b feat/rust-rewrite https://github.com/mirconegri/xournalpp.git
cd xournalpp

# Run in release mode
cargo run --release
```

### Running Automated Tests
```bash
cargo test
```
