use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    ruth(story)?;
    Ok(())
}

fn ruth(story: &mut Story) -> Result<(), StoryError> {
    ruth_1(story)?;
    ruth_2(story)?;
    ruth_3(story)?;
    ruth_4(story)?;
    Ok(())
}

fn ruth_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ruth", 1, "Naomi loses husband and sons in Moab, then starts home with Ruth and Orpah facing a future with no obvious maintainer.", "Ruth refuses to drop the relationship, binding her lifetime to Naomi's people and to a God she has only begun to know.");
    ruth_1_group_01(story)?;
    ruth_1_group_02(story)?;
    ruth_1_group_03(story)?;
    ruth_1_group_04(story)?;
    ruth_1_group_05(story)?;
    ruth_1_group_06(story)?;
    Ok(())
}

fn ruth_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ruth", 2, "Ruth gleans behind Boaz's harvesters, and the landowner notices a foreign worker whose loyalty has already passed the review.", "Boaz grants protection and extra grain, turning an ordinary field into a safe workspace for a vulnerable newcomer.");
    ruth_2_group_01(story)?;
    ruth_2_group_02(story)?;
    ruth_2_group_03(story)?;
    ruth_2_group_04(story)?;
    ruth_2_group_05(story)?;
    ruth_2_group_06(story)?;
    Ok(())
}

fn ruth_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ruth", 3, "Naomi designs a midnight appeal at the threshing floor, where Ruth asks Boaz to act as family redeemer.", "Boaz accepts the responsibility but reports a nearer claimant, preserving both kindness and the required order of operations.");
    ruth_3_group_01(story)?;
    ruth_3_group_02(story)?;
    ruth_3_group_03(story)?;
    ruth_3_group_04(story)?;
    ruth_3_group_05(story)?;
    Ok(())
}

fn ruth_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ruth", 4, "Boaz resolves the nearer redeemer's claim at the town gate, acquires the field, and marries Ruth.", "Their son Obed enters Naomi's restored household, and the genealogy quietly connects this small rescue to David's coming line.");
    ruth_4_group_01(story)?;
    ruth_4_group_02(story)?;
    ruth_4_group_03(story)?;
    ruth_4_group_04(story)?;
    ruth_4_group_05(story)?;
    ruth_4_group_06(story)?;
    Ok(())
}

