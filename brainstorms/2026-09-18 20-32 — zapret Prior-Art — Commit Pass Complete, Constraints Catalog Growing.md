# 2026-09-18 20:32 — zapret Prior-Art — Commit Pass Complete, Constraints Catalog Growing

Continuation of the earlier entry on the `zapret-prior-art` triage repo (private, separate from this one). Standing
instruction still applies: always backlog, and continue.

## Progress

- **Commit pass complete:** 974/974 commits tagged. Depth is subject + touched-file level only; **no diff was opened**,
  so root causes for commits rest on their messages and the linked issues.
- **Issues:** 461/830 analyzed (read, tagged, summarised by hand); next is #680. The remainder still carry only
  keyword-guessed tags.
- The constraints catalog (`indexes/constraints.md`, rules a typed strategy layer must enforce) now lists 109 tagged records.

## Findings worth keeping

- **Trigger-to-action model (#389, maintainer):** the DPI has a trigger and an action (block, throttle, ...); bypass defeats
  the trigger, so the action is not applied. Throttling and blocking share one cause.
- **#556:** a user asked for exactly the automation atpret targets (stop the tool, run blockcheck, apply the result) because
  DPI settings change so often. Maintainer: stop copy-pasting, record strategies, find the common denominator.
- **#599 / #629:** the maintainer refuses to auto-convert community Windows presets into router configs ("guessing what the
  user wrote is thankless"). This bears on the still-deferred decision about strategy-pack ingestion.
- **Restrictions outlive their reason:** nfqws rejected `--ipset` on purpose in Oct 2024 (#653); the prohibition was removed
  in May 2025 once an ip-to-hostname cache existed.
- **Detector bugs matter:** a retransmission counter incremented once per reassembly piece would inflate autohostlist's
  failure detector (`c45c3f00`).
- **List sources have lifespans:** a list service disappeared and its scripts were deleted (`c33720de`).
- **Recurring patterns:** delayed-onset failures a few minutes in (#344, #424) and "trash flood" recipes without a cutoff
  (#479, #584, #677).

## Still open

- Placement of the triage repo (standalone vs a folder here) and its licence.
- **zapret2** is untouched. atpret wraps its binaries, so its history and issues are likely more relevant than z1's.
- About 369 z1 issues remain; commit diffs are unread.
