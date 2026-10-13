# The Rustament

The Rustament is a complete Rust-style parody retelling of the Protestant
66-book Bible. It is original creative writing, not a translation, devotional
text, or replacement for any Bible edition.

The project keeps the whole story in executable Rust syntax: creation,
fracture, covenant, exile, prophetic hope, incarnation, crucifixion,
resurrection, mission, letters, judgment, and renewal. It is written to be
read as code, with the Rust design carrying the narrative rather than serving
as a decorative comment layer.

## Scope

- 66 books
- 1,189 chapters
- 31,102 verses, represented by executable `verse!` calls
- Eight source partitions in Protestant canonical order
- At least two original prose strings for every chapter
- One typed semantic `meaning!` payload for every verse
- A no-dependency Cargo binary

The chapter prose is the readable narrative retelling. The verse ledger is the
fine-grained Rust retelling: every canonical book/chapter/verse coordinate is
present as a checked `Verse` with a genre, actor, action, focus, register,
Rust-shaped form, and one or more thematic signals. The ledger intentionally
contains no copied Bible wording, prose strings, comments, ranges, or
placeholders.

## Run

```sh
cargo run --quiet
```

The binary prints the complete retelling in canonical order. To validate the
source without running it:

```sh
node verify-completeness.mjs
```

The validator checks all 66 books, all 1,189 chapters, and all 31,102 verses.
It parses the Rust macro calls, verifies canonical ordering and per-chapter
verse limits, requires a semantic payload and Rust-shaped form for every verse,
and rejects generic `Unclassified` signals, string/character literals,
comments, and placeholders in the verse ledger.

The chapter conversion script is retained as a reproducible authoring tool:

```sh
node tools/convert_chapters_to_rust.mjs
```

It reads the original chapter prose already present in each partition,
verifies its order against `canon.json`, and emits real `chapter!` calls.

The verse ledger is regenerated from a local versification source with:

```sh
node tools/generate_verse_rust.mjs /tmp/rustament-source/kjv.json
cargo fmt
```

Generation requires the standard Protestant total of 31,102 verses and writes
the chapter-by-chapter counts into `canon.json` and `verse_counts.rs`.

## Rust design

- `Story` owns the accumulated chapter stream and the current narrative state.
- `Story` also owns a verse stream whose entries carry typed semantic meaning.
- `StoryState` models the movement from `Uninitialized` through `Garden`,
  `Fractured`, `Covenanted`, `Exiled`, `AwaitingDeliverance`, `Incarnate`,
  `Crucified`, `Risen`, `Sent`, and `Renewed`.
- `Covenant<'a>` borrows witnesses before transferring them into
  `OwnedCovenant`, making ownership and lifetime language part of the story.
- `Result` and `?` make every chapter load pass through a checked pipeline.
- `match` drives both state transitions and the final success or failure path.
- The chapter cursor rejects missing, duplicated, or out-of-order books and
  chapters at runtime.
- The verse cursor rejects missing, duplicated, out-of-order, or out-of-range
  verses at runtime.
- Each partition exposes `pub fn apply`, and each biblical book has a private
  function containing its chapter calls. The matching `verses/` partitions
  contain one function and one executable call per verse.

The eight partitions are:

1. [`books/old-testament-pentateuch.rs`](books/old-testament-pentateuch.rs):
   Genesis through Deuteronomy, 187 chapters
2. [`books/old-testament-history.rs`](books/old-testament-history.rs):
   Joshua through Esther, 249 chapters
3. [`books/old-testament-wisdom.rs`](books/old-testament-wisdom.rs):
   Job through Song of Solomon, 243 chapters
4. [`books/old-testament-major-prophets.rs`](books/old-testament-major-prophets.rs):
   Isaiah through Daniel, 183 chapters
5. [`books/old-testament-minor-prophets.rs`](books/old-testament-minor-prophets.rs):
   Hosea through Malachi, 67 chapters
6. [`books/new-testament-gospels-acts.rs`](books/new-testament-gospels-acts.rs):
   Matthew through Acts, 117 chapters
7. [`books/new-testament-pauline.rs`](books/new-testament-pauline.rs):
   Romans through Philemon, 87 chapters
8. [`books/new-testament-general-revelation.rs`](books/new-testament-general-revelation.rs):
   Hebrews through Revelation, 56 chapters
