use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    malachi(story)?;
    Ok(())
}

fn malachi(story: &mut Story) -> Result<(), StoryError> {
    malachi_1(story)?;
    malachi_2(story)?;
    malachi_3(story)?;
    malachi_4(story)?;
    Ok(())
}

fn malachi_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Malachi",
        1,
        "Malachi began with a dispute over love: the people asked for evidence,",
        "and the answer pointed to their protected branch beside its rejected twin.",
        "The priests then offered damaged animals and called the defective output",
        "good enough, while the Maintainer insisted that honor is not a label",
        "pasted onto a careless release."
    );
    malachi_1_group_01(story)?;
    malachi_1_group_02(story)?;
    malachi_1_group_03(story)?;
    malachi_1_group_04(story)?;
    Ok(())
}

fn malachi_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Malachi",
        2,
        "The priestly covenant was reviewed, and its guardians were accused of",
        "teaching partial law, showing favoritism, and making many stumble.",
        "Judah also broke faith in marriage and filled the altar with tears,",
        "asking why heaven did not accept their offerings; the answer named",
        "treachery and the need for a trustworthy covenant witness."
    );
    malachi_2_group_01(story)?;
    malachi_2_group_02(story)?;
    malachi_2_group_03(story)?;
    malachi_2_group_04(story)?;
    Ok(())
}

fn malachi_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Malachi",
        3,
        "A messenger would arrive before the Maintainer, who would enter the",
        "temple like a refiner testing metal and a cleaner scrubbing cloth.",
        "The people were challenged to stop withholding what sustained worship,",
        "and they were promised that faithful remembrance would be logged in a",
        "book for those who honored the Maintainer amid a cynical generation."
    );
    malachi_3_group_01(story)?;
    malachi_3_group_02(story)?;
    malachi_3_group_03(story)?;
    malachi_3_group_04(story)?;
    malachi_3_group_05(story)?;
    Ok(())
}

fn malachi_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Malachi",
        4,
        "A coming day would burn like an overheated build, consuming arrogance",
        "while the faithful received healing like sunlight on newly planted soil.",
        "The book closed with a memory of Moses' instruction and a promise that",
        "Elijah would turn hearts toward one another before the great day, so the",
        "final warning still left room for reconciliation before the repository",
        "reached its irreversible state."
    );
    malachi_4_group_01(story)?;
    malachi_4_group_02(story)?;
    Ok(())
}

