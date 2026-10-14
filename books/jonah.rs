use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    jonah(story)?;
    Ok(())
}

fn jonah(story: &mut Story) -> Result<(), StoryError> {
    jonah_1(story)?;
    jonah_2(story)?;
    jonah_3(story)?;
    jonah_4(story)?;
    Ok(())
}

fn jonah_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Jonah",
        1,
        "Jonah received a deployment request to warn Nineveh and immediately tried",
        "to route his obedience in the opposite direction by ship.",
        "A storm exposed the runaway process, the sailors prayed while Jonah slept,",
        "and a great sea creature became an improbable containment sandbox after",
        "the prophet was thrown overboard."
    );
    jonah_1_group_01(story)?;
    jonah_1_group_02(story)?;
    jonah_1_group_03(story)?;
    jonah_1_group_04(story)?;
    Ok(())
}

fn jonah_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Jonah",
        2,
        "From the creature's interior, Jonah composed a prayer about drowning,",
        "lost footing, and being remembered when every ordinary escape had failed.",
        "He promised gratitude and obedience, and the Maintainer issued the only",
        "release command required: the creature returned Jonah to dry land with",
        "the assignment still open."
    );
    jonah_2_group_01(story)?;
    jonah_2_group_02(story)?;
    jonah_2_group_03(story)?;
    Ok(())
}

fn jonah_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Jonah",
        3,
        "Jonah entered Nineveh with the shortest warning message imaginable, and",
        "the enormous city responded with a surprisingly thorough incident review.",
        "King, citizens, and animals joined the fast, turning from violence; when",
        "the Maintainer saw the change, the announced disaster was withdrawn, to",
        "Jonah's considerable irritation and the city's unexpected relief."
    );
    jonah_3_group_01(story)?;
    jonah_3_group_02(story)?;
    jonah_3_group_03(story)?;
    Ok(())
}

fn jonah_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Jonah",
        4,
        "Jonah admitted that mercy was exactly why he had fled: he knew the",
        "Maintainer was patient, and he did not want Nineveh receiving grace.",
        "He sat outside the city, celebrated a shade plant, mourned its death,",
        "and learned that compassion for a withered leaf had exposed his narrow",
        "ownership policy toward an entire population."
    );
    jonah_4_group_01(story)?;
    jonah_4_group_02(story)?;
    jonah_4_group_03(story)?;
    Ok(())
}

