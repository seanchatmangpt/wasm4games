# wasm4games migration plan v26.11.16

Standing: UNVERIFIED

## Identity
- Path: /Users/sac/wasm4games
- Remote: https://github.com/seanchatmangpt/wasm4games.git
- Branch: main@6357b95 (origin/main: ahead 7, behind 12 — DIVERGED)
- Last commit: 2026-09-16; commits: 12; size: 303 MB (.git 302 MB; tree ~1.5 MB)
- Languages/build: Rust no_std (Cargo.toml, Makefile.toml, deny.toml), ggen ontology
- Nested repos: none

## Observed state
- Dirty 0. Untracked 3: `docs/jira/` entries. Stash 0. No target/ (recently untracked
  build artifacts, which explains the 302 MB .git).
- Local branch exp/wasm32-execution-proof (merged into main; 1 unpushed).

## What it is
Branchless no_std game-pattern foundry on bcinr-logic: 75 ggen-generated kernels across 15
families; generated files under src/patterns are never hand-edited.

## Classification
WARM (census agrees).

## Migration risk
MEDIUM. 7 unpushed commits on a diverged main; working tree is otherwise clean.

## Recommended action
COMMIT-PUSH
1. `cd /Users/sac/wasm4games && git log --oneline main..origin/main` (12 remote commits).
2. DECISION: rebase/merge main onto origin/main, then `git push origin main`.
3. `git add docs/jira && git commit -m "docs: jira catalogue v26.11.16"` before pushing.
4. CLONE-FRESH afterwards (history shrinks if build artifacts are not in the new clone;
   if they remain, consider history rewrite — DECISION).

## First commands on the new machine
`cargo check --workspace && cargo make check`

## See Also
- ../v26.9.19/
- /Users/sac/docs/jira/v26.11.16/README.md
