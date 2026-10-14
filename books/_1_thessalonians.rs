use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _1_thessalonians(story)?;
    Ok(())
}

fn _1_thessalonians(story: &mut Story) -> Result<(), StoryError> {
    _1_thessalonians_1(story)?;
    _1_thessalonians_2(story)?;
    _1_thessalonians_3(story)?;
    _1_thessalonians_4(story)?;
    _1_thessalonians_5(story)?;
    Ok(())
}

fn _1_thessalonians_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Thessalonians", 1, "Paul remembers how the Thessalonians received the message with joy amid pressure and became an example whose faith report traveled widely.", "Their conversion meant abandoning idols to serve the living God and waiting for Jesus, the rescuer from the coming judgment.");
    _1_thessalonians_1_group_01(story)?;
    _1_thessalonians_1_group_02(story)?;
    _1_thessalonians_1_group_03(story)?;
    Ok(())
}

fn _1_thessalonians_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Thessalonians", 2, "Paul describes the mission as free from flattery, greed, and status games, combining a mother's tenderness with a father's steady encouragement.", "The Thessalonians accepted the word as God's work, endured local hostility, and became Paul's joy even while separated by repeated travel barriers.");
    _1_thessalonians_2_group_01(story)?;
    _1_thessalonians_2_group_02(story)?;
    _1_thessalonians_2_group_03(story)?;
    _1_thessalonians_2_group_04(story)?;
    _1_thessalonians_2_group_05(story)?;
    Ok(())
}

fn _1_thessalonians_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Thessalonians", 3, "Timothy is sent to strengthen the church under affliction, and his good report revives Paul with news of their faith and love.", "Paul prays that their love overflow toward everyone and that their hearts become blameless and steady before the Lord's arrival.");
    _1_thessalonians_3_group_01(story)?;
    _1_thessalonians_3_group_02(story)?;
    _1_thessalonians_3_group_03(story)?;
    Ok(())
}

fn _1_thessalonians_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Thessalonians", 4, "Paul urges holiness, honorable relationships, quiet work, and practical care, because love should not become an excuse for disorderly dependence.", "He comforts mourners with the promise that the dead in Christ are not lost from the coming gathering, which will reunite the people with the Lord.");
    _1_thessalonians_4_group_01(story)?;
    _1_thessalonians_4_group_02(story)?;
    _1_thessalonians_4_group_03(story)?;
    _1_thessalonians_4_group_04(story)?;
    _1_thessalonians_4_group_05(story)?;
    Ok(())
}

fn _1_thessalonians_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Thessalonians", 5, "The day of the Lord arrives unexpectedly, so believers should stay awake, sober, and dressed in faith, love, and hope rather than panic.", "Paul gives a dense community checklist: respect workers, help the weak, rejoice, pray, test prophecies, hold good, reject evil, and pursue peace.");
    _1_thessalonians_5_group_01(story)?;
    _1_thessalonians_5_group_02(story)?;
    _1_thessalonians_5_group_03(story)?;
    _1_thessalonians_5_group_04(story)?;
    _1_thessalonians_5_group_05(story)?;
    _1_thessalonians_5_group_06(story)?;
    _1_thessalonians_5_group_07(story)?;
    Ok(())
}

