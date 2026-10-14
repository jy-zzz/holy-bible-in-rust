#[macro_export]
macro_rules! chapter {
    ($story:expr, $book:literal, $number:literal, $($line:literal),+ $(,)?) => {{
        $story.record_chapter($book, $number, &[$($line),+])?;
    }};
}

#[rustfmt::skip]
#[path = "books/genesis.rs"]
mod book_genesis;
#[rustfmt::skip]
#[path = "books/exodus.rs"]
mod book_exodus;
#[rustfmt::skip]
#[path = "books/leviticus.rs"]
mod book_leviticus;
#[rustfmt::skip]
#[path = "books/numbers.rs"]
mod book_numbers;
#[rustfmt::skip]
#[path = "books/deuteronomy.rs"]
mod book_deuteronomy;
#[rustfmt::skip]
#[path = "books/joshua.rs"]
mod book_joshua;
#[rustfmt::skip]
#[path = "books/judges.rs"]
mod book_judges;
#[rustfmt::skip]
#[path = "books/ruth.rs"]
mod book_ruth;
#[rustfmt::skip]
#[path = "books/_1_samuel.rs"]
mod book_1_samuel;
#[rustfmt::skip]
#[path = "books/_2_samuel.rs"]
mod book_2_samuel;
#[rustfmt::skip]
#[path = "books/_1_kings.rs"]
mod book_1_kings;
#[rustfmt::skip]
#[path = "books/_2_kings.rs"]
mod book_2_kings;
#[rustfmt::skip]
#[path = "books/_1_chronicles.rs"]
mod book_1_chronicles;
#[rustfmt::skip]
#[path = "books/_2_chronicles.rs"]
mod book_2_chronicles;
#[rustfmt::skip]
#[path = "books/ezra.rs"]
mod book_ezra;
#[rustfmt::skip]
#[path = "books/nehemiah.rs"]
mod book_nehemiah;
#[rustfmt::skip]
#[path = "books/esther.rs"]
mod book_esther;
#[rustfmt::skip]
#[path = "books/job.rs"]
mod book_job;
#[rustfmt::skip]
#[path = "books/psalms.rs"]
mod book_psalms;
#[rustfmt::skip]
#[path = "books/proverbs.rs"]
mod book_proverbs;
#[rustfmt::skip]
#[path = "books/ecclesiastes.rs"]
mod book_ecclesiastes;
#[rustfmt::skip]
#[path = "books/song_of_solomon.rs"]
mod book_song_of_solomon;
#[rustfmt::skip]
#[path = "books/isaiah.rs"]
mod book_isaiah;
#[rustfmt::skip]
#[path = "books/jeremiah.rs"]
mod book_jeremiah;
#[rustfmt::skip]
#[path = "books/lamentations.rs"]
mod book_lamentations;
#[rustfmt::skip]
#[path = "books/ezekiel.rs"]
mod book_ezekiel;
#[rustfmt::skip]
#[path = "books/daniel.rs"]
mod book_daniel;
#[rustfmt::skip]
#[path = "books/hosea.rs"]
mod book_hosea;
#[rustfmt::skip]
#[path = "books/joel.rs"]
mod book_joel;
#[rustfmt::skip]
#[path = "books/amos.rs"]
mod book_amos;
#[rustfmt::skip]
#[path = "books/obadiah.rs"]
mod book_obadiah;
#[rustfmt::skip]
#[path = "books/jonah.rs"]
mod book_jonah;
#[rustfmt::skip]
#[path = "books/micah.rs"]
mod book_micah;
#[rustfmt::skip]
#[path = "books/nahum.rs"]
mod book_nahum;
#[rustfmt::skip]
#[path = "books/habakkuk.rs"]
mod book_habakkuk;
#[rustfmt::skip]
#[path = "books/zephaniah.rs"]
mod book_zephaniah;
#[rustfmt::skip]
#[path = "books/haggai.rs"]
mod book_haggai;
#[rustfmt::skip]
#[path = "books/zechariah.rs"]
mod book_zechariah;
#[rustfmt::skip]
#[path = "books/malachi.rs"]
mod book_malachi;
#[rustfmt::skip]
#[path = "books/matthew.rs"]
mod book_matthew;
#[rustfmt::skip]
#[path = "books/mark.rs"]
mod book_mark;
#[rustfmt::skip]
#[path = "books/luke.rs"]
mod book_luke;
#[rustfmt::skip]
#[path = "books/john.rs"]
mod book_john;
#[rustfmt::skip]
#[path = "books/acts.rs"]
mod book_acts;
#[rustfmt::skip]
#[path = "books/romans.rs"]
mod book_romans;
#[rustfmt::skip]
#[path = "books/_1_corinthians.rs"]
mod book_1_corinthians;
#[rustfmt::skip]
#[path = "books/_2_corinthians.rs"]
mod book_2_corinthians;
#[rustfmt::skip]
#[path = "books/galatians.rs"]
mod book_galatians;
#[rustfmt::skip]
#[path = "books/ephesians.rs"]
mod book_ephesians;
#[rustfmt::skip]
#[path = "books/philippians.rs"]
mod book_philippians;
#[rustfmt::skip]
#[path = "books/colossians.rs"]
mod book_colossians;
#[rustfmt::skip]
#[path = "books/_1_thessalonians.rs"]
mod book_1_thessalonians;
#[rustfmt::skip]
#[path = "books/_2_thessalonians.rs"]
mod book_2_thessalonians;
#[rustfmt::skip]
#[path = "books/_1_timothy.rs"]
mod book_1_timothy;
#[rustfmt::skip]
#[path = "books/_2_timothy.rs"]
mod book_2_timothy;
#[rustfmt::skip]
#[path = "books/titus.rs"]
mod book_titus;
#[rustfmt::skip]
#[path = "books/philemon.rs"]
mod book_philemon;
#[rustfmt::skip]
#[path = "books/hebrews.rs"]
mod book_hebrews;
#[rustfmt::skip]
#[path = "books/james.rs"]
mod book_james;
#[rustfmt::skip]
#[path = "books/_1_peter.rs"]
mod book_1_peter;
#[rustfmt::skip]
#[path = "books/_2_peter.rs"]
mod book_2_peter;
#[rustfmt::skip]
#[path = "books/_1_john.rs"]
mod book_1_john;
#[rustfmt::skip]
#[path = "books/_2_john.rs"]
mod book_2_john;
#[rustfmt::skip]
#[path = "books/_3_john.rs"]
mod book_3_john;
#[rustfmt::skip]
#[path = "books/jude.rs"]
mod book_jude;
#[rustfmt::skip]
#[path = "books/revelation.rs"]
mod book_revelation;
mod verse_counts;

