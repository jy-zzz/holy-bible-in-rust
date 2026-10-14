use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    haggai(story)?;
    Ok(())
}

fn haggai(story: &mut Story) -> Result<(), StoryError> {
    haggai_1(story)?;
    haggai_2(story)?;
    Ok(())
}

fn haggai_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Haggai",
        1,
        "The returned community had paneled its own houses while the Maintainer's",
        "house remained a foundation with weeds in the build directory.",
        "Haggai asked them to inspect their drought, failed harvests, and empty",
        "wages, then urged Zerubbabel, Joshua, and everyone else to rebuild;",
        "obedience began when the people stopped postponing the obvious task."
    );
    haggai_1_group_01(story)?;
    haggai_1_group_02(story)?;
    haggai_1_group_03(story)?;
    haggai_1_group_04(story)?;
    Ok(())
}

fn haggai_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Haggai",
        2,
        "Some elders remembered the former temple and thought the new foundation",
        "looked embarrassing beside its archived glory.",
        "Haggai answered with courage: the Maintainer would shake the nations,",
        "fill the house with glory, and give Zerubbabel a signet-like role, turning",
        "a small restoration commit into a promise about a future kingdom."
    );
    haggai_2_group_01(story)?;
    haggai_2_group_02(story)?;
    haggai_2_group_03(story)?;
    haggai_2_group_04(story)?;
    haggai_2_group_05(story)?;
    haggai_2_group_06(story)?;
    Ok(())
}

