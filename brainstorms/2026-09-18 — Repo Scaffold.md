# 2026-09-18 — Repo Scaffold

## What this entry covers

The first real commit to `atpret` itself, after the naming/
architecture/sequencing decisions already recorded in
`sync-dot-mesh/.github`'s `brainstorms/argenv-atpret/` folder. This
entry is the implementation of that finalized plan, not new
decisions — see that folder's 2026-09-18 16:00 entry for the actual
reasoning behind each choice referenced below.

## Scaffold

- Devenv (Nix/direnv, `init_devenv.sh`, isolated VSCodium), `.cargo/`
  linker config, `.gitattributes`, `.gitignore`, `rustfmt.toml`,
  `clippy.toml` — copied verbatim from `sync-mesh-core`, all
  genuinely project-agnostic, nothing needed adapting.
- `LICENSE` — AGPLv3, same file as `core`'s, consistent with the rest
  of the org.
- Cargo workspace: `crates/engine` (`atpret-engine`), `crates/cli`
  (`atpret-cli`), `crates/daemon` (`atpret-daemon`, binary `atpret`)
  — the hexagonal shape from the plan, minus the platform-adapter
  crates the wrapper architecture doesn't need yet (those come once
  the real `nfqws`/`winws` invocation is written).
- `atpret-cli` depends on the real, published `argenv = "0.1"` from
  crates.io — not a path dependency to a sibling repo, exactly the
  sequencing the whole `argenv`-first plan was for. Verified for
  real: `cargo test` resolved and compiled against the genuine
  crates.io release, not a local copy.

## CI, adapted from both `core` and `argenv`, not copied blind

- `ci.yml` / `pr-title.yml`: same shape as `argenv`'s, scopes changed
  to match this project (`engine`/`cli`/`daemon`/`strategy`/
  `platform`/`ci`/`deps`/`release`).
- `dependabot.yml`: identical convention (weekly, grouped minor/patch,
  cargo + github-actions).
- `release-plz.toml`: **workspace-level `publish = false`**, not a
  per-crate override on each of the three binary crates — this
  specifically applies the lesson `argenv`'s own backlog already
  flagged (a per-crate override in `sync-mesh-core` went stale and
  silently broke `release-plz` twice when the workspace was
  restructured). None of `atpret`'s crates are meant for crates.io —
  `release-plz` here only maintains the changelog and cuts GitHub
  Releases, no `cargo publish` step at all.

## Honest current state: skeleton, not a real engine yet

`atpret-engine::Strategy` is a placeholder — a name and nothing else.
No `nfqws`/`winws` invocation exists. `atpret-cli` declares exactly
one input (`--backend`/`ATPRET_BACKEND`) as a template for the rest.
`atpret-daemon`'s `main` prints a line and does nothing else. This is
deliberate: the point of this commit is a verified-working workspace
skeleton (compiles, lints clean, tests pass, wired to the real
published `argenv`) to build the actual strategy model and process
wrapper on top of — not a claim that any of it works yet.

## Verified locally before pushing

`cargo fmt --check`, `cargo clippy -- -D warnings`, and
`cargo test --workspace --all-features` all run clean in this
session's sandbox (with the `mold`-linker config temporarily bypassed
locally only, since `mold` is provided by `shell.nix`'s nix devenv
and isn't present in this plain sandbox — the real linker config is
unchanged in what's committed).

## What's next

The actual strategy ADT (desync mode, fooling method, split markers,
phase-ordering and mode-compatibility validated at construction) and
the real `nfqws` process-invocation wrapper — the next real
implementation work, not yet started.
