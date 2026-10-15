# Holy Bible in Rust Complete Edition

Holy Bible in Rust is an original computational modernization of the Protestant
66-book Bible as genuine Rust source. It is designed as a secure, auditable,
AI-native representation: deterministic, explicit, and efficient to inspect.
It is not intended to replace a Bible translation and must not reproduce the
wording of an existing Bible edition.

This document is the architecture contract for the complete edition. The
implementation, generator, verifier, and canonical data are separate concerns,
but they are all checked together by the repository's generation and
verification commands.

## Completeness contract

The canonical target is 66 books, 1,189 chapters, and 31,102 verses in
Protestant/KJV versification. `canon.json` remains the machine-readable source
of truth for book names, chapter counts, and per-chapter verse counts.

There must be exactly one Rust source file for each canonical book:

```text
books/genesis.rs
books/exodus.rs
books/leviticus.rs
...
books/revelation.rs
```

The file stem is a lowercase Rust identifier derived from the canonical book
name: punctuation becomes underscores, and a leading underscore is added when
the name begins with a number. Examples include `_1_samuel.rs`,
`_2_corinthians.rs`, and `song_of_solomon.rs`.

Every book file must contain:

1. One public `apply` function that loads the book in canonical order.
2. Chapter prose calls in canonical order.
3. Explicit verse-group functions that call `story.segment(...)`.
4. A direct typed Rust operation for every verse coordinate in each group.
5. A final `segment.commit()` for every successfully built group.

Verse groups are contiguous and normally contain three to five verses. If a
canonical book or chapter boundary makes a final group shorter than three,
that tail must still be represented by an explicit group function and explicit
verse operations. It must never be padded, merged invisibly, or omitted.

There is no detached verse ledger. There is no `verses/` directory, no eight-way
partition layout, and no file whose purpose is to hide verse content behind a
range, placeholder, or aggregate semantic record.

## Direct verse translation

The verse-level representation is an incremental Rust translation, not a
semantic-only index. Each verse is represented by a direct typed operation whose
arguments retain lexical tokens derived from the source wording. The operation
may be a construction, borrow, match, guard, return, move, iteration, drop,
result, reinitialization, or another established Rust-shaped action.

An operation must identify its verse coordinate, either explicitly in its
arguments or through the segment's checked verse cursor. The segment boundary
must not be used to hide which verses were translated.

Illustrative shape:

```rust
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

Later groups may use other typed operations when the source wording calls for
different Rust mechanics:

```rust
fn romans_8_group_31(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        31,
        34,
        "guard -> return_value -> move_value -> match_value",
        &["stand", "all", "chosen", "condemnation"],
    );

    segment.guard(
        31,
        "god",
        &["who", "can", "stand"],
        &["who", "can", "stand"],
    )?;
    segment.return_value(
        32,
        "god",
        &["all", "things"],
        &["all", "things", "given"],
    )?;
    segment.move_value(
        33,
        "god",
        &["chosen"],
        &["chosen", "accuse"],
    )?;
    segment.match_value(
        34,
        "christ",
        &["condemnation", "intercession"],
        &["condemnation", "intercession"],
    )?;

    segment.commit()?;
    Ok(())
}
```

These snippets define the required authoring shape, not a license to replace
source-derived words with a generic `Meaning` payload. Token types may become
more specific, but each verse must continue to carry its own direct operation
and lexical evidence.

## Book module structure

Every book module keeps its chapter prose beside its verse groups:

```rust
use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    genesis_1(story)?;
    // Continue through every Genesis chapter in canonical order.
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
```

The illustrative comment above is not a substitute for the remaining calls.
The completed source must contain every chapter function and every group
function. Group names should expose their location, for example
`genesis_1_group_01` or `genesis_1_group_02`, so a reviewer can locate a verse
without searching an auxiliary index.

## Segment builder

`Story::segment` creates a checked builder for one contiguous verse span. The
builder owns the pending operations until commit:

- `construct` expresses creation, formation, naming, or establishment.
- `borrow` expresses witnesses, references, promises, or temporary access.
- `match_value` expresses distinctions, alternatives, judgments, or fulfillment.
- `guard` expresses conditions, warnings, boundaries, and covenant checks.
- `return_value` expresses restoration, blessing, answer, or release.
- `move_value` expresses transfer, departure, inheritance, or changed ownership.
- `iterate` expresses genealogies, repeated journeys, recurring instruction, or
  measured progression.
- `drop_value` expresses death, loss, judgment, or relinquishment.
- `result` expresses a checked outcome and its error path.
- `reinitialize` expresses renewal, return, or a new beginning.
- `commit` validates and appends the complete segment to `Story`.

The vocabulary is extensible, but a new operation must remain a real typed Rust
mechanic and must retain the verse's source-derived lexical tokens. A prose
summary, a single semantic enum, or a `1..=N` range without one operation per
verse is not a valid translation.

## Runtime architecture

- `holy_bible_in_rust.rs` defines `Story`, chapter storage, segment storage, typed
  operation records, `StoryState`, `StoryError`, and `whole_story`.
- The root module imports the 66 book modules in Protestant canonical order.
- Each book module exports `pub fn apply(story: &mut Story) -> Result<(), StoryError>`.
- `StoryState` provides the narrative state machine from creation to renewal.
- `Covenant<'a>` demonstrates borrowing and lifetime-scoped data before
  transferring it into `OwnedCovenant`.
