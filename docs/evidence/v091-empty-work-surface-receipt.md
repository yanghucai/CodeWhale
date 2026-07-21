# v0.9.1 empty Work-surface and product-capture receipt

This receipt backs the canonical v0.9.1 image used by the root README and
website. The image was captured from the real TUI built from the empty-surface
fix, not composed from a product mockup.

## Source

- Version: `codewhale-tui 0.9.1 (5c3eb8245512)`
- Commit: `5c3eb8245512cf790a933484453d3e300eb4c7af`
- Branch at capture: `codex/fix-empty-work-surface-091`
- Terminal: `106x32`
- Route: isolated local Ollama configuration using `qwen3-coder`; no API key
- Workspace: clean public fixture shown as `~/codewhale-demo`
- Theme: Blue Stage dark

## Capture

The binary was built with:

```bash
cargo build -p codewhale-tui --bin codewhale-tui --locked
```

It was launched in a 106-column by 32-row PTY with isolated state:

```bash
CODEWHALE_HOME=/tmp/codewhale-v091-receipt/home \\
CODEWHALE_CONFIG_PATH=/tmp/codewhale-v091-receipt/home/config.toml \\
target/debug/codewhale-tui \\
  --skip-onboarding \\
  --fresh \\
  --no-project-config \\
  --no-mouse-capture \\
  --workspace /tmp/codewhale-v091-receipt/codewhale-demo
```

The PTY frame was rendered at 1280x720 without changing its content. A small
receipt label records the version, source SHA, and terminal dimensions. The
frame contains no username, credential, account identifier, private repository
path, error state, or unsupported product claim.

Canonical copies:

- `assets/screenshot.png`
- `web/public/codewhale-tui.png`

An automated web contract requires the two PNG files to remain byte-identical,
1280x720, and below 500 KiB.

## Acceptance

The fresh active session contains the header, idle canvas, composer, and footer.
It does not contain `Work · empty`, a Work heading, or reserved Work rows.
Active, error, and disconnected Work projections remain covered separately by
the TUI unit suite.

Verification at capture:

```text
cargo test -p codewhale-tui --bin codewhale-tui --locked
test result: ok. 7799 passed; 0 failed; 3 ignored

cargo test -p codewhale-tui --bin codewhale-tui --locked tui::work_surface::tests::
test result: ok. 23 passed; 0 failed
```
