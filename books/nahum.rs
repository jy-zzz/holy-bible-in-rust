use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    nahum(story)?;
    Ok(())
}

fn nahum(story: &mut Story) -> Result<(), StoryError> {
    nahum_1(story)?;
    nahum_2(story)?;
    nahum_3(story)?;
    Ok(())
}

fn nahum_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Nahum",
        1,
        "Nahum announced Nineveh's coming deletion, not as a petty revenge commit",
        "but as judgment on an empire built from cruelty and intimidation.",
        "The Maintainer was portrayed as patient yet unmanageable by proud power:",
        "a refuge for those under pressure, and a storm that no fortified city",
        "could out-engineer."
    );
    nahum_1_group_01(story)?;
    nahum_1_group_02(story)?;
    nahum_1_group_03(story)?;
    nahum_1_group_04(story)?;
    Ok(())
}

fn nahum_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Nahum",
        2,
        "The siege arrived in red armor and racing chariots, with breached walls",
        "and palace treasures carried away while the city lost its roar.",
        "Nineveh had once been a den where predators stored what they seized;",
        "now the same machinery of terror turned inward, and the vaults emptied",
        "before the invaders' frantic release."
    );
    nahum_2_group_01(story)?;
    nahum_2_group_02(story)?;
    nahum_2_group_03(story)?;
    Ok(())
}

fn nahum_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Nahum",
        3,
        "The city was called bloody because its commerce, magic, and diplomacy",
        "had made exploitation look like a polished public interface.",
        "No cure could be found for the wound: other nations would clap at the",
        "collapse of a tormentor, and every observer would understand that the",
        "empire's pain was the accumulated output of its own violence."
    );
    nahum_3_group_01(story)?;
    nahum_3_group_02(story)?;
    nahum_3_group_03(story)?;
    nahum_3_group_04(story)?;
    nahum_3_group_05(story)?;
    Ok(())
}

