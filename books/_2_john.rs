use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _2_john(story)?;
    Ok(())
}

fn _2_john(story: &mut Story) -> Result<(), StoryError> {
    _2_john_1(story)?;
    Ok(())
}

fn _2_john_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "2 John",
        1,
        "The elder greeted the chosen lady and her children with truth and love,",
        "then linked joy to walking faithfully in the command received from the",
        "beginning.",
        "Hospitality required discernment: a deceiver who denied the Son was not",
        "to be installed as a trusted dependency, lest polite welcome become an",
        "endorsement of a hostile package."
    );
    _2_john_1_group_01(story)?;
    _2_john_1_group_02(story)?;
    _2_john_1_group_03(story)?;
    Ok(())
}

fn _2_john_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_John,
        1,
        1,
        4,
        "children guard -> expression -> borrow children elder unto",
        &["children", "elder", "unto", "elect", "lady"],
    );
    segment.guard(
        1,
        "children",
        &["elder", "unto", "elect"],
        &["children", "elder", "unto", "elect", "lady"],
    )?;
    segment.expression(
        2,
        "truth's",
        &["sake", "dwelleth", "shall"],
        &["truth's", "sake", "dwelleth", "shall", "ever"],
    )?;
    segment.borrow(
        3,
        "god",
        &["grace", "mercy", "peace"],
        &["god", "grace", "mercy", "peace", "father"],
    )?;
    segment.guard(
        4,
        "children",
        &["rejoiced", "greatly", "found"],
        &["children", "rejoiced", "greatly", "found", "walking"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_john_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_John,
        1,
        5,
        8,
        "now reinitialize -> guard -> move_value now beseech thee",
        &["now", "beseech", "thee", "lady", "not"],
    );
    segment.reinitialize(
        5,
        "now",
        &["beseech", "thee", "lady"],
        &["now", "beseech", "thee", "lady", "not"],
    )?;
    segment.guard(
        6,
        "love",
        &["walk", "commandments", "commandment"],
        &["love", "walk", "commandments", "commandment", "heard"],
    )?;
    segment.move_value(
        7,
        "jesus",
        &["many", "deceivers", "entered"],
        &["jesus", "many", "deceivers", "entered", "world"],
    )?;
    segment.guard(
        8,
        "look",
        &["yourselves", "lose", "not"],
        &["look", "yourselves", "lose", "not", "things"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_john_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_John,
        1,
        9,
        13,
        "christ guard -> move_value -> expression christ whosoever transgresseth",
        &["christ", "whosoever", "transgresseth", "abideth", "not"],
    );
    segment.guard(
        9,
        "christ",
        &["whosoever", "transgresseth", "abideth"],
        &["christ", "whosoever", "transgresseth", "abideth", "not"],
    )?;
    segment.move_value(
        10,
        "any",
        &["unto", "bring", "not"],
        &["any", "unto", "bring", "not", "doctrine"],
    )?;
    segment.expression(
        11,
        "god",
        &["biddeth", "speed", "partaker"],
        &["god", "biddeth", "speed", "partaker", "evil"],
    )?;
    segment.move_value(
        12,
        "having",
        &["many", "things", "write"],
        &["having", "many", "things", "write", "unto"],
    )?;
    segment.expression(
        13,
        "children",
        &["elect", "sister", "greet"],
        &["children", "elect", "sister", "greet", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}
