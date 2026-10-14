use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    zephaniah(story)?;
    Ok(())
}

fn zephaniah(story: &mut Story) -> Result<(), StoryError> {
    zephaniah_1(story)?;
    zephaniah_2(story)?;
    zephaniah_3(story)?;
    Ok(())
}

fn zephaniah_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Zephaniah",
        1,
        "Zephaniah described the day of the Maintainer as a system-wide sweep:",
        "idols, complacent officials, and silent observers would all be found.",
        "The rich would lose their stored wealth, the city would hear alarms,",
        "and no amount of gold could purchase a bypass once judgment had entered",
        "every room of the compromised repository."
    );
    zephaniah_1_group_01(story)?;
    zephaniah_1_group_02(story)?;
    zephaniah_1_group_03(story)?;
    zephaniah_1_group_04(story)?;
    zephaniah_1_group_05(story)?;
    Ok(())
}

fn zephaniah_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Zephaniah",
        2,
        "Before the decree executed, the humble were told to seek justice,",
        "humility, and shelter rather than assuming heritage would protect them.",
        "The surrounding nations received their own warnings, and the proud",
        "coastal cities would become grazing ground, proving that arrogance can",
        "turn a famous deployment into an empty field."
    );
    zephaniah_2_group_01(story)?;
    zephaniah_2_group_02(story)?;
    zephaniah_2_group_03(story)?;
    zephaniah_2_group_04(story)?;
    Ok(())
}

fn zephaniah_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Zephaniah",
        3,
        "Jerusalem was called rebellious because its officials, judges, prophets,",
        "and priests had made corruption a team-wide convention.",
        "Yet the final commit promised purified speech, gathered exiles, songs of",
        "delight, and a Maintainer who would rejoice over a restored people instead",
        "of keeping them at arm's length as a permanent incident."
    );
    zephaniah_3_group_01(story)?;
    zephaniah_3_group_02(story)?;
    zephaniah_3_group_03(story)?;
    zephaniah_3_group_04(story)?;
    zephaniah_3_group_05(story)?;
    Ok(())
}

