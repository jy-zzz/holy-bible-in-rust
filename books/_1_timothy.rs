use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _1_timothy(story)?;
    Ok(())
}

fn _1_timothy(story: &mut Story) -> Result<(), StoryError> {
    _1_timothy_1(story)?;
    _1_timothy_2(story)?;
    _1_timothy_3(story)?;
    _1_timothy_4(story)?;
    _1_timothy_5(story)?;
    _1_timothy_6(story)?;
    Ok(())
}

fn _1_timothy_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Timothy", 1, "Paul charges Timothy to stop teachers from drifting into speculative myths and endless genealogical debates that generate noise instead of faithful stewardship.", "The law exposes wrongdoing, while mercy appoints a former persecutor as a living demonstration that Christ came to save sinners.");
    _1_timothy_1_group_01(story)?;
    _1_timothy_1_group_02(story)?;
    _1_timothy_1_group_03(story)?;
    _1_timothy_1_group_04(story)?;
    _1_timothy_1_group_05(story)?;
    Ok(())
}

fn _1_timothy_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Timothy", 2, "The church is urged to pray for rulers and all people, seeking a peaceful life in which the saving purpose of God can be made known.", "Paul discusses conduct in gathered worship, emphasizing modesty, teachability, and the ordering of instruction within the community.");
    _1_timothy_2_group_01(story)?;
    _1_timothy_2_group_02(story)?;
    _1_timothy_2_group_03(story)?;
    _1_timothy_2_group_04(story)?;
    Ok(())
}

fn _1_timothy_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Timothy", 3, "Paul lists the character required of overseers and servants: steady households, good reputations, self-control, hospitality, and tested faith.", "The church is called God's household and the support of truth, with Christ's revealed mystery as its central confession.");
    _1_timothy_3_group_01(story)?;
    _1_timothy_3_group_02(story)?;
    _1_timothy_3_group_03(story)?;
    _1_timothy_3_group_04(story)?;
    Ok(())
}

fn _1_timothy_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Timothy", 4, "Later distortions will forbid good gifts and promote counterfeit holiness, so Timothy must reject myths and train for godliness.", "His public reading, teaching, example, and gift must keep advancing, letting progress become visible rather than hidden in private intention.");
    _1_timothy_4_group_01(story)?;
    _1_timothy_4_group_02(story)?;
    _1_timothy_4_group_03(story)?;
    _1_timothy_4_group_04(story)?;
    Ok(())
}

fn _1_timothy_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Timothy", 5, "Timothy is taught how to treat older and younger believers, widows, elders, and accusations, balancing family responsibility with careful communal support.", "Leaders deserve honor and impartial judgment, while hurried appointments and selective favoritism are warned against as dangerous shortcuts.");
    _1_timothy_5_group_01(story)?;
    _1_timothy_5_group_02(story)?;
    _1_timothy_5_group_03(story)?;
    _1_timothy_5_group_04(story)?;
    _1_timothy_5_group_05(story)?;
    _1_timothy_5_group_06(story)?;
    Ok(())
}

fn _1_timothy_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Timothy", 6, "Paul addresses servants, contentment, and the trap of turning religion into a revenue strategy that pierces hearts with many griefs.", "The rich are told to be generous and hope in God, while Timothy is charged to guard the deposit and avoid arguments mislabeled as knowledge.");
    _1_timothy_6_group_01(story)?;
    _1_timothy_6_group_02(story)?;
    _1_timothy_6_group_03(story)?;
    _1_timothy_6_group_04(story)?;
    _1_timothy_6_group_05(story)?;
    Ok(())
}