fn ruth_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        1,
        1,
        4,
        "now move_value -> drop_value -> expression now pass days",
        &["now", "pass", "days", "judges", "ruled"],
    );
    segment.move_value(
        1,
        "now",
        &["pass", "days", "judges"],
        &["now", "pass", "days", "judges", "ruled"],
    )?;
    segment.move_value(
        2,
        "man",
        &["name", "elimelech", "wife"],
        &["man", "name", "elimelech", "wife", "naomi"],
    )?;
    segment.drop_value(
        3,
        "elimelech",
        &["naomi's", "husband", "died"],
        &["elimelech", "naomi's", "husband", "died", "left"],
    )?;
    segment.expression(
        4,
        "women",
        &["took", "wives", "moab"],
        &["women", "took", "wives", "moab", "name"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        1,
        5,
        8,
        "woman drop_value -> reinitialize -> move_value woman mahlon chilion",
        &["woman", "mahlon", "chilion", "died", "both"],
    );
    segment.drop_value(
        5,
        "woman",
        &["mahlon", "chilion", "died"],
        &["woman", "mahlon", "chilion", "died", "both"],
    )?;
    segment.reinitialize(
        6,
        "arose",
        &["daughters", "law", "might"],
        &["arose", "daughters", "law", "might", "return"],
    )?;
    segment.move_value(
        7,
        "wherefore",
        &["went", "forth", "place"],
        &["wherefore", "went", "forth", "place", "two"],
    )?;
    segment.move_value(
        8,
        "naomi",
        &["unto", "two", "daughters"],
        &["naomi", "unto", "two", "daughters", "law"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        1,
        9,
        12,
        "lord result -> call -> move_value lord grant find",
        &["lord", "grant", "find", "rest", "each"],
    );
    segment.result(
        9,
        "lord",
        &["grant", "find", "rest"],
        &["lord", "grant", "find", "rest", "each"],
    )?;
    segment.call(
        10,
        "people",
        &["unto", "surely", "will"],
        &["people", "unto", "surely", "will", "return"],
    )?;
    segment.move_value(
        11,
        "naomi",
        &["turn", "again", "daughters"],
        &["naomi", "turn", "again", "daughters", "why"],
    )?;
    segment.move_value(
        12,
        "turn",
        &["again", "daughters", "way"],
        &["turn", "again", "daughters", "way", "too"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        1,
        13,
        16,
        "would move_value -> guard would tarry till",
        &["would", "tarry", "till", "grown", "stay"],
    );
    segment.move_value(
        13,
        "would",
        &["tarry", "till", "grown"],
        &["would", "tarry", "till", "grown", "stay"],
    )?;
    segment.guard(
        14,
        "lifted",
        &["voice", "wept", "again"],
        &["lifted", "voice", "wept", "again", "orpah"],
    )?;
    segment.move_value(
        15,
        "people",
        &["behold", "sister", "law"],
        &["people", "behold", "sister", "law", "gone"],
    )?;
    segment.move_value(
        16,
        "ruth",
        &["intreat", "not", "leave"],
        &["ruth", "intreat", "not", "leave", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        1,
        17,
        19,
        "lord drop_value -> move_value lord diest will",
        &["lord", "diest", "will", "die", "buried"],
    );
    segment.drop_value(
        17,
        "lord",
        &["diest", "will", "die"],
        &["lord", "diest", "will", "die", "buried"],
    )?;
    segment.move_value(
        18,
        "saw",
        &["stedfastly", "minded", "left"],
        &["saw", "stedfastly", "minded", "left", "speaking"],
    )?;
    segment.move_value(
        19,
        "two",
        &["went", "until", "bethlehem"],
        &["two", "went", "until", "bethlehem", "pass"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        1,
        20,
        22,
        "unto guard -> move_value unto call not",
        &["unto", "call", "not", "naomi", "mara"],
    );
    segment.guard(
        20,
        "unto",
        &["call", "not", "naomi"],
        &["unto", "call", "not", "naomi", "mara"],
    )?;
    segment.move_value(
        21,
        "lord",
        &["went", "full", "hath"],
        &["lord", "went", "full", "hath", "brought"],
    )?;
    segment.move_value(
        22,
        "naomi",
        &["returned", "ruth", "moabitess"],
        &["naomi", "returned", "ruth", "moabitess", "daughter"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        2,
        1,
        4,
        "man expression -> move_value man naomi kinsman",
        &["man", "naomi", "kinsman", "husband's", "mighty"],
    );
    segment.expression(
        1,
        "man",
        &["naomi", "kinsman", "husband's"],
        &["man", "naomi", "kinsman", "husband's", "mighty"],
    )?;
    segment.move_value(
        2,
        "ruth",
        &["moabitess", "unto", "naomi"],
        &["ruth", "moabitess", "unto", "naomi", "now"],
    )?;
    segment.move_value(
        3,
        "went",
        &["gleaned", "field", "reapers"],
        &["went", "gleaned", "field", "reapers", "hap"],
    )?;
    segment.move_value(
        4,
        "lord",
        &["behold", "boaz", "bethlehem"],
        &["lord", "behold", "boaz", "bethlehem", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        2,
        5,
        8,
        "boaz call -> move_value boaz unto servant",
        &["boaz", "unto", "servant", "set", "over"],
    );
    segment.call(
        5,
        "boaz",
        &["unto", "servant", "set"],
        &["boaz", "unto", "servant", "set", "over"],
    )?;
    segment.move_value(
        6,
        "servant",
        &["set", "over", "reapers"],
        &["servant", "set", "over", "reapers", "answered"],
    )?;
    segment.move_value(
        7,
        "pray",
        &["glean", "gather", "reapers"],
        &["pray", "glean", "gather", "reapers", "among"],
    )?;
    segment.move_value(
        8,
        "boaz",
        &["unto", "ruth", "hearest"],
        &["boaz", "unto", "ruth", "hearest", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        2,
        9,
        12,
        "men move_value -> call men thine eyes",
        &["men", "thine", "eyes", "field", "reap"],
    );
    segment.move_value(
        9,
        "men",
        &["thine", "eyes", "field"],
        &["men", "thine", "eyes", "field", "reap"],
    )?;
    segment.call(
        10,
        "fell",
        &["face", "bowed", "herself"],
        &["fell", "face", "bowed", "herself", "ground"],
    )?;
    segment.move_value(
        11,
        "boaz",
        &["answered", "unto", "hath"],
        &["boaz", "answered", "unto", "hath", "fully"],
    )?;
    segment.move_value(
        12,
        "lord",
        &["recompense", "work", "full"],
        &["lord", "recompense", "work", "full", "reward"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        2,
        13,
        16,
        "lord guard -> move_value -> reinitialize lord find favour",
        &["lord", "find", "favour", "sight", "hast"],
    );
    segment.guard(
        13,
        "lord",
        &["find", "favour", "sight"],
        &["lord", "find", "favour", "sight", "hast"],
    )?;
    segment.move_value(
        14,
        "boaz",
        &["unto", "mealtime", "hither"],
        &["boaz", "unto", "mealtime", "hither", "eat"],
    )?;
    segment.reinitialize(
        15,
        "men",
        &["risen", "glean", "boaz"],
        &["men", "risen", "glean", "boaz", "commanded"],
    )?;
    segment.guard(
        16,
        "fall",
        &["handfuls", "purpose", "leave"],
        &["fall", "handfuls", "purpose", "leave", "glean"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        2,
        17,
        20,
        "gleaned expression -> move_value -> guard -> drop_value gleaned field until",
        &["gleaned", "field", "until", "even", "beat"],
    );
    segment.expression(
        17,
        "gleaned",
        &["field", "until", "even"],
        &["gleaned", "field", "until", "even", "beat"],
    )?;
    segment.move_value(
        18,
        "took",
        &["went", "city", "mother"],
        &["took", "went", "city", "mother", "law"],
    )?;
    segment.guard(
        19,
        "mother",
        &["law", "unto", "hast"],
        &["mother", "law", "unto", "hast", "gleaned"],
    )?;
    segment.drop_value(
        20,
        "lord",
        &["naomi", "unto", "daughter"],
        &["lord", "naomi", "unto", "daughter", "law"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        2,
        21,
        23,
        "men guard -> move_value men ruth moabitess",
        &["men", "ruth", "moabitess", "unto", "shalt"],
    );
    segment.guard(
        21,
        "men",
        &["ruth", "moabitess", "unto"],
        &["men", "ruth", "moabitess", "unto", "shalt"],
    )?;
    segment.move_value(
        22,
        "naomi",
        &["unto", "ruth", "daughter"],
        &["naomi", "unto", "ruth", "daughter", "law"],
    )?;
    segment.guard(
        23,
        "kept",
        &["fast", "maidens", "boaz"],
        &["kept", "fast", "maidens", "boaz", "glean"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        3,
        1,
        4,
        "naomi guard -> construct -> move_value naomi mother law",
        &["naomi", "mother", "law", "unto", "daughter"],
    );
    segment.guard(
        1,
        "naomi",
        &["mother", "law", "unto"],
        &["naomi", "mother", "law", "unto", "daughter"],
    )?;
    segment.guard(
        2,
        "now",
        &["not", "boaz", "kindred"],
        &["now", "not", "boaz", "kindred", "whose"],
    )?;
    segment.construct(
        3,
        "wash",
        &["thyself", "therefore", "anoint"],
        &["wash", "thyself", "therefore", "anoint", "thee"],
    )?;
    segment.move_value(
        4,
        "shall",
        &["lieth", "down", "shalt"],
        &["shall", "lieth", "down", "shalt", "mark"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        3,
        5,
        8,
        "unto call -> move_value unto sayest will",
        &["unto", "sayest", "will", "went", "down"],
    );
    segment.call(5, "unto", &["sayest", "will"], &["unto", "sayest", "will"])?;
    segment.move_value(
        6,
        "went",
        &["down", "unto", "floor"],
        &["went", "down", "unto", "floor", "according"],
    )?;
    segment.move_value(
        7,
        "boaz",
        &["eaten", "drunk", "heart"],
        &["boaz", "eaten", "drunk", "heart", "merry"],
    )?;
    segment.move_value(
        8,
        "man",
        &["pass", "midnight", "afraid"],
        &["man", "pass", "midnight", "afraid", "turned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        3,
        9,
        12,
        "art call -> guard -> expression art answered ruth",
        &["art", "answered", "ruth", "thine", "handmaid"],
    );
    segment.call(
        9,
        "art",
        &["answered", "ruth", "thine"],
        &["art", "answered", "ruth", "thine", "handmaid"],
    )?;
    segment.guard(
        10,
        "lord",
        &["blessed", "daughter", "hast"],
        &["lord", "blessed", "daughter", "hast", "shewed"],
    )?;
    segment.guard(
        11,
        "now",
        &["daughter", "fear", "not"],
        &["now", "daughter", "fear", "not", "will"],
    )?;
    segment.expression(
        12,
        "now",
        &["true", "near", "kinsman"],
        &["now", "true", "near", "kinsman", "howbeit"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        3,
        13,
        15,
        "tarry guard -> move_value tarry night shall",
        &["tarry", "night", "shall", "morning", "will"],
    );
    segment.guard(
        13,
        "tarry",
        &["night", "shall", "morning"],
        &["tarry", "night", "shall", "morning", "will"],
    )?;
    segment.move_value(
        14,
        "lay",
        &["feet", "until", "morning"],
        &["lay", "feet", "until", "morning", "rose"],
    )?;
    segment.move_value(
        15,
        "bring",
        &["vail", "hast", "thee"],
        &["bring", "vail", "hast", "thee", "hold"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        3,
        16,
        18,
        "man move_value -> guard man mother law",
        &["man", "mother", "law", "art", "daughter"],
    );
    segment.move_value(
        16,
        "man",
        &["mother", "law", "art"],
        &["man", "mother", "law", "art", "daughter"],
    )?;
    segment.move_value(
        17,
        "six",
        &["measures", "barley", "gave"],
        &["six", "measures", "barley", "gave", "not"],
    )?;
    segment.guard(
        18,
        "sit",
        &["still", "daughter", "until"],
        &["sit", "still", "daughter", "until", "know"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        4,
        1,
        4,
        "went move_value -> call -> guard went boaz gate",
        &["went", "boaz", "gate", "sat", "down"],
    );
    segment.move_value(
        1,
        "went",
        &["boaz", "gate", "sat"],
        &["went", "boaz", "gate", "sat", "down"],
    )?;
    segment.call(
        2,
        "men",
        &["took", "ten", "elders"],
        &["men", "took", "ten", "elders", "city"],
    )?;
    segment.move_value(
        3,
        "unto",
        &["kinsman", "naomi", "again"],
        &["unto", "kinsman", "naomi", "again", "country"],
    )?;
    segment.guard(
        4,
        "people",
        &["thought", "advertise", "thee"],
        &["people", "thought", "advertise", "thee", "saying"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        4,
        5,
        8,
        "boaz drop_value -> call -> expression boaz day buyest",
        &["boaz", "day", "buyest", "field", "hand"],
    );
    segment.drop_value(
        5,
        "boaz",
        &["day", "buyest", "field"],
        &["boaz", "day", "buyest", "field", "hand"],
    )?;
    segment.call(
        6,
        "kinsman",
        &["cannot", "redeem", "myself"],
        &["kinsman", "cannot", "redeem", "myself", "lest"],
    )?;
    segment.expression(
        7,
        "israel",
        &["now", "manner", "former"],
        &["israel", "now", "manner", "former", "time"],
    )?;
    segment.call(
        8,
        "therefore",
        &["kinsman", "unto", "boaz"],
        &["therefore", "kinsman", "unto", "boaz", "buy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        4,
        9,
        12,
        "people call -> drop_value -> construct -> expression people boaz unto",
        &["people", "boaz", "unto", "elders", "witnesses"],
    );
    segment.call(
        9,
        "people",
        &["boaz", "unto", "elders"],
        &["people", "boaz", "unto", "elders", "witnesses"],
    )?;
    segment.drop_value(
        10,
        "moreover",
        &["ruth", "moabitess", "wife"],
        &["moreover", "ruth", "moabitess", "wife", "mahlon"],
    )?;
    segment.construct(
        11,
        "people",
        &["gate", "elders", "witnesses"],
        &["people", "gate", "elders", "witnesses", "lord"],
    )?;
    segment.expression(
        12,
        "judah",
        &["house", "like", "pharez"],
        &["judah", "house", "like", "pharez", "tamar"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        4,
        13,
        16,
        "lord move_value -> guard -> construct -> expression lord boaz took",
        &["lord", "boaz", "took", "ruth", "wife"],
    );
    segment.move_value(
        13,
        "lord",
        &["boaz", "took", "ruth"],
        &["lord", "boaz", "took", "ruth", "wife"],
    )?;
    segment.guard(
        14,
        "women",
        &["unto", "naomi", "blessed"],
        &["women", "unto", "naomi", "blessed", "lord"],
    )?;
    segment.construct(
        15,
        "shall",
        &["unto", "thee", "restorer"],
        &["shall", "unto", "thee", "restorer", "life"],
    )?;
    segment.expression(
        16,
        "child",
        &["naomi", "took", "laid"],
        &["child", "naomi", "took", "laid", "bosom"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        4,
        17,
        19,
        "women construct women neighbours gave",
        &["women", "neighbours", "gave", "name", "saying"],
    );
    segment.construct(
        17,
        "women",
        &["neighbours", "gave", "name"],
        &["women", "neighbours", "gave", "name", "saying"],
    )?;
    segment.construct(
        18,
        "now",
        &["generations", "pharez", "begat"],
        &["now", "generations", "pharez", "begat", "hezron"],
    )?;
    segment.construct(
        19,
        "hezron",
        &["begat", "ram", "amminadab"],
        &["hezron", "begat", "ram", "amminadab"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ruth_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ruth,
        4,
        20,
        22,
        "amminadab construct amminadab begat nahshon",
        &["amminadab", "begat", "nahshon", "salmon", "boaz"],
    );
    segment.construct(
        20,
        "amminadab",
        &["begat", "nahshon", "salmon"],
        &["amminadab", "begat", "nahshon", "salmon"],
    )?;
    segment.construct(
        21,
        "salmon",
        &["begat", "boaz", "obed"],
        &["salmon", "begat", "boaz", "obed"],
    )?;
    segment.construct(
        22,
        "david",
        &["obed", "begat", "jesse"],
        &["david", "obed", "begat", "jesse"],
    )?;
    segment.commit()?;
    Ok(())
}
