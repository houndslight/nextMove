# nextMove | No ads. No trackers. Just your applications.

**nextMove** is a simple, open-source job application tracker built in **Rust** with [egui](https://github.com/emilk/egui).  
Keep your career organized without handing your data to a third party.

---

## Features
- Add and manage job applications (company, position, status)
- Track multiple interview dates per application
- Update statuses as applications progress
- Local data storage via `job_data.json` (no external servers)
- Clean, distraction-free interface powered by egui
- Fully open-source, zero ads, zero telemetry

---

## Preview
*(screenshot placeholder — add a UI screenshot here)*

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable recommended)
- Cargo (comes with Rust)

### Clone & Run
```bash
git clone https://github.com/yourusername/nextMove.git
cd nextMove
cargo run

### Build For Release

cargo build --release

### Data Storage

All job application data is saved locally in:
job_data.json

### Roadmap

- Export applications to CSV/Markdown

- Search and filter jobs

- Tags/labels for applications

- Pre-built cross-platform binaries

### Contributing

Contributions are welcome.

- Fork the repository

- Create a feature branch (git checkout -b feature/my-feature)

- Commit changes (git commit -m "Add my feature")

- Push to your branch (git push origin feature/my-feature)

- Open a Pull Request

### License

Licensed under the MIT License.
Free to use, modify, and distribute.

###Philosophy

##nextMove was designed to be:

- Local-first — all data stays on your machine

- Minimal — no unnecessary features or bloat

- Free & Open — built in Rust and shared with the community
