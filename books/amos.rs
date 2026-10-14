use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    amos(story)?;
    Ok(())
}

fn amos(story: &mut Story) -> Result<(), StoryError> {
    amos_1(story)?;
    amos_2(story)?;
    amos_3(story)?;
    amos_4(story)?;
    amos_5(story)?;
    amos_6(story)?;
    amos_7(story)?;
    amos_8(story)?;
    amos_9(story)?;
    Ok(())
}

fn amos_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        1,
        "Amos began with a shepherd's report and a thunderous series of regional",
        "audits: Damascus, Gaza, Tyre, Edom, Ammon, and Moab had exceeded every",
        "limit of cruelty.",
        "Each foreign system had treated human lives as disposable resources, and",
        "the repeated verdict made clear that no nation gets immunity merely",
        "because its repository has an impressive README."
    );
    amos_1_group_01(story)?;
    amos_1_group_02(story)?;
    amos_1_group_03(story)?;
    amos_1_group_04(story)?;
    Ok(())
}

fn amos_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        2,
        "Judah was included in the audit for trading away instruction, and Israel",
        "for selling the poor, trampling the vulnerable, and corrupting justice.",
        "The Maintainer recalled rescuing the people from bondage and raising",
        "prophets among them, but privilege could not excuse a team that used",
        "freedom to weaponize the weak."
    );
    amos_2_group_01(story)?;
    amos_2_group_02(story)?;
    amos_2_group_03(story)?;
    amos_2_group_04(story)?;
    Ok(())
}

fn amos_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        3,
        "Amos explained that a prophet's warning is not random noise: a trumpet",
        "sounds because danger has entered the system.",
        "Israel was invited to inspect the evidence in its palaces, where luxury,",
        "theft, and violence had been stored together; revelation had arrived",
        "because the covenant made the relationship accountable."
    );
    amos_3_group_01(story)?;
    amos_3_group_02(story)?;
    amos_3_group_03(story)?;
    amos_3_group_04(story)?;
    Ok(())
}

fn amos_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        4,
        "The wealthy women of Samaria were compared to well-fed cattle who urged",
        "their household to keep extracting from the poor and ordering another round.",
        "The people visited shrines, brought offerings, and performed religious",
        "routines while ignoring the disasters already sent as diagnostic alerts;",
        "the final instruction was simple and terrifying: prepare to meet the One",
        "who formed the mountains and knows every hidden branch."
    );
    amos_4_group_01(story)?;
    amos_4_group_02(story)?;
    amos_4_group_03(story)?;
    Ok(())
}

fn amos_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        5,
        "Israel was sung as a fallen virgin city, not to romanticize the ruin but",
        "to make the loss impossible to miss in the audit log.",
        "The prophet urged the people to seek good, establish justice at the gate,",
        "and stop confusing festivals with faithfulness, because songs and music",
        "cannot compensate for a riverbed where righteousness has run dry."
    );
    amos_5_group_01(story)?;
    amos_5_group_02(story)?;
    amos_5_group_03(story)?;
    amos_5_group_04(story)?;
    amos_5_group_05(story)?;
    amos_5_group_06(story)?;
    amos_5_group_07(story)?;
    Ok(())
}

fn amos_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        6,
        "Those reclining on ivory couches treated national collapse as someone",
        "else's pager duty and composed elegant playlists during the outage.",
        "Amos warned that their ease would end in exile, since the first among",
        "nations had turned status into insulation and had stopped grieving over",
        "the broken state of the community."
    );
    amos_6_group_01(story)?;
    amos_6_group_02(story)?;
    amos_6_group_03(story)?;
    amos_6_group_04(story)?;
    Ok(())
}

fn amos_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        7,
        "Amos saw locusts, fire, and a plumb line: successive diagnostic visions",
        "showing that the wall of Israel was no longer aligned with justice.",
        "Amaziah, the court priest, tried to evict the prophet from the royal",
        "channel, but Amos answered that he had not chosen the job; the Maintainer",
        "had interrupted his shepherd schedule and sent him with the report."
    );
    amos_7_group_01(story)?;
    amos_7_group_02(story)?;
    amos_7_group_03(story)?;
    amos_7_group_04(story)?;
    Ok(())
}