use std::error::Error;
use std::fmt;

use verse_counts::{CANONICAL_VERSE_COUNTS, CANONICAL_VERSE_TOTAL};

const CANONICAL_BOOKS: &[&str] = &[
    "Genesis",
    "Exodus",
    "Leviticus",
    "Numbers",
    "Deuteronomy",
    "Joshua",
    "Judges",
    "Ruth",
    "1 Samuel",
    "2 Samuel",
    "1 Kings",
    "2 Kings",
    "1 Chronicles",
    "2 Chronicles",
    "Ezra",
    "Nehemiah",
    "Esther",
    "Job",
    "Psalms",
    "Proverbs",
    "Ecclesiastes",
    "Song of Solomon",
    "Isaiah",
    "Jeremiah",
    "Lamentations",
    "Ezekiel",
    "Daniel",
    "Hosea",
    "Joel",
    "Amos",
    "Obadiah",
    "Jonah",
    "Micah",
    "Nahum",
    "Habakkuk",
    "Zephaniah",
    "Haggai",
    "Zechariah",
    "Malachi",
    "Matthew",
    "Mark",
    "Luke",
    "John",
    "Acts",
    "Romans",
    "1 Corinthians",
    "2 Corinthians",
    "Galatians",
    "Ephesians",
    "Philippians",
    "Colossians",
    "1 Thessalonians",
    "2 Thessalonians",
    "1 Timothy",
    "2 Timothy",
    "Titus",
    "Philemon",
    "Hebrews",
    "James",
    "1 Peter",
    "2 Peter",
    "1 John",
    "2 John",
    "3 John",
    "Jude",
    "Revelation",
];
const CANONICAL_CHAPTER_COUNT: usize = 1_189;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    _1_Samuel,
    _2_Samuel,
    _1_Kings,
    _2_Kings,
    _1_Chronicles,
    _2_Chronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    Song_Of_Solomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    _1_Corinthians,
    _2_Corinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    _1_Thessalonians,
    _2_Thessalonians,
    _1_Timothy,
    _2_Timothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    _1_Peter,
    _2_Peter,
    _1_John,
    _2_John,
    _3_John,
    Jude,
    Revelation,
}

