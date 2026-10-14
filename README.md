# The Rustament

The Rustament is a complete Rust-style parody retelling of the Protestant
66-book Bible. It is original creative writing, not a translation, devotional
text, or replacement for any Bible edition.

The project keeps the whole story in executable Rust syntax: creation,
fracture, covenant, exile, prophetic hope, incarnation, crucifixion,
resurrection, mission, letters, judgment, and renewal. It is written to be
read as code, with Rust's ownership, control flow, and type system carrying the
narrative rather than serving as a decorative comment layer.

## Scope

- 66 books
- 1,189 chapters
- 31,102 verses
- 66 individual `books/*.rs` files, one canonical book per file
- Chapter prose and verse-group functions together in each book file
- Contiguous verse groups of three to five verses, with any unavoidable
  canonical tail represented explicitly rather than padded or omitted
- A direct typed Rust operation for every verse
- Source-derived lexical tokens preserved inside those typed operations
- No detached `verses/` directory
- No eight-way partition files
- No placeholder text, omitted ranges, or "the rest is similar" shortcuts
- A no-dependency Cargo binary

The readable retelling is split by book so every book can be read and reviewed
on its own. Each book contains its chapter prose and the small verse-group
functions that translate the surrounding source wording into Rust operations.
The verse groups are not a semantic-only ledger: every canonical verse has its
own coordinate and its own direct operation inside a committed segment.

## Book layout

The `books/` directory contains one file for every canonical book. File stems
use lowercase Rust identifiers, with punctuation converted to underscores and a
leading underscore added when a book name begins with a number, for example:

```text
books/genesis.rs
books/exodus.rs
books/_1_samuel.rs
books/song_of_solomon.rs
books/matthew.rs
books/revelation.rs
```

Each book exports one `apply` function and keeps its chapter prose and verse
groups together:

```rust
use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    genesis_1(story)?;
    // The remaining Genesis chapters follow in canonical order.
    Ok(())
}

fn genesis_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Genesis",
        1,
        "The unshaped world waits for a word that can begin a build.",
        "Light becomes the first visible commit, and the story learns to divide."
    );
    genesis_1_group_01(story)?;
    Ok(())
}

fn genesis_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Genesis,
        1,
        1,
        4,
        "god construct -> borrow -> match_value beginning",
        &["beginning", "light", "waters", "day", "night"],
    );
    segment.construct(
        1,
        "god",
        &["beginning", "created", "light"],
        &["beginning", "god", "created", "light"],
    )?;
    segment.borrow(
        2,
        "god",
        &["waters", "above", "below"],
        &["waters", "above", "below", "separate"],
    )?;
    segment.match_value(
        3,
        "god",
        &["day", "night"],
        &["day", "night", "called"],
    )?;
    segment.return_value(
        4,
        "god",
        &["light", "good"],
        &["saw", "light", "good"],
    )?;
    segment.commit()?;
    Ok(())
}
```

The exact operation arguments may grow as the typed vocabulary grows, but the
shape is fixed: `story.segment(...)` establishes a small verse span, every
verse in that span receives a direct typed operation, source-derived lexical
tokens remain visible in the subject, objects, and token slices, and
`segment.commit()` records the group atomically.

## Rust design

- `Story` owns the accumulated chapter prose, committed segments, and narrative
  state.
- `Story::segment` creates a bounded builder for a contiguous verse group.
- A segment validates its book, chapter, verse span, and operation order before
  it can be committed.
- `construct`, `borrow`, `match_value`, `guard`, `return_value`, `move_value`,
  `iterate`, `drop_value`, `result`, and `reinitialize` are narrative operations as
  well as real Rust-shaped mechanics.
- The typed subject, object, and lexical-token slices preserve words and short
  lexical units derived from the source wording. They are not replaced by a
  generic semantic label such as `Meaning::Narrative`.
- `segment.commit()` is the boundary at which the verse group enters the
  story. An uncommitted segment does not count toward completeness.
- `StoryState` models the movement from `Uninitialized` through `Garden`,
  `Fractured`, `Covenanted`, `Exiled`, `AwaitingDeliverance`, `Incarnate`,
  `Crucified`, `Risen`, `Sent`, and `Renewed`.
- `Covenant<'a>` borrows witnesses before transferring them into
  `OwnedCovenant`, making ownership and lifetime language part of the story.
- `Result` and `?` make book loading and segment commits pass through a
  checked pipeline.
- `match` drives both state transitions and the final success or failure path.
- The canonical cursor rejects missing, duplicated, out-of-order, and
  out-of-range book, chapter, and verse coordinates.

The project should feel like a gradual translation into Rust. A group may use
several operations, but a group function must never collapse its verses into a
single summary string, a semantic-only record, or a range that hides the
individual verse operations.

## Run

From the repository root:

```sh
cargo fmt -- --check
cargo check
cargo test --quiet
cargo clippy -- -D warnings
cargo run --quiet
```

The binary prints the complete retelling in canonical order. A successful run
must finish with all 66 books, 1,189 chapters, and 31,102 verses committed and
the final state `Renewed`.

## Generation

Generation is book-aware. The generator reads the canonical coordinates and
source wording, then writes only the 66 book files under `books/`. For each
book it must:

1. Keep chapters in canonical order and emit their original parody prose.
2. Create explicit verse-group functions for contiguous groups of three to
   five verses, representing a short canonical tail explicitly when needed.
3. Emit one direct typed operation per verse inside the group.
4. Preserve source-derived lexical tokens in the operation arguments.
5. Finish each group with `segment.commit()`.
6. Leave no auxiliary verse ledger, partition file, placeholder, or range
   shortcut behind.

The book-generator entry point is invoked with the local versification source:

```sh
node tools/generate_verse_rust.mjs /tmp/rustament-source/kjv.json
cargo fmt
```

The generator's output is the source of the executable book modules. The
generator and verifier enforce the same book-level contract described here.

## Verification

Run the completeness verifier before treating a generated edition as finished:

```sh
node verify-completeness.mjs
```

The verifier must prove all of the following:

- exactly 66 canonical book files exist under `books/`;
- no `verses/` directory or eight-way partition file is part of the source;
- every canonical book and chapter appears exactly once;
- every verse coordinate appears exactly once inside a committed segment;
- every verse group is explicit and bounded, normally to three to five verses;
- every verse has a direct typed operation with source-derived lexical tokens;
- chapter prose and verse operations live in the same book module;
- no placeholder text, omitted range, copied translation block, or semantic-only
  ledger substitutes for a verse;
- the totals are exactly 66 books, 1,189 chapters, and 31,102 verses.

The complete acceptance pass is:

```sh
node verify-completeness.mjs
cargo fmt -- --check
cargo check
cargo test --quiet
cargo clippy -- -D warnings
cargo run --quiet > /tmp/rustament-output.txt
```
