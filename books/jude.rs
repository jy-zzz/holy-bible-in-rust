use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    jude(story)?;
    Ok(())
}

fn jude(story: &mut Story) -> Result<(), StoryError> {
    jude_1(story)?;
    Ok(())
}

fn jude_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Jude",
        1,
        "Jude urged the beloved community to contend for the entrusted faith because",
        "certain intruders had turned grace into permission for corruption.",
        "The church was told to remember past judgments, reject arrogant rebellion,",
        "show mercy with discernment, and rescue people from fire without importing",
        "the stain of the failing environment into its own clothes."
    );
    jude_1_group_01(story)?;
    jude_1_group_02(story)?;
    jude_1_group_03(story)?;
    jude_1_group_04(story)?;
    jude_1_group_05(story)?;
    jude_1_group_06(story)?;
    Ok(())
}

fn jude_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jude,
        1,
        1,
        4,
        "jesus expression -> borrow -> await_value jesus jude servant",
        &["jesus", "jude", "servant", "christ", "brother"],
    );
    segment.expression(
        1,
        "jesus",
        &["jude", "servant", "christ"],
        &["jesus", "jude", "servant", "christ", "brother"],
    )?;
    segment.borrow(
        2,
        "mercy",
        &["unto", "peace", "love"],
        &["mercy", "unto", "peace", "love", "multiplied"],
    )?;
    segment.await_value(
        3,
        "beloved",
        &["gave", "diligence", "write"],
        &["beloved", "gave", "diligence", "write", "unto"],
    )?;
    segment.borrow(
        4,
        "men",
        &["certain", "crept", "unawares"],
        &["men", "certain", "crept", "unawares", "old"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jude_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jude,
        1,
        5,
        8,
        "lord drop_value -> guard -> expression -> call lord will therefore",
        &["lord", "will", "therefore", "put", "remembrance"],
    );
    segment.drop_value(
        5,
        "lord",
        &["will", "therefore", "put"],
        &["lord", "will", "therefore", "put", "remembrance"],
    )?;
    segment.guard(
        6,
        "angels",
        &["kept", "not", "first"],
        &["angels", "kept", "not", "first", "estate"],
    )?;
    segment.expression(
        7,
        "even",
        &["sodom", "gomorrha", "cities"],
        &["even", "sodom", "gomorrha", "cities", "like"],
    )?;
    segment.call(
        8,
        "likewise",
        &["filthy", "dreamers", "defile"],
        &["likewise", "filthy", "dreamers", "defile", "flesh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jude_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jude,
        1,
        9,
        12,
        "moses move_value -> guard -> drop_value moses yet michael",
        &["moses", "yet", "michael", "archangel", "contending"],
    );
    segment.move_value(
        9,
        "moses",
        &["yet", "michael", "archangel"],
        &["moses", "yet", "michael", "archangel", "contending"],
    )?;
    segment.guard(
        10,
        "speak",
        &["evil", "things", "know"],
        &["speak", "evil", "things", "know", "not"],
    )?;
    segment.move_value(
        11,
        "woe",
        &["unto", "gone", "way"],
        &["woe", "unto", "gone", "way", "cain"],
    )?;
    segment.drop_value(
        12,
        "spots",
        &["feasts", "charity", "feast"],
        &["spots", "feasts", "charity", "feast", "feeding"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jude_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jude,
        1,
        13,
        16,
        "raging expression -> match_value raging waves sea",
        &["raging", "waves", "sea", "foaming", "own"],
    );
    segment.expression(
        13,
        "raging",
        &["waves", "sea", "foaming"],
        &["raging", "waves", "sea", "foaming", "own"],
    )?;
    segment.match_value(
        14,
        "lord",
        &["enoch", "seventh", "adam"],
        &["lord", "enoch", "seventh", "adam", "prophesied"],
    )?;
    segment.expression(
        15,
        "execute",
        &["judgment", "convince", "ungodly"],
        &["execute", "judgment", "convince", "ungodly", "among"],
    )?;
    segment.expression(
        16,
        "murmurers",
        &["complainers", "walking", "own"],
        &["murmurers", "complainers", "walking", "own", "lusts"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jude_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jude,
        1,
        17,
        20,
        "lord borrow -> expression -> guard -> await_value lord beloved remember",
        &["lord", "beloved", "remember", "words", "spoken"],
    );
    segment.borrow(
        17,
        "lord",
        &["beloved", "remember", "words"],
        &["lord", "beloved", "remember", "words", "spoken"],
    )?;
    segment.expression(
        18,
        "told",
        &["mockers", "last", "time"],
        &["told", "mockers", "last", "time", "walk"],
    )?;
    segment.guard(
        19,
        "spirit",
        &["separate", "themselves", "sensual"],
        &["spirit", "separate", "themselves", "sensual", "having"],
    )?;
    segment.await_value(
        20,
        "beloved",
        &["building", "yourselves", "most"],
        &["beloved", "building", "yourselves", "most", "holy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn jude_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Jude,
        1,
        21,
        25,
        "god guard -> borrow -> panic -> trait_bound god keep yourselves",
        &["god", "keep", "yourselves", "love", "looking"],
    );
    segment.guard(
        21,
        "god",
        &["keep", "yourselves", "love"],
        &["god", "keep", "yourselves", "love", "looking"],
    )?;
    segment.borrow(
        22,
        "compassion",
        &["making", "difference"],
        &["compassion", "making", "difference"],
    )?;
    segment.panic(
        23,
        "others",
        &["save", "fear", "pulling"],
        &["others", "save", "fear", "pulling", "fire"],
    )?;
    segment.guard(
        24,
        "now",
        &["unto", "able", "keep"],
        &["now", "unto", "able", "keep", "falling"],
    )?;
    segment.trait_bound(
        25,
        "god",
        &["wise", "saviour", "glory"],
        &["god", "wise", "saviour", "glory", "majesty"],
    )?;
    segment.commit()?;
    Ok(())
}
