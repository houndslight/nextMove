# nextMove | No ads. No trackers. Just your applications.

A simple, open-source job application tracker built in **Rust** with [egui](https://github.com/emilk/egui).  
Keep your career organized without handing your data to a third party.

---

## Features
- Add and manage job applications (company, position, status).
- Track interview dates (with calendar-style sorting).
- Update statuses as your application progresses.
- Data persistence via `job_data.json` (local-first, no servers needed).
- Clean, distraction-free interface powered by egui.
- 100% FOSS. Zero ads. Zero telemetry.

---

## Preview
*(screenshot placeholder — add one once you have a UI shot!)*

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

---

Build

cargo build --release

---

The binary will be available in target/release/.
Data Storage

All job application data is saved to a local JSON file:

job_data.json

This means your data never leaves your machine.
If you want to back it up or sync across devices, you control how.
🔧 Roadmap

Export applications to CSV/Markdown.

Search & filter jobs.

Tags/labels for applications.

Dark mode toggle.

    Cross-platform binaries.

Contributing

---

Contributions are welcome!

    Fork the repo

    Create a feature branch (git checkout -b feature/my-feature)

    Commit changes (git commit -m "Add my feature")

    Push to branch (git push origin feature/my-feature)

    Open a Pull Request


---


License

Licensed under the MIT License.
Free to use, modify, and distribute.


---


Philosophy

Your job hunt is your data.
nextMove was built to be:

    Local-first – nothing leaves your machine.

    Minimal – no unnecessary bloat.

    Free & open – powered by Rust, shared with the community.
