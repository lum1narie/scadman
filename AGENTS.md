# AGENT GUIDE

## Purpose
Document how to work in this repository: commands, structure, conventions, and gotchas.

## Project Overview
- **Language:** Rust (edition 2021, requires Rust 1.85)
- **Goal:** Generate OpenSCAD code via a typed Rust abstraction (`ScadObject`, traits, modifiers, builders)
- **Key focuses:** primitives/modifiers/blocks across 2D, 3D, and mixed dimensions plus rendering to strings

## Essential Commands
- `cargo fmt` – format Rust code
- `cargo clippy --all-targets --no-deps --all-features` – lint code with strict rules (matching `Cargo.toml` clippy config)
- `cargo clippy --fix --allow-staged --all-targets --no-deps --all-features` – apply fix-it hints during tidy
- `cargo test` – run tests located under `tests/`
- `cargo check` – ensure code compiles (used implicitly by `clippy`)
- `cargo run` – not used here but available for experimentation
- Use `cargo install cargo-make` + `cargo make`? (No Makefile; tasks defined in `Makefile.toml` should be used with `cargo make <task>`.)
- Formatting + linting combined via `cargo make tidy` (runs `clippy-fix` and `format` from `Makefile.toml`)

## Code Organization

- `src/lib.rs` exports principal helpers (`primitive_*`, `modifier_*`, `block_*`) and re-exports modules (`scad_display`, `scad_sentence`, `value_type`, etc.).
- `src/common.rs` defines core types (`ScadObject`, `ScadObjectBody`, `ScadObjectTrait`, helper aliases like `Point2D/3D`, builder traits, operators `Add/Sub/Mul`).
- `src/scad_2d.rs`, `src/scad_3d.rs`, `src/scad_mixed.rs` contain dimension-specific logic (primitives, modifiers, blocks). 
- `src/scad_sentence/` handles primitive/modifier builders split by dimension (with further submodules `primitive` and `modifier`).
- `src/scad_display.rs` provides `ScadDisplay` implementations for values and objects.
- `src/value_type.rs` defines OpenSCAD-specific scalar/vector helpers.
- `src/internal.rs`, `src/macros.rs` – likely shared infra/utilities referenced from other modules.
  - macros is must be in either of these files.
    - internal includes macros for implementation.
    - macros includes macros for library users.
- `tests/` includes integration-style assertions comparing `to_code()` outputs (e.g., `desk_clamp.rs` shows builder usage pattern).

## Naming and Style
- Builders follow `Type::build_with(|builder| builder.field(val))` pattern; panic or return `Option` depending on `modifier_*` variant.
- Dual `try_*` functions return `Option<ScadObject>`; non-try versions panic on mismatch.
- Comments can be attached via `.commented()` or `_commented` helpers, which embed `/* comment */` into generated SCAD.
- Operators `Add`, `Sub`, `Mul` are overloaded to create boolean operations, flattening existing unions/differences when owned object matches dimension.
- `ScadObject` dimension (`Object2D`, `Object3D`, `ObjectMixed`) is enforced before combining or modifying.
- Trait implementations heavily rely on `Rc<ScadObject>` for shared ownership (modifiers store references to children).
- Doc comments are thorough; follow the same style for new helpers.
- Formatting rules (for Clippy and code) are enforced via `cargo fmt` and `rustfmt` defaults.

## Testing Approach
- Tests live under `tests/` and largely assert string output from `.to_code()` (verifying generated SCAD source).
- Use `assert_eq!(scad_object.to_code(), r"expected code")` instead of comparing object structure.
- Tests cover operator overload flattening behavior, builder configuration, and multi-dimensional combinations.

## Gotchas / Non-obvious Patterns
- `modifier_*` panics when `ScadModifier::try_new` fails; prefer `try_*` variants when inputs may be mismatched.
- `ScadObject` arithmetic operations flatten nested unions/differences only on the left-hand side of associative operators.
- `ScadObject` dimension mismatch panics with messages like `Object2D + Object3D` – tests assert these panics.
- SCAD string output preserves indentation of 2 spaces (constant `INDENT = 2`).
- Clippy configuration (in `Cargo.toml`) enables many strict warnings; avoid `allow` unless justified.
- Duplicate `#[allow]` hints in tests for `unused` crate dependencies.
- `Makefile.toml` tasks can chain formatting/lint via `cargo make tidy`.

## Configuration and Environment
- Strict lint defaults defined in `Cargo.toml` under `[lints.clippy]` and `[lints.rust]`.
- Additional Clippy doc rules declared in `clippy.toml` to allow doc identifiers such as `OpenSCAD`.
- Project aims for `cargo test` coverage verifying SCAD output; no binaries or docs beyond `README.md`.

## Workflow Tips for Agents
1. Read `src/lib.rs` and related modules before adding helpers to understand builder/trait patterns.
2. When adding new SCAD sentences, extend `scad_sentence` modules (split by primitive vs modifier).
3. Update tests in `tests/binary.rs` or `tests/desk_clamp.rs` to cover new behaviors; prefer verifying `.to_code()` outputs.
4. Run `cargo make tidy` to ensure formatting and lint rules pass before merging changes.
5. Watch for `Rc` usage and ensure modifiers/blocks maintain shared ownership semantics.
6. Keep SCAD output indentation consistent with `INDENT = 2`; verifying `to_code()` strings is the primary correctness check.

