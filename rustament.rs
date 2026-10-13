#[macro_export]
macro_rules! chapter {
    ($story:expr, $book:literal, $number:literal, $($line:literal),+ $(,)?) => {{
        $story.record_chapter($book, $number, &[$($line),+])?;
    }};
}

#[macro_export]
macro_rules! meaning {
    (
        $genre:expr,
        $actor:expr,
        $action:expr,
        $focus:expr,
        $register:expr,
        $rust_form:expr,
        $($signal:expr),+
        $(,)?
    ) => {
        $crate::Meaning {
            genre: $genre,
            actor: $actor,
            action: $action,
            focus: $focus,
            register: $register,
            rust_form: $rust_form,
            signals: &[$($signal),+],
        }
    };
}

#[macro_export]
macro_rules! verse {
    ($story:expr, $book:expr, $chapter:literal, $number:literal, $meaning:expr $(,)?) => {{
        $story.record_verse($crate::Verse {
            book: $book,
            chapter: $chapter,
            number: $number,
            meaning: $meaning,
        })?;
    }};
}

#[path = "books/new-testament-general-revelation.rs"]
mod new_testament_general_revelation;
#[path = "books/new-testament-gospels-acts.rs"]
mod new_testament_gospels_acts;
#[path = "books/new-testament-pauline.rs"]
mod new_testament_pauline;
#[path = "books/old-testament-history.rs"]
mod old_testament_history;
#[path = "books/old-testament-major-prophets.rs"]
mod old_testament_major_prophets;
#[path = "books/old-testament-minor-prophets.rs"]
mod old_testament_minor_prophets;
#[path = "books/old-testament-pentateuch.rs"]
mod old_testament_pentateuch;
#[path = "books/old-testament-wisdom.rs"]
mod old_testament_wisdom;
mod verse_counts;
#[path = "verses/new-testament-general-revelation.rs"]
mod verses_new_testament_general_revelation;
#[path = "verses/new-testament-gospels-acts.rs"]
mod verses_new_testament_gospels_acts;
#[path = "verses/new-testament-pauline.rs"]
mod verses_new_testament_pauline;
#[path = "verses/old-testament-history.rs"]
mod verses_old_testament_history;
#[path = "verses/old-testament-major-prophets.rs"]
mod verses_old_testament_major_prophets;
#[path = "verses/old-testament-minor-prophets.rs"]
mod verses_old_testament_minor_prophets;
#[path = "verses/old-testament-pentateuch.rs"]
mod verses_old_testament_pentateuch;
#[path = "verses/old-testament-wisdom.rs"]
mod verses_old_testament_wisdom;

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
pub enum Genre {
    Narrative,
    Law,
    Wisdom,
    Prophecy,
    Gospel,
    Letter,
    Vision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    God,
    Humanity,
    Community,
    Witness,
    King,
    Priest,
    Messenger,
    Spirit,
    Messiah,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Generate,
    Die,
    Rise,
    Heal,
    Suffer,
    Deliver,
    Judge,
    Restore,
    Command,
    Worship,
    Praise,
    Pray,
    Reveal,
    Teach,
    Speak,
    Move,
    Provide,
    Rule,
    Love,
    Trust,
    Fail,
    Witness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Cosmos,
    Covenant,
    Law,
    Temple,
    Kingdom,
    Household,
    Justice,
    Wisdom,
    Spirit,
    Adversary,
    Messiah,
    Renewal,
    Israel,
    Nations,
    Community,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Narration,
    Instruction,
    Promise,
    Warning,
    Petition,
    Vision,
    Praise,
    Lament,
    Genealogy,
    Wisdom,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Beginning,
    Separation,
    Naming,
    Genealogy,
    Covenant,
    Exodus,
    Wilderness,
    Land,
    Kingship,
    Exile,
    Return,
    Temple,
    Sacrifice,
    Justice,
    Mercy,
    Wisdom,
    Praise,
    Lament,
    Prophecy,
    Messenger,
    Healing,
    Hospitality,
    Conflict,
    Persecution,
    Resurrection,
    Mission,
    Church,
    Judgment,
    Renewal,
    Provision,
    Prayer,
    Marriage,
    Birth,
    Death,
    Famine,
    Harvest,
    Law,
    Spirit,
    Baptism,
    Faith,
    Love,
    Neighbor,
    Adversary,
    Vision,
    Witness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meaning {
    pub genre: Genre,
    pub actor: Actor,
    pub action: Action,
    pub focus: Focus,
    pub register: Register,
    pub rust_form: RustForm,
    pub signals: &'static [Signal],
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
        let label = match self {
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
        };
        formatter.write_str(label)
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
pub struct Verse {
    pub book: Book,
    pub chapter: u16,
    pub number: u16,
    pub meaning: Meaning,
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
    InvalidVerseOrder {
        book: Book,
        chapter: u16,
        number: u16,
        expected: String,
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
            Self::InvalidVerseOrder {
                book,
                chapter,
                number,
                expected,
            } => write!(
                formatter,
                "received {book} {chapter}:{number}, but the verse cursor expected {expected}"
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
    verses: Vec<Verse>,
    people: Vec<String>,
    covenant: Option<OwnedCovenant>,
}

impl Story {
    fn new() -> Self {
        Self {
            state: StoryState::Uninitialized,
            chapters: Vec::new(),
            verses: Vec::new(),
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

    pub(crate) fn record_verse(&mut self, verse: Verse) -> Result<(), StoryError> {
        let book_index = verse.book.index();
        let chapter_index = usize::from(verse.chapter.saturating_sub(1));
        let max = CANONICAL_VERSE_COUNTS
            .get(book_index)
            .and_then(|chapters| chapters.get(chapter_index))
            .copied()
            .ok_or(StoryError::VerseOutOfRange {
                book: verse.book,
                chapter: verse.chapter,
                number: verse.number,
                max: 0,
            })?;

        if verse.number == 0 || verse.number > max {
            return Err(StoryError::VerseOutOfRange {
                book: verse.book,
                chapter: verse.chapter,
                number: verse.number,
                max,
            });
        }

        if let Some(previous) = self.verses.last() {
            let expected = if previous.book == verse.book && previous.chapter == verse.chapter {
                format!("{} {}:{}", verse.book, verse.chapter, previous.number + 1)
            } else if previous.book == verse.book {
                format!("{} {}:1", verse.book, previous.chapter + 1)
            } else {
                let next_book = CANONICAL_BOOKS
                    .get(previous.book.index() + 1)
                    .copied()
                    .unwrap_or("end of canon");
                format!("{next_book} 1:1")
            };
            let in_order = if previous.book == verse.book && previous.chapter == verse.chapter {
                verse.number == previous.number + 1
            } else if previous.book == verse.book {
                verse.chapter == previous.chapter + 1 && verse.number == 1
            } else {
                verse.book.index() == previous.book.index() + 1
                    && verse.chapter == 1
                    && verse.number == 1
            };
            if !in_order {
                return Err(StoryError::InvalidVerseOrder {
                    book: verse.book,
                    chapter: verse.chapter,
                    number: verse.number,
                    expected,
                });
            }
        } else if verse.book != Book::Genesis || verse.chapter != 1 || verse.number != 1 {
            return Err(StoryError::InvalidVerseOrder {
                book: verse.book,
                chapter: verse.chapter,
                number: verse.number,
                expected: "Genesis 1:1".to_owned(),
            });
        }

        self.verses.push(verse);
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

    fn chapters(&self) -> &[Chapter] {
        &self.chapters
    }

    fn verses(&self) -> &[Verse] {
        &self.verses
    }

    fn render(&self) {
        println!("THE RUSTAMENT");
        println!("==============");
        println!(
            "A complete, non-devotional Rust parody: {} chapters and {} verses, final state `{}`.",
            self.chapters.len(),
            self.verses.len(),
            self.state
        );
        if let Some(covenant) = &self.covenant {
            println!(
                "Covenant ownership: {} witnesses retained for {:?}.",
                covenant.witnesses.len(),
                covenant.promise
            );
        }

        for chapter in self.chapters() {
            println!(
                "\n// CHAPTER {} {} [{}]",
                chapter.book, chapter.number, chapter.state
            );
            for line in &chapter.lines {
                println!("{line}");
            }
        }

        println!("\nVERSE-LEVEL SEMANTIC LEDGER");
        println!("===========================");
        for verse in self.verses() {
            println!(
                "{} {}:{} => {:?}",
                verse.book, verse.chapter, verse.number, verse.meaning
            );
        }
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

    old_testament_pentateuch::apply(&mut story)?;
    old_testament_history::apply(&mut story)?;
    old_testament_wisdom::apply(&mut story)?;
    old_testament_major_prophets::apply(&mut story)?;
    old_testament_minor_prophets::apply(&mut story)?;
    new_testament_gospels_acts::apply(&mut story)?;
    new_testament_pauline::apply(&mut story)?;
    new_testament_general_revelation::apply(&mut story)?;
    verses_old_testament_pentateuch::apply(&mut story)?;
    verses_old_testament_history::apply(&mut story)?;
    verses_old_testament_wisdom::apply(&mut story)?;
    verses_old_testament_major_prophets::apply(&mut story)?;
    verses_old_testament_minor_prophets::apply(&mut story)?;
    verses_new_testament_gospels_acts::apply(&mut story)?;
    verses_new_testament_pauline::apply(&mut story)?;
    verses_new_testament_general_revelation::apply(&mut story)?;

    if story.chapters.len() != CANONICAL_CHAPTER_COUNT
        || story.verses.len() != CANONICAL_VERSE_TOTAL
    {
        return Err(StoryError::IncompleteCanon {
            chapters: story.chapters.len(),
            verses: story.verses.len(),
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
        assert_eq!(story.verses.len(), CANONICAL_VERSE_TOTAL);
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
                .verses
                .first()
                .map(|verse| (verse.book, verse.chapter, verse.number)),
            Some((Book::Genesis, 1, 1))
        );
        assert_eq!(
            story
                .verses
                .last()
                .map(|verse| (verse.book, verse.chapter, verse.number)),
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
    fn verse_cursor_rejects_a_skipped_first_verse() {
        let mut story = Story::new();
        let result = story.record_verse(Verse {
            book: Book::Genesis,
            chapter: 1,
            number: 2,
            meaning: Meaning {
                genre: Genre::Narrative,
                actor: Actor::God,
                action: Action::Speak,
                focus: Focus::Cosmos,
                register: Register::Narration,
                rust_form: RustForm::Call,
                signals: &[Signal::Witness],
            },
        });
        assert!(matches!(result, Err(StoryError::InvalidVerseOrder { .. })));
    }
}
