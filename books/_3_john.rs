use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _3_john(story)?;
    Ok(())
}

fn _3_john(story: &mut Story) -> Result<(), StoryError> {
    _3_john_1(story)?;
    Ok(())
}

fn _3_john_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "3 John",
        1,
        "Gaius was praised for supporting traveling workers with dependable",
        "hospitality, helping the truth advance through a network of generous",
        "collaborators.",
        "Diotrephes wanted the central branch, rejected the elder's messages, and",
        "expelled faithful users, while Demetrius had a good report; the letter",
        "promised a face-to-face visit because some conflicts need more than logs."
    );
    _3_john_1_group_01(story)?;
    _3_john_1_group_02(story)?;
    _3_john_1_group_03(story)?;
    _3_john_1_group_04(story)?;
    Ok(())
}

fn _3_john_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_3_John,
        1,
        1,
        4,
        "elder borrow -> move_value -> guard elder unto wellbeloved",
        &["elder", "unto", "wellbeloved", "gaius", "love"],
    );
    segment.borrow(
        1,
        "elder",
        &["unto", "wellbeloved", "gaius"],
        &["elder", "unto", "wellbeloved", "gaius", "love"],
    )?;
    segment.borrow(
        2,
        "beloved",
        &["wish", "above", "things"],
        &["beloved", "wish", "above", "things", "mayest"],
    )?;
    segment.move_value(
        3,
        "rejoiced",
        &["greatly", "brethren", "testified"],
        &["rejoiced", "greatly", "brethren", "testified", "truth"],
    )?;
    segment.guard(
        4,
        "children",
        &["greater", "joy", "hear"],
        &["children", "greater", "joy", "hear", "walk"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _3_john_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_3_John,
        1,
        5,
        8,
        "beloved borrow -> move_value -> expression beloved doest faithfully",
        &["beloved", "doest", "faithfully", "whatsoever", "brethren"],
    );
    segment.borrow(
        5,
        "beloved",
        &["doest", "faithfully", "whatsoever"],
        &["beloved", "doest", "faithfully", "whatsoever", "brethren"],
    )?;
    segment.move_value(
        6,
        "borne",
        &["witness", "charity", "church"],
        &["borne", "witness", "charity", "church", "bring"],
    )?;
    segment.move_value(
        7,
        "name's",
        &["sake", "went", "forth"],
        &["name's", "sake", "went", "forth", "taking"],
    )?;
    segment.expression(
        8,
        "therefore",
        &["ought", "receive", "such"],
        &["therefore", "ought", "receive", "such", "might"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _3_john_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_3_John,
        1,
        9,
        11,
        "wrote guard -> move_value wrote unto church",
        &["wrote", "unto", "church", "diotrephes", "loveth"],
    );
    segment.guard(
        9,
        "wrote",
        &["unto", "church", "diotrephes"],
        &["wrote", "unto", "church", "diotrephes", "loveth"],
    )?;
    segment.move_value(
        10,
        "wherefore",
        &["will", "remember", "deeds"],
        &["wherefore", "will", "remember", "deeds", "doeth"],
    )?;
    segment.guard(
        11,
        "god",
        &["beloved", "follow", "not"],
        &["god", "beloved", "follow", "not", "evil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _3_john_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_3_John,
        1,
        12,
        14,
        "men expression -> guard -> call men demetrius hath",
        &["men", "demetrius", "hath", "good", "report"],
    );
    segment.expression(
        12,
        "men",
        &["demetrius", "hath", "good"],
        &["men", "demetrius", "hath", "good", "report"],
    )?;
    segment.guard(
        13,
        "many",
        &["things", "write", "will"],
        &["many", "things", "write", "will", "not"],
    )?;
    segment.call(
        14,
        "trust",
        &["shall", "shortly", "see"],
        &["trust", "shall", "shortly", "see", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}