impl Book {
    fn index(self) -> usize {
        match self {
            Self::Genesis => 0,
            Self::Exodus => 1,
            Self::Leviticus => 2,
            Self::Numbers => 3,
            Self::Deuteronomy => 4,
            Self::Joshua => 5,
            Self::Judges => 6,
            Self::Ruth => 7,
            Self::_1_Samuel => 8,
            Self::_2_Samuel => 9,
            Self::_1_Kings => 10,
            Self::_2_Kings => 11,
            Self::_1_Chronicles => 12,
            Self::_2_Chronicles => 13,
            Self::Ezra => 14,
            Self::Nehemiah => 15,
            Self::Esther => 16,
            Self::Job => 17,
            Self::Psalms => 18,
            Self::Proverbs => 19,
            Self::Ecclesiastes => 20,
            Self::Song_Of_Solomon => 21,
            Self::Isaiah => 22,
            Self::Jeremiah => 23,
            Self::Lamentations => 24,
            Self::Ezekiel => 25,
            Self::Daniel => 26,
            Self::Hosea => 27,
            Self::Joel => 28,
            Self::Amos => 29,
            Self::Obadiah => 30,
            Self::Jonah => 31,
            Self::Micah => 32,
            Self::Nahum => 33,
            Self::Habakkuk => 34,
            Self::Zephaniah => 35,
            Self::Haggai => 36,
            Self::Zechariah => 37,
            Self::Malachi => 38,
            Self::Matthew => 39,
            Self::Mark => 40,
            Self::Luke => 41,
            Self::John => 42,
            Self::Acts => 43,
            Self::Romans => 44,
            Self::_1_Corinthians => 45,
            Self::_2_Corinthians => 46,
            Self::Galatians => 47,
            Self::Ephesians => 48,
            Self::Philippians => 49,
            Self::Colossians => 50,
            Self::_1_Thessalonians => 51,
            Self::_2_Thessalonians => 52,
            Self::_1_Timothy => 53,
            Self::_2_Timothy => 54,
            Self::Titus => 55,
            Self::Philemon => 56,
            Self::Hebrews => 57,
            Self::James => 58,
            Self::_1_Peter => 59,
            Self::_2_Peter => 60,
            Self::_1_John => 61,
            Self::_2_John => 62,
            Self::_3_John => 63,
            Self::Jude => 64,
            Self::Revelation => 65,
        }
    }

    fn name(self) -> &'static str {
        CANONICAL_BOOKS[self.index()]
    }
}

impl fmt::Display for Book {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustForm {
    Construct,
    Move,
    Drop,
    Reinitialize,
    Guard,
    Result,
    Match,
    Call,
    Trait,
    Panic,
    Await,
    Borrow,
    Return,
    Iterate,
    Expression,
}

impl fmt::Display for RustForm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Construct => "construct",
            Self::Move => "move",
            Self::Drop => "drop",
            Self::Reinitialize => "reinitialize",
            Self::Guard => "guard",
            Self::Result => "result",
            Self::Match => "match",
            Self::Call => "call",
            Self::Trait => "trait",
            Self::Panic => "panic",
            Self::Await => "await",
            Self::Borrow => "borrow",
            Self::Return => "return",
            Self::Iterate => "iterate",
            Self::Expression => "expression",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoryState {
    Uninitialized,
    Garden,
    Fractured,
    Covenanted,
    Exiled,
    AwaitingDeliverance,
    Incarnate,
    Crucified,
    Risen,
    Sent,
    Renewed,
}

impl StoryState {
    fn rank(self) -> usize {
        match self {
            Self::Uninitialized => 0,
            Self::Garden => 1,
            Self::Fractured => 2,
            Self::Covenanted => 3,
            Self::Exiled => 4,
            Self::AwaitingDeliverance => 5,
            Self::Incarnate => 6,
            Self::Crucified => 7,
            Self::Risen => 8,
            Self::Sent => 9,
            Self::Renewed => 10,
        }
    }
}

impl fmt::Display for StoryState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Uninitialized => "uninitialized",
            Self::Garden => "garden",
            Self::Fractured => "fractured",
            Self::Covenanted => "covenanted",
            Self::Exiled => "exiled",
            Self::AwaitingDeliverance => "awaiting deliverance",
            Self::Incarnate => "incarnate",
            Self::Crucified => "crucified",
            Self::Risen => "risen",
            Self::Sent => "sent",
            Self::Renewed => "renewed",
        })
    }
}