fn nahum_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        1,
        1,
        4,
        "burden match_value -> panic -> guard -> expression burden nineveh book",
        &["burden", "nineveh", "book", "vision", "nahum"],
    );
    segment.match_value(
        1,
        "burden",
        &["nineveh", "book", "vision"],
        &["burden", "nineveh", "book", "vision", "nahum"],
    )?;
    segment.panic(
        2,
        "god",
        &["jealous", "lord", "revengeth"],
        &["god", "jealous", "lord", "revengeth", "furious"],
    )?;
    segment.guard(
        3,
        "lord",
        &["slow", "anger", "great"],
        &["lord", "slow", "anger", "great", "power"],
    )?;
    segment.expression(
        4,
        "rebuketh",
        &["sea", "maketh", "dry"],
        &["rebuketh", "sea", "maketh", "dry", "drieth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        1,
        5,
        8,
        "mountains expression -> await_value -> construct mountains quake hills",
        &["mountains", "quake", "hills", "melt", "earth"],
    );
    segment.expression(
        5,
        "mountains",
        &["quake", "hills", "melt"],
        &["mountains", "quake", "hills", "melt", "earth"],
    )?;
    segment.expression(
        6,
        "stand",
        &["indignation", "abide", "fierceness"],
        &["stand", "indignation", "abide", "fierceness", "anger"],
    )?;
    segment.await_value(
        7,
        "lord",
        &["good", "strong", "hold"],
        &["lord", "good", "strong", "hold", "day"],
    )?;
    segment.construct(
        8,
        "overrunning",
        &["flood", "will", "make"],
        &["overrunning", "flood", "will", "make", "utter"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        1,
        9,
        12,
        "lord construct -> expression -> move_value -> guard lord imagine against",
        &["lord", "imagine", "against", "will", "make"],
    );
    segment.construct(
        9,
        "lord",
        &["imagine", "against", "will"],
        &["lord", "imagine", "against", "will", "make"],
    )?;
    segment.expression(
        10,
        "while",
        &["folden", "together", "thorns"],
        &["while", "folden", "together", "thorns", "drunken"],
    )?;
    segment.move_value(
        11,
        "lord",
        &["thee", "imagineth", "evil"],
        &["lord", "thee", "imagineth", "evil", "against"],
    )?;
    segment.guard(
        12,
        "lord",
        &["thus", "saith", "though"],
        &["lord", "thus", "saith", "though", "quiet"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        1,
        13,
        15,
        "now expression -> construct -> guard now will break",
        &["now", "will", "break", "yoke", "off"],
    );
    segment.expression(
        13,
        "now",
        &["will", "break", "yoke"],
        &["now", "will", "break", "yoke", "off"],
    )?;
    segment.construct(
        14,
        "lord",
        &["hath", "given", "commandment"],
        &["lord", "hath", "given", "commandment", "concerning"],
    )?;
    segment.guard(
        15,
        "behold",
        &["mountains", "feet", "bringeth"],
        &["behold", "mountains", "feet", "bringeth", "good"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        2,
        1,
        4,
        "dasheth construct -> expression dasheth pieces face",
        &["dasheth", "pieces", "face", "keep", "munition"],
    );
    segment.construct(
        1,
        "dasheth",
        &["pieces", "face", "keep"],
        &["dasheth", "pieces", "face", "keep", "munition"],
    )?;
    segment.expression(
        2,
        "lord",
        &["hath", "turned", "away"],
        &["lord", "hath", "turned", "away", "excellency"],
    )?;
    segment.construct(
        3,
        "men",
        &["shield", "mighty", "made"],
        &["men", "shield", "mighty", "made", "red"],
    )?;
    segment.expression(
        4,
        "chariots",
        &["shall", "rage", "streets"],
        &["chariots", "shall", "rage", "streets", "justle"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        2,
        5,
        8,
        "shall construct -> expression -> move_value shall recount worthies",
        &["shall", "recount", "worthies", "stumble", "walk"],
    );
    segment.construct(
        5,
        "shall",
        &["recount", "worthies", "stumble"],
        &["shall", "recount", "worthies", "stumble", "walk"],
    )?;
    segment.expression(
        6,
        "gates",
        &["rivers", "shall", "opened"],
        &["gates", "rivers", "shall", "opened", "palace"],
    )?;
    segment.move_value(
        7,
        "huzzab",
        &["shall", "led", "away"],
        &["huzzab", "shall", "led", "away", "captive"],
    )?;
    segment.move_value(
        8,
        "nineveh",
        &["old", "like", "pool"],
        &["nineveh", "old", "like", "pool", "water"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        2,
        9,
        13,
        "take trait_bound -> expression -> construct -> guard take spoil silver",
        &["take", "spoil", "silver", "gold", "none"],
    );
    segment.trait_bound(
        9,
        "take",
        &["spoil", "silver", "gold"],
        &["take", "spoil", "silver", "gold", "none"],
    )?;
    segment.expression(
        10,
        "empty",
        &["void", "waste", "heart"],
        &["empty", "void", "waste", "heart", "melteth"],
    )?;
    segment.construct(
        11,
        "dwelling",
        &["lions", "feedingplace", "young"],
        &["dwelling", "lions", "feedingplace", "young", "lion"],
    )?;
    segment.expression(
        12,
        "lion",
        &["tear", "pieces", "enough"],
        &["lion", "tear", "pieces", "enough", "whelps"],
    )?;
    segment.guard(
        13,
        "lord",
        &["behold", "against", "thee"],
        &["lord", "behold", "against", "thee", "saith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        3,
        1,
        4,
        "woe guard -> expression -> drop_value woe bloody city",
        &["woe", "bloody", "city", "full", "lies"],
    );
    segment.guard(
        1,
        "woe",
        &["bloody", "city", "full"],
        &["woe", "bloody", "city", "full", "lies"],
    )?;
    segment.expression(
        2,
        "noise",
        &["whip", "rattling", "wheels"],
        &["noise", "whip", "rattling", "wheels", "pransing"],
    )?;
    segment.drop_value(
        3,
        "horseman",
        &["lifteth", "both", "bright"],
        &["horseman", "lifteth", "both", "bright", "sword"],
    )?;
    segment.expression(
        4,
        "nations",
        &["multitude", "whoredoms", "wellfavoured"],
        &[
            "nations",
            "multitude",
            "whoredoms",
            "wellfavoured",
            "harlot",
        ],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        3,
        5,
        8,
        "lord match_value -> construct -> move_value -> guard lord behold against",
        &["lord", "behold", "against", "thee", "saith"],
    );
    segment.match_value(
        5,
        "lord",
        &["behold", "against", "thee"],
        &["lord", "behold", "against", "thee", "saith"],
    )?;
    segment.construct(
        6,
        "will",
        &["cast", "abominable", "filth"],
        &["will", "cast", "abominable", "filth", "thee"],
    )?;
    segment.move_value(
        7,
        "shall",
        &["pass", "look", "thee"],
        &["shall", "pass", "look", "thee", "flee"],
    )?;
    segment.guard(
        8,
        "art",
        &["better", "populous", "situate"],
        &["art", "better", "populous", "situate", "among"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        3,
        9,
        12,
        "ethiopia expression -> move_value -> guard ethiopia egypt strength",
        &["ethiopia", "egypt", "strength", "infinite", "put"],
    );
    segment.expression(
        9,
        "ethiopia",
        &["egypt", "strength", "infinite"],
        &["ethiopia", "egypt", "strength", "infinite", "put"],
    )?;
    segment.move_value(
        10,
        "children",
        &["yet", "carried", "away"],
        &["children", "yet", "carried", "away", "went"],
    )?;
    segment.guard(
        11,
        "shalt",
        &["drunken", "hid", "seek"],
        &["shalt", "drunken", "hid", "seek", "strength"],
    )?;
    segment.expression(
        12,
        "strong",
        &["holds", "shall", "like"],
        &["strong", "holds", "shall", "like", "fig"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        3,
        13,
        16,
        "people match_value -> construct -> expression people behold midst",
        &["people", "behold", "midst", "thee", "women"],
    );
    segment.match_value(
        13,
        "people",
        &["behold", "midst", "thee"],
        &["people", "behold", "midst", "thee", "women"],
    )?;
    segment.construct(
        14,
        "draw",
        &["thee", "waters", "siege"],
        &["draw", "thee", "waters", "siege", "fortify"],
    )?;
    segment.construct(
        15,
        "shall",
        &["fire", "devour", "thee"],
        &["shall", "fire", "devour", "thee", "sword"],
    )?;
    segment.expression(
        16,
        "hast",
        &["multiplied", "merchants", "above"],
        &["hast", "multiplied", "merchants", "above", "stars"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nahum_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nahum,
        3,
        17,
        19,
        "crowned move_value -> guard crowned locusts captains",
        &["crowned", "locusts", "captains", "great", "grasshoppers"],
    );
    segment.move_value(
        17,
        "crowned",
        &["locusts", "captains", "great"],
        &["crowned", "locusts", "captains", "great", "grasshoppers"],
    )?;
    segment.guard(
        18,
        "king",
        &["shepherds", "slumber", "assyria"],
        &["king", "shepherds", "slumber", "assyria", "nobles"],
    )?;
    segment.guard(
        19,
        "healing",
        &["bruise", "wound", "grievous"],
        &["healing", "bruise", "wound", "grievous", "hear"],
    )?;
    segment.commit()?;
    Ok(())
}
