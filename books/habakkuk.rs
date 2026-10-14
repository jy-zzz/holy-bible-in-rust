use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    habakkuk(story)?;
    Ok(())
}

fn habakkuk(story: &mut Story) -> Result<(), StoryError> {
    habakkuk_1(story)?;
    habakkuk_2(story)?;
    habakkuk_3(story)?;
    Ok(())
}

fn habakkuk_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Habakkuk",
        1,
        "Habakkuk opened an honest support ticket: why does violence remain online",
        "while the Maintainer appears to tolerate every broken court?",
        "The answer was more unsettling than silence: Babylon, a ruthless outside",
        "force, would be allowed to sweep across nations, leaving the prophet",
        "wondering why the cure looked worse than the bug."
    );
    habakkuk_1_group_01(story)?;
    habakkuk_1_group_02(story)?;
    habakkuk_1_group_03(story)?;
    habakkuk_1_group_04(story)?;
    Ok(())
}

fn habakkuk_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Habakkuk",
        2,
        "The prophet climbed the watchtower and waited for a response that could",
        "survive delay, doubt, and the temptation to edit the complaint.",
        "The vision said the proud system is inflated and unstable, while the",
        "righteous live by faithful persistence; five woe reports then exposed",
        "plunder, exploitation, drunken power, and idol-making as doomed designs."
    );
    habakkuk_2_group_01(story)?;
    habakkuk_2_group_02(story)?;
    habakkuk_2_group_03(story)?;
    habakkuk_2_group_04(story)?;
    habakkuk_2_group_05(story)?;
    Ok(())
}

fn habakkuk_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Habakkuk",
        3,
        "Habakkuk turned his argument into a song, remembering the Maintainer's",
        "terrifying march through history and the trembling of mountains and seas.",
        "Even when fig trees, crops, flocks, and servers all return empty values,",
        "the prophet chose joy in the One who gives steady footing on impossible",
        "heights rather than in the success metrics of the current release."
    );
    habakkuk_3_group_01(story)?;
    habakkuk_3_group_02(story)?;
    habakkuk_3_group_03(story)?;
    habakkuk_3_group_04(story)?;
    habakkuk_3_group_05(story)?;
    Ok(())
}