#[derive(Debug)]
pub struct Chapter {
    pub book: &'static str,
    pub number: u16,
    pub state: StoryState,
    pub lines: Vec<&'static str>,
}

#[derive(Debug)]
pub struct RustStep {
    pub book: Book,
    pub chapter: u16,
    pub verse: u16,
    pub form: RustForm,
    pub subject: &'static str,
    pub objects: &'static [&'static str],
    pub tokens: &'static [&'static str],
}

#[derive(Debug)]
pub struct VerseSegment {
    pub book: Book,
    pub chapter: u16,
    pub start: u16,
    pub end: u16,
    pub summary: &'static str,
    pub context: &'static [&'static str],
    pub steps: Vec<RustStep>,
}

#[derive(Debug)]
pub enum StoryError {
    UnknownBook(&'static str),
    InvalidChapterOrder {
        book: &'static str,
        number: u16,
        expected: String,
    },
    EmptyChapter {
        book: &'static str,
        number: u16,
    },
    ChapterOutOfRange {
        book: &'static str,
        number: u16,
        max: u16,
    },
    InvalidSegmentRange {
        book: Book,
        chapter: u16,
        start: u16,
        end: u16,
    },
    InvalidSegmentVerseOrder {
        book: Book,
        chapter: u16,
        verse: u16,
        expected: u16,
    },
    MissingSegmentVerse {
        book: Book,
        chapter: u16,
        start: u16,
        end: u16,
        found: usize,
    },
    SegmentOutOfOrder {
        book: Book,
        chapter: u16,
        start: u16,
        end: u16,
        expected: String,
    },
    EmptyStep {
        book: Book,
        chapter: u16,
        verse: u16,
    },
    VerseOutOfRange {
        book: Book,
        chapter: u16,
        number: u16,
        max: u16,
    },
    InvalidStateTransition {
        from: StoryState,
        to: StoryState,
        book: &'static str,
        number: u16,
    },
    IncompleteCanon {
        chapters: usize,
        verses: usize,
    },
}

impl fmt::Display for StoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownBook(book) => write!(formatter, "book {book:?} is not in the canon"),
            Self::InvalidChapterOrder {
                book,
                number,
                expected,
            } => write!(
                formatter,
                "received {book} {number}, but the canonical cursor expected {expected}"
            ),
            Self::EmptyChapter { book, number } => {
                write!(formatter, "{book} {number} arrived without prose")
            }
            Self::ChapterOutOfRange { book, number, max } => write!(
                formatter,
                "received {book} {number}, but that book has {max} chapters"
            ),
            Self::InvalidSegmentRange {
                book,
                chapter,
                start,
                end,
            } => write!(formatter, "invalid segment {book} {chapter}:{start}-{end}"),
            Self::InvalidSegmentVerseOrder {
                book,
                chapter,
                verse,
                expected,
            } => write!(
                formatter,
                "segment received {book} {chapter}:{verse}, expected verse {expected}"
            ),
            Self::MissingSegmentVerse {
                book,
                chapter,
                start,
                end,
                found,
            } => write!(
                formatter,
                "segment {book} {chapter}:{start}-{end} contains {found} steps"
            ),
            Self::SegmentOutOfOrder {
                book,
                chapter,
                start,
                end,
                expected,
            } => write!(
                formatter,
                "received segment {book} {chapter}:{start}-{end}, expected {expected}"
            ),
            Self::EmptyStep {
                book,
                chapter,
                verse,
            } => write!(
                formatter,
                "{book} {chapter}:{verse} has no translated operands"
            ),
            Self::VerseOutOfRange {
                book,
                chapter,
                number,
                max,
            } => write!(
                formatter,
                "received {book} {chapter}:{number}, but that chapter has {max} verses"
            ),
            Self::InvalidStateTransition {
                from,
                to,
                book,
                number,
            } => write!(
                formatter,
                "state transition {from} -> {to} was rejected at {book} {number}"
            ),
            Self::IncompleteCanon { chapters, verses } => write!(
                formatter,
                "canonical build contains {chapters} chapters and {verses} verses"
            ),
        }
    }
}

impl Error for StoryError {}

#[derive(Debug)]
pub struct OwnedCovenant {
    promise: String,
    witnesses: Vec<String>,
}

pub struct Covenant<'a> {
    promise: &'a str,
    witnesses: &'a [String],
}

