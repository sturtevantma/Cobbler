# Cobbler — Feature Planning

A scripting language that compiles to Minecraft Java Edition data packs.

**Baseline target:** Minecraft Java Edition 26.2 (pack format 107.1), with the intent to support future updates as they release.

---

## Scope

Cobbler aims to cover the **entire data pack surface** described on the [Minecraft Wiki Data Pack page](https://minecraft.wiki/w/Data_pack), including experimental/worldgen registries — not just functions.

There are two fundamentally different kinds of content in a data pack, and Cobbler treats them differently:

1. **Functions** — imperative logic (commands, control flow). Cobbler compiles these from real code.
2. **Everything else (JSON/NBT registries)** — declarative data (loot tables, recipes, advancements, worldgen, etc.). Cobbler treats these uniformly via an OOP-style object system rather than giving each registry its own bespoke syntax.

---

## Phase 1 — Language Core & Functions

The imperative half of the language.

- [ ] **Variables**
  - [ ] Int type (scoreboard-backed)
  - [ ] Float type (storage/NBT-backed)
  - [ ] Bool type
  - [ ] String type
  - [ ] Array/list type
  - [ ] Compound/struct type
- [ ] **Control flow**
  - [ ] `if` / `else`
  - [ ] `while` loop
  - [ ] `for` loop
  - [ ] `break` / `continue`
  - [ ] Early `return`
- [ ] **Operators**
  - [ ] Arithmetic operators
  - [ ] Comparison operators
  - [ ] Boolean logic operators
- [ ] **Range/array shorthand** `[5..10]`
  - [ ] Stepped ranges (e.g. `[0..10..2]`) — stretch goal
- [ ] **Functions**
  - [ ] Parameters usable as regular variables (no macro syntax exposed to author)
  - [ ] Compiler-internal macro/storage-passing calling convention
  - [ ] Compile-time constant inlining for calls where possible
  - [ ] Return values
- [ ] **Raw command passthrough**
- [ ] **Comments**
- [ ] **Dependencies / Standard Library**
  - [ ] Import system
  - [ ] Package manager for reusable code and shared definitions
  - [ ] Subfolder organization of emitted files driven by import/file structure

---

## Phase 2 — Declarative Object System (all JSON/NBT registries)

Rather than bespoke syntax per registry type, all data-driven content (loot tables, recipes, advancements, tags, predicates, item modifiers, damage types, chat types, enchantments, and all experimental/worldgen registries — dimensions, biomes, worldgen features, mob variants, banner patterns, trim materials, instruments, jukebox songs, trial spawners, dialogs, timelines, world clocks, villager trades, GameTest definitions, etc.) is modeled the same way.

### Core object system
- [ ] Class-based declaration syntax (`new ClassName()`)
- [ ] Object-initializer syntax (`{ field: value, ... }`)
- [ ] Constructor argument syntax
- [ ] Fields settable interchangeably via constructor OR initializer
- [ ] Compile-time validation of required fields before emission
- [ ] Discriminated-union subclassing (base class + typed subclasses auto-emitting correct `"type"` field)
- [ ] Object cross-referencing by variable (e.g. `new LootTableLootEntry(bonusLoot)`), resolved to `namespace:path`
- [ ] Schema-driven registry definitions (internal table mapping registry → folder → fields → per-pack-format variants)

### Scoping rule: file emission vs. inlining
- [ ] Global-scope declaration → own file at `data/<namespace>/<registry>/<path>.json`
- [ ] Local/anonymous declaration → inlined into containing JSON structure, no separate file

### Registry coverage — core (non-experimental)
- [ ] Tags (function, block, item, entity, fluid, game_event, biome, etc.)
- [ ] Loot tables
- [ ] Recipes (all recipe types)
- [ ] Advancements
- [ ] Predicates
- [ ] Item modifiers
- [ ] Damage types
- [ ] Chat types
- [ ] Enchantments
- [ ] Enchantment providers

### Registry coverage — experimental / worldgen
- [ ] Dimension
- [ ] Dimension type
- [ ] Worldgen: biome
- [ ] Worldgen: configured/placed feature
- [ ] Worldgen: carver
- [ ] Worldgen: density function
- [ ] Worldgen: noise / noise settings
- [ ] Worldgen: structure / structure set / template pool / processor list
- [ ] Worldgen: world preset / flat level generator preset
- [ ] Mob variant definitions (cat, cow, chicken, pig, frog, wolf, wolf sounds, zombie nautilus)
- [ ] Banner patterns
- [ ] Trim materials / trim patterns
- [ ] Painting variants
- [ ] Instrument definitions
- [ ] Jukebox song definitions
- [ ] Trial spawner configuration
- [ ] Sulfur cube archetype
- [ ] Dialogs
- [ ] Timelines
- [ ] World clocks
- [ ] Villager trades / trade sets
- [ ] Test environment / test instance (GameTest)
- [ ] Slot sources
- [ ] Number providers
- [ ] Decorated pot patterns (upcoming, 26.3)

---

## Naming & Path Resolution

- [ ] `PACK <name>` declaration (namespace + drives `pack.mcmeta` generation)
- [ ] Namespace resolution from `PACK`
- [ ] Subfolder resolution from source file directory path (not filename)
- [ ] Filename resolution from variable name, converted to snake_case
- [ ] Automatic cross-reference resolution as `<PACK>:<snake_case_variable_name>`

Example:
```
project/
  main.cob          -> PACK mypack
  chests/
    common.cob      -> declares bonusLoot
    rare.cob        -> declares legendaryLoot
```
Both files live under `chests/`, so both emit to `data/mypack/loot_table/chests/...json`:
- `bonusLoot` → `data/mypack/loot_table/chests/bonus_loot.json`
- `legendaryLoot` → `data/mypack/loot_table/chests/legendary_loot.json`

---

## Phase 3 — Tooling

- [ ] CLI: build command
- [ ] CLI: watch command
- [ ] CLI: project-init command
- [ ] Version targeting: schema-version abstraction mapping language constructs to correct JSON shape per target pack format
- [ ] Diagnostics with line numbers and helpful compile errors
- [ ] Structure file (`.nbt`) referencing (treated as binary asset, not authored in Cobbler)
- [ ] Syntax highlighting — future
- [ ] Language server / autocomplete — future

---

## Open Questions / To Revisit

- [ ] Whether a local variable referenced from multiple call sites within a function should ever be deduplicated/promoted, or always stays inline-only per call
- [ ] Standard library contents
- [ ] Error/validation reporting behavior in more detail
- [ ] How conditionals/loops interact with object declarations (e.g. conditionally declaring a global object)
- [ ] Full list of ergonomic shorthands beyond `[5..10]` (e.g. inline predicate/item shorthand)