## Additional Notes
- No AGENTS.md existed previously.
- No `.cursor` or `.cursorrules` directories found.
- Repository is a library rather than an executable application.

## Code Rule
- Documentation
  - If exists, include Arguments, Returns, and Exceptions in function documentation
    - Also describe any unpredictable behavior
      that may arise from the overview.
  - Don't write too many. Keep simple and clear.
- Strive to keep every line of code and documentation within 80
  characters whenever possible. If allowed by context or grammar, use
  line breaks to meet the character limit.
- Documentation and comments must be written in English

## AI Rule
- The AI must respond in the same language as the user's prompt.

### Development AI Rule

- Testing
  - Add test in `./tests` diretctories if you make new function.
  - Never change the things already exists in `./tests`.
    - Only unless the function specification change is directly
      requested in the prompt.
- Language
  - Write comments and documentations in **English**.
- Document
  - If you make changes that affect users, **update `README.md`**.
  - If you define new types or functions, write documentation.
- Git
  - **Never try to run the `git` command** unless the user directly,
    or nix flake requests the execution of a git command in the prompt.
    (Some commands of it will not work in dirty status.)
    - **You will be fined if you use git without user's demand.**
  - You are not required to generate a commit message unless the
    user directly requests it.
  - When requested to think about git commit messages, if the
    current staged content contains multiple contexts, please also
    suggest how to split them.
    - However, if there are both staged and unstaged changes
      within the same file, there is no need to split them.

## Commit Rule