impl<'a> Covenant<'a> {
    fn into_owned(self) -> OwnedCovenant {
        OwnedCovenant {
            promise: self.promise.to_owned(),
            witnesses: self.witnesses.to_vec(),
        }
    }
}

pub struct BorrowedPeople<'a> {
    pub names: &'a [String],
}

pub struct Story {
    state: StoryState,
    chapters: Vec<Chapter>,
    segments: Vec<VerseSegment>,
    people: Vec<String>,
    covenant: Option<OwnedCovenant>,
}

pub struct Segment<'a> {
    story: &'a mut Story,
    book: Book,
    chapter: u16,
    start: u16,
    end: u16,
    summary: &'static str,
    context: &'static [&'static str],
    next: u16,
    steps: Vec<RustStep>,
}

impl Story {
    fn new() -> Self {
        Self {
            state: StoryState::Uninitialized,
            chapters: Vec::new(),
            segments: Vec::new(),
            people: vec![
                "humanity".to_owned(),
                "Israel".to_owned(),
                "the nations".to_owned(),
                "the church".to_owned(),
            ],
            covenant: None,
        }
    }

    pub(crate) fn record_chapter(
        &mut self,
        book: &'static str,
        number: u16,
        lines: &[&'static str],
    ) -> Result<(), StoryError> {
        let book_index = CANONICAL_BOOKS
            .iter()
            .position(|candidate| *candidate == book)
            .ok_or(StoryError::UnknownBook(book))?;

        if lines.len() < 2 {
            return Err(StoryError::EmptyChapter { book, number });
        }
        let max_chapters = CANONICAL_VERSE_COUNTS
            .get(book_index)
            .map(|chapters| chapters.len() as u16)
            .unwrap_or(0);
        if number == 0 || number > max_chapters {
            return Err(StoryError::ChapterOutOfRange {
                book,
                number,
                max: max_chapters,
            });
        }

        if let Some(previous) = self.chapters.last() {
            let previous_index = CANONICAL_BOOKS
                .iter()
                .position(|candidate| *candidate == previous.book)
                .expect("stored chapters always use canonical books");
            let expected = if book_index == previous_index {
                format!("{book} {}", previous.number + 1)
            } else {
                format!("{} 1", CANONICAL_BOOKS[previous_index + 1])
            };
            let in_order = if book_index == previous_index {
                number == previous.number + 1
            } else {
                book_index == previous_index + 1 && number == 1
            };
            if !in_order {
                return Err(StoryError::InvalidChapterOrder {
                    book,
                    number,
                    expected,
                });
            }
        } else if number != 1 || book_index != 0 {
            return Err(StoryError::InvalidChapterOrder {
                book,
                number,
                expected: "Genesis 1".to_owned(),
            });
        }

        if let Some(next_state) = Self::milestone_state(book, number) {
            if next_state.rank() < self.state.rank() {
                return Err(StoryError::InvalidStateTransition {
                    from: self.state,
                    to: next_state,
                    book,
                    number,
                });
            }
            self.state = next_state;
        }

        self.chapters.push(Chapter {
            book,
            number,
            state: self.state,
            lines: lines.to_vec(),
        });
        Ok(())
    }

    pub(crate) fn segment(
        &mut self,
        book: Book,
        chapter: u16,
        start: u16,
        end: u16,
        summary: &'static str,
        context: &'static [&'static str],
    ) -> Segment<'_> {
        Segment {
            story: self,
            book,
            chapter,
            start,
            end,
            summary,
            context,
            next: start,
            steps: Vec::with_capacity(usize::from(end.saturating_sub(start)) + 1),
        }
    }

    fn record_segment(&mut self, segment: VerseSegment) -> Result<(), StoryError> {
        let max = CANONICAL_VERSE_COUNTS
            .get(segment.book.index())
            .and_then(|chapters| chapters.get(usize::from(segment.chapter.saturating_sub(1))))
            .copied()
            .ok_or(StoryError::VerseOutOfRange {
                book: segment.book,
                chapter: segment.chapter,
                number: segment.start,
                max: 0,
            })?;

        if segment.start == 0 || segment.start > segment.end || segment.end > max {
            return Err(StoryError::InvalidSegmentRange {
                book: segment.book,
                chapter: segment.chapter,
                start: segment.start,
                end: segment.end,
            });
        }

        let expected_steps = usize::from(segment.end - segment.start + 1);
        if segment.steps.len() != expected_steps {
            return Err(StoryError::MissingSegmentVerse {
                book: segment.book,
                chapter: segment.chapter,
                start: segment.start,
                end: segment.end,
                found: segment.steps.len(),
            });
        }

        if let Some(previous) = self.segments.last() {
            let previous_step = previous
                .steps
                .last()
                .expect("committed segments are non-empty");
            let expected =
                if previous_step.book == segment.book && previous_step.chapter == segment.chapter {
                    format!(
                        "{} {}:{}",
                        segment.book,
                        segment.chapter,
                        previous_step.verse + 1
                    )
                } else if previous_step.book == segment.book {
                    format!("{} {}:1", segment.book, previous_step.chapter + 1)
                } else {
                    let next_book = CANONICAL_BOOKS
                        .get(previous_step.book.index() + 1)
                        .copied()
                        .unwrap_or("end of canon");
                    format!("{next_book} 1:1")
                };
            let in_order =
                if previous_step.book == segment.book && previous_step.chapter == segment.chapter {
                    segment.start == previous_step.verse + 1
                } else if previous_step.book == segment.book {
                    segment.chapter == previous_step.chapter + 1 && segment.start == 1
                } else {
                    segment.book.index() == previous_step.book.index() + 1
                        && segment.chapter == 1
                        && segment.start == 1
                };
            if !in_order {
                return Err(StoryError::SegmentOutOfOrder {
                    book: segment.book,
                    chapter: segment.chapter,
                    start: segment.start,
                    end: segment.end,
                    expected,
                });
            }
        } else if segment.book != Book::Genesis || segment.chapter != 1 || segment.start != 1 {
            return Err(StoryError::SegmentOutOfOrder {
                book: segment.book,
                chapter: segment.chapter,
                start: segment.start,
                end: segment.end,
                expected: "Genesis 1:1".to_owned(),
            });
        }

        for (offset, step) in segment.steps.iter().enumerate() {
            let expected_verse = segment.start + offset as u16;
            if step.book != segment.book
                || step.chapter != segment.chapter
                || step.verse != expected_verse
            {
                return Err(StoryError::InvalidSegmentVerseOrder {
                    book: segment.book,
                    chapter: segment.chapter,
                    verse: step.verse,
                    expected: expected_verse,
                });
            }
        }

        self.segments.push(segment);
        Ok(())
    }

    fn milestone_state(book: &str, number: u16) -> Option<StoryState> {
        match (book, number) {
            ("Genesis", 1) => Some(StoryState::Garden),
            ("Genesis", 3) => Some(StoryState::Fractured),
            ("Genesis", 12) => Some(StoryState::Covenanted),
            ("Exodus", 1) => Some(StoryState::Exiled),
            ("Isaiah", 40) => Some(StoryState::AwaitingDeliverance),
            ("Matthew", 1) => Some(StoryState::Incarnate),
            ("Matthew", 27) => Some(StoryState::Crucified),
            ("Matthew", 28) => Some(StoryState::Risen),
            ("Acts", 2) => Some(StoryState::Sent),
            ("Revelation", 21) => Some(StoryState::Renewed),
            _ => None,
        }
    }

    fn install_covenant(&mut self, covenant: OwnedCovenant) {
        self.covenant = Some(covenant);
    }

    fn borrowed_people(&self) -> BorrowedPeople<'_> {
        BorrowedPeople {
            names: &self.people,
        }
    }

    fn verse_count(&self) -> usize {
        self.segments
            .iter()
            .map(|segment| segment.steps.len())
            .sum()
    }

    fn render(&self) {
        println!("THE RUSTAMENT");
        println!("==============");
        println!(
            "A complete, non-devotional Rust parody: {} chapters, {} verse steps in {} segments, final state `{:?}`.",
            self.chapters.len(),
            self.verse_count(),
            self.segments.len(),
            self.state
        );
        if let Some(covenant) = &self.covenant {
            println!(
                "Covenant ownership: {} witnesses retained for {:?}.",
                covenant.witnesses.len(),
                covenant.promise
            );
        }

        for chapter in &self.chapters {
            println!(
                "\n// CHAPTER {} {} [{}]",
                chapter.book, chapter.number, chapter.state
            );
            for line in &chapter.lines {
                println!("{line}");
            }
        }

        println!("\nRUST SEGMENTS");
        println!("=============");
        for segment in &self.segments {
            println!(
                "\n// {} {}:{}-{} :: {} [{}]",
                segment.book,
                segment.chapter,
                segment.start,
                segment.end,
                segment.summary,
                segment.context.join(", ")
            );
            for step in &segment.steps {
                println!(
                    "{}:{} {} {} -> {} | {}",
                    step.chapter,
                    step.verse,
                    step.form,
                    step.subject,
                    step.objects.join(", "),
                    step.tokens.join(", ")
                );
            }
        }
    }
}