fn amos_8(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        8,
        "A basket of summer fruit signaled that Israel's harvest and its patience",
        "had reached their final state, ripe for judgment rather than another delay.",
        "Merchants waited for sacred rest days to end so they could shrink measures,",
        "inflate prices, and purchase the poor; the prophet foresaw a famine not",
        "of bread but of trustworthy words."
    );
    amos_8_group_01(story)?;
    amos_8_group_02(story)?;
    amos_8_group_03(story)?;
    amos_8_group_04(story)?;
    Ok(())
}

fn amos_9(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Amos",
        9,
        "Amos saw the Maintainer standing beside the altar, able to reach fugitives",
        "in the sea, the underworld, or the farthest server rack.",
        "Yet the ending opened a restoration branch: David's fallen shelter would",
        "be rebuilt, the land would overflow with fruit, and the returned people",
        "would plant vineyards whose harvest could finally remain in their hands."
    );
    amos_9_group_01(story)?;
    amos_9_group_02(story)?;
    amos_9_group_03(story)?;
    amos_9_group_04(story)?;
    Ok(())
}

fn amos_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        1,
        1,
        4,
        "israel match_value -> call -> guard -> move_value israel words amos",
        &["israel", "words", "amos", "among", "herdmen"],
    );
    segment.match_value(
        1,
        "israel",
        &["words", "amos", "among"],
        &["israel", "words", "amos", "among", "herdmen"],
    )?;
    segment.call(
        2,
        "lord",
        &["will", "roar", "zion"],
        &["lord", "will", "roar", "zion", "utter"],
    )?;
    segment.guard(
        3,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.move_value(
        4,
        "will",
        &["send", "fire", "house"],
        &["will", "send", "fire", "house", "hazael"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        1,
        5,
        8,
        "will move_value -> guard -> expression will break bar",
        &["will", "break", "bar", "damascus", "cut"],
    );
    segment.move_value(
        5,
        "will",
        &["break", "bar", "damascus"],
        &["will", "break", "bar", "damascus", "cut"],
    )?;
    segment.guard(
        6,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.move_value(
        7,
        "will",
        &["send", "fire", "wall"],
        &["will", "send", "fire", "wall", "gaza"],
    )?;
    segment.expression(
        8,
        "will",
        &["cut", "off", "inhabitant"],
        &["will", "cut", "off", "inhabitant", "ashdod"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        1,
        9,
        12,
        "lord guard -> move_value lord thus saith",
        &["lord", "thus", "saith", "three", "transgressions"],
    );
    segment.guard(
        9,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.move_value(
        10,
        "will",
        &["send", "fire", "wall"],
        &["will", "send", "fire", "wall", "tyrus"],
    )?;
    segment.guard(
        11,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.move_value(
        12,
        "will",
        &["send", "fire", "teman"],
        &["will", "send", "fire", "teman", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        1,
        13,
        15,
        "lord guard -> expression -> move_value lord thus saith",
        &["lord", "thus", "saith", "three", "transgressions"],
    );
    segment.guard(
        13,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.expression(
        14,
        "will",
        &["kindle", "fire", "wall"],
        &["will", "kindle", "fire", "wall", "rabbah"],
    )?;
    segment.move_value(
        15,
        "king",
        &["shall", "captivity", "princes"],
        &["king", "shall", "captivity", "princes", "together"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        2,
        1,
        4,
        "lord guard -> move_value -> expression lord thus saith",
        &["lord", "thus", "saith", "three", "transgressions"],
    );
    segment.guard(
        1,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.move_value(
        2,
        "will",
        &["send", "fire", "moab"],
        &["will", "send", "fire", "moab", "shall"],
    )?;
    segment.expression(
        3,
        "will",
        &["cut", "off", "judge"],
        &["will", "cut", "off", "judge", "midst"],
    )?;
    segment.guard(
        4,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        2,
        5,
        8,
        "judah move_value -> guard -> trait_bound judah will send",
        &["judah", "will", "send", "fire", "shall"],
    );
    segment.move_value(
        5,
        "judah",
        &["will", "send", "fire"],
        &["judah", "will", "send", "fire", "shall"],
    )?;
    segment.guard(
        6,
        "lord",
        &["thus", "saith", "three"],
        &["lord", "thus", "saith", "three", "transgressions"],
    )?;
    segment.move_value(
        7,
        "pant",
        &["dust", "earth", "head"],
        &["pant", "dust", "earth", "head", "poor"],
    )?;
    segment.trait_bound(
        8,
        "lay",
        &["themselves", "down", "clothes"],
        &["lay", "themselves", "down", "clothes", "laid"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        2,
        9,
        12,
        "yet drop_value -> move_value -> reinitialize -> guard yet destroyed amorite",
        &["yet", "destroyed", "amorite", "whose", "height"],
    );
    segment.drop_value(
        9,
        "yet",
        &["destroyed", "amorite", "whose"],
        &["yet", "destroyed", "amorite", "whose", "height"],
    )?;
    segment.move_value(
        10,
        "brought",
        &["land", "egypt", "led"],
        &["brought", "land", "egypt", "led", "forty"],
    )?;
    segment.reinitialize(
        11,
        "men",
        &["raised", "sons", "prophets"],
        &["men", "raised", "sons", "prophets", "young"],
    )?;
    segment.guard(
        12,
        "gave",
        &["nazarites", "wine", "drink"],
        &["gave", "nazarites", "wine", "drink", "commanded"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        2,
        13,
        16,
        "behold match_value -> guard -> move_value behold pressed under",
        &["behold", "pressed", "under", "cart", "full"],
    );
    segment.match_value(
        13,
        "behold",
        &["pressed", "under", "cart"],
        &["behold", "pressed", "under", "cart", "full"],
    )?;
    segment.guard(
        14,
        "therefore",
        &["flight", "shall", "perish"],
        &["therefore", "flight", "shall", "perish", "swift"],
    )?;
    segment.guard(
        15,
        "neither",
        &["shall", "stand", "handleth"],
        &["neither", "shall", "stand", "handleth", "bow"],
    )?;
    segment.move_value(
        16,
        "courageous",
        &["among", "mighty", "shall"],
        &["courageous", "among", "mighty", "shall", "flee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        3,
        1,
        4,
        "lord move_value -> expression -> guard lord hear word",
        &["lord", "hear", "word", "hath", "spoken"],
    );
    segment.move_value(
        1,
        "lord",
        &["hear", "word", "hath"],
        &["lord", "hear", "word", "hath", "spoken"],
    )?;
    segment.expression(
        2,
        "known",
        &["families", "earth", "therefore"],
        &["known", "families", "earth", "therefore", "will"],
    )?;
    segment.expression(
        3,
        "two",
        &["walk", "together", "except"],
        &["two", "walk", "together", "except", "agreed"],
    )?;
    segment.guard(
        4,
        "will",
        &["lion", "roar", "forest"],
        &["will", "lion", "roar", "forest", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        3,
        5,
        8,
        "bird guard -> expression bird fall snare",
        &["bird", "fall", "snare", "earth", "gin"],
    );
    segment.guard(
        5,
        "bird",
        &["fall", "snare", "earth"],
        &["bird", "fall", "snare", "earth", "gin"],
    )?;
    segment.guard(
        6,
        "people",
        &["shall", "trumpet", "blown"],
        &["people", "shall", "trumpet", "blown", "city"],
    )?;
    segment.expression(
        7,
        "lord",
        &["surely", "god", "will"],
        &["lord", "surely", "god", "will", "nothing"],
    )?;
    segment.guard(
        8,
        "lord",
        &["lion", "hath", "roared"],
        &["lord", "lion", "hath", "roared", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        3,
        9,
        12,
        "publish match_value -> guard -> move_value -> expression publish palaces ashdod",
        &["publish", "palaces", "ashdod", "land", "egypt"],
    );
    segment.match_value(
        9,
        "publish",
        &["palaces", "ashdod", "land"],
        &["publish", "palaces", "ashdod", "land", "egypt"],
    )?;
    segment.guard(
        10,
        "lord",
        &["know", "not", "right"],
        &["lord", "know", "not", "right", "saith"],
    )?;
    segment.move_value(
        11,
        "lord",
        &["therefore", "thus", "saith"],
        &["lord", "therefore", "thus", "saith", "god"],
    )?;
    segment.expression(
        12,
        "lord",
        &["thus", "saith", "shepherd"],
        &["lord", "thus", "saith", "shepherd", "taketh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        3,
        13,
        15,
        "jacob expression -> trait_bound jacob hear testify",
        &["jacob", "hear", "testify", "house", "saith"],
    );
    segment.expression(
        13,
        "jacob",
        &["hear", "testify", "house"],
        &["jacob", "hear", "testify", "house", "saith"],
    )?;
    segment.trait_bound(
        14,
        "israel",
        &["day", "shall", "visit"],
        &["israel", "day", "shall", "visit", "transgressions"],
    )?;
    segment.expression(
        15,
        "will",
        &["smite", "winter", "house"],
        &["will", "smite", "winter", "house", "summer"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        4,
        1,
        4,
        "hear move_value hear word kine",
        &["hear", "word", "kine", "bashan", "mountain"],
    );
    segment.move_value(
        1,
        "hear",
        &["word", "kine", "bashan"],
        &["hear", "word", "kine", "bashan", "mountain"],
    )?;
    segment.move_value(
        2,
        "lord",
        &["god", "hath", "sworn"],
        &["lord", "god", "hath", "sworn", "holiness"],
    )?;
    segment.move_value(
        3,
        "lord",
        &["shall", "breaches", "every"],
        &["lord", "shall", "breaches", "every", "cow"],
    )?;
    segment.move_value(
        4,
        "bethel",
        &["transgress", "gilgal", "multiply"],
        &[
            "bethel",
            "transgress",
            "gilgal",
            "multiply",
            "transgression",
        ],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        4,
        5,
        8,
        "offer trait_bound -> guard offer sacrifice thanksgiving",
        &["offer", "sacrifice", "thanksgiving", "leaven", "proclaim"],
    );
    segment.trait_bound(
        5,
        "offer",
        &["sacrifice", "thanksgiving", "leaven"],
        &["offer", "sacrifice", "thanksgiving", "leaven", "proclaim"],
    )?;
    segment.guard(
        6,
        "given",
        &["cleanness", "teeth", "cities"],
        &["given", "cleanness", "teeth", "cities", "want"],
    )?;
    segment.guard(
        7,
        "withholden",
        &["rain", "yet", "three"],
        &["withholden", "rain", "yet", "three", "months"],
    )?;
    segment.guard(
        8,
        "two",
        &["three", "cities", "wandered"],
        &["two", "three", "cities", "wandered", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        4,
        9,
        13,
        "smitten guard -> construct -> expression smitten blasting mildew",
        &["smitten", "blasting", "mildew", "gardens", "vineyards"],
    );
    segment.guard(
        9,
        "smitten",
        &["blasting", "mildew", "gardens"],
        &["smitten", "blasting", "mildew", "gardens", "vineyards"],
    )?;
    segment.construct(
        10,
        "men",
        &["sent", "among", "pestilence"],
        &["men", "sent", "among", "pestilence", "manner"],
    )?;
    segment.guard(
        11,
        "god",
        &["overthrown", "overthrew", "sodom"],
        &["god", "overthrown", "overthrew", "sodom", "gomorrah"],
    )?;
    segment.expression(
        12,
        "israel",
        &["therefore", "thus", "will"],
        &["israel", "therefore", "thus", "will", "unto"],
    )?;
    segment.expression(
        13,
        "man",
        &["formeth", "mountains", "createth"],
        &["man", "formeth", "mountains", "createth", "wind"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        5,
        1,
        4,
        "israel call -> guard -> move_value -> expression israel hear word",
        &["israel", "hear", "word", "take", "against"],
    );
    segment.call(
        1,
        "israel",
        &["hear", "word", "take"],
        &["israel", "hear", "word", "take", "against"],
    )?;
    segment.guard(
        2,
        "israel",
        &["virgin", "fallen", "shall"],
        &["israel", "virgin", "fallen", "shall", "rise"],
    )?;
    segment.move_value(
        3,
        "lord",
        &["thus", "saith", "god"],
        &["lord", "thus", "saith", "god", "city"],
    )?;
    segment.expression(
        4,
        "lord",
        &["thus", "saith", "unto"],
        &["lord", "thus", "saith", "unto", "house"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        5,
        5,
        8,
        "seek move_value -> expression -> drop_value seek not bethel",
        &["seek", "not", "bethel", "nor", "enter"],
    );
    segment.move_value(
        5,
        "seek",
        &["not", "bethel", "nor"],
        &["seek", "not", "bethel", "nor", "enter"],
    )?;
    segment.expression(
        6,
        "lord",
        &["seek", "shall", "live"],
        &["lord", "seek", "shall", "live", "lest"],
    )?;
    segment.expression(
        7,
        "turn",
        &["judgment", "wormwood", "leave"],
        &["turn", "judgment", "wormwood", "leave", "off"],
    )?;
    segment.drop_value(
        8,
        "seek",
        &["maketh", "seven", "stars"],
        &["seek", "maketh", "seven", "stars", "orion"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        5,
        9,
        12,
        "strengtheneth move_value -> expression -> construct strengtheneth spoiled against",
        &["strengtheneth", "spoiled", "against", "strong", "shall"],
    );
    segment.move_value(
        9,
        "strengtheneth",
        &["spoiled", "against", "strong"],
        &["strengtheneth", "spoiled", "against", "strong", "shall"],
    )?;
    segment.expression(
        10,
        "hate",
        &["rebuketh", "gate", "abhor"],
        &["hate", "rebuketh", "gate", "abhor", "speaketh"],
    )?;
    segment.construct(
        11,
        "forasmuch",
        &["therefore", "treading", "poor"],
        &["forasmuch", "therefore", "treading", "poor", "take"],
    )?;
    segment.expression(
        12,
        "know",
        &["manifold", "transgressions", "mighty"],
        &["know", "manifold", "transgressions", "mighty", "sins"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        5,
        13,
        16,
        "therefore guard -> borrow -> call therefore prudent shall",
        &["therefore", "prudent", "shall", "keep", "silence"],
    );
    segment.guard(
        13,
        "therefore",
        &["prudent", "shall", "keep"],
        &["therefore", "prudent", "shall", "keep", "silence"],
    )?;
    segment.guard(
        14,
        "lord",
        &["seek", "good", "not"],
        &["lord", "seek", "good", "not", "evil"],
    )?;
    segment.borrow(
        15,
        "lord",
        &["hate", "evil", "love"],
        &["lord", "hate", "evil", "love", "good"],
    )?;
    segment.call(
        16,
        "lord",
        &["therefore", "god", "hosts"],
        &["lord", "therefore", "god", "hosts", "saith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        5,
        17,
        20,
        "vineyards expression -> guard -> move_value vineyards shall wailing",
        &["vineyards", "shall", "wailing", "will", "pass"],
    );
    segment.expression(
        17,
        "vineyards",
        &["shall", "wailing", "will"],
        &["vineyards", "shall", "wailing", "will", "pass"],
    )?;
    segment.guard(
        18,
        "lord",
        &["woe", "unto", "desire"],
        &["lord", "woe", "unto", "desire", "day"],
    )?;
    segment.move_value(
        19,
        "man",
        &["flee", "lion", "bear"],
        &["man", "flee", "lion", "bear", "met"],
    )?;
    segment.guard(
        20,
        "lord",
        &["shall", "not", "day"],
        &["lord", "shall", "not", "day", "darkness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_5_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        5,
        21,
        24,
        "hate guard -> expression hate despise feast",
        &["hate", "despise", "feast", "days", "will"],
    );
    segment.guard(
        21,
        "hate",
        &["despise", "feast", "days"],
        &["hate", "despise", "feast", "days", "will"],
    )?;
    segment.guard(
        22,
        "though",
        &["offer", "burnt", "offerings"],
        &["though", "offer", "burnt", "offerings", "meat"],
    )?;
    segment.guard(
        23,
        "take",
        &["away", "noise", "songs"],
        &["take", "away", "noise", "songs", "will"],
    )?;
    segment.expression(
        24,
        "judgment",
        &["run", "down", "waters"],
        &["judgment", "run", "down", "waters", "righteousness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_5_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        5,
        25,
        27,
        "offered trait_bound -> construct -> move_value offered unto sacrifices",
        &["offered", "unto", "sacrifices", "offerings", "wilderness"],
    );
    segment.trait_bound(
        25,
        "offered",
        &["unto", "sacrifices", "offerings"],
        &["offered", "unto", "sacrifices", "offerings", "wilderness"],
    )?;
    segment.construct(
        26,
        "god",
        &["borne", "tabernacle", "moloch"],
        &["god", "borne", "tabernacle", "moloch", "chiun"],
    )?;
    segment.move_value(
        27,
        "lord",
        &["therefore", "will", "cause"],
        &["lord", "therefore", "will", "cause", "captivity"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        6,
        1,
        4,
        "woe move_value -> expression woe ease zion",
        &["woe", "ease", "zion", "trust", "mountain"],
    );
    segment.move_value(
        1,
        "woe",
        &["ease", "zion", "trust"],
        &["woe", "ease", "zion", "trust", "mountain"],
    )?;
    segment.move_value(
        2,
        "pass",
        &["unto", "calneh", "see"],
        &["pass", "unto", "calneh", "see", "thence"],
    )?;
    segment.move_value(
        3,
        "put",
        &["far", "away", "evil"],
        &["put", "far", "away", "evil", "day"],
    )?;
    segment.expression(
        4,
        "lie",
        &["beds", "ivory", "stretch"],
        &["lie", "beds", "ivory", "stretch", "themselves"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        6,
        5,
        8,
        "chant expression -> guard -> move_value chant sound viol",
        &["chant", "sound", "viol", "invent", "themselves"],
    );
    segment.expression(
        5,
        "chant",
        &["sound", "viol", "invent"],
        &["chant", "sound", "viol", "invent", "themselves"],
    )?;
    segment.guard(
        6,
        "drink",
        &["wine", "bowls", "anoint"],
        &["drink", "wine", "bowls", "anoint", "themselves"],
    )?;
    segment.move_value(
        7,
        "therefore",
        &["now", "shall", "captive"],
        &["therefore", "now", "shall", "captive", "first"],
    )?;
    segment.expression(
        8,
        "lord",
        &["god", "hath", "sworn"],
        &["lord", "god", "hath", "sworn", "himself"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        6,
        9,
        11,
        "men move_value -> construct -> match_value men shall pass",
        &["men", "shall", "pass", "remain", "ten"],
    );
    segment.move_value(
        9,
        "men",
        &["shall", "pass", "remain"],
        &["men", "shall", "pass", "remain", "ten"],
    )?;
    segment.construct(
        10,
        "man's",
        &["uncle", "shall", "take"],
        &["man's", "uncle", "shall", "take", "burneth"],
    )?;
    segment.match_value(
        11,
        "lord",
        &["behold", "commandeth", "will"],
        &["lord", "behold", "commandeth", "will", "smite"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        6,
        12,
        14,
        "shall expression -> guard -> match_value shall horses run",
        &["shall", "horses", "run", "rock", "will"],
    );
    segment.expression(
        12,
        "shall",
        &["horses", "run", "rock"],
        &["shall", "horses", "run", "rock", "will"],
    )?;
    segment.guard(
        13,
        "rejoice",
        &["thing", "nought", "say"],
        &["rejoice", "thing", "nought", "say", "not"],
    )?;
    segment.match_value(
        14,
        "nation",
        &["behold", "will", "raise"],
        &["nation", "behold", "will", "raise", "against"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        7,
        1,
        4,
        "lord construct -> guard -> match_value lord thus hath",
        &["lord", "thus", "hath", "god", "shewed"],
    );
    segment.construct(
        1,
        "lord",
        &["thus", "hath", "god"],
        &["lord", "thus", "hath", "god", "shewed"],
    )?;
    segment.construct(
        2,
        "lord",
        &["pass", "made", "end"],
        &["lord", "pass", "made", "end", "eating"],
    )?;
    segment.guard(
        3,
        "lord",
        &["repented", "shall", "not"],
        &["lord", "repented", "shall", "not", "saith"],
    )?;
    segment.match_value(
        4,
        "lord",
        &["thus", "hath", "god"],
        &["lord", "thus", "hath", "god", "shewed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        7,
        5,
        8,
        "lord call -> guard -> construct lord god cease",
        &["lord", "god", "cease", "beseech", "thee"],
    );
    segment.call(
        5,
        "lord",
        &["god", "cease", "beseech"],
        &["lord", "god", "cease", "beseech", "thee"],
    )?;
    segment.guard(
        6,
        "lord",
        &["repented", "shall", "not"],
        &["lord", "repented", "shall", "not", "saith"],
    )?;
    segment.construct(
        7,
        "lord",
        &["thus", "shewed", "behold"],
        &["lord", "thus", "shewed", "behold", "stood"],
    )?;
    segment.guard(
        8,
        "lord",
        &["unto", "amos", "seest"],
        &["lord", "unto", "amos", "seest", "plumbline"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        7,
        9,
        12,
        "isaac expression -> move_value isaac high places",
        &["isaac", "high", "places", "shall", "desolate"],
    );
    segment.expression(
        9,
        "isaac",
        &["high", "places", "shall"],
        &["isaac", "high", "places", "shall", "desolate"],
    )?;
    segment.move_value(
        10,
        "priest",
        &["amaziah", "bethel", "sent"],
        &["priest", "amaziah", "bethel", "sent", "jeroboam"],
    )?;
    segment.expression(
        11,
        "israel",
        &["thus", "amos", "saith"],
        &["israel", "thus", "amos", "saith", "jeroboam"],
    )?;
    segment.move_value(
        12,
        "amaziah",
        &["unto", "amos", "seer"],
        &["amaziah", "unto", "amos", "seer", "flee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_7_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        7,
        13,
        17,
        "prophesy guard -> move_value prophesy not again",
        &["prophesy", "not", "again", "any", "bethel"],
    );
    segment.guard(
        13,
        "prophesy",
        &["not", "again", "any"],
        &["prophesy", "not", "again", "any", "bethel"],
    )?;
    segment.guard(
        14,
        "prophet",
        &["answered", "amos", "amaziah"],
        &["prophet", "answered", "amos", "amaziah", "neither"],
    )?;
    segment.move_value(
        15,
        "lord",
        &["took", "followed", "flock"],
        &["lord", "took", "followed", "flock", "unto"],
    )?;
    segment.guard(
        16,
        "lord",
        &["now", "therefore", "hear"],
        &["lord", "now", "therefore", "hear", "word"],
    )?;
    segment.move_value(
        17,
        "lord",
        &["therefore", "thus", "saith"],
        &["lord", "therefore", "thus", "saith", "wife"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_8_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        8,
        1,
        4,
        "lord match_value -> move_value -> drop_value -> construct lord thus hath",
        &["lord", "thus", "hath", "god", "shewed"],
    );
    segment.match_value(
        1,
        "lord",
        &["thus", "hath", "god"],
        &["lord", "thus", "hath", "god", "shewed"],
    )?;
    segment.move_value(
        2,
        "lord",
        &["amos", "seest", "basket"],
        &["lord", "amos", "seest", "basket", "summer"],
    )?;
    segment.drop_value(
        3,
        "lord",
        &["songs", "temple", "shall"],
        &["lord", "songs", "temple", "shall", "howlings"],
    )?;
    segment.construct(
        4,
        "hear",
        &["swallow", "needy", "even"],
        &["hear", "swallow", "needy", "even", "make"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_8_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        8,
        5,
        8,
        "saying move_value -> expression -> guard saying will new",
        &["saying", "will", "new", "moon", "gone"],
    );
    segment.move_value(
        5,
        "saying",
        &["will", "new", "moon"],
        &["saying", "will", "new", "moon", "gone"],
    )?;
    segment.expression(
        6,
        "buy",
        &["poor", "silver", "needy"],
        &["buy", "poor", "silver", "needy", "pair"],
    )?;
    segment.expression(
        7,
        "lord",
        &["hath", "sworn", "excellency"],
        &["lord", "hath", "sworn", "excellency", "jacob"],
    )?;
    segment.guard(
        8,
        "shall",
        &["not", "land", "tremble"],
        &["shall", "not", "land", "tremble", "every"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_8_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        8,
        9,
        11,
        "lord move_value -> construct lord shall pass",
        &["lord", "shall", "pass", "day", "saith"],
    );
    segment.move_value(
        9,
        "lord",
        &["shall", "pass", "day"],
        &["lord", "shall", "pass", "day", "saith"],
    )?;
    segment.construct(
        10,
        "will",
        &["turn", "feasts", "mourning"],
        &["will", "turn", "feasts", "mourning", "songs"],
    )?;
    segment.move_value(
        11,
        "lord",
        &["behold", "days", "saith"],
        &["lord", "behold", "days", "saith", "god"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_8_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        8,
        12,
        14,
        "shall guard -> expression -> call shall wander sea",
        &["shall", "wander", "sea", "north", "even"],
    );
    segment.guard(
        12,
        "shall",
        &["wander", "sea", "north"],
        &["shall", "wander", "sea", "north", "even"],
    )?;
    segment.expression(
        13,
        "men",
        &["day", "shall", "fair"],
        &["men", "day", "shall", "fair", "virgins"],
    )?;
    segment.call(
        14,
        "god",
        &["swear", "sin", "samaria"],
        &["god", "swear", "sin", "samaria", "say"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_9_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        9,
        1,
        4,
        "lord move_value -> guard lord saw standing",
        &["lord", "saw", "standing", "altar", "smite"],
    );
    segment.move_value(
        1,
        "lord",
        &["saw", "standing", "altar"],
        &["lord", "saw", "standing", "altar", "smite"],
    )?;
    segment.move_value(
        2,
        "though",
        &["dig", "hell", "thence"],
        &["though", "dig", "hell", "thence", "shall"],
    )?;
    segment.guard(
        3,
        "though",
        &["hide", "themselves", "top"],
        &["though", "hide", "themselves", "top", "carmel"],
    )?;
    segment.move_value(
        4,
        "though",
        &["captivity", "enemies", "thence"],
        &["though", "captivity", "enemies", "thence", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_9_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        9,
        5,
        8,
        "lord expression -> move_value -> drop_value lord god hosts",
        &["lord", "god", "hosts", "toucheth", "land"],
    );
    segment.expression(
        5,
        "lord",
        &["god", "hosts", "toucheth"],
        &["lord", "god", "hosts", "toucheth", "land"],
    )?;
    segment.expression(
        6,
        "buildeth",
        &["stories", "heaven", "hath"],
        &["buildeth", "stories", "heaven", "hath", "founded"],
    )?;
    segment.move_value(
        7,
        "children",
        &["not", "ethiopians", "unto"],
        &["children", "not", "ethiopians", "unto", "israel"],
    )?;
    segment.drop_value(
        8,
        "lord",
        &["behold", "eyes", "god"],
        &["lord", "behold", "eyes", "god", "sinful"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_9_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        9,
        9,
        12,
        "israel guard -> construct -> expression israel will command",
        &["israel", "will", "command", "sift", "house"],
    );
    segment.guard(
        9,
        "israel",
        &["will", "command", "sift"],
        &["israel", "will", "command", "sift", "house"],
    )?;
    segment.guard(
        10,
        "people",
        &["sinners", "shall", "die"],
        &["people", "sinners", "shall", "die", "sword"],
    )?;
    segment.construct(
        11,
        "david",
        &["day", "will", "raise"],
        &["david", "day", "will", "raise", "tabernacle"],
    )?;
    segment.expression(
        12,
        "lord",
        &["possess", "remnant", "edom"],
        &["lord", "possess", "remnant", "edom", "heathen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn amos_9_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Amos,
        9,
        13,
        15,
        "lord move_value -> construct -> guard lord behold days",
        &["lord", "behold", "days", "saith", "plowman"],
    );
    segment.move_value(
        13,
        "lord",
        &["behold", "days", "saith"],
        &["lord", "behold", "days", "saith", "plowman"],
    )?;
    segment.construct(
        14,
        "people",
        &["will", "bring", "again"],
        &["people", "will", "bring", "again", "captivity"],
    )?;
    segment.guard(
        15,
        "lord",
        &["will", "plant", "land"],
        &["lord", "will", "plant", "land", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}
