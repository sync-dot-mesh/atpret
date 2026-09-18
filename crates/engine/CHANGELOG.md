# Changelog

All notable changes to atpret are documented here.
## [0.1.0]


### Features
- **scaffold:** none — this is the project's first commit, nothing previously existed to change. * fix(ci): install mold on the runner, add missing scaffold scope The CI job was failing to link entirely — .cargo/config.toml pins the mold linker (present via the local Nix dev shell, not on GitHub's runner by default), and unlike ci.yml copied from argenv (which has no such pin), this workspace's config needs mold installed explicitly. Same fix sync-mesh-core's own CI already applies for the identical reason — checked the real file rather than guessed. Also adds the missing 'scaffold' PR-title scope — this PR's own title was the first real usage and the check correctly caught its absence. ([33f7403])


