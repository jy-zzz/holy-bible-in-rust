# Rustament Complete Edition

The Rustament is an original parody retelling of the Protestant 66-book Bible
as genuine Rust source. It is not intended to replace a Bible translation and
must not reproduce the wording of an existing translation.

## Completeness contract

The canonical target is 66 books, 1,189 chapters, and 31,102 verses in
Protestant/KJV versification. `canon.json` is the machine-readable source of
truth for book names, chapter counts, per-chapter verse counts, and partition
files.

Every book in `canon.json` must appear in its assigned Rust partition. Every
chapter must appear exactly once as an executable call with this shape:

```rust
crate::chapter!(story, "Book Name", 1, "first prose line", "second prose line");
```

Each chapter call must contain at least two original prose strings. A chapter
may contain more strings when the source prose benefits from several
paragraph-sized lines.

Every verse must also appear exactly once as a typed executable call in the
matching `verses/` partition:

```rust
crate::verse!(
    story,
    Book::Genesis,
    1,
    1,
    crate::meaning!(
        Genre::Narrative,
        Actor::God,
        Action::Create,
        Focus::Cosmos,
        Register::Narration,
        RustForm::Construct,
        Signal::Beginning
    )
);
```

Verse calls are the Rustament's verse-level parody representation. They encode
the verse's interpreted narrative role and a Rust-shaped operation as data
rather than reproducing the wording of an existing Bible translation. Every
verse must have one `RustForm` and at least one thematic `Signal`, and verse
ledger files must contain no scripture strings, comments, ranges,
placeholders, or generic `Unclassified` fallbacks.

The eight partitions are authoring boundaries only. The executable pipeline
loads them in canonical order and rejects a missing, duplicated, or
out-of-order chapter or verse through `StoryError`.

## Executable architecture

- `Cargo.toml` defines the no-dependency binary.
- `rustament.rs` defines `Story`, `Chapter`, `Verse`, `Meaning`, the semantic
  enums (including `RustForm`), `StoryState`, `StoryError`, the `chapter!` and
  `verse!` macros, and `whole_story`.
- Each partition exports `pub fn apply(story: &mut Story) -> Result<(), StoryError>`.
- Each biblical book has a private function inside its partition.
- Each matching `verses/` partition exports the verse ledger in the same
  canonical order.
- `StoryState` provides a narrative state machine from creation to renewal.
- `Covenant<'a>` demonstrates borrowing and lifetime-scoped data before
  transferring it into `OwnedCovenant`.
- `Result`, `?`, `match`, `Vec`, slices, `String`, enums, and module boundaries
  are used as actual Rust mechanisms and as parody language.

The project should remain readable from Genesis through Revelation. Rust
structures are allowed to compress repeated mechanics, but they must not
replace the chapter- or verse-level story with a catalog, a range, or a
placeholder.

## Style

- Use original parody prose rather than quotations.
- Let ownership, borrowing, lifetimes, traits, state transitions, panics,
  migrations, merge conflicts, logs, and releases carry the metaphor.
- Preserve the distinctive movement of every individual chapter, including
  genealogies, poems, laws, speeches, visions, letters, and narrative turns.
- Keep the tone playful without treating religious belief as the punchline.
- Never add `TODO`, `CHAPTER OMITTED`, `SUMMARY OMITTED`, `[insert]`,
  chapter/verse ranges, or “the rest is similar.”

## Verification

Run:

```sh
node verify-completeness.mjs
cargo check
cargo test
cargo clippy -- -D warnings
cargo run --quiet > /tmp/rustament-output.txt
```

The validator checks executable calls, book functions, prose-string counts,
canonical ordering, per-chapter verse counts, semantic payload presence,
literal/comment absence in verse files, placeholder absence, and the total
counts of 1,189 chapters and 31,102 verses. The Rust runtime repeats the
canonical cursor checks and refuses to return a `Story` unless both totals are
loaded.