fn _1_thessalonians_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        1,
        1,
        4,
        "paul borrow -> expression -> await_value paul silvanus timotheus",
        &["paul", "silvanus", "timotheus", "unto", "church"],
    );
    segment.borrow(
        1,
        "paul",
        &["silvanus", "timotheus", "unto"],
        &["paul", "silvanus", "timotheus", "unto", "church"],
    )?;
    segment.expression(
        2,
        "god",
        &["give", "thanks", "always"],
        &["god", "give", "thanks", "always", "making"],
    )?;
    segment.await_value(
        3,
        "remembering",
        &["without", "ceasing", "work"],
        &["remembering", "without", "ceasing", "work", "faith"],
    )?;
    segment.borrow(
        4,
        "god",
        &["knowing", "brethren", "beloved"],
        &["god", "knowing", "brethren", "beloved", "election"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        1,
        5,
        7,
        "gospel move_value -> call -> await_value gospel not unto",
        &["gospel", "not", "unto", "word", "power"],
    );
    segment.move_value(
        5,
        "gospel",
        &["not", "unto", "word"],
        &["gospel", "not", "unto", "word", "power"],
    )?;
    segment.call(
        6,
        "lord",
        &["became", "followers", "having"],
        &["lord", "became", "followers", "having", "received"],
    )?;
    segment.await_value(
        7,
        "ensamples",
        &["believe", "macedonia", "achaia"],
        &["ensamples", "believe", "macedonia", "achaia"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        1,
        8,
        10,
        "lord guard -> expression -> move_value lord sounded word",
        &["lord", "sounded", "word", "not", "macedonia"],
    );
    segment.guard(
        8,
        "lord",
        &["sounded", "word", "not"],
        &["lord", "sounded", "word", "not", "macedonia"],
    )?;
    segment.expression(
        9,
        "god",
        &["themselves", "shew", "manner"],
        &["god", "themselves", "shew", "manner", "entering"],
    )?;
    segment.move_value(
        10,
        "jesus",
        &["wait", "son", "heaven"],
        &["jesus", "wait", "son", "heaven", "raised"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        2,
        1,
        4,
        "yourselves guard -> call yourselves brethren know",
        &["yourselves", "brethren", "know", "entrance", "unto"],
    );
    segment.guard(
        1,
        "yourselves",
        &["brethren", "know", "entrance"],
        &["yourselves", "brethren", "know", "entrance", "unto"],
    )?;
    segment.call(
        2,
        "god",
        &["even", "suffered", "shamefully"],
        &["god", "even", "suffered", "shamefully", "entreated"],
    )?;
    segment.guard(
        3,
        "exhortation",
        &["not", "deceit", "nor"],
        &["exhortation", "not", "deceit", "nor", "uncleanness"],
    )?;
    segment.guard(
        4,
        "god",
        &["allowed", "put", "trust"],
        &["god", "allowed", "put", "trust", "gospel"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        2,
        5,
        8,
        "neither guard -> expression neither any time",
        &["neither", "any", "time", "used", "flattering"],
    );
    segment.guard(
        5,
        "neither",
        &["any", "time", "used"],
        &["neither", "any", "time", "used", "flattering"],
    )?;
    segment.guard(
        6,
        "men",
        &["nor", "sought", "glory"],
        &["men", "nor", "sought", "glory", "neither"],
    )?;
    segment.expression(
        7,
        "children",
        &["gentle", "among", "even"],
        &["children", "gentle", "among", "even", "nurse"],
    )?;
    segment.guard(
        8,
        "god",
        &["affectionately", "desirous", "willing"],
        &["god", "affectionately", "desirous", "willing", "imparted"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        2,
        9,
        12,
        "remember guard -> await_value -> iterate -> trait_bound remember brethren labour",
        &["remember", "brethren", "labour", "travail", "labouring"],
    );
    segment.guard(
        9,
        "remember",
        &["brethren", "labour", "travail"],
        &["remember", "brethren", "labour", "travail", "labouring"],
    )?;
    segment.await_value(
        10,
        "god",
        &["witnesses", "holily", "justly"],
        &["god", "witnesses", "holily", "justly", "unblameably"],
    )?;
    segment.iterate(
        11,
        "children",
        &["know", "exhorted", "comforted"],
        &["children", "know", "exhorted", "comforted", "charged"],
    )?;
    segment.trait_bound(
        12,
        "god",
        &["would", "walk", "worthy"],
        &["god", "would", "walk", "worthy", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        2,
        13,
        16,
        "god guard -> expression -> drop_value -> move_value god cause thank",
        &["god", "cause", "thank", "without", "ceasing"],
    );
    segment.guard(
        13,
        "god",
        &["cause", "thank", "without"],
        &["god", "cause", "thank", "without", "ceasing"],
    )?;
    segment.expression(
        14,
        "god",
        &["brethren", "became", "followers"],
        &["god", "brethren", "became", "followers", "churches"],
    )?;
    segment.drop_value(
        15,
        "lord",
        &["both", "killed", "jesus"],
        &["lord", "both", "killed", "jesus", "own"],
    )?;
    segment.move_value(
        16,
        "forbidding",
        &["speak", "gentiles", "might"],
        &["forbidding", "speak", "gentiles", "might", "saved"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        2,
        17,
        20,
        "brethren guard -> move_value -> trait_bound brethren taken short",
        &["brethren", "taken", "short", "time", "presence"],
    );
    segment.guard(
        17,
        "brethren",
        &["taken", "short", "time"],
        &["brethren", "taken", "short", "time", "presence"],
    )?;
    segment.move_value(
        18,
        "paul",
        &["wherefore", "would", "unto"],
        &["paul", "wherefore", "would", "unto", "even"],
    )?;
    segment.guard(
        19,
        "lord",
        &["hope", "joy", "crown"],
        &["lord", "hope", "joy", "crown", "rejoicing"],
    )?;
    segment.trait_bound(20, "glory", &["joy"], &["glory", "joy"])?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        3,
        1,
        4,
        "wherefore guard -> move_value wherefore longer forbear",
        &["wherefore", "longer", "forbear", "thought", "good"],
    );
    segment.guard(
        1,
        "wherefore",
        &["longer", "forbear", "thought"],
        &["wherefore", "longer", "forbear", "thought", "good"],
    )?;
    segment.move_value(
        2,
        "god",
        &["sent", "timotheus", "brother"],
        &["god", "sent", "timotheus", "brother", "minister"],
    )?;
    segment.guard(
        3,
        "man",
        &["moved", "afflictions", "yourselves"],
        &["man", "moved", "afflictions", "yourselves", "know"],
    )?;
    segment.move_value(
        4,
        "verily",
        &["told", "suffer", "tribulation"],
        &["verily", "told", "suffer", "tribulation", "even"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        3,
        5,
        8,
        "cause move_value -> await_value -> expression cause longer forbear",
        &["cause", "longer", "forbear", "sent", "know"],
    );
    segment.move_value(
        5,
        "cause",
        &["longer", "forbear", "sent"],
        &["cause", "longer", "forbear", "sent", "know"],
    )?;
    segment.move_value(
        6,
        "now",
        &["timotheus", "unto", "brought"],
        &["now", "timotheus", "unto", "brought", "good"],
    )?;
    segment.await_value(
        7,
        "therefore",
        &["brethren", "comforted", "over"],
        &["therefore", "brethren", "comforted", "over", "affliction"],
    )?;
    segment.expression(
        8,
        "lord",
        &["now", "live", "stand"],
        &["lord", "now", "live", "stand", "fast"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        3,
        9,
        13,
        "god expression -> await_value -> construct god thanks render",
        &["god", "thanks", "render", "again", "joy"],
    );
    segment.expression(
        9,
        "god",
        &["thanks", "render", "again"],
        &["god", "thanks", "render", "again", "joy"],
    )?;
    segment.await_value(
        10,
        "night",
        &["day", "praying", "exceedingly"],
        &["night", "day", "praying", "exceedingly", "might"],
    )?;
    segment.expression(
        11,
        "god",
        &["now", "himself", "father"],
        &["god", "now", "himself", "father", "lord"],
    )?;
    segment.construct(
        12,
        "lord",
        &["make", "increase", "abound"],
        &["lord", "make", "increase", "abound", "love"],
    )?;
    segment.expression(
        13,
        "god",
        &["end", "stablish", "hearts"],
        &["god", "end", "stablish", "hearts", "unblameable"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        4,
        1,
        4,
        "lord expression -> iterate lord furthermore beseech",
        &["lord", "furthermore", "beseech", "brethren", "exhort"],
    );
    segment.expression(
        1,
        "lord",
        &["furthermore", "beseech", "brethren"],
        &["lord", "furthermore", "beseech", "brethren", "exhort"],
    )?;
    segment.expression(
        2,
        "lord",
        &["know", "commandments", "gave"],
        &["lord", "know", "commandments", "gave", "jesus"],
    )?;
    segment.expression(
        3,
        "god",
        &["will", "even", "sanctification"],
        &["god", "will", "even", "sanctification", "abstain"],
    )?;
    segment.iterate(
        4,
        "every",
        &["know", "possess", "vessel"],
        &["every", "know", "possess", "vessel", "sanctification"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        4,
        5,
        8,
        "god guard -> move_value god not lust",
        &["god", "not", "lust", "concupiscence", "even"],
    );
    segment.guard(
        5,
        "god",
        &["not", "lust", "concupiscence"],
        &["god", "not", "lust", "concupiscence", "even"],
    )?;
    segment.move_value(
        6,
        "man",
        &["beyond", "defraud", "brother"],
        &["man", "beyond", "defraud", "brother", "any"],
    )?;
    segment.guard(
        7,
        "god",
        &["hath", "not", "called"],
        &["god", "hath", "not", "called", "unto"],
    )?;
    segment.guard(
        8,
        "man",
        &["therefore", "despiseth", "not"],
        &["man", "therefore", "despiseth", "not", "god"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        4,
        9,
        12,
        "touching guard -> expression touching brotherly love",
        &["touching", "brotherly", "love", "need", "not"],
    );
    segment.guard(
        9,
        "touching",
        &["brotherly", "love", "need"],
        &["touching", "brotherly", "love", "need", "not"],
    )?;
    segment.expression(
        10,
        "indeed",
        &["toward", "brethren", "macedonia"],
        &["indeed", "toward", "brethren", "macedonia", "beseech"],
    )?;
    segment.expression(
        11,
        "study",
        &["quiet", "own", "business"],
        &["study", "quiet", "own", "business", "work"],
    )?;
    segment.expression(
        12,
        "walk",
        &["honestly", "toward", "without"],
        &["walk", "honestly", "toward", "without", "lack"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        4,
        13,
        15,
        "would guard -> move_value would not ignorant",
        &["would", "not", "ignorant", "brethren", "concerning"],
    );
    segment.guard(
        13,
        "would",
        &["not", "ignorant", "brethren"],
        &["would", "not", "ignorant", "brethren", "concerning"],
    )?;
    segment.move_value(
        14,
        "jesus",
        &["believe", "died", "rose"],
        &["jesus", "believe", "died", "rose", "again"],
    )?;
    segment.guard(
        15,
        "lord",
        &["say", "unto", "word"],
        &["lord", "say", "unto", "word", "alive"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        4,
        16,
        18,
        "lord drop_value -> expression lord himself shall",
        &["lord", "himself", "shall", "descend", "heaven"],
    );
    segment.drop_value(
        16,
        "lord",
        &["himself", "shall", "descend"],
        &["lord", "himself", "shall", "descend", "heaven"],
    )?;
    segment.expression(
        17,
        "lord",
        &["alive", "remain", "shall"],
        &["lord", "alive", "remain", "shall", "caught"],
    )?;
    segment.expression(
        18,
        "wherefore",
        &["comfort", "another", "words"],
        &["wherefore", "comfort", "another", "words"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        5,
        1,
        4,
        "times guard -> expression times seasons brethren",
        &["times", "seasons", "brethren", "need", "write"],
    );
    segment.guard(
        1,
        "times",
        &["seasons", "brethren", "need"],
        &["times", "seasons", "brethren", "need", "write"],
    )?;
    segment.expression(
        2,
        "lord",
        &["yourselves", "know", "perfectly"],
        &["lord", "yourselves", "know", "perfectly", "day"],
    )?;
    segment.guard(
        3,
        "shall",
        &["say", "peace", "safety"],
        &["shall", "say", "peace", "safety", "sudden"],
    )?;
    segment.guard(
        4,
        "brethren",
        &["not", "darkness", "day"],
        &["brethren", "not", "darkness", "day", "overtake"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        5,
        5,
        8,
        "children guard -> expression -> await_value children light day",
        &["children", "light", "day", "not", "night"],
    );
    segment.guard(
        5,
        "children",
        &["light", "day", "not"],
        &["children", "light", "day", "not", "night"],
    )?;
    segment.guard(
        6,
        "therefore",
        &["not", "sleep", "others"],
        &["therefore", "not", "sleep", "others", "watch"],
    )?;
    segment.expression(
        7,
        "sleep",
        &["night", "drunken"],
        &["sleep", "night", "drunken"],
    )?;
    segment.await_value(
        8,
        "day",
        &["sober", "putting", "breastplate"],
        &["day", "sober", "putting", "breastplate", "faith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        5,
        9,
        12,
        "god guard -> drop_value -> expression god hath not",
        &["god", "hath", "not", "appointed", "wrath"],
    );
    segment.guard(
        9,
        "god",
        &["hath", "not", "appointed"],
        &["god", "hath", "not", "appointed", "wrath"],
    )?;
    segment.drop_value(
        10,
        "died",
        &["whether", "wake", "sleep"],
        &["died", "whether", "wake", "sleep", "live"],
    )?;
    segment.expression(
        11,
        "wherefore",
        &["comfort", "yourselves", "together"],
        &["wherefore", "comfort", "yourselves", "together", "edify"],
    )?;
    segment.expression(
        12,
        "lord",
        &["beseech", "brethren", "know"],
        &["lord", "beseech", "brethren", "know", "labour"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        5,
        13,
        16,
        "esteem borrow -> expression esteem very highly",
        &["esteem", "very", "highly", "love", "work's"],
    );
    segment.borrow(
        13,
        "esteem",
        &["very", "highly", "love"],
        &["esteem", "very", "highly", "love", "work's"],
    )?;
    segment.expression(
        14,
        "now",
        &["exhort", "brethren", "warn"],
        &["now", "exhort", "brethren", "warn", "unruly"],
    )?;
    segment.expression(
        15,
        "man",
        &["see", "none", "render"],
        &["man", "see", "none", "render", "evil"],
    )?;
    segment.expression(16, "rejoice", &["evermore"], &["rejoice", "evermore"])?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        5,
        17,
        20,
        "pray result -> iterate -> guard pray without ceasing",
        &["pray", "without", "ceasing", "god", "every"],
    );
    segment.result(
        17,
        "pray",
        &["without", "ceasing"],
        &["pray", "without", "ceasing"],
    )?;
    segment.iterate(
        18,
        "god",
        &["every", "thing", "give"],
        &["god", "every", "thing", "give", "thanks"],
    )?;
    segment.guard(
        19,
        "spirit",
        &["quench", "not"],
        &["spirit", "quench", "not"],
    )?;
    segment.guard(
        20,
        "despise",
        &["not", "prophesyings"],
        &["despise", "not", "prophesyings"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_5_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        5,
        21,
        24,
        "prove expression -> result prove things hold",
        &["prove", "things", "hold", "fast", "good"],
    );
    segment.expression(
        21,
        "prove",
        &["things", "hold", "fast"],
        &["prove", "things", "hold", "fast", "good"],
    )?;
    segment.expression(
        22,
        "abstain",
        &["appearance", "evil"],
        &["abstain", "appearance", "evil"],
    )?;
    segment.result(
        23,
        "god",
        &["very", "peace", "sanctify"],
        &["god", "very", "peace", "sanctify", "wholly"],
    )?;
    segment.expression(
        24,
        "faithful",
        &["calleth", "will"],
        &["faithful", "calleth", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_thessalonians_5_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Thessalonians,
        5,
        25,
        28,
        "brethren result -> expression -> borrow brethren pray greet",
        &["brethren", "pray", "greet", "holy", "kiss"],
    );
    segment.result(25, "brethren", &["pray"], &["brethren", "pray"])?;
    segment.expression(
        26,
        "greet",
        &["brethren", "holy", "kiss"],
        &["greet", "brethren", "holy", "kiss"],
    )?;
    segment.expression(
        27,
        "lord",
        &["charge", "epistle", "read"],
        &["lord", "charge", "epistle", "read", "unto"],
    )?;
    segment.borrow(
        28,
        "lord",
        &["grace", "jesus", "christ"],
        &["lord", "grace", "jesus", "christ", "amen"],
    )?;
    segment.commit()?;
    Ok(())
}