- This project uses gitflow.
- This project uses gitmoji.
- Commit message must be in English.
- Commit title format: `<type>: <emoji>(<context>) <body>`
  (e.g., 'refactor: :recycle:(backend) split `calc_fraction()` into
  multiple functions')
  - Select one of emoji from the list below apppropriate to show
    context of the commit.
    - Refer `kind` of emoji.
  - `<type>`: Commit type (e.g., fix, feat, docs). `type` column in
    the table.
    - Must match one of the IDs from the emoji list.
  - `<emoji>`: The emoji, `emoji` column in the table, written as a
    string enclosed in double colons (e.g., `:bug:`, `:sparkles:`,
    `:books:`, `:rocket:`), NOT as a Unicode emoji.
    - When writing the emoji. always use the double colon notation
      (e.g., `:sparkles:`), not the Unicode emoji symbol.
      - For instance, write `:sparkles:` instead of ✨.
  - `<context>`: The context of the commit, representing an
    externally visible unit, such as a module or issue number (e.g.,
    `main`, `reader`, `fix: #1234`, `docs: #1234`).
    - `(<context>)` is optional, including the parentheses.
    - Avoid using code-specific units like function names or class
      names as context.
    - If the change type is broad or affects multiple areas, omit
      the context.
  - `<body>`: The description of commit.
    (e.g. `add function to calculate the length of meridian line`)
- Optionally, include a commit body for:
  - Details on what was changed.
  - Explanation of why the change was made.
- If you make commit body, please separate it from the title with line
  break.

### Conditions of the sentence of commit message

- Use universally understandable abbreviations.
- Keep titles under 80 characters, ideally under 50 unless necessary.
- Limit the body to 300 characters (5 lines).
  - Each line is under 80 characters.
  - Each paragraph is separated by a blank line.
  - Without counting the blank lines.

### gitmojis

The meaning of row:

- `type`, `emoji`: as above
- `description`: The description of emoji.
- `kind`: The category of emoji.
- `utf8`: emoji in utf8 format.

| type | emoji | description | kind | utf8 |
| :------------- | :-------------------------- | :------------------------------------------------------------------- | :---------- | :--- |
| feat | :sparkles: | introduce new features | general | ✨ |
| breaking | :boom: | breaking: Introduce breaking changes | general | 💥 |
| validation | :safety_vest: | validation: Add or update code related to validation. | general | 🦺 |
| perf | :zap: | performance: improve performance | performance | ⚡️ |
| thread | :thread: | thread: Add or update code related to multithreading or concurrency. | performance | 🧵 |
| comment | :bulb: | comment: Add or update comments in source code. | codes | 💡 |
| refactor | :recycle: | recycle: refactor code | codes | ♻️ |
| fmt | :art: | format: improve structure / format of the code | codes | 🎨 |
| rm | :fire: | remove: remove code or files | codes | 🔥 |
| archit | :building_construction: | architecture: Make architectural changes. | codes | 🏗️ |
| types | :label: | types: Add or update types. | codes | 🏷️ |
| deprecate | :wastebasket: | deprecate: Deprecate code that needs to be cleaned up. | codes | 🗑️ |
| UI | :lipstick: | UI: add or update the UI and style files | looks | 💄 |
| txt | :speech_balloon: | text: Add or update text and literals. | looks | 💬 |
| locale | :globe_with_meridians: | locale: Internationalization and localization. | looks | 🌐 |
| a11y | :wheelchair: | a11y: Improve accessibility. | looks | ♿️ |
| UX | :children_crossing: | UX: Improve user experience / usability. | looks | 🚸 |
| hot-fix | :ambulance: | hot-fix: critical hotfix | fix | 🚑️ |
| small-fix | :adhesive_bandage: | small-fix: simple fix for a non-critical issue | fix | 🩹 |
| bug | :bug: | bug: fix a bug | fix | 🐛 |
| typo | :pencil2: | typo: Fix typos. | fix | ✏️ |
| warning-fix | :rotating_light: | warning-fix: fix compiler / linter warnings | fix | 🚨 |
| revert | :rewind: | revert: revert changes | fix | ⏪️ |
| secure | :lock: | secure: Fix security issues | fix | 🔒️ |
| DB | :card_file_box: | database: Perform database related changes. | resources | 🗃️ |
| rename | :truck: | rename: Move or rename resources (e.g.: files, paths, routes). | resources | 🚚 |
| secret | :closed_lock_with_key: | secret: add or update secrets | resources | 🔐 |
| assets | :bento: | assets: Add or update assets. | resources | 🍱 |
| seed | :seedling: | seed: Add or update seed files. | resources | 🌱 |
| doc | :memo: | doc: add or update documentation | document | 📝 |
| license | :page_facing_up: | license: Add or update license. | document | 📄 |
| log | :loud_sound: | add log: Add or update logs. | document | 🔊 |
| rm-log | :mute: | remove log: Remove logs. | document | 🔇 |
| test | :white_check_mark: | test: add, update, or pass tests | test | ✅ |
| analytics | :chart_with_upwards_trend: | analytics: Add or update analytics or track code. | tools | 📈 |
| CI | :green_heart: | CI: fix CI build | tools | 💚 |
| insp | :monocle_face: | inspect: Data exploration/inspection. | tools | 🧐 |
| init | :tada: | init: begin a project | status | 🎉 |
| WIP | :construction: | WIP: work in progress | status | 🚧 |
| deploy | :rocket: | deploy: deploy stuff | status | 🚀 |
| tag | :bookmark: | tag: release / version tags | status | 🔖 |
| merge | :twisted_rightwards_arrows: | merge: Merge branches. | status | 🔀 |
| pkg | :package: | package: Add or update compiled files or packages. | status | 📦️ |
| conf | :wrench: | conf: Add or update configuration files. | config | 🔧 |
| dev | :hammer: | dev: Add or update development scripts. | config | 🔨 |
| CI-conf | :construction_worker: | CI-conf: Add or update CI build system. | config | 👷 |
| depen | :heavy_plus_sign: | add dependency: Add a dependency. | dependency | ➕ |
| rm-depen | :heavy_minus_sign: | remove dependency: Remove a dependency. | dependency | ➖ |
| downgrade | :agrrow_down: | downgrade: Downgrade dependencies. | dependency | ⬇️ |
| upgrade | :arrow_up: | upgrade: Upgrade dependencies. | dependency | ⬆️ |
| pin | :pushpin: | pin: Pin dependencies to specific versions. | dependency | 📌 |
| ext-API | :alien: | external API: Update code due to external API changes. | dependency | 👽️ |
| contrib | :busts_in_silhouette: | contributor: Add or update contributor(s). | real world | 👥 |
| dev-exp | :technologist: | developer: Improve developer experience. | real world | 🧑‍💻 |
| money | :money_with_wings: | money: Add sponsorships or money related infrastructure. | real world | 💸 |
| bad | :poop: | bad: Write bad code that needs to be improved. | trash | 💩 |
| drunk | :beers: | drunk: Write code drunkenly. | trash | 🍻 |
| resp | :iphone: | responsive: Work on responsive design. | todo | 📱 |
| mock | :clown_face: | mock: Mock things. | todo | 🤡 |
| egg | :egg: | egg: Add or update an easter egg. | todo | 🥚 |
| ignore | :see_no_evil: | ignore: Add or update a .gitignore file. | todo | 🙈 |
| snap | :camera_flash: | snapshots: Add or update snapshots. | todo | 📸 |
| expt | :alembic: | experiments: Perform experiments. | todo | ⚗️ |
| SEO | :mag: | SEO: Improve SEO. | todo | 🔍️ |
| error | :goal_net: | errors: Catch errors. | todo | 🥅 |
| flag | :triangular_flag_on_post: | flag: Add, update, or remove feature flags. | todo | 🚩 |
| animation | :dizzy: | animation: Add or update animations and transitions. | todo | 💫 |
| roles | :passport_control: | roles: Work on code related to authorization, roles and permissions. | todo | 🛂 |
| rm-dead | :coffin: | remove dead: Remove dead code. | todo | ⚰️ |
| fail-test | :test_tube: | fail-test: Add a failing test. | todo | 🧪 |
| biz | :necktie: | business: Add or update business logic. | todo | 👔 |
| health | :stethoscope: | health: Add or update healthcheck. | todo | 🩺 |
| infrastructure | :bricks: | infrastructure: Infrastructure related changes. | todo | 🧱 |
