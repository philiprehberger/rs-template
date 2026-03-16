# Changelog

## 0.1.5 (2026-03-16)

- Add README badges
- Synchronize version across Cargo.toml, README, and CHANGELOG

## 0.1.0 (2026-03-15)

- Initial release
- Named placeholders with `{name}` syntax
- Dot notation for nested map access (`{user.email}`)
- Conditionals (`{#if flag}...{/if}`) with negation support
- Loops (`{#each items}...{/each}`) over lists and lists of maps
- Built-in filters: `upper`, `lower`, `trim`
- Escaped braces with `{{`
- Zero dependencies