fn jonah_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        1,
        1,
        4,
        "lord move_value lord now word",
        &["lord", "now", "word", "unto", "jonah"],
    );
    segment.move_value(
        1,
        "lord",
        &["now", "word", "unto"],
        &["lord", "now", "word", "unto", "jonah"],
    )?;
    segment.move_value(
        2,
        "arise",
        &["nineveh", "great", "city"],
        &["arise", "nineveh", "great", "city", "cry"],
    )?;
    segment.move_value(
        3,
        "lord",
        &["jonah", "rose", "flee"],
        &["lord", "jonah", "rose", "flee", "unto"],
    )?;
    segment.move_value(
        4,
        "lord",
        &["sent", "great", "wind"],
        &["lord", "sent", "great", "wind", "sea"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        1,
        5,
        8,
        "man move_value -> result man mariners afraid",
        &["man", "mariners", "afraid", "cried", "every"],
    );
    segment.move_value(
        5,
        "man",
        &["mariners", "afraid", "cried"],
        &["man", "mariners", "afraid", "cried", "every"],
    )?;
    segment.move_value(
        6,
        "god",
        &["shipmaster", "unto", "meanest"],
        &["god", "shipmaster", "unto", "meanest", "sleeper"],
    )?;
    segment.move_value(
        7,
        "every",
        &["fellow", "cast", "lots"],
        &["every", "fellow", "cast", "lots", "know"],
    )?;
    segment.result(
        8,
        "unto",
        &["tell", "pray", "thee"],
        &["unto", "tell", "pray", "thee", "whose"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        1,
        9,
        12,
        "lord construct -> move_value -> call lord unto hebrew",
        &["lord", "unto", "hebrew", "fear", "god"],
    );
    segment.construct(
        9,
        "lord",
        &["unto", "hebrew", "fear"],
        &["lord", "unto", "hebrew", "fear", "god"],
    )?;
    segment.move_value(
        10,
        "men",
        &["exceedingly", "afraid", "unto"],
        &["men", "exceedingly", "afraid", "unto", "why"],
    )?;
    segment.call(
        11,
        "unto",
        &["shall", "thee", "sea"],
        &["unto", "shall", "thee", "sea", "calm"],
    )?;
    segment.call(
        12,
        "unto",
        &["take", "cast", "forth"],
        &["unto", "take", "cast", "forth", "sea"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        1,
        13,
        17,
        "men move_value -> guard -> expression -> construct men nevertheless rowed",
        &["men", "nevertheless", "rowed", "hard", "bring"],
    );
    segment.move_value(
        13,
        "men",
        &["nevertheless", "rowed", "hard"],
        &["men", "nevertheless", "rowed", "hard", "bring"],
    )?;
    segment.guard(
        14,
        "lord",
        &["wherefore", "cried", "unto"],
        &["lord", "wherefore", "cried", "unto", "beseech"],
    )?;
    segment.expression(
        15,
        "took",
        &["jonah", "cast", "forth"],
        &["took", "jonah", "cast", "forth", "sea"],
    )?;
    segment.construct(
        16,
        "men",
        &["feared", "lord", "exceedingly"],
        &["men", "feared", "lord", "exceedingly", "offered"],
    )?;
    segment.expression(
        17,
        "lord",
        &["now", "prepared", "great"],
        &["lord", "now", "prepared", "great", "fish"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        2,
        1,
        4,
        "lord expression -> result -> call lord jonah prayed",
        &["lord", "jonah", "prayed", "unto", "god"],
    );
    segment.expression(
        1,
        "lord",
        &["jonah", "prayed", "unto"],
        &["lord", "jonah", "prayed", "unto", "god"],
    )?;
    segment.result(
        2,
        "lord",
        &["cried", "reason", "mine"],
        &["lord", "cried", "reason", "mine", "affliction"],
    )?;
    segment.expression(
        3,
        "hadst",
        &["cast", "deep", "midst"],
        &["hadst", "cast", "deep", "midst", "seas"],
    )?;
    segment.call(
        4,
        "cast",
        &["sight", "yet", "will"],
        &["cast", "sight", "yet", "will", "look"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        2,
        5,
        7,
        "waters expression -> move_value waters compassed even",
        &["waters", "compassed", "even", "soul", "depth"],
    );
    segment.expression(
        5,
        "waters",
        &["compassed", "even", "soul"],
        &["waters", "compassed", "even", "soul", "depth"],
    )?;
    segment.move_value(
        6,
        "went",
        &["down", "bottoms", "mountains"],
        &["went", "down", "bottoms", "mountains", "earth"],
    )?;
    segment.move_value(
        7,
        "lord",
        &["soul", "fainted", "within"],
        &["lord", "soul", "fainted", "within", "remembered"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        2,
        8,
        10,
        "observe borrow -> call observe lying vanities",
        &["observe", "lying", "vanities", "forsake", "own"],
    );
    segment.borrow(
        8,
        "observe",
        &["lying", "vanities", "forsake"],
        &["observe", "lying", "vanities", "forsake", "own"],
    )?;
    segment.call(
        9,
        "will",
        &["sacrifice", "unto", "thee"],
        &["will", "sacrifice", "unto", "thee", "voice"],
    )?;
    segment.call(
        10,
        "lord",
        &["spake", "unto", "fish"],
        &["lord", "spake", "unto", "fish", "vomited"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        3,
        1,
        4,
        "lord move_value lord word unto",
        &["lord", "word", "unto", "jonah", "second"],
    );
    segment.move_value(
        1,
        "lord",
        &["word", "unto", "jonah"],
        &["lord", "word", "unto", "jonah", "second"],
    )?;
    segment.move_value(
        2,
        "arise",
        &["unto", "nineveh", "great"],
        &["arise", "unto", "nineveh", "great", "city"],
    )?;
    segment.move_value(
        3,
        "lord",
        &["jonah", "arose", "went"],
        &["lord", "jonah", "arose", "went", "unto"],
    )?;
    segment.move_value(
        4,
        "jonah",
        &["began", "enter", "city"],
        &["jonah", "began", "enter", "city", "day's"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        3,
        5,
        7,
        "people await_value -> move_value -> guard people nineveh believed",
        &["people", "nineveh", "believed", "god", "proclaimed"],
    );
    segment.await_value(
        5,
        "people",
        &["nineveh", "believed", "god"],
        &["people", "nineveh", "believed", "god", "proclaimed"],
    )?;
    segment.move_value(
        6,
        "king",
        &["word", "unto", "nineveh"],
        &["king", "word", "unto", "nineveh", "arose"],
    )?;
    segment.guard(
        7,
        "king",
        &["caused", "proclaimed", "published"],
        &["king", "caused", "proclaimed", "published", "through"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        3,
        8,
        10,
        "man result -> guard man beast covered",
        &["man", "beast", "covered", "sackcloth", "cry"],
    );
    segment.result(
        8,
        "man",
        &["beast", "covered", "sackcloth"],
        &["man", "beast", "covered", "sackcloth", "cry"],
    )?;
    segment.guard(
        9,
        "god",
        &["tell", "will", "turn"],
        &["god", "tell", "will", "turn", "repent"],
    )?;
    segment.guard(
        10,
        "god",
        &["saw", "works", "turned"],
        &["god", "saw", "works", "turned", "evil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        4,
        1,
        4,
        "displeased expression -> move_value -> call displeased jonah exceedingly",
        &["displeased", "jonah", "exceedingly", "very", "angry"],
    );
    segment.expression(
        1,
        "displeased",
        &["jonah", "exceedingly", "very"],
        &["displeased", "jonah", "exceedingly", "very", "angry"],
    )?;
    segment.move_value(
        2,
        "lord",
        &["prayed", "unto", "pray"],
        &["lord", "prayed", "unto", "pray", "thee"],
    )?;
    segment.expression(
        3,
        "lord",
        &["therefore", "now", "take"],
        &["lord", "therefore", "now", "take", "beseech"],
    )?;
    segment.call(
        4,
        "lord",
        &["doest", "well", "angry"],
        &["lord", "doest", "well", "angry"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        4,
        5,
        8,
        "jonah construct -> expression -> move_value jonah went city",
        &["jonah", "went", "city", "sat", "east"],
    );
    segment.construct(
        5,
        "jonah",
        &["went", "city", "sat"],
        &["jonah", "went", "city", "sat", "east"],
    )?;
    segment.construct(
        6,
        "lord",
        &["god", "prepared", "gourd"],
        &["lord", "god", "prepared", "gourd", "made"],
    )?;
    segment.expression(
        7,
        "god",
        &["prepared", "worm", "morning"],
        &["god", "prepared", "worm", "morning", "rose"],
    )?;
    segment.move_value(
        8,
        "god",
        &["pass", "sun", "arise"],
        &["god", "pass", "sun", "arise", "prepared"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jonah_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jonah,
        4,
        9,
        11,
        "god drop_value -> move_value -> guard god jonah doest",
        &["god", "jonah", "doest", "well", "angry"],
    );
    segment.drop_value(
        9,
        "god",
        &["jonah", "doest", "well"],
        &["god", "jonah", "doest", "well", "angry"],
    )?;
    segment.move_value(
        10,
        "lord",
        &["hast", "pity", "gourd"],
        &["lord", "hast", "pity", "gourd", "not"],
    )?;
    segment.guard(
        11,
        "not",
        &["spare", "nineveh", "great"],
        &["not", "spare", "nineveh", "great", "city"],
    )?;
    segment.commit()?;
    Ok(())
}
