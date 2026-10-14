use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    james(story)?;
    Ok(())
}

fn james(story: &mut Story) -> Result<(), StoryError> {
    james_1(story)?;
    james_2(story)?;
    james_3(story)?;
    james_4(story)?;
    james_5(story)?;
    Ok(())
}

fn james_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "James",
        1,
        "Trials were recast as a build matrix that can produce endurance when the",
        "team asks for wisdom instead of blaming the compiler.",
        "The poor and rich were both reminded that status is temporary metadata,",
        "while hearers became complete only when they implemented the word through",
        "action rather than admiring it in a read-only pane."
    );
    james_1_group_01(story)?;
    james_1_group_02(story)?;
    james_1_group_03(story)?;
    james_1_group_04(story)?;
    james_1_group_05(story)?;
    james_1_group_06(story)?;
    james_1_group_07(story)?;
    Ok(())
}

fn james_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "James",
        2,
        "Favoritism was a broken permission policy: seating the wealthy up front",
        "while humiliating the poor violated the royal command to love a neighbor.",
        "Faith without works was compared to a function signature with no body,",
        "because trust that never feeds, clothes, or helps anyone has not reached",
        "executable compassion."
    );
    james_2_group_01(story)?;
    james_2_group_02(story)?;
    james_2_group_03(story)?;
    james_2_group_04(story)?;
    james_2_group_05(story)?;
    james_2_group_06(story)?;
    james_2_group_07(story)?;
    Ok(())
}

fn james_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "James",
        3,
        "The tongue was a tiny control surface capable of steering a whole system",
        "or igniting a forest of incidents.",
        "Wisdom from above was recognizable by gentleness, mercy, impartiality, and",
        "good fruit, unlike the bitter branch where envy and rivalry generated",
        "disorder in every dependent module."
    );
    james_3_group_01(story)?;
    james_3_group_02(story)?;
    james_3_group_03(story)?;
    james_3_group_04(story)?;
    james_3_group_05(story)?;
    Ok(())
}

fn james_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "James",
        4,
        "Conflicts came from cravings that fought for ownership inside the users,",
        "producing requests, envy, and even violence when desire met a closed gate.",
        "The letter called the church to resist the adversary, draw near to God,",
        "and speak about tomorrow humbly, since every schedule is only a vapor."
    );
    james_4_group_01(story)?;
    james_4_group_02(story)?;
    james_4_group_03(story)?;
    james_4_group_04(story)?;
    Ok(())
}

fn james_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "James",
        5,
        "Wealth hoarded by exploitative owners became evidence in the final audit:",
        "withheld wages cried out while luxury continued on a system built from",
        "another person's labor.",
        "The suffering community was told to wait like a farmer for rain, pray",
        "through every state, confess faults, and bring a wandering sibling back",
        "from a destructive branch."
    );
    james_5_group_01(story)?;
    james_5_group_02(story)?;
    james_5_group_03(story)?;
    james_5_group_04(story)?;
    james_5_group_05(story)?;
    Ok(())
}