fn habakkuk_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        1,
        1,
        4,
        "prophet expression -> guard -> match_value -> move_value prophet burden habakkuk",
        &["prophet", "burden", "habakkuk", "see", "lord"],
    );
    segment.expression(
        1,
        "prophet",
        &["burden", "habakkuk", "see"],
        &["prophet", "burden", "habakkuk", "see"],
    )?;
    segment.guard(
        2,
        "lord",
        &["long", "shall", "cry"],
        &["lord", "long", "shall", "cry", "wilt"],
    )?;
    segment.match_value(
        3,
        "why",
        &["dost", "shew", "iniquity"],
        &["why", "dost", "shew", "iniquity", "cause"],
    )?;
    segment.move_value(
        4,
        "therefore",
        &["law", "slacked", "judgment"],
        &["therefore", "law", "slacked", "judgment", "doth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        1,
        5,
        8,
        "behold guard -> expression -> move_value behold among heathen",
        &["behold", "among", "heathen", "regard", "wonder"],
    );
    segment.guard(
        5,
        "behold",
        &["among", "heathen", "regard"],
        &["behold", "among", "heathen", "regard", "wonder"],
    )?;
    segment.guard(
        6,
        "nation",
        &["raise", "chaldeans", "bitter"],
        &["nation", "raise", "chaldeans", "bitter", "hasty"],
    )?;
    segment.expression(
        7,
        "terrible",
        &["dreadful", "judgment", "dignity"],
        &["terrible", "dreadful", "judgment", "dignity", "shall"],
    )?;
    segment.move_value(
        8,
        "horses",
        &["swifter", "leopards", "fierce"],
        &["horses", "swifter", "leopards", "fierce", "evening"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        1,
        9,
        12,
        "shall move_value -> iterate -> expression -> guard shall violence faces",
        &["shall", "violence", "faces", "sup", "east"],
    );
    segment.move_value(
        9,
        "shall",
        &["violence", "faces", "sup"],
        &["shall", "violence", "faces", "sup", "east"],
    )?;
    segment.iterate(
        10,
        "shall",
        &["scoff", "kings", "princes"],
        &["shall", "scoff", "kings", "princes", "scorn"],
    )?;
    segment.expression(
        11,
        "shall",
        &["mind", "change", "pass"],
        &["shall", "mind", "change", "pass", "over"],
    )?;
    segment.guard(
        12,
        "lord",
        &["art", "not", "everlasting"],
        &["lord", "art", "not", "everlasting", "god"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        1,
        13,
        17,
        "art guard -> expression -> trait_bound art purer eyes",
        &["art", "purer", "eyes", "behold", "evil"],
    );
    segment.guard(
        13,
        "art",
        &["purer", "eyes", "behold"],
        &["art", "purer", "eyes", "behold", "evil"],
    )?;
    segment.guard(
        14,
        "men",
        &["makest", "fishes", "sea"],
        &["men", "makest", "fishes", "sea", "creeping"],
    )?;
    segment.expression(
        15,
        "take",
        &["angle", "catch", "net"],
        &["take", "angle", "catch", "net", "gather"],
    )?;
    segment.trait_bound(
        16,
        "therefore",
        &["sacrifice", "unto", "net"],
        &["therefore", "sacrifice", "unto", "net", "burn"],
    )?;
    segment.guard(
        17,
        "shall",
        &["therefore", "empty", "net"],
        &["shall", "therefore", "empty", "net", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        2,
        1,
        4,
        "will call -> construct -> move_value -> guard will stand watch",
        &["will", "stand", "watch", "set", "tower"],
    );
    segment.call(
        1,
        "will",
        &["stand", "watch", "set"],
        &["will", "stand", "watch", "set", "tower"],
    )?;
    segment.construct(
        2,
        "lord",
        &["answered", "write", "vision"],
        &["lord", "answered", "write", "vision", "make"],
    )?;
    segment.move_value(
        3,
        "vision",
        &["yet", "appointed", "time"],
        &["vision", "yet", "appointed", "time", "end"],
    )?;
    segment.guard(
        4,
        "behold",
        &["soul", "lifted", "not"],
        &["behold", "soul", "lifted", "not", "upright"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        2,
        5,
        8,
        "man drop_value -> guard -> expression man yea transgresseth",
        &["man", "yea", "transgresseth", "wine", "proud"],
    );
    segment.drop_value(
        5,
        "man",
        &["yea", "transgresseth", "wine"],
        &["man", "yea", "transgresseth", "wine", "proud"],
    )?;
    segment.guard(
        6,
        "shall",
        &["not", "take", "parable"],
        &["shall", "not", "take", "parable", "against"],
    )?;
    segment.guard(
        7,
        "shall",
        &["not", "rise", "suddenly"],
        &["shall", "not", "rise", "suddenly", "bite"],
    )?;
    segment.expression(
        8,
        "nations",
        &["hast", "spoiled", "many"],
        &["nations", "hast", "spoiled", "many", "remnant"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        2,
        9,
        12,
        "woe panic -> expression -> result woe coveteth evil",
        &["woe", "coveteth", "evil", "covetousness", "house"],
    );
    segment.panic(
        9,
        "woe",
        &["coveteth", "evil", "covetousness"],
        &["woe", "coveteth", "evil", "covetousness", "house"],
    )?;
    segment.expression(
        10,
        "people",
        &["hast", "consulted", "shame"],
        &["people", "hast", "consulted", "shame", "house"],
    )?;
    segment.result(
        11,
        "stone",
        &["shall", "cry", "wall"],
        &["stone", "shall", "cry", "wall", "beam"],
    )?;
    segment.panic(
        12,
        "woe",
        &["buildeth", "town", "blood"],
        &["woe", "buildeth", "town", "blood", "stablisheth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        2,
        13,
        16,
        "lord guard -> trait_bound -> panic lord behold not",
        &["lord", "behold", "not", "hosts", "people"],
    );
    segment.guard(
        13,
        "lord",
        &["behold", "not", "hosts"],
        &["lord", "behold", "not", "hosts", "people"],
    )?;
    segment.trait_bound(
        14,
        "lord",
        &["earth", "shall", "filled"],
        &["lord", "earth", "shall", "filled", "knowledge"],
    )?;
    segment.panic(
        15,
        "woe",
        &["unto", "giveth", "neighbour"],
        &["woe", "unto", "giveth", "neighbour", "drink"],
    )?;
    segment.trait_bound(
        16,
        "art",
        &["filled", "shame", "glory"],
        &["art", "filled", "shame", "glory", "drink"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        2,
        17,
        20,
        "violence construct -> guard violence lebanon shall",
        &["violence", "lebanon", "shall", "cover", "thee"],
    );
    segment.construct(
        17,
        "violence",
        &["lebanon", "shall", "cover"],
        &["violence", "lebanon", "shall", "cover", "thee"],
    )?;
    segment.construct(
        18,
        "profiteth",
        &["graven", "image", "maker"],
        &["profiteth", "graven", "image", "maker", "thereof"],
    )?;
    segment.guard(
        19,
        "woe",
        &["unto", "saith", "wood"],
        &["woe", "unto", "saith", "wood", "awake"],
    )?;
    segment.guard(
        20,
        "lord",
        &["holy", "temple", "earth"],
        &["lord", "holy", "temple", "earth", "keep"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        3,
        1,
        4,
        "prophet result -> construct -> move_value -> expression prophet prayer habakkuk",
        &["prophet", "prayer", "habakkuk", "shigionoth", "lord"],
    );
    segment.result(
        1,
        "prophet",
        &["prayer", "habakkuk", "shigionoth"],
        &["prophet", "prayer", "habakkuk", "shigionoth"],
    )?;
    segment.construct(
        2,
        "lord",
        &["heard", "speech", "afraid"],
        &["lord", "heard", "speech", "afraid", "revive"],
    )?;
    segment.move_value(
        3,
        "god",
        &["teman", "holy", "mount"],
        &["god", "teman", "holy", "mount", "paran"],
    )?;
    segment.expression(
        4,
        "brightness",
        &["light", "horns", "coming"],
        &["brightness", "light", "horns", "coming", "hand"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        3,
        5,
        8,
        "went move_value -> expression -> match_value -> panic went pestilence burning",
        &["went", "pestilence", "burning", "coals", "forth"],
    );
    segment.move_value(
        5,
        "went",
        &["pestilence", "burning", "coals"],
        &["went", "pestilence", "burning", "coals", "forth"],
    )?;
    segment.expression(
        6,
        "nations",
        &["stood", "measured", "earth"],
        &["nations", "stood", "measured", "earth", "beheld"],
    )?;
    segment.match_value(
        7,
        "saw",
        &["tents", "cushan", "affliction"],
        &["saw", "tents", "cushan", "affliction", "curtains"],
    )?;
    segment.panic(
        8,
        "lord",
        &["displeased", "against", "rivers"],
        &["lord", "displeased", "against", "rivers", "thine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        3,
        9,
        12,
        "bow construct -> match_value -> move_value -> expression bow made quite",
        &["bow", "made", "quite", "naked", "according"],
    );
    segment.construct(
        9,
        "bow",
        &["made", "quite", "naked"],
        &["bow", "made", "quite", "naked", "according"],
    )?;
    segment.match_value(
        10,
        "mountains",
        &["saw", "thee", "trembled"],
        &["mountains", "saw", "thee", "trembled", "overflowing"],
    )?;
    segment.move_value(
        11,
        "sun",
        &["moon", "stood", "still"],
        &["sun", "moon", "stood", "still", "habitation"],
    )?;
    segment.expression(
        12,
        "didst",
        &["march", "through", "land"],
        &["didst", "march", "through", "land", "indignation"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        3,
        13,
        16,
        "people expression -> move_value -> call people wentest forth",
        &["people", "wentest", "forth", "salvation", "even"],
    );
    segment.expression(
        13,
        "people",
        &["wentest", "forth", "salvation"],
        &["people", "wentest", "forth", "salvation", "even"],
    )?;
    segment.move_value(
        14,
        "didst",
        &["strike", "through", "staves"],
        &["didst", "strike", "through", "staves", "head"],
    )?;
    segment.expression(
        15,
        "didst",
        &["walk", "through", "sea"],
        &["didst", "walk", "through", "sea", "thine"],
    )?;
    segment.call(
        16,
        "heard",
        &["belly", "trembled", "lips"],
        &["heard", "belly", "trembled", "lips", "quivered"],
    )?;
    segment.commit()?;
    Ok(())
}

fn habakkuk_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Habakkuk,
        3,
        17,
        19,
        "although guard -> expression -> construct although fig tree",
        &["although", "fig", "tree", "shall", "not"],
    );
    segment.guard(
        17,
        "although",
        &["fig", "tree", "shall"],
        &["although", "fig", "tree", "shall", "not"],
    )?;
    segment.expression(
        18,
        "lord",
        &["yet", "will", "rejoice"],
        &["lord", "yet", "will", "rejoice", "joy"],
    )?;
    segment.construct(
        19,
        "lord",
        &["god", "strength", "will"],
        &["lord", "god", "strength", "will", "make"],
    )?;
    segment.commit()?;
    Ok(())
}