macro_rules! segment_operations {
    ($($method:ident => $form:ident),+ $(,)?) => {
        $(
            pub(crate) fn $method(
                &mut self,
                verse: u16,
                subject: &'static str,
                objects: &'static [&'static str],
                tokens: &'static [&'static str],
            ) -> Result<(), StoryError> {
                self.step(verse, RustForm::$form, subject, objects, tokens)
            }
        )+
    };
}

impl<'a> Segment<'a> {
    fn step(
        &mut self,
        verse: u16,
        form: RustForm,
        subject: &'static str,
        objects: &'static [&'static str],
        tokens: &'static [&'static str],
    ) -> Result<(), StoryError> {
        if verse != self.next {
            return Err(StoryError::InvalidSegmentVerseOrder {
                book: self.book,
                chapter: self.chapter,
                verse,
                expected: self.next,
            });
        }
        if subject.is_empty() || objects.is_empty() || tokens.is_empty() {
            return Err(StoryError::EmptyStep {
                book: self.book,
                chapter: self.chapter,
                verse,
            });
        }
        self.steps.push(RustStep {
            book: self.book,
            chapter: self.chapter,
            verse,
            form,
            subject,
            objects,
            tokens,
        });
        self.next += 1;
        Ok(())
    }

    segment_operations!(
        construct => Construct,
        move_value => Move,
        drop_value => Drop,
        reinitialize => Reinitialize,
        guard => Guard,
        result => Result,
        match_value => Match,
        call => Call,
        trait_bound => Trait,
        panic => Panic,
        await_value => Await,
        borrow => Borrow,
        return_value => Return,
        iterate => Iterate,
        expression => Expression,
    );

    pub(crate) fn commit(self) -> Result<(), StoryError> {
        if self.next != self.end.saturating_add(1) {
            return Err(StoryError::MissingSegmentVerse {
                book: self.book,
                chapter: self.chapter,
                start: self.start,
                end: self.end,
                found: self.steps.len(),
            });
        }
        let story = self.story;
        story.record_segment(VerseSegment {
            book: self.book,
            chapter: self.chapter,
            start: self.start,
            end: self.end,
            summary: self.summary,
            context: self.context,
            steps: self.steps,
        })
    }
}