fn james_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        1,
        1,
        4,
        "god iterate -> expression -> await_value god james servant",
        &["god", "james", "servant", "lord", "jesus"],
    );
    segment.iterate(
        1,
        "god",
        &["james", "servant", "lord"],
        &["god", "james", "servant", "lord", "jesus"],
    )?;
    segment.expression(
        2,
        "brethren",
        &["count", "joy", "fall"],
        &["brethren", "count", "joy", "fall", "divers"],
    )?;
    segment.await_value(
        3,
        "knowing",
        &["trying", "faith", "worketh"],
        &["knowing", "trying", "faith", "worketh", "patience"],
    )?;
    segment.expression(
        4,
        "patience",
        &["perfect", "work", "entire"],
        &["patience", "perfect", "work", "entire", "wanting"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        1,
        5,
        8,
        "god guard -> result -> expression god any lack",
        &["god", "any", "lack", "wisdom", "ask"],
    );
    segment.guard(
        5,
        "god",
        &["any", "lack", "wisdom"],
        &["god", "any", "lack", "wisdom", "ask"],
    )?;
    segment.result(
        6,
        "ask",
        &["faith", "nothing", "wavering"],
        &["ask", "faith", "nothing", "wavering", "wavereth"],
    )?;
    segment.guard(
        7,
        "man",
        &["not", "think", "shall"],
        &["man", "not", "think", "shall", "receive"],
    )?;
    segment.expression(
        8,
        "man",
        &["double", "minded", "unstable"],
        &["man", "double", "minded", "unstable", "ways"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        1,
        9,
        12,
        "brother expression -> construct -> reinitialize -> borrow brother low degree",
        &["brother", "low", "degree", "rejoice", "exalted"],
    );
    segment.expression(
        9,
        "brother",
        &["low", "degree", "rejoice"],
        &["brother", "low", "degree", "rejoice", "exalted"],
    )?;
    segment.construct(
        10,
        "rich",
        &["made", "low", "flower"],
        &["rich", "made", "low", "flower", "grass"],
    )?;
    segment.reinitialize(
        11,
        "sun",
        &["sooner", "risen", "burning"],
        &["sun", "sooner", "risen", "burning", "heat"],
    )?;
    segment.borrow(
        12,
        "man",
        &["blessed", "endureth", "temptation"],
        &["man", "blessed", "endureth", "temptation", "tried"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        1,
        13,
        16,
        "man guard -> iterate -> drop_value man say tempted",
        &["man", "say", "tempted", "god", "cannot"],
    );
    segment.guard(
        13,
        "man",
        &["say", "tempted", "god"],
        &["man", "say", "tempted", "god", "cannot"],
    )?;
    segment.iterate(
        14,
        "man",
        &["every", "tempted", "drawn"],
        &["man", "every", "tempted", "drawn", "away"],
    )?;
    segment.drop_value(
        15,
        "lust",
        &["hath", "conceived", "bringeth"],
        &["lust", "hath", "conceived", "bringeth", "forth"],
    )?;
    segment.guard(
        16,
        "not",
        &["err", "beloved", "brethren"],
        &["not", "err", "beloved", "brethren"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        1,
        17,
        20,
        "every guard -> construct -> call every good gift",
        &["every", "good", "gift", "perfect", "above"],
    );
    segment.guard(
        17,
        "every",
        &["good", "gift", "perfect"],
        &["every", "good", "gift", "perfect", "above"],
    )?;
    segment.construct(
        18,
        "own",
        &["will", "begat", "word"],
        &["own", "will", "begat", "word", "truth"],
    )?;
    segment.call(
        19,
        "man",
        &["wherefore", "beloved", "brethren"],
        &["man", "wherefore", "beloved", "brethren", "every"],
    )?;
    segment.guard(
        20,
        "man",
        &["wrath", "worketh", "not"],
        &["man", "wrath", "worketh", "not", "righteousness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        1,
        21,
        24,
        "wherefore call -> guard -> expression wherefore lay apart",
        &["wherefore", "lay", "apart", "filthiness", "superfluity"],
    );
    segment.call(
        21,
        "wherefore",
        &["lay", "apart", "filthiness"],
        &["wherefore", "lay", "apart", "filthiness", "superfluity"],
    )?;
    segment.guard(
        22,
        "doers",
        &["word", "not", "hearers"],
        &["doers", "word", "not", "hearers", "deceiving"],
    )?;
    segment.guard(
        23,
        "man",
        &["any", "hearer", "word"],
        &["man", "any", "hearer", "word", "not"],
    )?;
    segment.expression(
        24,
        "man",
        &["beholdeth", "himself", "goeth"],
        &["man", "beholdeth", "himself", "goeth", "way"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_1_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        1,
        25,
        27,
        "whoso guard whoso looketh perfect",
        &["whoso", "looketh", "perfect", "law", "liberty"],
    );
    segment.guard(
        25,
        "whoso",
        &["looketh", "perfect", "law"],
        &["whoso", "looketh", "perfect", "law", "liberty"],
    )?;
    segment.guard(
        26,
        "man",
        &["any", "among", "seem"],
        &["man", "any", "among", "seem", "religious"],
    )?;
    segment.guard(
        27,
        "god",
        &["pure", "religion", "undefiled"],
        &["god", "pure", "religion", "undefiled", "father"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        2,
        1,
        4,
        "lord guard -> move_value -> call lord brethren not",
        &["lord", "brethren", "not", "faith", "jesus"],
    );
    segment.guard(
        1,
        "lord",
        &["brethren", "not", "faith"],
        &["lord", "brethren", "not", "faith", "jesus"],
    )?;
    segment.move_value(
        2,
        "man",
        &["unto", "assembly", "gold"],
        &["man", "unto", "assembly", "gold", "ring"],
    )?;
    segment.call(
        3,
        "respect",
        &["weareth", "gay", "clothing"],
        &["respect", "weareth", "gay", "clothing", "say"],
    )?;
    segment.guard(
        4,
        "not",
        &["partial", "yourselves", "become"],
        &["not", "partial", "yourselves", "become", "judges"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        2,
        5,
        8,
        "god guard god hearken beloved",
        &["god", "hearken", "beloved", "brethren", "hath"],
    );
    segment.guard(
        5,
        "god",
        &["hearken", "beloved", "brethren"],
        &["god", "hearken", "beloved", "brethren", "hath"],
    )?;
    segment.guard(
        6,
        "men",
        &["despised", "poor", "not"],
        &["men", "despised", "poor", "not", "rich"],
    )?;
    segment.guard(
        7,
        "not",
        &["blaspheme", "worthy", "name"],
        &["not", "blaspheme", "worthy", "name", "called"],
    )?;
    segment.guard(
        8,
        "fulfil",
        &["royal", "law", "according"],
        &["fulfil", "royal", "law", "according", "scripture"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        2,
        9,
        12,
        "respect guard -> drop_value respect persons commit",
        &["respect", "persons", "commit", "sin", "convinced"],
    );
    segment.guard(
        9,
        "respect",
        &["persons", "commit", "sin"],
        &["respect", "persons", "commit", "sin", "convinced"],
    )?;
    segment.guard(
        10,
        "whosoever",
        &["shall", "keep", "whole"],
        &["whosoever", "shall", "keep", "whole", "law"],
    )?;
    segment.drop_value(
        11,
        "not",
        &["commit", "adultery", "kill"],
        &["not", "commit", "adultery", "kill", "now"],
    )?;
    segment.guard(
        12,
        "speak",
        &["shall", "judged", "law"],
        &["speak", "shall", "judged", "law", "liberty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        2,
        13,
        16,
        "shall guard -> expression -> move_value shall judgment without",
        &["shall", "judgment", "without", "mercy", "hath"],
    );
    segment.guard(
        13,
        "shall",
        &["judgment", "without", "mercy"],
        &["shall", "judgment", "without", "mercy", "hath"],
    )?;
    segment.guard(
        14,
        "man",
        &["doth", "profit", "brethren"],
        &["man", "doth", "profit", "brethren", "though"],
    )?;
    segment.expression(
        15,
        "brother",
        &["sister", "naked", "destitute"],
        &["brother", "sister", "naked", "destitute", "daily"],
    )?;
    segment.move_value(
        16,
        "say",
        &["unto", "depart", "peace"],
        &["say", "unto", "depart", "peace", "warmed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        2,
        17,
        20,
        "even drop_value -> call -> await_value even faith hath",
        &["even", "faith", "hath", "not", "works"],
    );
    segment.drop_value(
        17,
        "even",
        &["faith", "hath", "not"],
        &["even", "faith", "hath", "not", "works"],
    )?;
    segment.call(
        18,
        "man",
        &["yea", "say", "hast"],
        &["man", "yea", "say", "hast", "faith"],
    )?;
    segment.await_value(
        19,
        "god",
        &["believest", "doest", "well"],
        &["god", "believest", "doest", "well", "devils"],
    )?;
    segment.drop_value(
        20,
        "man",
        &["wilt", "know", "vain"],
        &["man", "wilt", "know", "vain", "faith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        2,
        21,
        23,
        "abraham guard -> construct -> await_value abraham not father",
        &["abraham", "not", "father", "justified", "works"],
    );
    segment.guard(
        21,
        "abraham",
        &["not", "father", "justified"],
        &["abraham", "not", "father", "justified", "works"],
    )?;
    segment.construct(
        22,
        "seest",
        &["faith", "wrought", "works"],
        &["seest", "faith", "wrought", "works", "made"],
    )?;
    segment.await_value(
        23,
        "abraham",
        &["scripture", "fulfilled", "saith"],
        &["abraham", "scripture", "fulfilled", "saith", "believed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        2,
        24,
        26,
        "man guard -> move_value -> drop_value man see works",
        &["man", "see", "works", "justified", "not"],
    );
    segment.guard(
        24,
        "man",
        &["see", "works", "justified"],
        &["man", "see", "works", "justified", "not"],
    )?;
    segment.move_value(
        25,
        "likewise",
        &["not", "rahab", "harlot"],
        &["likewise", "not", "rahab", "harlot", "justified"],
    )?;
    segment.drop_value(
        26,
        "spirit",
        &["body", "without", "dead"],
        &["spirit", "body", "without", "dead", "faith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        3,
        1,
        4,
        "brethren guard -> match_value brethren not many",
        &["brethren", "not", "many", "masters", "knowing"],
    );
    segment.guard(
        1,
        "brethren",
        &["not", "many", "masters"],
        &["brethren", "not", "many", "masters", "knowing"],
    )?;
    segment.guard(
        2,
        "man",
        &["many", "things", "offend"],
        &["man", "many", "things", "offend", "any"],
    )?;
    segment.match_value(
        3,
        "behold",
        &["put", "bits", "horses"],
        &["behold", "put", "bits", "horses", "mouths"],
    )?;
    segment.match_value(
        4,
        "behold",
        &["ships", "though", "great"],
        &["behold", "ships", "though", "great", "driven"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        3,
        5,
        8,
        "even match_value -> expression -> iterate -> guard even tongue little",
        &["even", "tongue", "little", "member", "boasteth"],
    );
    segment.match_value(
        5,
        "even",
        &["tongue", "little", "member"],
        &["even", "tongue", "little", "member", "boasteth"],
    )?;
    segment.expression(
        6,
        "tongue",
        &["fire", "world", "iniquity"],
        &["tongue", "fire", "world", "iniquity", "among"],
    )?;
    segment.iterate(
        7,
        "every",
        &["kind", "beasts", "birds"],
        &["every", "kind", "beasts", "birds", "serpents"],
    )?;
    segment.guard(
        8,
        "man",
        &["tongue", "tame", "unruly"],
        &["man", "tongue", "tame", "unruly", "evil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        3,
        9,
        12,
        "god construct -> guard -> move_value god therewith bless",
        &["god", "therewith", "bless", "even", "father"],
    );
    segment.construct(
        9,
        "god",
        &["therewith", "bless", "even"],
        &["god", "therewith", "bless", "even", "father"],
    )?;
    segment.guard(
        10,
        "same",
        &["mouth", "proceedeth", "blessing"],
        &["same", "mouth", "proceedeth", "blessing", "cursing"],
    )?;
    segment.move_value(
        11,
        "doth",
        &["fountain", "send", "forth"],
        &["doth", "fountain", "send", "forth", "same"],
    )?;
    segment.guard(
        12,
        "fig",
        &["tree", "brethren", "bear"],
        &["fig", "tree", "brethren", "bear", "olive"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        3,
        13,
        15,
        "man expression -> guard man wise endued",
        &["man", "wise", "endued", "knowledge", "among"],
    );
    segment.expression(
        13,
        "man",
        &["wise", "endued", "knowledge"],
        &["man", "wise", "endued", "knowledge", "among"],
    )?;
    segment.guard(
        14,
        "bitter",
        &["envying", "strife", "hearts"],
        &["bitter", "envying", "strife", "hearts", "glory"],
    )?;
    segment.guard(
        15,
        "wisdom",
        &["descendeth", "not", "above"],
        &["wisdom", "descendeth", "not", "above", "earthly"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        3,
        16,
        18,
        "envying iterate -> borrow -> construct envying strife confusion",
        &["envying", "strife", "confusion", "every", "evil"],
    );
    segment.iterate(
        16,
        "envying",
        &["strife", "confusion", "every"],
        &["envying", "strife", "confusion", "every", "evil"],
    )?;
    segment.borrow(
        17,
        "wisdom",
        &["above", "first", "pure"],
        &["wisdom", "above", "first", "pure", "peaceable"],
    )?;
    segment.construct(
        18,
        "fruit",
        &["righteousness", "sown", "peace"],
        &["fruit", "righteousness", "sown", "peace", "make"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        4,
        1,
        4,
        "whence move_value -> drop_value -> guard whence wars fightings",
        &["whence", "wars", "fightings", "among", "not"],
    );
    segment.move_value(
        1,
        "whence",
        &["wars", "fightings", "among"],
        &["whence", "wars", "fightings", "among", "not"],
    )?;
    segment.drop_value(
        2,
        "lust",
        &["not", "kill", "desire"],
        &["lust", "not", "kill", "desire", "cannot"],
    )?;
    segment.guard(
        3,
        "ask",
        &["receive", "not", "amiss"],
        &["ask", "receive", "not", "amiss", "consume"],
    )?;
    segment.guard(
        4,
        "god",
        &["adulterers", "adulteresses", "know"],
        &["god", "adulterers", "adulteresses", "know", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        4,
        5,
        8,
        "spirit expression -> borrow -> move_value spirit think scripture",
        &["spirit", "think", "scripture", "saith", "vain"],
    );
    segment.expression(
        5,
        "spirit",
        &["think", "scripture", "saith"],
        &["spirit", "think", "scripture", "saith", "vain"],
    )?;
    segment.borrow(
        6,
        "god",
        &["giveth", "grace", "wherefore"],
        &["god", "giveth", "grace", "wherefore", "saith"],
    )?;
    segment.move_value(
        7,
        "god",
        &["submit", "yourselves", "therefore"],
        &["god", "submit", "yourselves", "therefore", "resist"],
    )?;
    segment.expression(
        8,
        "god",
        &["draw", "nigh", "will"],
        &["god", "draw", "nigh", "will", "cleanse"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        4,
        9,
        12,
        "afflicted result -> expression -> guard -> drop_value afflicted mourn weep",
        &["afflicted", "mourn", "weep", "laughter", "turned"],
    );
    segment.result(
        9,
        "afflicted",
        &["mourn", "weep", "laughter"],
        &["afflicted", "mourn", "weep", "laughter", "turned"],
    )?;
    segment.expression(
        10,
        "lord",
        &["humble", "yourselves", "sight"],
        &["lord", "humble", "yourselves", "sight", "shall"],
    )?;
    segment.guard(
        11,
        "speak",
        &["not", "evil", "another"],
        &["speak", "not", "evil", "another", "brethren"],
    )?;
    segment.drop_value(
        12,
        "lawgiver",
        &["able", "save", "destroy"],
        &["lawgiver", "able", "save", "destroy", "art"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        4,
        13,
        17,
        "now move_value -> guard -> call -> expression now say day",
        &["now", "say", "day", "morrow", "will"],
    );
    segment.move_value(
        13,
        "now",
        &["say", "day", "morrow"],
        &["now", "say", "day", "morrow", "will"],
    )?;
    segment.guard(
        14,
        "whereas",
        &["know", "not", "shall"],
        &["whereas", "know", "not", "shall", "morrow"],
    )?;
    segment.call(
        15,
        "lord",
        &["ought", "say", "will"],
        &["lord", "ought", "say", "will", "shall"],
    )?;
    segment.expression(
        16,
        "now",
        &["rejoice", "boastings", "such"],
        &["now", "rejoice", "boastings", "such", "rejoicing"],
    )?;
    segment.guard(
        17,
        "therefore",
        &["knoweth", "good", "doeth"],
        &["therefore", "knoweth", "good", "doeth", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        5,
        1,
        4,
        "men move_value -> expression -> match_value men now rich",
        &["men", "now", "rich", "weep", "howl"],
    );
    segment.move_value(
        1,
        "men",
        &["now", "rich", "weep"],
        &["men", "now", "rich", "weep", "howl"],
    )?;
    segment.expression(
        2,
        "riches",
        &["corrupted", "garments", "motheaten"],
        &["riches", "corrupted", "garments", "motheaten"],
    )?;
    segment.expression(
        3,
        "gold",
        &["silver", "cankered", "rust"],
        &["gold", "silver", "cankered", "rust", "shall"],
    )?;
    segment.match_value(
        4,
        "behold",
        &["hire", "labourers", "reaped"],
        &["behold", "hire", "labourers", "reaped", "down"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        5,
        5,
        8,
        "lived expression -> drop_value -> match_value lived pleasure earth",
        &["lived", "pleasure", "earth", "wanton", "nourished"],
    );
    segment.expression(
        5,
        "lived",
        &["pleasure", "earth", "wanton"],
        &["lived", "pleasure", "earth", "wanton", "nourished"],
    )?;
    segment.drop_value(
        6,
        "condemned",
        &["killed", "just", "doth"],
        &["condemned", "killed", "just", "doth", "not"],
    )?;
    segment.match_value(
        7,
        "lord",
        &["patient", "therefore", "brethren"],
        &["lord", "patient", "therefore", "brethren", "unto"],
    )?;
    segment.expression(
        8,
        "lord",
        &["patient", "stablish", "hearts"],
        &["lord", "patient", "stablish", "hearts", "coming"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        5,
        9,
        12,
        "grudge guard -> expression -> match_value grudge not against",
        &["grudge", "not", "against", "another", "brethren"],
    );
    segment.guard(
        9,
        "grudge",
        &["not", "against", "another"],
        &["grudge", "not", "against", "another", "brethren"],
    )?;
    segment.expression(
        10,
        "lord",
        &["take", "brethren", "prophets"],
        &["lord", "take", "brethren", "prophets", "spoken"],
    )?;
    segment.match_value(
        11,
        "behold",
        &["count", "happy", "endure"],
        &["behold", "count", "happy", "endure", "heard"],
    )?;
    segment.guard(
        12,
        "above",
        &["things", "brethren", "swear"],
        &["above", "things", "brethren", "swear", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        5,
        13,
        16,
        "any result any among afflicted",
        &["any", "among", "afflicted", "pray", "merry"],
    );
    segment.result(
        13,
        "any",
        &["among", "afflicted", "pray"],
        &["any", "among", "afflicted", "pray", "merry"],
    )?;
    segment.result(
        14,
        "any",
        &["sick", "among", "call"],
        &["any", "sick", "among", "call", "elders"],
    )?;
    segment.result(
        15,
        "lord",
        &["prayer", "faith", "shall"],
        &["lord", "prayer", "faith", "shall", "save"],
    )?;
    segment.result(
        16,
        "confess",
        &["faults", "another", "pray"],
        &["confess", "faults", "another", "pray", "healed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn james_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::James,
        5,
        17,
        20,
        "man guard -> move_value -> expression -> drop_value man elias subject",
        &["man", "elias", "subject", "like", "passions"],
    );
    segment.guard(
        17,
        "man",
        &["elias", "subject", "like"],
        &["man", "elias", "subject", "like", "passions"],
    )?;
    segment.move_value(
        18,
        "prayed",
        &["again", "heaven", "gave"],
        &["prayed", "again", "heaven", "gave", "rain"],
    )?;
    segment.expression(
        19,
        "brethren",
        &["any", "err", "truth"],
        &["brethren", "any", "err", "truth", "convert"],
    )?;
    segment.drop_value(
        20,
        "know",
        &["converteth", "sinner", "error"],
        &["know", "converteth", "sinner", "error", "way"],
    )?;
    segment.commit()?;
    Ok(())
}