fn _1_timothy_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        1,
        1,
        4,
        "paul guard -> await_value -> move_value paul apostle jesus",
        &["paul", "apostle", "jesus", "christ", "commandment"],
    );
    segment.guard(
        1,
        "paul",
        &["apostle", "jesus", "christ"],
        &["paul", "apostle", "jesus", "christ", "commandment"],
    )?;
    segment.await_value(
        2,
        "unto",
        &["timothy", "own", "son"],
        &["unto", "timothy", "own", "son", "faith"],
    )?;
    segment.move_value(
        3,
        "besought",
        &["thee", "abide", "still"],
        &["besought", "thee", "abide", "still", "ephesus"],
    )?;
    segment.guard(
        4,
        "neither",
        &["give", "heed", "fables"],
        &["neither", "give", "heed", "fables", "endless"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        1,
        5,
        8,
        "now guard -> expression now end commandment",
        &["now", "end", "commandment", "charity", "pure"],
    );
    segment.guard(
        5,
        "now",
        &["end", "commandment", "charity"],
        &["now", "end", "commandment", "charity", "pure"],
    )?;
    segment.expression(
        6,
        "having",
        &["swerved", "turned", "aside"],
        &["having", "swerved", "turned", "aside", "unto"],
    )?;
    segment.guard(
        7,
        "desiring",
        &["teachers", "law", "understanding"],
        &["desiring", "teachers", "law", "understanding", "neither"],
    )?;
    segment.guard(
        8,
        "man",
        &["know", "law", "good"],
        &["man", "know", "law", "good", "use"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        1,
        9,
        12,
        "man construct -> expression -> await_value -> iterate man knowing law",
        &["man", "knowing", "law", "not", "made"],
    );
    segment.construct(
        9,
        "man",
        &["knowing", "law", "not"],
        &["man", "knowing", "law", "not", "made"],
    )?;
    segment.expression(
        10,
        "whoremongers",
        &["defile", "themselves", "mankind"],
        &[
            "whoremongers",
            "defile",
            "themselves",
            "mankind",
            "menstealers",
        ],
    )?;
    segment.await_value(
        11,
        "god",
        &["according", "glorious", "gospel"],
        &["god", "according", "glorious", "gospel", "blessed"],
    )?;
    segment.iterate(
        12,
        "christ",
        &["thank", "jesus", "lord"],
        &["christ", "thank", "jesus", "lord", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        1,
        13,
        16,
        "blasphemer borrow -> await_value -> move_value blasphemer persecutor injurious",
        &["blasphemer", "persecutor", "injurious", "obtained", "mercy"],
    );
    segment.borrow(
        13,
        "blasphemer",
        &["persecutor", "injurious", "obtained"],
        &["blasphemer", "persecutor", "injurious", "obtained", "mercy"],
    )?;
    segment.await_value(
        14,
        "lord",
        &["grace", "exceeding", "abundant"],
        &["lord", "grace", "exceeding", "abundant", "faith"],
    )?;
    segment.move_value(
        15,
        "christ",
        &["faithful", "saying", "worthy"],
        &["christ", "faithful", "saying", "worthy", "acceptation"],
    )?;
    segment.await_value(
        16,
        "jesus",
        &["howbeit", "cause", "obtained"],
        &["jesus", "howbeit", "cause", "obtained", "mercy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        1,
        17,
        20,
        "king trait_bound -> move_value -> construct -> guard king now unto",
        &["king", "now", "unto", "eternal", "immortal"],
    );
    segment.trait_bound(
        17,
        "king",
        &["now", "unto", "eternal"],
        &["king", "now", "unto", "eternal", "immortal"],
    )?;
    segment.move_value(
        18,
        "charge",
        &["commit", "unto", "thee"],
        &["charge", "commit", "unto", "thee", "son"],
    )?;
    segment.construct(
        19,
        "holding",
        &["faith", "good", "conscience"],
        &["holding", "faith", "good", "conscience", "having"],
    )?;
    segment.guard(
        20,
        "hymenaeus",
        &["alexander", "delivered", "unto"],
        &["hymenaeus", "alexander", "delivered", "unto", "satan"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        2,
        1,
        4,
        "exhort construct -> expression -> move_value exhort therefore first",
        &["exhort", "therefore", "first", "supplications", "prayers"],
    );
    segment.construct(
        1,
        "exhort",
        &["therefore", "first", "supplications"],
        &["exhort", "therefore", "first", "supplications", "prayers"],
    )?;
    segment.expression(
        2,
        "kings",
        &["authority", "lead", "quiet"],
        &["kings", "authority", "lead", "quiet", "peaceable"],
    )?;
    segment.expression(
        3,
        "god",
        &["good", "acceptable", "sight"],
        &["god", "good", "acceptable", "sight", "saviour"],
    )?;
    segment.move_value(
        4,
        "men",
        &["will", "saved", "unto"],
        &["men", "will", "saved", "unto", "knowledge"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        2,
        5,
        8,
        "god expression -> guard -> result god mediator between",
        &["god", "mediator", "between", "men", "man"],
    );
    segment.expression(
        5,
        "god",
        &["mediator", "between", "men"],
        &["god", "mediator", "between", "men", "man"],
    )?;
    segment.expression(
        6,
        "gave",
        &["himself", "ransom", "testified"],
        &["gave", "himself", "ransom", "testified", "due"],
    )?;
    segment.guard(
        7,
        "christ",
        &["whereunto", "ordained", "preacher"],
        &["christ", "whereunto", "ordained", "preacher", "apostle"],
    )?;
    segment.result(
        8,
        "men",
        &["will", "therefore", "pray"],
        &["men", "will", "therefore", "pray", "every"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        2,
        9,
        12,
        "women guard -> expression women like manner",
        &["women", "like", "manner", "adorn", "themselves"],
    );
    segment.guard(
        9,
        "women",
        &["like", "manner", "adorn"],
        &["women", "like", "manner", "adorn", "themselves"],
    )?;
    segment.expression(
        10,
        "women",
        &["becometh", "professing", "godliness"],
        &["women", "becometh", "professing", "godliness", "good"],
    )?;
    segment.expression(
        11,
        "woman",
        &["learn", "silence", "subjection"],
        &["woman", "learn", "silence", "subjection"],
    )?;
    segment.guard(
        12,
        "woman",
        &["suffer", "not", "teach"],
        &["woman", "suffer", "not", "teach", "nor"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        2,
        13,
        15,
        "adam construct -> guard -> await_value adam first formed",
        &["adam", "first", "formed", "eve", "woman"],
    );
    segment.construct(
        13,
        "adam",
        &["first", "formed", "eve"],
        &["adam", "first", "formed", "eve"],
    )?;
    segment.guard(
        14,
        "woman",
        &["adam", "not", "deceived"],
        &["woman", "adam", "not", "deceived", "transgression"],
    )?;
    segment.await_value(
        15,
        "notwithstanding",
        &["shall", "saved", "childbearing"],
        &[
            "notwithstanding",
            "shall",
            "saved",
            "childbearing",
            "continue",
        ],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        3,
        1,
        4,
        "man expression -> guard man true saying",
        &["man", "true", "saying", "desire", "office"],
    );
    segment.expression(
        1,
        "man",
        &["true", "saying", "desire"],
        &["man", "true", "saying", "desire", "office"],
    )?;
    segment.guard(
        2,
        "bishop",
        &["must", "blameless", "husband"],
        &["bishop", "must", "blameless", "husband", "wife"],
    )?;
    segment.guard(
        3,
        "not",
        &["given", "wine", "striker"],
        &["not", "given", "wine", "striker", "greedy"],
    )?;
    segment.expression(
        4,
        "children",
        &["ruleth", "well", "own"],
        &["children", "ruleth", "well", "own", "house"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        3,
        5,
        8,
        "man guard -> drop_value man know not",
        &["man", "know", "not", "rule", "own"],
    );
    segment.guard(
        5,
        "man",
        &["know", "not", "rule"],
        &["man", "know", "not", "rule", "own"],
    )?;
    segment.guard(
        6,
        "not",
        &["novice", "lest", "lifted"],
        &["not", "novice", "lest", "lifted", "pride"],
    )?;
    segment.guard(
        7,
        "moreover",
        &["must", "good", "report"],
        &["moreover", "must", "good", "report", "without"],
    )?;
    segment.drop_value(
        8,
        "likewise",
        &["must", "deacons", "grave"],
        &["likewise", "must", "deacons", "grave", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        3,
        9,
        12,
        "holding await_value -> expression -> drop_value holding mystery faith",
        &["holding", "mystery", "faith", "pure", "conscience"],
    );
    segment.await_value(
        9,
        "holding",
        &["mystery", "faith", "pure"],
        &["holding", "mystery", "faith", "pure", "conscience"],
    )?;
    segment.expression(
        10,
        "first",
        &["proved", "use", "office"],
        &["first", "proved", "use", "office", "deacon"],
    )?;
    segment.drop_value(
        11,
        "even",
        &["must", "wives", "grave"],
        &["even", "must", "wives", "grave", "not"],
    )?;
    segment.expression(
        12,
        "children",
        &["deacons", "husbands", "wife"],
        &["children", "deacons", "husbands", "wife", "ruling"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        3,
        13,
        16,
        "used await_value -> move_value -> expression -> trait_bound used office deacon",
        &["used", "office", "deacon", "well", "purchase"],
    );
    segment.await_value(
        13,
        "used",
        &["office", "deacon", "well"],
        &["used", "office", "deacon", "well", "purchase"],
    )?;
    segment.move_value(
        14,
        "things",
        &["write", "unto", "thee"],
        &["things", "write", "unto", "thee", "hoping"],
    )?;
    segment.expression(
        15,
        "tarry",
        &["long", "mayest", "know"],
        &["tarry", "long", "mayest", "know", "oughtest"],
    )?;
    segment.trait_bound(
        16,
        "god",
        &["without", "controversy", "great"],
        &["god", "without", "controversy", "great", "mystery"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        4,
        1,
        4,
        "spirit move_value -> expression -> construct -> iterate spirit now speaketh",
        &["spirit", "now", "speaketh", "expressly", "latter"],
    );
    segment.move_value(
        1,
        "spirit",
        &["now", "speaketh", "expressly"],
        &["spirit", "now", "speaketh", "expressly", "latter"],
    )?;
    segment.expression(
        2,
        "speaking",
        &["lies", "hypocrisy", "having"],
        &["speaking", "lies", "hypocrisy", "having", "conscience"],
    )?;
    segment.construct(
        3,
        "god",
        &["forbidding", "marry", "commanding"],
        &["god", "forbidding", "marry", "commanding", "abstain"],
    )?;
    segment.iterate(
        4,
        "god",
        &["every", "creature", "good"],
        &["god", "every", "creature", "good", "nothing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        4,
        5,
        8,
        "god result -> guard -> expression -> move_value god sanctified word",
        &["god", "sanctified", "word", "prayer", "jesus"],
    );
    segment.result(
        5,
        "god",
        &["sanctified", "word", "prayer"],
        &["god", "sanctified", "word", "prayer"],
    )?;
    segment.guard(
        6,
        "jesus",
        &["put", "brethren", "remembrance"],
        &["jesus", "put", "brethren", "remembrance", "things"],
    )?;
    segment.expression(
        7,
        "refuse",
        &["profane", "old", "wives"],
        &["refuse", "profane", "old", "wives", "fables"],
    )?;
    segment.move_value(
        8,
        "bodily",
        &["exercise", "profiteth", "little"],
        &["bodily", "exercise", "profiteth", "little", "godliness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        4,
        9,
        12,
        "faithful expression -> await_value -> guard faithful saying worthy",
        &["faithful", "saying", "worthy", "acceptation", "god"],
    );
    segment.expression(
        9,
        "faithful",
        &["saying", "worthy", "acceptation"],
        &["faithful", "saying", "worthy", "acceptation"],
    )?;
    segment.await_value(
        10,
        "god",
        &["therefore", "both", "labour"],
        &["god", "therefore", "both", "labour", "suffer"],
    )?;
    segment.guard(
        11,
        "things",
        &["command", "teach"],
        &["things", "command", "teach"],
    )?;
    segment.guard(
        12,
        "man",
        &["despise", "youth", "example"],
        &["man", "despise", "youth", "example", "believers"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        4,
        13,
        16,
        "till move_value -> guard -> expression till give attendance",
        &["till", "give", "attendance", "reading", "exhortation"],
    );
    segment.move_value(
        13,
        "till",
        &["give", "attendance", "reading"],
        &["till", "give", "attendance", "reading", "exhortation"],
    )?;
    segment.guard(
        14,
        "neglect",
        &["not", "gift", "thee"],
        &["neglect", "not", "gift", "thee", "given"],
    )?;
    segment.expression(
        15,
        "meditate",
        &["things", "give", "thyself"],
        &["meditate", "things", "give", "thyself", "wholly"],
    )?;
    segment.guard(
        16,
        "take",
        &["heed", "unto", "thyself"],
        &["take", "heed", "unto", "thyself", "doctrine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        5,
        1,
        4,
        "men guard -> expression men rebuke not",
        &["men", "rebuke", "not", "elder", "intreat"],
    );
    segment.guard(
        1,
        "men",
        &["rebuke", "not", "elder"],
        &["men", "rebuke", "not", "elder", "intreat"],
    )?;
    segment.expression(
        2,
        "women",
        &["elder", "mothers", "younger"],
        &["women", "elder", "mothers", "younger", "sisters"],
    )?;
    segment.expression(
        3,
        "honour",
        &["widows", "indeed"],
        &["honour", "widows", "indeed"],
    )?;
    segment.expression(
        4,
        "children",
        &["any", "widow", "nephews"],
        &["children", "any", "widow", "nephews", "learn"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        5,
        5,
        8,
        "god expression -> drop_value -> guard god now widow",
        &["god", "now", "widow", "indeed", "desolate"],
    );
    segment.expression(
        5,
        "god",
        &["now", "widow", "indeed"],
        &["god", "now", "widow", "indeed", "desolate"],
    )?;
    segment.drop_value(
        6,
        "liveth",
        &["pleasure", "dead", "while"],
        &["liveth", "pleasure", "dead", "while"],
    )?;
    segment.expression(
        7,
        "things",
        &["give", "charge", "blameless"],
        &["things", "give", "charge", "blameless"],
    )?;
    segment.guard(
        8,
        "any",
        &["provide", "not", "own"],
        &["any", "provide", "not", "own", "specially"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        5,
        9,
        12,
        "not guard -> move_value -> expression -> await_value not widow taken",
        &["not", "widow", "taken", "number", "under"],
    );
    segment.guard(
        9,
        "not",
        &["widow", "taken", "number"],
        &["not", "widow", "taken", "number", "under"],
    )?;
    segment.move_value(
        10,
        "children",
        &["well", "reported", "good"],
        &["children", "well", "reported", "good", "works"],
    )?;
    segment.expression(
        11,
        "christ",
        &["younger", "widows", "refuse"],
        &["christ", "younger", "widows", "refuse", "begun"],
    )?;
    segment.await_value(
        12,
        "having",
        &["damnation", "cast", "off"],
        &["having", "damnation", "cast", "off", "first"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        5,
        13,
        16,
        "withal guard -> call -> expression withal learn idle",
        &["withal", "learn", "idle", "wandering", "house"],
    );
    segment.guard(
        13,
        "withal",
        &["learn", "idle", "wandering"],
        &["withal", "learn", "idle", "wandering", "house"],
    )?;
    segment.call(
        14,
        "women",
        &["will", "therefore", "younger"],
        &["women", "will", "therefore", "younger", "marry"],
    )?;
    segment.expression(
        15,
        "already",
        &["turned", "aside", "satan"],
        &["already", "turned", "aside", "satan"],
    )?;
    segment.guard(
        16,
        "man",
        &["any", "woman", "believeth"],
        &["man", "any", "woman", "believeth", "widows"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        5,
        17,
        20,
        "elders call -> guard -> panic elders rule well",
        &["elders", "rule", "well", "counted", "worthy"],
    );
    segment.call(
        17,
        "elders",
        &["rule", "well", "counted"],
        &["elders", "rule", "well", "counted", "worthy"],
    )?;
    segment.guard(
        18,
        "scripture",
        &["saith", "shalt", "not"],
        &["scripture", "saith", "shalt", "not", "muzzle"],
    )?;
    segment.guard(
        19,
        "against",
        &["elder", "receive", "not"],
        &["against", "elder", "receive", "not", "accusation"],
    )?;
    segment.panic(
        20,
        "sin",
        &["rebuke", "others", "fear"],
        &["sin", "rebuke", "others", "fear"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_5_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        5,
        21,
        25,
        "god expression -> guard god charge thee",
        &["god", "charge", "thee", "lord", "jesus"],
    );
    segment.expression(
        21,
        "god",
        &["charge", "thee", "lord"],
        &["god", "charge", "thee", "lord", "jesus"],
    )?;
    segment.guard(
        22,
        "man",
        &["lay", "hands", "suddenly"],
        &["man", "lay", "hands", "suddenly", "neither"],
    )?;
    segment.guard(
        23,
        "drink",
        &["longer", "water", "use"],
        &["drink", "longer", "water", "use", "little"],
    )?;
    segment.expression(
        24,
        "men",
        &["men's", "sins", "open"],
        &["men", "men's", "sins", "open", "beforehand"],
    )?;
    segment.expression(
        25,
        "likewise",
        &["good", "works", "manifest"],
        &["likewise", "good", "works", "manifest", "beforehand"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        6,
        1,
        4,
        "many guard -> expression many servants under",
        &["many", "servants", "under", "yoke", "count"],
    );
    segment.guard(
        1,
        "many",
        &["servants", "under", "yoke"],
        &["many", "servants", "under", "yoke", "count"],
    )?;
    segment.guard(
        2,
        "believing",
        &["masters", "not", "despise"],
        &["believing", "masters", "not", "despise", "brethren"],
    )?;
    segment.guard(
        3,
        "man",
        &["any", "teach", "otherwise"],
        &["man", "any", "teach", "otherwise", "consent"],
    )?;
    segment.expression(
        4,
        "proud",
        &["knowing", "nothing", "doting"],
        &["proud", "knowing", "nothing", "doting", "questions"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        6,
        5,
        8,
        "men expression -> move_value men perverse disputings",
        &["men", "perverse", "disputings", "corrupt", "minds"],
    );
    segment.expression(
        5,
        "men",
        &["perverse", "disputings", "corrupt"],
        &["men", "perverse", "disputings", "corrupt", "minds"],
    )?;
    segment.expression(
        6,
        "godliness",
        &["contentment", "great", "gain"],
        &["godliness", "contentment", "great", "gain"],
    )?;
    segment.move_value(
        7,
        "brought",
        &["nothing", "world", "certain"],
        &["brought", "nothing", "world", "certain", "carry"],
    )?;
    segment.expression(
        8,
        "having",
        &["food", "raiment", "therewith"],
        &["having", "food", "raiment", "therewith", "content"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        6,
        9,
        12,
        "will expression -> await_value -> move_value will rich fall",
        &["will", "rich", "fall", "temptation", "snare"],
    );
    segment.expression(
        9,
        "will",
        &["rich", "fall", "temptation"],
        &["will", "rich", "fall", "temptation", "snare"],
    )?;
    segment.await_value(
        10,
        "love",
        &["money", "root", "evil"],
        &["love", "money", "root", "evil", "while"],
    )?;
    segment.move_value(
        11,
        "man",
        &["god", "flee", "things"],
        &["man", "god", "flee", "things", "follow"],
    )?;
    segment.await_value(
        12,
        "fight",
        &["good", "faith", "lay"],
        &["fight", "good", "faith", "lay", "hold"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        6,
        13,
        16,
        "god expression -> guard -> return_value god give thee",
        &["god", "give", "thee", "charge", "sight"],
    );
    segment.expression(
        13,
        "god",
        &["give", "thee", "charge"],
        &["god", "give", "thee", "charge", "sight"],
    )?;
    segment.guard(
        14,
        "lord",
        &["keep", "commandment", "without"],
        &["lord", "keep", "commandment", "without", "spot"],
    )?;
    segment.return_value(
        15,
        "king",
        &["times", "shall", "shew"],
        &["king", "times", "shall", "shew", "blessed"],
    )?;
    segment.guard(
        16,
        "man",
        &["hath", "immortality", "dwelling"],
        &["man", "hath", "immortality", "dwelling", "light"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_timothy_6_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Timothy,
        6,
        17,
        21,
        "charge guard -> expression -> move_value -> await_value charge rich world",
        &["charge", "rich", "world", "not", "highminded"],
    );
    segment.guard(
        17,
        "charge",
        &["rich", "world", "not"],
        &["charge", "rich", "world", "not", "highminded"],
    )?;
    segment.expression(
        18,
        "good",
        &["rich", "works", "ready"],
        &["good", "rich", "works", "ready", "distribute"],
    )?;
    segment.move_value(
        19,
        "laying",
        &["store", "themselves", "good"],
        &["laying", "store", "themselves", "good", "foundation"],
    )?;
    segment.guard(
        20,
        "timothy",
        &["keep", "committed", "trust"],
        &["timothy", "keep", "committed", "trust", "avoiding"],
    )?;
    segment.await_value(
        21,
        "professing",
        &["erred", "concerning", "faith"],
        &["professing", "erred", "concerning", "faith", "grace"],
    )?;
    segment.commit()?;
    Ok(())
}
