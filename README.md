# atpret

[![CI](https://github.com/sync-dot-mesh/atpret/actions/workflows/ci.yml/badge.svg)](https://github.com/sync-dot-mesh/atpret/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](./LICENSE)

Automated, cross-platform DPI-bypass orchestrator. The name reads as
"automated" + "zapret" (Russian for "block/prohibit"), with a
deliberate a-for-o substitution as wordplay ("open up").

## Status

Skeleton stage. Workspace and CI are real; the actual strategy engine
is not yet implemented. See [`brainstorms/`](./brainstorms) for the
full reasoning behind every decision so far.

## Shape of the project

**Starting architecture**: a typed wrapper around the real, proven
`nfqws` (Linux/OpenWrt) and `winws` (Windows) binaries from
[`bol-van/zapret2`](https://github.com/bol-van/zapret2) — not a
from-scratch reimplementation of the underlying DPI-evasion logic.
Lower risk, ships against a proven engine, with a typed orchestration
layer (the strategy model, illegal-phase-ordering and
incompatible-mode-pair prevention enforced at construction) on top.

**Structure**, hexagonal, mirroring
[`sync-mesh-core`](https://github.com/sync-dot-mesh/core):
- `crates/engine` (`atpret-engine`) — pure domain: the strategy model,
  and the wrapper logic that invokes `nfqws`/`winws`.
- `crates/cli` (`atpret-cli`) — atpret's entire argv/env surface,
  declared via [`argenv`](https://crates.io/crates/argenv) rather
  than hand-rolled flag parsing.
- `crates/daemon` (`atpret-daemon`, binary name `atpret`) —
  composition root wiring engine + CLI-parsed config together.

**Build order**: Linux/OpenWrt first, exclusively — validated against
real, known-blocked targets before Windows is touched at all. Windows
next (`WinDivert` invocation, same strategy logic). macOS explicitly
best-effort/partial, matching upstream's own stated support level.

**Licensing**: AGPLv3, consistent with the rest of `sync-dot-mesh`.
Deliberately different from `argenv`'s permissive `MIT OR Apache-2.0`
— `argenv` is a general-purpose library meant for external adoption
independent of `atpret` entirely.

**Org placement**: lives under `sync-dot-mesh` for now, alongside
`core` and `argenv`'s sibling projects, for convenience during early
development — a move to its own org is intended later, not guaranteed.

---

## Where the actual history lives

This README is the current, synthesized shape of the project. The
reasoning behind each decision — including the ones reversed along the
way — lives chronologically in [`brainstorms/`](./brainstorms).
