//! `atpret-engine` — the typed domain core of `atpret`.
//!
//! Starting architecture (confirmed 2026-09-18, reverting an earlier
//! full-reimplementation decision): a typed wrapper around the real,
//! proven `nfqws` (Linux/OpenWrt) and `winws` (Windows) binaries from
//! `bol-van/zapret2`, not a from-scratch reimplementation of the
//! underlying DPI-evasion logic. See `brainstorms/` for the full
//! reasoning.
//!
//! This crate is intentionally still a skeleton: the real strategy ADT
//! (desync mode, fooling method, split markers, multi-profile chains,
//! phase-ordering and mode-compatibility validated at construction)
//! is the next piece of actual implementation work, not yet written.

/// A single desync strategy, validated at construction so illegal
/// phase orderings or incompatible mode combinations are unrepresentable
/// rather than discovered at runtime.
///
/// Placeholder shape — the real fields (desync mode, fooling method,
/// split markers, TTL/fooling parameters, multi-profile chaining) are
/// not yet modeled. Exists so the crate compiles and the workspace
/// wiring (cli -> engine, daemon -> engine + cli) can be verified now,
/// ahead of the real domain model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strategy {
    name: String,
}

impl Strategy {
    /// Construct a named, empty strategy. Will grow real validated
    /// fields as the domain model is implemented.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Which platform binary a strategy will ultimately be invoked
/// against. Real invocation (spawning `nfqws`/`winws` with the
/// strategy's flags, capturing output, handling failure) is not yet
/// implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// `nfqws`, via NFQUEUE — Linux/OpenWrt.
    Nfqws,
    /// `winws`, via WinDivert — Windows.
    Winws,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_carries_its_name() {
        let s = Strategy::new("baseline");
        assert_eq!(s.name(), "baseline");
    }
}
