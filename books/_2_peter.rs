use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _2_peter(story)?;
    Ok(())
}

fn _2_peter(story: &mut Story) -> Result<(), StoryError> {
    _2_peter_1(story)?;
    _2_peter_2(story)?;
    _2_peter_3(story)?;
    Ok(())
}

fn _2_peter_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "2 Peter",
        1,
        "Divine power supplied everything needed for life and godliness, so the",
        "readers were told to chain faith to virtue, knowledge, endurance, and love.",
        "The apostolic witness was not a cleverly fabricated release note; it was",
        "grounded in the transfiguration and in prophecy carried by the Spirit,",
        "which does not originate from a private authoring session."
    );
    _2_peter_1_group_01(story)?;
    _2_peter_1_group_02(story)?;
    _2_peter_1_group_03(story)?;
    _2_peter_1_group_04(story)?;
    _2_peter_1_group_05(story)?;
    Ok(())
}

fn _2_peter_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "2 Peter",
        2,
        "False teachers smuggled destructive dependencies into the community,",
        "promising freedom while remaining owned by corruption.",
        "Their invoices would be answered by judgment, as the flood, the ruined",
        "cities, and the rescue of the righteous showed that the Maintainer can",
        "distinguish faithful users from systems devoted to exploitation."
    );
    _2_peter_2_group_01(story)?;
    _2_peter_2_group_02(story)?;
    _2_peter_2_group_03(story)?;
    _2_peter_2_group_04(story)?;
    _2_peter_2_group_05(story)?;
    _2_peter_2_group_06(story)?;
    Ok(())
}

fn _2_peter_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "2 Peter",
        3,
        "Scoffers mocked the promised return because the scheduler seemed slow,",
        "forgetting that divine patience measures time differently and leaves room",
        "for repentance.",
        "The day would arrive unexpectedly, dissolving the old environment before",
        "a renewed heavens-and-earth deployment, so the readers were told to live",
        "holy lives and grow in grace while waiting for the promise."
    );
    _2_peter_3_group_01(story)?;
    _2_peter_3_group_02(story)?;
    _2_peter_3_group_03(story)?;
    _2_peter_3_group_04(story)?;
    _2_peter_3_group_05(story)?;
    Ok(())
}