fn zephaniah_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        1,
        1,
        4,
        "lord move_value -> expression lord word unto",
        &["lord", "word", "unto", "zephaniah", "son"],
    );
    segment.move_value(
        1,
        "lord",
        &["word", "unto", "zephaniah"],
        &["lord", "word", "unto", "zephaniah", "son"],
    )?;
    segment.expression(
        2,
        "lord",
        &["will", "utterly", "consume"],
        &["lord", "will", "utterly", "consume", "things"],
    )?;
    segment.expression(
        3,
        "man",
        &["will", "consume", "beast"],
        &["man", "will", "consume", "beast", "fowls"],
    )?;
    segment.expression(
        4,
        "judah",
        &["will", "stretch", "mine"],
        &["judah", "will", "stretch", "mine", "hand"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        1,
        5,
        8,
        "lord trait_bound -> guard -> move_value lord worship host",
        &["lord", "worship", "host", "heaven", "housetops"],
    );
    segment.trait_bound(
        5,
        "lord",
        &["worship", "host", "heaven"],
        &["lord", "worship", "host", "heaven", "housetops"],
    )?;
    segment.guard(
        6,
        "lord",
        &["turned", "back", "not"],
        &["lord", "turned", "back", "not", "sought"],
    )?;
    segment.trait_bound(
        7,
        "lord",
        &["hold", "peace", "presence"],
        &["lord", "hold", "peace", "presence", "god"],
    )?;
    segment.move_value(
        8,
        "shall",
        &["pass", "day", "lord's"],
        &["shall", "pass", "day", "lord's", "sacrifice"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        1,
        9,
        12,
        "same expression -> move_value same day will",
        &["same", "day", "will", "punish", "leap"],
    );
    segment.expression(
        9,
        "same",
        &["day", "will", "punish"],
        &["same", "day", "will", "punish", "leap"],
    )?;
    segment.move_value(
        10,
        "lord",
        &["shall", "pass", "day"],
        &["lord", "shall", "pass", "day", "saith"],
    )?;
    segment.expression(
        11,
        "people",
        &["howl", "inhabitants", "maktesh"],
        &["people", "howl", "inhabitants", "maktesh", "merchant"],
    )?;
    segment.move_value(
        12,
        "shall",
        &["pass", "time", "will"],
        &["shall", "pass", "time", "will", "search"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        1,
        13,
        15,
        "therefore construct -> result -> panic therefore goods shall",
        &["therefore", "goods", "shall", "become", "booty"],
    );
    segment.construct(
        13,
        "therefore",
        &["goods", "shall", "become"],
        &["therefore", "goods", "shall", "become", "booty"],
    )?;
    segment.result(
        14,
        "lord",
        &["great", "day", "near"],
        &["lord", "great", "day", "near", "hasteth"],
    )?;
    segment.panic(
        15,
        "day",
        &["wrath", "trouble", "distress"],
        &["day", "wrath", "trouble", "distress", "wasteness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        1,
        16,
        18,
        "day expression -> move_value -> construct day trumpet alarm",
        &["day", "trumpet", "alarm", "against", "fenced"],
    );
    segment.expression(
        16,
        "day",
        &["trumpet", "alarm", "against"],
        &["day", "trumpet", "alarm", "against", "fenced"],
    )?;
    segment.move_value(
        17,
        "men",
        &["will", "bring", "distress"],
        &["men", "will", "bring", "distress", "shall"],
    )?;
    segment.construct(
        18,
        "neither",
        &["silver", "nor", "gold"],
        &["neither", "silver", "nor", "gold", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        2,
        1,
        4,
        "nation guard -> move_value -> expression nation gather yourselves",
        &["nation", "gather", "yourselves", "together", "yea"],
    );
    segment.guard(
        1,
        "nation",
        &["gather", "yourselves", "together"],
        &["nation", "gather", "yourselves", "together", "yea"],
    )?;
    segment.move_value(
        2,
        "decree",
        &["bring", "forth", "day"],
        &["decree", "bring", "forth", "day", "pass"],
    )?;
    segment.expression(
        3,
        "lord",
        &["seek", "meek", "earth"],
        &["lord", "seek", "meek", "earth", "wrought"],
    )?;
    segment.expression(
        4,
        "gaza",
        &["shall", "forsaken", "ashkelon"],
        &["gaza", "shall", "forsaken", "ashkelon", "desolation"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        2,
        5,
        8,
        "nation drop_value -> expression nation woe unto",
        &["nation", "woe", "unto", "inhabitants", "sea"],
    );
    segment.drop_value(
        5,
        "nation",
        &["woe", "unto", "inhabitants"],
        &["nation", "woe", "unto", "inhabitants", "sea"],
    )?;
    segment.expression(
        6,
        "sea",
        &["coast", "shall", "dwellings"],
        &["sea", "coast", "shall", "dwellings", "cottages"],
    )?;
    segment.expression(
        7,
        "judah",
        &["coast", "shall", "remnant"],
        &["judah", "coast", "shall", "remnant", "house"],
    )?;
    segment.expression(
        8,
        "children",
        &["heard", "reproach", "moab"],
        &["children", "heard", "reproach", "moab", "revilings"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        2,
        9,
        12,
        "lord expression -> trait_bound -> drop_value lord therefore live",
        &["lord", "therefore", "live", "saith", "hosts"],
    );
    segment.expression(
        9,
        "lord",
        &["therefore", "live", "saith"],
        &["lord", "therefore", "live", "saith", "hosts"],
    )?;
    segment.expression(
        10,
        "people",
        &["shall", "pride", "reproached"],
        &["people", "shall", "pride", "reproached", "magnified"],
    )?;
    segment.trait_bound(
        11,
        "lord",
        &["will", "terrible", "unto"],
        &["lord", "will", "terrible", "unto", "famish"],
    )?;
    segment.drop_value(
        12,
        "ethiopians",
        &["shall", "slain", "sword"],
        &["ethiopians", "shall", "slain", "sword"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        2,
        13,
        15,
        "will construct -> call will stretch hand",
        &["will", "stretch", "hand", "against", "north"],
    );
    segment.construct(
        13,
        "will",
        &["stretch", "hand", "against"],
        &["will", "stretch", "hand", "against", "north"],
    )?;
    segment.call(
        14,
        "nations",
        &["flocks", "shall", "lie"],
        &["nations", "flocks", "shall", "lie", "down"],
    )?;
    segment.call(
        15,
        "rejoicing",
        &["city", "dwelt", "carelessly"],
        &["rejoicing", "city", "dwelt", "carelessly", "heart"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        3,
        1,
        4,
        "woe panic -> guard woe filthy polluted",
        &["woe", "filthy", "polluted", "oppressing", "city"],
    );
    segment.panic(
        1,
        "woe",
        &["filthy", "polluted", "oppressing"],
        &["woe", "filthy", "polluted", "oppressing", "city"],
    )?;
    segment.guard(
        2,
        "lord",
        &["obeyed", "not", "voice"],
        &["lord", "obeyed", "not", "voice", "received"],
    )?;
    segment.guard(
        3,
        "princes",
        &["within", "roaring", "lions"],
        &["princes", "within", "roaring", "lions", "judges"],
    )?;
    segment.guard(
        4,
        "prophets",
        &["light", "treacherous", "persons"],
        &["prophets", "light", "treacherous", "persons", "priests"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        3,
        5,
        8,
        "lord move_value -> construct -> guard -> await_value lord just midst",
        &["lord", "just", "midst", "thereof", "will"],
    );
    segment.move_value(
        5,
        "lord",
        &["just", "midst", "thereof"],
        &["lord", "just", "midst", "thereof", "will"],
    )?;
    segment.construct(
        6,
        "nations",
        &["cut", "off", "towers"],
        &["nations", "cut", "off", "towers", "desolate"],
    )?;
    segment.guard(
        7,
        "surely",
        &["wilt", "fear", "receive"],
        &["surely", "wilt", "fear", "receive", "instruction"],
    )?;
    segment.await_value(
        8,
        "lord",
        &["therefore", "wait", "saith"],
        &["lord", "therefore", "wait", "saith", "until"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        3,
        9,
        12,
        "people expression -> move_value -> guard -> await_value people will turn",
        &["people", "will", "turn", "pure", "language"],
    );
    segment.expression(
        9,
        "people",
        &["will", "turn", "pure"],
        &["people", "will", "turn", "pure", "language"],
    )?;
    segment.move_value(
        10,
        "beyond",
        &["rivers", "ethiopia", "suppliants"],
        &["beyond", "rivers", "ethiopia", "suppliants", "even"],
    )?;
    segment.guard(
        11,
        "day",
        &["shalt", "not", "ashamed"],
        &["day", "shalt", "not", "ashamed", "doings"],
    )?;
    segment.await_value(
        12,
        "people",
        &["will", "leave", "midst"],
        &["people", "will", "leave", "midst", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        3,
        13,
        16,
        "israel construct -> trait_bound -> guard israel remnant shall",
        &["israel", "remnant", "shall", "not", "iniquity"],
    );
    segment.construct(
        13,
        "israel",
        &["remnant", "shall", "not"],
        &["israel", "remnant", "shall", "not", "iniquity"],
    )?;
    segment.trait_bound(
        14,
        "israel",
        &["sing", "daughter", "zion"],
        &["israel", "sing", "daughter", "zion", "shout"],
    )?;
    segment.guard(
        15,
        "lord",
        &["hath", "taken", "away"],
        &["lord", "hath", "taken", "away", "judgments"],
    )?;
    segment.guard(
        16,
        "day",
        &["shall", "jerusalem", "fear"],
        &["day", "shall", "jerusalem", "fear", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn zephaniah_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Zephaniah,
        3,
        17,
        20,
        "lord borrow -> expression -> match_value -> construct lord god midst",
        &["lord", "god", "midst", "thee", "mighty"],
    );
    segment.borrow(
        17,
        "lord",
        &["god", "midst", "thee"],
        &["lord", "god", "midst", "thee", "mighty"],
    )?;
    segment.expression(
        18,
        "will",
        &["gather", "sorrowful", "solemn"],
        &["will", "gather", "sorrowful", "solemn", "assembly"],
    )?;
    segment.match_value(
        19,
        "behold",
        &["time", "will", "undo"],
        &["behold", "time", "will", "undo", "afflict"],
    )?;
    segment.construct(
        20,
        "time",
        &["will", "bring", "again"],
        &["time", "will", "bring", "again", "even"],
    )?;
    segment.commit()?;
    Ok(())
}
