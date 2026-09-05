# Today in Cinema — dev environment
# Install everything here with: brew bundle
#
# Rust itself is deliberately NOT listed here — install it via rustup
# (https://rustup.rs), not `brew install rust`, so `rustup component add
# clippy rustfmt` and multiple toolchains work cleanly. See
# docs/dev-guide.md for details.

brew "sqlite"      # sqlite3 CLI, for inspecting data/app.db directly
brew "python@3.14" # runs scripts/seed/mine_corpus.py (Phase 7 corpus mining)
brew "gh"          # GitHub CLI
brew "watchexec"   # optional: generic file-watcher, e.g. `watchexec -e rs -- cargo run`

cask "visual-studio-code" # optional, if not already installed
