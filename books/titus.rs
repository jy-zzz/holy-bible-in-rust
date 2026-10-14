use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    titus(story)?;
    Ok(())
}

fn titus(story: &mut Story) -> Result<(), StoryError> {
    titus_1(story)?;
    titus_2(story)?;
    titus_3(story)?;
    Ok(())
}

fn titus_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Titus", 1, "Titus is left in Crete to appoint elders whose households, character, and teaching can withstand disorder and empty talk.", "Rebellious teachers exploit households for dishonest gain, so their mouths must be stopped and their claims tested against the hard local reputation.");
    titus_1_group_01(story)?;
    titus_1_group_02(story)?;
    titus_1_group_03(story)?;
    titus_1_group_04(story)?;
    Ok(())
}

fn titus_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Titus", 2, "Different generations and roles are given concrete patterns of self-control, dignity, faithfulness, and good teaching suited to a healthy household.", "Grace trains believers to reject worldly cravings while waiting for Christ's appearing and devoting themselves to a people eager for good work.");
    titus_2_group_01(story)?;
    titus_2_group_02(story)?;
    titus_2_group_03(story)?;
    titus_2_group_04(story)?;
    Ok(())
}

fn titus_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Titus", 3, "Christians should respect authorities, avoid slander, show gentleness, and remember that salvation came from mercy and renewal rather than personal achievements.", "After warning against divisive speculation, Paul gives practical travel notes, support instructions, and a final greeting grounded in grace.");
    titus_3_group_01(story)?;
    titus_3_group_02(story)?;
    titus_3_group_03(story)?;
    titus_3_group_04(story)?;
    Ok(())
}

