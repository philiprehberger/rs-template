# Changelog

## 0.2.0 (2026-03-21)

- Add Value accessor methods: as_str(), as_f64(), as_bool(), as_list(), as_map()
- Add Value type check methods: is_string(), is_number(), is_bool(), is_list(), is_map()
- Add Default trait implementation for Value
- Add {:else} support in {#if} conditional blocks
- Add #[must_use] attributes on Template::parse() and Template::render()

## 0.1.7 (2026-03-17)

- Add readme, rust-version, documentation to Cargo.toml
- Add Development section to README
## 0.1.6 (2026-03-16)

- Update install snippet to use full version

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