pub fn whole_story() -> Result<Story, StoryError> {
    let founding_witnesses = vec![
        "Noah".to_owned(),
        "Abraham".to_owned(),
        "Moses".to_owned(),
        "David".to_owned(),
    ];
    let borrowed_covenant = Covenant {
        promise: "blessing for all peoples",
        witnesses: &founding_witnesses,
    };
    let owned_covenant = borrowed_covenant.into_owned();

    let mut story = Story::new();
    story.install_covenant(owned_covenant);
    let people_view = story.borrowed_people();
    if people_view.names.is_empty() {
        return Err(StoryError::EmptyChapter {
            book: "Genesis",
            number: 1,
        });
    }

    book_genesis::apply(&mut story)?;
    book_exodus::apply(&mut story)?;
    book_leviticus::apply(&mut story)?;
    book_numbers::apply(&mut story)?;
    book_deuteronomy::apply(&mut story)?;
    book_joshua::apply(&mut story)?;
    book_judges::apply(&mut story)?;
    book_ruth::apply(&mut story)?;
    book_1_samuel::apply(&mut story)?;
    book_2_samuel::apply(&mut story)?;
    book_1_kings::apply(&mut story)?;
    book_2_kings::apply(&mut story)?;
    book_1_chronicles::apply(&mut story)?;
    book_2_chronicles::apply(&mut story)?;
    book_ezra::apply(&mut story)?;
    book_nehemiah::apply(&mut story)?;
    book_esther::apply(&mut story)?;
    book_job::apply(&mut story)?;
    book_psalms::apply(&mut story)?;
    book_proverbs::apply(&mut story)?;
    book_ecclesiastes::apply(&mut story)?;
    book_song_of_solomon::apply(&mut story)?;
    book_isaiah::apply(&mut story)?;
    book_jeremiah::apply(&mut story)?;
    book_lamentations::apply(&mut story)?;
    book_ezekiel::apply(&mut story)?;
    book_daniel::apply(&mut story)?;
    book_hosea::apply(&mut story)?;
    book_joel::apply(&mut story)?;
    book_amos::apply(&mut story)?;
    book_obadiah::apply(&mut story)?;
    book_jonah::apply(&mut story)?;
    book_micah::apply(&mut story)?;
    book_nahum::apply(&mut story)?;
    book_habakkuk::apply(&mut story)?;
    book_zephaniah::apply(&mut story)?;
    book_haggai::apply(&mut story)?;
    book_zechariah::apply(&mut story)?;
    book_malachi::apply(&mut story)?;
    book_matthew::apply(&mut story)?;
    book_mark::apply(&mut story)?;
    book_luke::apply(&mut story)?;
    book_john::apply(&mut story)?;
    book_acts::apply(&mut story)?;
    book_romans::apply(&mut story)?;
    book_1_corinthians::apply(&mut story)?;
    book_2_corinthians::apply(&mut story)?;
    book_galatians::apply(&mut story)?;
    book_ephesians::apply(&mut story)?;
    book_philippians::apply(&mut story)?;
    book_colossians::apply(&mut story)?;
    book_1_thessalonians::apply(&mut story)?;
    book_2_thessalonians::apply(&mut story)?;
    book_1_timothy::apply(&mut story)?;
    book_2_timothy::apply(&mut story)?;
    book_titus::apply(&mut story)?;
    book_philemon::apply(&mut story)?;
    book_hebrews::apply(&mut story)?;
    book_james::apply(&mut story)?;
    book_1_peter::apply(&mut story)?;
    book_2_peter::apply(&mut story)?;
    book_1_john::apply(&mut story)?;
    book_2_john::apply(&mut story)?;
    book_3_john::apply(&mut story)?;
    book_jude::apply(&mut story)?;
    book_revelation::apply(&mut story)?;

    if story.chapters.len() != CANONICAL_CHAPTER_COUNT
        || story.verse_count() != CANONICAL_VERSE_TOTAL
    {
        return Err(StoryError::IncompleteCanon {
            chapters: story.chapters.len(),
            verses: story.verse_count(),
        });
    }

    Ok(story)
}