fn malachi_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        1,
        1,
        4,
        "lord call -> guard -> expression -> construct lord burden word",
        &["lord", "burden", "word", "israel", "malachi"],
    );
    segment.call(
        1,
        "lord",
        &["burden", "word", "israel"],
        &["lord", "burden", "word", "israel", "malachi"],
    )?;
    segment.guard(
        2,
        "lord",
        &["loved", "saith", "yet"],
        &["lord", "loved", "saith", "yet", "say"],
    )?;
    segment.expression(
        3,
        "hated",
        &["esau", "laid", "mountains"],
        &["hated", "esau", "laid", "mountains", "heritage"],
    )?;
    segment.construct(
        4,
        "whereas",
        &["edom", "saith", "impoverished"],
        &["whereas", "edom", "saith", "impoverished", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        1,
        5,
        8,
        "lord call -> guard lord eyes shall",
        &["lord", "eyes", "shall", "see", "say"],
    );
    segment.call(
        5,
        "lord",
        &["eyes", "shall", "see"],
        &["lord", "eyes", "shall", "see", "say"],
    )?;
    segment.call(
        6,
        "son",
        &["honoureth", "father", "servant"],
        &["son", "honoureth", "father", "servant", "master"],
    )?;
    segment.call(
        7,
        "offer",
        &["polluted", "bread", "mine"],
        &["offer", "polluted", "bread", "mine", "altar"],
    )?;
    segment.guard(
        8,
        "offer",
        &["blind", "sacrifice", "not"],
        &["offer", "blind", "sacrifice", "not", "evil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        1,
        9,
        11,
        "god result -> guard -> trait_bound god now pray",
        &["god", "now", "pray", "beseech", "will"],
    );
    segment.result(
        9,
        "god",
        &["now", "pray", "beseech"],
        &["god", "now", "pray", "beseech", "will"],
    )?;
    segment.guard(
        10,
        "even",
        &["among", "would", "shut"],
        &["even", "among", "would", "shut", "doors"],
    )?;
    segment.trait_bound(
        11,
        "rising",
        &["sun", "even", "unto"],
        &["rising", "sun", "even", "unto", "going"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        1,
        12,
        14,
        "lord call -> move_value -> expression lord profaned say",
        &["lord", "profaned", "say", "table", "polluted"],
    );
    segment.call(
        12,
        "lord",
        &["profaned", "say", "table"],
        &["lord", "profaned", "say", "table", "polluted"],
    )?;
    segment.move_value(
        13,
        "lord",
        &["behold", "weariness", "snuffed"],
        &["lord", "behold", "weariness", "snuffed", "saith"],
    )?;
    segment.expression(
        14,
        "cursed",
        &["deceiver", "hath", "flock"],
        &["cursed", "deceiver", "hath", "flock", "male"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        2,
        1,
        4,
        "now guard -> move_value -> match_value now priests commandment",
        &["now", "priests", "commandment", "will", "not"],
    );
    segment.guard(
        1,
        "now",
        &["priests", "commandment"],
        &["now", "priests", "commandment"],
    )?;
    segment.move_value(
        2,
        "will",
        &["not", "hear", "lay"],
        &["will", "not", "hear", "lay", "heart"],
    )?;
    segment.match_value(
        3,
        "behold",
        &["will", "corrupt", "seed"],
        &["behold", "will", "corrupt", "seed", "spread"],
    )?;
    segment.move_value(
        4,
        "shall",
        &["know", "sent", "commandment"],
        &["shall", "know", "sent", "commandment", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        2,
        5,
        8,
        "covenant return_value -> guard covenant life peace",
        &["covenant", "life", "peace", "gave", "fear"],
    );
    segment.return_value(
        5,
        "covenant",
        &["life", "peace", "gave"],
        &["covenant", "life", "peace", "gave", "fear"],
    )?;
    segment.guard(
        6,
        "law",
        &["truth", "mouth", "iniquity"],
        &["law", "truth", "mouth", "iniquity", "not"],
    )?;
    segment.guard(
        7,
        "priest's",
        &["lips", "keep", "knowledge"],
        &["priest's", "lips", "keep", "knowledge", "seek"],
    )?;
    segment.guard(
        8,
        "departed",
        &["way", "caused", "many"],
        &["departed", "way", "caused", "many", "stumble"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        2,
        9,
        12,
        "people construct -> expression -> trait_bound people therefore made",
        &["people", "therefore", "made", "contemptible", "base"],
    );
    segment.construct(
        9,
        "people",
        &["therefore", "made", "contemptible"],
        &["people", "therefore", "made", "contemptible", "base"],
    )?;
    segment.construct(
        10,
        "god",
        &["not", "father", "hath"],
        &["god", "not", "father", "hath", "created"],
    )?;
    segment.expression(
        11,
        "judah",
        &["hath", "dealt", "treacherously"],
        &["judah", "hath", "dealt", "treacherously", "abomination"],
    )?;
    segment.trait_bound(
        12,
        "lord",
        &["will", "cut", "off"],
        &["lord", "will", "cut", "off", "man"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        2,
        13,
        17,
        "lord guard -> call -> construct lord done again",
        &["lord", "done", "again", "covering", "altar"],
    );
    segment.guard(
        13,
        "lord",
        &["done", "again", "covering"],
        &["lord", "done", "again", "covering", "altar"],
    )?;
    segment.call(
        14,
        "lord",
        &["yet", "say", "wherefore"],
        &["lord", "yet", "say", "wherefore", "hath"],
    )?;
    segment.construct(
        15,
        "spirit",
        &["not", "make", "yet"],
        &["spirit", "not", "make", "yet", "residue"],
    )?;
    segment.guard(
        16,
        "lord",
        &["god", "israel", "saith"],
        &["lord", "god", "israel", "saith", "hateth"],
    )?;
    segment.call(
        17,
        "lord",
        &["wearied", "words", "yet"],
        &["lord", "wearied", "words", "yet", "say"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        3,
        1,
        4,
        "lord move_value -> expression -> trait_bound lord behold will",
        &["lord", "behold", "will", "send", "messenger"],
    );
    segment.move_value(
        1,
        "lord",
        &["behold", "will", "send"],
        &["lord", "behold", "will", "send", "messenger"],
    )?;
    segment.expression(
        2,
        "abide",
        &["day", "coming", "shall"],
        &["abide", "day", "coming", "shall", "stand"],
    )?;
    segment.trait_bound(
        3,
        "shall",
        &["sit", "refiner", "purifier"],
        &["shall", "sit", "refiner", "purifier", "silver"],
    )?;
    segment.trait_bound(
        4,
        "judah",
        &["shall", "offering", "jerusalem"],
        &["judah", "shall", "offering", "jerusalem", "pleasant"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        3,
        5,
        8,
        "will move_value -> guard -> call will near judgment",
        &["will", "near", "judgment", "swift", "witness"],
    );
    segment.move_value(
        5,
        "will",
        &["near", "judgment", "swift"],
        &["will", "near", "judgment", "swift", "witness"],
    )?;
    segment.guard(
        6,
        "lord",
        &["change", "not", "therefore"],
        &["lord", "change", "not", "therefore", "sons"],
    )?;
    segment.move_value(
        7,
        "even",
        &["days", "fathers", "gone"],
        &["even", "days", "fathers", "gone", "away"],
    )?;
    segment.call(
        8,
        "man",
        &["will", "rob", "god"],
        &["man", "will", "rob", "god", "yet"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        3,
        9,
        12,
        "nation expression -> move_value -> drop_value -> return_value nation cursed curse",
        &["nation", "cursed", "curse", "robbed", "even"],
    );
    segment.expression(
        9,
        "nation",
        &["cursed", "curse", "robbed"],
        &["nation", "cursed", "curse", "robbed", "even"],
    )?;
    segment.move_value(
        10,
        "bring",
        &["tithes", "storehouse", "meat"],
        &["bring", "tithes", "storehouse", "meat", "mine"],
    )?;
    segment.drop_value(
        11,
        "will",
        &["rebuke", "devourer", "sakes"],
        &["will", "rebuke", "devourer", "sakes", "shall"],
    )?;
    segment.return_value(
        12,
        "nations",
        &["shall", "call", "blessed"],
        &["nations", "shall", "call", "blessed", "delightsome"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        3,
        13,
        15,
        "lord call -> guard -> expression lord words stout",
        &["lord", "words", "stout", "against", "saith"],
    );
    segment.call(
        13,
        "lord",
        &["words", "stout", "against"],
        &["lord", "words", "stout", "against", "saith"],
    )?;
    segment.guard(
        14,
        "god",
        &["vain", "serve", "profit"],
        &["god", "vain", "serve", "profit", "kept"],
    )?;
    segment.expression(
        15,
        "now",
        &["call", "proud", "happy"],
        &["now", "call", "proud", "happy", "yea"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        3,
        16,
        18,
        "lord call -> construct -> guard lord feared spake",
        &["lord", "feared", "spake", "often", "another"],
    );
    segment.call(
        16,
        "lord",
        &["feared", "spake", "often"],
        &["lord", "feared", "spake", "often", "another"],
    )?;
    segment.construct(
        17,
        "lord",
        &["shall", "mine", "saith"],
        &["lord", "shall", "mine", "saith", "hosts"],
    )?;
    segment.guard(
        18,
        "god",
        &["shall", "return", "discern"],
        &["god", "shall", "return", "discern", "between"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        4,
        1,
        3,
        "behold guard -> move_value -> expression behold day cometh",
        &["behold", "day", "cometh", "shall", "burn"],
    );
    segment.guard(
        1,
        "behold",
        &["day", "cometh", "shall"],
        &["behold", "day", "cometh", "shall", "burn"],
    )?;
    segment.move_value(
        2,
        "unto",
        &["fear", "name", "shall"],
        &["unto", "fear", "name", "shall", "sun"],
    )?;
    segment.expression(
        3,
        "shall",
        &["tread", "down", "wicked"],
        &["shall", "tread", "down", "wicked", "ashes"],
    )?;
    segment.commit()?;
    Ok(())
}

fn malachi_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Malachi,
        4,
        4,
        6,
        "moses guard -> move_value moses remember law",
        &["moses", "remember", "law", "servant", "commanded"],
    );
    segment.guard(
        4,
        "moses",
        &["remember", "law", "servant"],
        &["moses", "remember", "law", "servant", "commanded"],
    )?;
    segment.move_value(
        5,
        "prophet",
        &["behold", "will", "send"],
        &["prophet", "behold", "will", "send", "elijah"],
    )?;
    segment.move_value(
        6,
        "children",
        &["shall", "turn", "heart"],
        &["children", "shall", "turn", "heart", "fathers"],
    )?;
    segment.commit()?;
    Ok(())
}