fn titus_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        1,
        1,
        4,
        "paul await_value -> guard paul servant god",
        &["paul", "servant", "god", "apostle", "jesus"],
    );
    segment.await_value(
        1,
        "paul",
        &["servant", "god", "apostle"],
        &["paul", "servant", "god", "apostle", "jesus"],
    )?;
    segment.await_value(
        2,
        "god",
        &["hope", "eternal", "life"],
        &["god", "hope", "eternal", "life", "cannot"],
    )?;
    segment.guard(
        3,
        "hath",
        &["due", "times", "manifested"],
        &["hath", "due", "times", "manifested", "word"],
    )?;
    segment.await_value(
        4,
        "titus",
        &["mine", "own", "son"],
        &["titus", "mine", "own", "son", "common"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        1,
        5,
        8,
        "cause iterate -> guard -> expression cause left thee",
        &["cause", "left", "thee", "crete", "shouldest"],
    );
    segment.iterate(
        5,
        "cause",
        &["left", "thee", "crete"],
        &["cause", "left", "thee", "crete", "shouldest"],
    )?;
    segment.guard(
        6,
        "children",
        &["any", "blameless", "husband"],
        &["children", "any", "blameless", "husband", "wife"],
    )?;
    segment.guard(
        7,
        "god",
        &["bishop", "must", "blameless"],
        &["god", "bishop", "must", "blameless", "steward"],
    )?;
    segment.expression(
        8,
        "men",
        &["lover", "hospitality", "good"],
        &["men", "lover", "hospitality", "good", "sober"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        1,
        9,
        12,
        "holding call -> expression -> guard holding fast faithful",
        &["holding", "fast", "faithful", "word", "hath"],
    );
    segment.call(
        9,
        "holding",
        &["fast", "faithful", "word"],
        &["holding", "fast", "faithful", "word", "hath"],
    )?;
    segment.expression(
        10,
        "many",
        &["unruly", "vain", "talkers"],
        &["many", "unruly", "vain", "talkers", "deceivers"],
    )?;
    segment.guard(
        11,
        "whose",
        &["mouths", "must", "stopped"],
        &["whose", "mouths", "must", "stopped", "subvert"],
    )?;
    segment.call(
        12,
        "prophet",
        &["themselves", "even", "own"],
        &["prophet", "themselves", "even", "own", "cretians"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        1,
        13,
        16,
        "witness await_value -> guard -> expression -> iterate witness true wherefore",
        &["witness", "true", "wherefore", "rebuke", "sharply"],
    );
    segment.await_value(
        13,
        "witness",
        &["true", "wherefore", "rebuke"],
        &["witness", "true", "wherefore", "rebuke", "sharply"],
    )?;
    segment.guard(
        14,
        "men",
        &["not", "giving", "heed"],
        &["men", "not", "giving", "heed", "jewish"],
    )?;
    segment.expression(
        15,
        "unto",
        &["pure", "things", "defiled"],
        &["unto", "pure", "things", "defiled", "unbelieving"],
    )?;
    segment.iterate(
        16,
        "god",
        &["profess", "know", "works"],
        &["god", "profess", "know", "works", "deny"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        2,
        1,
        4,
        "speak call -> drop_value -> guard speak things become",
        &["speak", "things", "become", "sound", "doctrine"],
    );
    segment.call(
        1,
        "speak",
        &["things", "become", "sound"],
        &["speak", "things", "become", "sound", "doctrine"],
    )?;
    segment.drop_value(
        2,
        "men",
        &["aged", "sober", "grave"],
        &["men", "aged", "sober", "grave", "temperate"],
    )?;
    segment.guard(
        3,
        "women",
        &["aged", "likewise", "behaviour"],
        &["women", "aged", "likewise", "behaviour", "becometh"],
    )?;
    segment.call(
        4,
        "women",
        &["teach", "young", "sober"],
        &["women", "teach", "young", "sober", "love"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        2,
        5,
        8,
        "discreet guard -> expression discreet chaste keepers",
        &["discreet", "chaste", "keepers", "home", "good"],
    );
    segment.guard(
        5,
        "discreet",
        &["chaste", "keepers", "home"],
        &["discreet", "chaste", "keepers", "home", "good"],
    )?;
    segment.expression(
        6,
        "men",
        &["young", "likewise", "exhort"],
        &["men", "young", "likewise", "exhort", "sober"],
    )?;
    segment.expression(
        7,
        "things",
        &["shewing", "thyself", "pattern"],
        &["things", "shewing", "thyself", "pattern", "good"],
    )?;
    segment.guard(
        8,
        "sound",
        &["speech", "cannot", "condemned"],
        &["sound", "speech", "cannot", "condemned", "contrary"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        2,
        9,
        12,
        "exhort guard -> borrow -> expression exhort servants obedient",
        &["exhort", "servants", "obedient", "unto", "own"],
    );
    segment.guard(
        9,
        "exhort",
        &["servants", "obedient", "unto"],
        &["exhort", "servants", "obedient", "unto", "own"],
    )?;
    segment.guard(
        10,
        "god",
        &["not", "purloining", "shewing"],
        &["god", "not", "purloining", "shewing", "good"],
    )?;
    segment.borrow(
        11,
        "god",
        &["grace", "bringeth", "salvation"],
        &["god", "grace", "bringeth", "salvation", "hath"],
    )?;
    segment.expression(
        12,
        "teaching",
        &["denying", "ungodliness", "worldly"],
        &["teaching", "denying", "ungodliness", "worldly", "lusts"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        2,
        13,
        15,
        "god await_value -> expression -> guard god looking blessed",
        &["god", "looking", "blessed", "hope", "glorious"],
    );
    segment.await_value(
        13,
        "god",
        &["looking", "blessed", "hope"],
        &["god", "looking", "blessed", "hope", "glorious"],
    )?;
    segment.expression(
        14,
        "gave",
        &["himself", "might", "redeem"],
        &["gave", "himself", "might", "redeem", "iniquity"],
    )?;
    segment.guard(
        15,
        "man",
        &["things", "speak", "exhort"],
        &["man", "things", "speak", "exhort", "rebuke"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        3,
        1,
        4,
        "put iterate -> guard -> expression -> borrow put mind subject",
        &["put", "mind", "subject", "principalities", "powers"],
    );
    segment.iterate(
        1,
        "put",
        &["mind", "subject", "principalities"],
        &["put", "mind", "subject", "principalities", "powers"],
    )?;
    segment.guard(
        2,
        "man",
        &["speak", "evil", "brawlers"],
        &["man", "speak", "evil", "brawlers", "gentle"],
    )?;
    segment.expression(
        3,
        "ourselves",
        &["sometimes", "foolish", "disobedient"],
        &[
            "ourselves",
            "sometimes",
            "foolish",
            "disobedient",
            "deceived",
        ],
    )?;
    segment.borrow(
        4,
        "god",
        &["kindness", "love", "saviour"],
        &["god", "kindness", "love", "saviour", "toward"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        3,
        5,
        8,
        "not guard -> expression -> construct -> await_value not works righteousness",
        &["not", "works", "righteousness", "done", "according"],
    );
    segment.guard(
        5,
        "not",
        &["works", "righteousness", "done"],
        &["not", "works", "righteousness", "done", "according"],
    )?;
    segment.expression(
        6,
        "jesus",
        &["shed", "abundantly", "through"],
        &["jesus", "shed", "abundantly", "through", "christ"],
    )?;
    segment.construct(
        7,
        "justified",
        &["grace", "made", "heirs"],
        &["justified", "grace", "made", "heirs", "according"],
    )?;
    segment.await_value(
        8,
        "god",
        &["faithful", "saying", "things"],
        &["god", "faithful", "saying", "things", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        3,
        9,
        12,
        "avoid guard -> expression -> move_value avoid foolish questions",
        &[
            "avoid",
            "foolish",
            "questions",
            "genealogies",
            "contentions",
        ],
    );
    segment.guard(
        9,
        "avoid",
        &["foolish", "questions", "genealogies"],
        &[
            "avoid",
            "foolish",
            "questions",
            "genealogies",
            "contentions",
        ],
    )?;
    segment.expression(
        10,
        "man",
        &["heretick", "first", "second"],
        &["man", "heretick", "first", "second", "admonition"],
    )?;
    segment.expression(
        11,
        "knowing",
        &["such", "subverted", "sinneth"],
        &["knowing", "such", "subverted", "sinneth", "condemned"],
    )?;
    segment.move_value(
        12,
        "shall",
        &["send", "artemas", "unto"],
        &["shall", "send", "artemas", "unto", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn titus_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Titus,
        3,
        13,
        15,
        "bring move_value -> guard -> await_value bring zenas lawyer",
        &["bring", "zenas", "lawyer", "apollos", "journey"],
    );
    segment.move_value(
        13,
        "bring",
        &["zenas", "lawyer", "apollos"],
        &["bring", "zenas", "lawyer", "apollos", "journey"],
    )?;
    segment.guard(
        14,
        "our's",
        &["learn", "maintain", "good"],
        &["our's", "learn", "maintain", "good", "works"],
    )?;
    segment.await_value(
        15,
        "salute",
        &["thee", "greet", "love"],
        &["salute", "thee", "greet", "love", "faith"],
    )?;
    segment.commit()?;
    Ok(())
}