fn main() {
    match whole_story() {
        Ok(story) => story.render(),
        Err(error) => {
            eprintln!("Rustament failed its canonical build: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_story_contains_the_complete_canon() {
        let story = whole_story().expect("the canonical story should load");
        assert_eq!(story.chapters.len(), CANONICAL_CHAPTER_COUNT);
        assert_eq!(story.verse_count(), CANONICAL_VERSE_TOTAL);
        assert!(story.segments.len() > 6_000);
        assert_eq!(story.state, StoryState::Renewed);
        assert_eq!(
            story.chapters.first().map(|chapter| chapter.book),
            Some("Genesis")
        );
        assert_eq!(
            story
                .chapters
                .last()
                .map(|chapter| (chapter.book, chapter.number)),
            Some(("Revelation", 22))
        );
        assert_eq!(
            story
                .segments
                .first()
                .and_then(|segment| segment.steps.first())
                .map(|step| (step.book, step.chapter, step.verse)),
            Some((Book::Genesis, 1, 1))
        );
        assert_eq!(
            story
                .segments
                .last()
                .and_then(|segment| segment.steps.last())
                .map(|step| (step.book, step.chapter, step.verse)),
            Some((Book::Revelation, 22, 21))
        );
    }

    #[test]
    fn chapter_cursor_rejects_a_skipped_first_chapter() {
        let mut story = Story::new();
        let result = story.record_chapter("Genesis", 2, &["one", "two"]);
        assert!(matches!(
            result,
            Err(StoryError::InvalidChapterOrder { .. })
        ));
    }

    #[test]
    fn chapter_cursor_rejects_a_chapter_beyond_the_canonical_book() {
        let mut story = Story::new();
        let result = story.record_chapter("Genesis", 51, &["one", "two"]);
        assert!(matches!(result, Err(StoryError::ChapterOutOfRange { .. })));
    }

    #[test]
    fn segment_cursor_rejects_a_skipped_first_verse() {
        let mut story = Story::new();
        let mut segment = story.segment(Book::Genesis, 1, 1, 1, "creation", &["beginning"]);
        let result = segment.construct(2, "god", &["heaven"], &["created"]);
        assert!(matches!(
            result,
            Err(StoryError::InvalidSegmentVerseOrder { .. })
        ));
    }
}