fn _2_peter_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        1,
        1,
        4,
        "peter await_value -> borrow -> trait_bound -> expression peter simon servant",
        &["peter", "simon", "servant", "apostle", "jesus"],
    );
    segment.await_value(
        1,
        "peter",
        &["simon", "servant", "apostle"],
        &["peter", "simon", "servant", "apostle", "jesus"],
    )?;
    segment.borrow(
        2,
        "god",
        &["grace", "peace", "multiplied"],
        &["god", "grace", "peace", "multiplied", "unto"],
    )?;
    segment.trait_bound(
        3,
        "according",
        &["divine", "power", "hath"],
        &["according", "divine", "power", "hath", "given"],
    )?;
    segment.expression(
        4,
        "whereby",
        &["given", "unto", "exceeding"],
        &["whereby", "given", "unto", "exceeding", "great"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        1,
        5,
        8,
        "beside await_value -> expression -> borrow -> construct beside giving diligence",
        &["beside", "giving", "diligence", "add", "faith"],
    );
    segment.await_value(
        5,
        "beside",
        &["giving", "diligence", "add"],
        &["beside", "giving", "diligence", "add", "faith"],
    )?;
    segment.expression(
        6,
        "knowledge",
        &["temperance", "patience", "godliness"],
        &["knowledge", "temperance", "patience", "godliness"],
    )?;
    segment.borrow(
        7,
        "godliness",
        &["brotherly", "kindness", "charity"],
        &["godliness", "brotherly", "kindness", "charity"],
    )?;
    segment.construct(
        8,
        "things",
        &["abound", "make", "shall"],
        &["things", "abound", "make", "shall", "neither"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        1,
        9,
        12,
        "lacketh expression -> construct -> guard lacketh things blind",
        &["lacketh", "things", "blind", "cannot", "see"],
    );
    segment.expression(
        9,
        "lacketh",
        &["things", "blind", "cannot"],
        &["lacketh", "things", "blind", "cannot", "see"],
    )?;
    segment.construct(
        10,
        "wherefore",
        &["rather", "brethren", "give"],
        &["wherefore", "rather", "brethren", "give", "diligence"],
    )?;
    segment.expression(
        11,
        "lord",
        &["entrance", "shall", "ministered"],
        &["lord", "entrance", "shall", "ministered", "unto"],
    )?;
    segment.guard(
        12,
        "wherefore",
        &["will", "not", "negligent"],
        &["wherefore", "will", "not", "negligent", "put"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        1,
        13,
        16,
        "yea trait_bound -> guard -> expression -> construct yea think meet",
        &["yea", "think", "meet", "long", "tabernacle"],
    );
    segment.trait_bound(
        13,
        "yea",
        &["think", "meet", "long"],
        &["yea", "think", "meet", "long", "tabernacle"],
    )?;
    segment.guard(
        14,
        "lord",
        &["knowing", "shortly", "must"],
        &["lord", "knowing", "shortly", "must", "put"],
    )?;
    segment.expression(
        15,
        "moreover",
        &["will", "endeavour", "able"],
        &["moreover", "will", "endeavour", "able", "decease"],
    )?;
    segment.construct(
        16,
        "not",
        &["followed", "cunningly", "devised"],
        &["not", "followed", "cunningly", "devised", "fables"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        1,
        17,
        21,
        "god move_value -> call -> guard god received father",
        &["god", "received", "father", "honour", "glory"],
    );
    segment.move_value(
        17,
        "god",
        &["received", "father", "honour"],
        &["god", "received", "father", "honour", "glory"],
    )?;
    segment.move_value(
        18,
        "voice",
        &["heaven", "heard", "holy"],
        &["voice", "heaven", "heard", "holy", "mount"],
    )?;
    segment.call(
        19,
        "sure",
        &["word", "prophecy", "whereunto"],
        &["sure", "word", "prophecy", "whereunto", "well"],
    )?;
    segment.guard(
        20,
        "knowing",
        &["first", "prophecy", "scripture"],
        &["knowing", "first", "prophecy", "scripture", "any"],
    )?;
    segment.move_value(
        21,
        "man",
        &["prophecy", "not", "old"],
        &["man", "prophecy", "not", "old", "time"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        2,
        1,
        4,
        "people move_value -> expression -> construct -> guard people false prophets",
        &["people", "false", "prophets", "among", "even"],
    );
    segment.move_value(
        1,
        "people",
        &["false", "prophets", "among"],
        &["people", "false", "prophets", "among", "even"],
    )?;
    segment.expression(
        2,
        "many",
        &["shall", "follow", "pernicious"],
        &["many", "shall", "follow", "pernicious", "ways"],
    )?;
    segment.construct(
        3,
        "through",
        &["covetousness", "shall", "feigned"],
        &["through", "covetousness", "shall", "feigned", "words"],
    )?;
    segment.guard(
        4,
        "god",
        &["spared", "not", "angels"],
        &["god", "spared", "not", "angels", "sinned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        2,
        5,
        8,
        "spared guard -> expression spared not old",
        &["spared", "not", "old", "world", "saved"],
    );
    segment.guard(
        5,
        "spared",
        &["not", "old", "world"],
        &["spared", "not", "old", "world", "saved"],
    )?;
    segment.expression(
        6,
        "turning",
        &["cities", "sodom", "gomorrha"],
        &["turning", "cities", "sodom", "gomorrha", "ashes"],
    )?;
    segment.expression(
        7,
        "delivered",
        &["just", "lot", "vexed"],
        &["delivered", "just", "lot", "vexed", "filthy"],
    )?;
    segment.expression(
        8,
        "man",
        &["righteous", "dwelling", "among"],
        &["man", "righteous", "dwelling", "among", "seeing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        2,
        9,
        12,
        "lord expression -> guard -> move_value -> construct lord knoweth deliver",
        &["lord", "knoweth", "deliver", "godly", "temptations"],
    );
    segment.expression(
        9,
        "lord",
        &["knoweth", "deliver", "godly"],
        &["lord", "knoweth", "deliver", "godly", "temptations"],
    )?;
    segment.guard(
        10,
        "chiefly",
        &["walk", "flesh", "lust"],
        &["chiefly", "walk", "flesh", "lust", "uncleanness"],
    )?;
    segment.move_value(
        11,
        "whereas",
        &["angels", "greater", "power"],
        &["whereas", "angels", "greater", "power", "might"],
    )?;
    segment.construct(
        12,
        "natural",
        &["brute", "beasts", "made"],
        &["natural", "brute", "beasts", "made", "taken"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        2,
        13,
        16,
        "shall expression -> move_value -> call shall receive reward",
        &["shall", "receive", "reward", "unrighteousness", "count"],
    );
    segment.expression(
        13,
        "shall",
        &["receive", "reward", "unrighteousness"],
        &["shall", "receive", "reward", "unrighteousness", "count"],
    )?;
    segment.expression(
        14,
        "having",
        &["eyes", "full", "adultery"],
        &["having", "eyes", "full", "adultery", "cannot"],
    )?;
    segment.move_value(
        15,
        "forsaken",
        &["right", "way", "gone"],
        &["forsaken", "right", "way", "gone", "astray"],
    )?;
    segment.call(
        16,
        "rebuked",
        &["iniquity", "dumb", "ass"],
        &["rebuked", "iniquity", "dumb", "ass", "speaking"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        2,
        17,
        19,
        "wells expression -> call -> move_value wells without water",
        &["wells", "without", "water", "clouds", "carried"],
    );
    segment.expression(
        17,
        "wells",
        &["without", "water", "clouds"],
        &["wells", "without", "water", "clouds", "carried"],
    )?;
    segment.call(
        18,
        "speak",
        &["great", "swelling", "words"],
        &["speak", "great", "swelling", "words", "vanity"],
    )?;
    segment.move_value(
        19,
        "man",
        &["while", "promise", "liberty"],
        &["man", "while", "promise", "liberty", "themselves"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        2,
        20,
        22,
        "lord expression -> guard lord escaped pollutions",
        &["lord", "escaped", "pollutions", "world", "through"],
    );
    segment.expression(
        20,
        "lord",
        &["escaped", "pollutions", "world"],
        &["lord", "escaped", "pollutions", "world", "through"],
    )?;
    segment.guard(
        21,
        "better",
        &["not", "known", "way"],
        &["better", "not", "known", "way", "righteousness"],
    )?;
    segment.expression(
        22,
        "happened",
        &["unto", "according", "true"],
        &["happened", "unto", "according", "true", "proverb"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        3,
        1,
        4,
        "second borrow -> guard -> move_value -> return_value second epistle beloved",
        &["second", "epistle", "beloved", "now", "write"],
    );
    segment.borrow(
        1,
        "second",
        &["epistle", "beloved", "now"],
        &["second", "epistle", "beloved", "now", "write"],
    )?;
    segment.guard(
        2,
        "lord",
        &["mindful", "words", "spoken"],
        &["lord", "mindful", "words", "spoken", "holy"],
    )?;
    segment.move_value(
        3,
        "knowing",
        &["first", "shall", "last"],
        &["knowing", "first", "shall", "last", "days"],
    )?;
    segment.return_value(
        4,
        "saying",
        &["promise", "coming", "since"],
        &["saying", "promise", "coming", "since", "fathers"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        3,
        5,
        8,
        "god call -> expression -> guard god willingly ignorant",
        &["god", "willingly", "ignorant", "word", "heavens"],
    );
    segment.call(
        5,
        "god",
        &["willingly", "ignorant", "word"],
        &["god", "willingly", "ignorant", "word", "heavens"],
    )?;
    segment.expression(
        6,
        "whereby",
        &["world", "overflowed", "water"],
        &["whereby", "world", "overflowed", "water", "perished"],
    )?;
    segment.call(
        7,
        "heavens",
        &["earth", "now", "same"],
        &["heavens", "earth", "now", "same", "word"],
    )?;
    segment.guard(
        8,
        "lord",
        &["beloved", "not", "ignorant"],
        &["lord", "beloved", "not", "ignorant", "thing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        3,
        9,
        12,
        "lord move_value -> expression lord not slack",
        &["lord", "not", "slack", "concerning", "promise"],
    );
    segment.move_value(
        9,
        "lord",
        &["not", "slack", "concerning"],
        &["lord", "not", "slack", "concerning", "promise"],
    )?;
    segment.move_value(
        10,
        "lord",
        &["day", "will", "thief"],
        &["lord", "day", "will", "thief", "night"],
    )?;
    segment.expression(
        11,
        "seeing",
        &["things", "shall", "dissolved"],
        &["seeing", "things", "shall", "dissolved", "manner"],
    )?;
    segment.expression(
        12,
        "god",
        &["looking", "hasting", "unto"],
        &["god", "looking", "hasting", "unto", "coming"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        3,
        13,
        15,
        "nevertheless reinitialize -> borrow nevertheless according promise",
        &["nevertheless", "according", "promise", "look", "new"],
    );
    segment.reinitialize(
        13,
        "nevertheless",
        &["according", "promise", "look"],
        &["nevertheless", "according", "promise", "look", "new"],
    )?;
    segment.borrow(
        14,
        "wherefore",
        &["beloved", "seeing", "look"],
        &["wherefore", "beloved", "seeing", "look", "such"],
    )?;
    segment.borrow(
        15,
        "lord",
        &["account", "longsuffering", "salvation"],
        &["lord", "account", "longsuffering", "salvation", "even"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_peter_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Peter,
        3,
        16,
        18,
        "epistles expression -> borrow -> trait_bound epistles speaking things",
        &["epistles", "speaking", "things", "hard", "understood"],
    );
    segment.expression(
        16,
        "epistles",
        &["speaking", "things", "hard"],
        &["epistles", "speaking", "things", "hard", "understood"],
    )?;
    segment.borrow(
        17,
        "therefore",
        &["beloved", "seeing", "know"],
        &["therefore", "beloved", "seeing", "know", "things"],
    )?;
    segment.trait_bound(
        18,
        "lord",
        &["grow", "grace", "knowledge"],
        &["lord", "grow", "grace", "knowledge", "saviour"],
    )?;
    segment.commit()?;
    Ok(())
}