- `Result`, `?`, `match`, `Vec`, slices, `String`, enums, and module boundaries
  are used as actual Rust mechanisms and as narrative language.
- The story cursor rejects missing, duplicated, out-of-order, and out-of-range
  book, chapter, and verse coordinates.
- A segment that fails validation cannot be committed and cannot count toward
  completeness.

Rust structures may compress repeated mechanics, but they must not replace the
chapter-level story or the direct verse operations with a catalog, range,
placeholder, or detached ledger.

## Style

- Use original modernization prose rather than quotations.
- Let ownership, borrowing, lifetimes, traits, state transitions, guards,
  results, moves, drops, migrations, merge conflicts, logs, and releases carry
  the metaphor.
- Preserve the distinctive movement of every individual chapter, including
  genealogies, poems, laws, speeches, visions, letters, and narrative turns.
- Maintain a respectful, serious tone while preserving the distinctive movement
  of each book.
- Keep chapter prose and verse operations in the same book file.
- Never add `TODO`, `CHAPTER OMITTED`, `SUMMARY OMITTED`, `[insert]`, a hidden
  chapter or verse range, or "the rest is similar."

## Generation

The generator is required to be book-aware. Given the canonical source, it must
write exactly 66 files under `books/` and no auxiliary verse directory. Its
steps are:

1. Read the canonical book, chapter, and verse coordinates.
2. Read the source wording needed to derive lexical tokens.
3. Emit each book's chapter prose and group functions into one book module.
4. Partition contiguous verses into explicit three-to-five-verse groups,
   preserving an unavoidable short tail explicitly.
5. Select a direct typed operation for every verse and pass source-derived
   tokens into it.
6. Emit `segment.commit()` after each valid group.
7. Format the 66 modules and leave no placeholder or range shortcut.

The target command is:

```sh
node .tools/generate_verse_rust.mjs /tmp/holy-bible-in-rust-source/kjv.json
cargo fmt
```

Generation is successful only when the resulting source follows this
specification. A generator that emits a detached ledger, semantic-only payload,
partition file, or range shortcut has not generated a complete Holy Bible in
Rust edition.

## Verification

The completeness verifier is an acceptance gate, not a count-only smoke test.
Run:

```sh
node .tools/verify-completeness.mjs
cargo fmt -- --check
cargo check
cargo test --quiet
cargo clippy -- -D warnings
cargo run --quiet > /tmp/holy-bible-in-rust-output.txt
```

The verifier must inspect the source tree and prove:

- exactly 66 `books/*.rs` files exist, with one file per canonical book;
- the root module imports those 66 books in canonical order;
- no `verses/` directory or eight-way partition file exists;
- every canonical chapter has chapter prose in its own book file;
- every verse coordinate appears exactly once in a committed segment;
- group functions expose contiguous three-to-five-verse spans, with explicit
  handling for an unavoidable short tail;
- every verse uses a direct typed operation;
- every operation carries source-derived lexical tokens;
- no semantic-only ledger, placeholder, omitted range, or copied translation
  block is being used as a substitute;
- the runtime commits exactly 66 books, 1,189 chapters, and 31,102 verses and
  reaches the final state `Renewed`.