fn haggai_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        1,
        1,
        4,
        "king move_value -> construct -> expression king second year",
        &["king", "second", "year", "darius", "sixth"],
    );
    segment.move_value(
        1,
        "king",
        &["second", "year", "darius"],
        &["king", "second", "year", "darius", "sixth"],
    )?;
    segment.construct(
        2,
        "lord",
        &["thus", "speaketh", "hosts"],
        &["lord", "thus", "speaketh", "hosts", "saying"],
    )?;
    segment.move_value(
        3,
        "lord",
        &["word", "haggai", "prophet"],
        &["lord", "word", "haggai", "prophet", "saying"],
    )?;
    segment.expression(
        4,
        "time",
        &["dwell", "cieled", "houses"],
        &["time", "dwell", "cieled", "houses", "house"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        1,
        5,
        8,
        "lord expression -> move_value -> construct lord now therefore",
        &["lord", "now", "therefore", "thus", "saith"],
    );
    segment.expression(
        5,
        "lord",
        &["now", "therefore", "thus"],
        &["lord", "now", "therefore", "thus", "saith"],
    )?;
    segment.move_value(
        6,
        "sown",
        &["much", "bring", "little"],
        &["sown", "much", "bring", "little", "eat"],
    )?;
    segment.expression(
        7,
        "lord",
        &["thus", "saith", "hosts"],
        &["lord", "thus", "saith", "hosts", "consider"],
    )?;
    segment.construct(
        8,
        "mountain",
        &["bring", "wood", "build"],
        &["mountain", "bring", "wood", "build", "house"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        1,
        9,
        12,
        "looked move_value -> expression -> reinitialize looked much little",
        &["looked", "much", "little", "brought", "home"],
    );
    segment.move_value(
        9,
        "looked",
        &["much", "little", "brought"],
        &["looked", "much", "little", "brought", "home"],
    )?;
    segment.expression(
        10,
        "therefore",
        &["heaven", "over", "stayed"],
        &["therefore", "heaven", "over", "stayed", "dew"],
    )?;
    segment.reinitialize(
        11,
        "called",
        &["drought", "land", "mountains"],
        &["called", "drought", "land", "mountains", "corn"],
    )?;
    segment.move_value(
        12,
        "priest",
        &["zerubbabel", "son", "shealtiel"],
        &["priest", "zerubbabel", "son", "shealtiel", "joshua"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        1,
        13,
        15,
        "people call -> move_value -> expression people spake haggai",
        &["people", "spake", "haggai", "lord's", "messenger"],
    );
    segment.call(
        13,
        "people",
        &["spake", "haggai", "lord's"],
        &["people", "spake", "haggai", "lord's", "messenger"],
    )?;
    segment.move_value(
        14,
        "lord",
        &["stirred", "spirit", "zerubbabel"],
        &["lord", "stirred", "spirit", "zerubbabel", "son"],
    )?;
    segment.expression(
        15,
        "four",
        &["twentieth", "day", "sixth"],
        &["four", "twentieth", "day", "sixth", "month"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        2,
        1,
        4,
        "lord move_value -> call -> guard -> expression lord seventh month",
        &["lord", "seventh", "month", "twentieth", "day"],
    );
    segment.move_value(
        1,
        "lord",
        &["seventh", "month", "twentieth"],
        &["lord", "seventh", "month", "twentieth", "day"],
    )?;
    segment.call(
        2,
        "judah",
        &["speak", "now", "zerubbabel"],
        &["judah", "speak", "now", "zerubbabel", "son"],
    )?;
    segment.guard(
        3,
        "left",
        &["among", "saw", "house"],
        &["left", "among", "saw", "house", "first"],
    )?;
    segment.expression(
        4,
        "lord",
        &["yet", "now", "strong"],
        &["lord", "yet", "now", "strong", "zerubbabel"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        2,
        5,
        8,
        "spirit move_value -> expression spirit according word",
        &["spirit", "according", "word", "covenanted", "egypt"],
    );
    segment.move_value(
        5,
        "spirit",
        &["according", "word", "covenanted"],
        &["spirit", "according", "word", "covenanted", "egypt"],
    )?;
    segment.expression(
        6,
        "lord",
        &["thus", "saith", "hosts"],
        &["lord", "thus", "saith", "hosts", "yet"],
    )?;
    segment.move_value(
        7,
        "nations",
        &["will", "shake", "desire"],
        &["nations", "will", "shake", "desire", "shall"],
    )?;
    segment.expression(
        8,
        "lord",
        &["silver", "mine", "gold"],
        &["lord", "silver", "mine", "gold", "saith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        2,
        9,
        12,
        "lord trait_bound -> move_value -> guard lord glory latter",
        &["lord", "glory", "latter", "house", "shall"],
    );
    segment.trait_bound(
        9,
        "lord",
        &["glory", "latter", "house"],
        &["lord", "glory", "latter", "house", "shall"],
    )?;
    segment.move_value(
        10,
        "four",
        &["twentieth", "day", "ninth"],
        &["four", "twentieth", "day", "ninth", "month"],
    )?;
    segment.guard(
        11,
        "lord",
        &["thus", "saith", "hosts"],
        &["lord", "thus", "saith", "hosts", "ask"],
    )?;
    segment.guard(
        12,
        "bear",
        &["holy", "flesh", "skirt"],
        &["bear", "holy", "flesh", "skirt", "garment"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        2,
        13,
        16,
        "haggai drop_value -> call -> result -> move_value haggai unclean dead",
        &["haggai", "unclean", "dead", "body", "touch"],
    );
    segment.drop_value(
        13,
        "haggai",
        &["unclean", "dead", "body"],
        &["haggai", "unclean", "dead", "body", "touch"],
    )?;
    segment.call(
        14,
        "people",
        &["answered", "haggai", "nation"],
        &["people", "answered", "haggai", "nation", "saith"],
    )?;
    segment.result(
        15,
        "now",
        &["pray", "consider", "day"],
        &["now", "pray", "consider", "day", "upward"],
    )?;
    segment.move_value(
        16,
        "since",
        &["days", "heap", "twenty"],
        &["since", "days", "heap", "twenty", "measures"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        2,
        17,
        20,
        "smote guard -> trait_bound -> move_value smote blasting mildew",
        &["smote", "blasting", "mildew", "hail", "labours"],
    );
    segment.guard(
        17,
        "smote",
        &["blasting", "mildew", "hail"],
        &["smote", "blasting", "mildew", "hail", "labours"],
    )?;
    segment.trait_bound(
        18,
        "consider",
        &["now", "day", "upward"],
        &["consider", "now", "day", "upward", "four"],
    )?;
    segment.move_value(
        19,
        "seed",
        &["yet", "barn", "yea"],
        &["seed", "yet", "barn", "yea", "vine"],
    )?;
    segment.move_value(
        20,
        "lord",
        &["again", "word", "unto"],
        &["lord", "again", "word", "unto", "haggai"],
    )?;
    segment.commit()?;
    Ok(())
}

fn haggai_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Haggai,
        2,
        21,
        23,
        "judah call -> move_value -> construct judah speak zerubbabel",
        &["judah", "speak", "zerubbabel", "governor", "saying"],
    );
    segment.call(
        21,
        "judah",
        &["speak", "zerubbabel", "governor"],
        &["judah", "speak", "zerubbabel", "governor", "saying"],
    )?;
    segment.move_value(
        22,
        "will",
        &["overthrow", "throne", "kingdoms"],
        &["will", "overthrow", "throne", "kingdoms", "destroy"],
    )?;
    segment.construct(
        23,
        "lord",
        &["day", "saith", "hosts"],
        &["lord", "day", "saith", "hosts", "will"],
    )?;
    segment.commit()?;
    Ok(())
}
