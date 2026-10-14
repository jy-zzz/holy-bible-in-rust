use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _2_timothy(story)?;
    Ok(())
}

fn _2_timothy(story: &mut Story) -> Result<(), StoryError> {
    _2_timothy_1(story)?;
    _2_timothy_2(story)?;
    _2_timothy_3(story)?;
    _2_timothy_4(story)?;
    Ok(())
}

fn _2_timothy_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Timothy", 1, "Paul remembers Timothy's sincere faith and urges him to rekindle the gift, because the Spirit supplies courage, love, and disciplined judgment rather than fear.", "He is not to be ashamed of the testimony or of Paul's chains, but to guard the good deposit through the Spirit's help.");
    _2_timothy_1_group_01(story)?;
    _2_timothy_1_group_02(story)?;
    _2_timothy_1_group_03(story)?;
    _2_timothy_1_group_04(story)?;
    _2_timothy_1_group_05(story)?;
    Ok(())
}

fn _2_timothy_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Timothy", 2, "Timothy must pass the teaching to reliable people, endure hardship like a soldier, compete lawfully like an athlete, and labor patiently like a farmer.", "He should avoid quarrels, cleanse himself for honorable use, and correct opponents gently in hope that God grants them a change of mind.");
    _2_timothy_2_group_01(story)?;
    _2_timothy_2_group_02(story)?;
    _2_timothy_2_group_03(story)?;
    _2_timothy_2_group_04(story)?;
    _2_timothy_2_group_05(story)?;
    _2_timothy_2_group_06(story)?;
    _2_timothy_2_group_07(story)?;
    Ok(())
}

fn _2_timothy_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Timothy", 3, "The last days will feature self-absorbed, money-loving, boastful people whose religious interface masks a denial of its power.", "Timothy must continue in the Scriptures that trained him for salvation and equipped him for every good work, remembering Paul's example and endurance.");
    _2_timothy_3_group_01(story)?;
    _2_timothy_3_group_02(story)?;
    _2_timothy_3_group_03(story)?;
    _2_timothy_3_group_04(story)?;
    Ok(())
}

fn _2_timothy_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Timothy", 4, "Paul gives his final charge to preach steadily through favorable and difficult seasons, because many will prefer teachers who echo their appetites.", "He looks toward the completed race, asks Timothy to bring the cloak and scrolls, names deserters and helpers, and trusts the Lord who stood by him.");
    _2_timothy_4_group_01(story)?;
    _2_timothy_4_group_02(story)?;
    _2_timothy_4_group_03(story)?;
    _2_timothy_4_group_04(story)?;
    _2_timothy_4_group_05(story)?;
    _2_timothy_4_group_06(story)?;
    Ok(())
}

fn _2_timothy_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        1,
        1,
        4,
        "paul return_value -> borrow -> expression paul apostle jesus",
        &["paul", "apostle", "jesus", "christ", "will"],
    );
    segment.return_value(
        1,
        "paul",
        &["apostle", "jesus", "christ"],
        &["paul", "apostle", "jesus", "christ", "will"],
    )?;
    segment.borrow(
        2,
        "god",
        &["timothy", "dearly", "beloved"],
        &["god", "timothy", "dearly", "beloved", "son"],
    )?;
    segment.expression(
        3,
        "god",
        &["thank", "serve", "forefathers"],
        &["god", "thank", "serve", "forefathers", "pure"],
    )?;
    segment.expression(
        4,
        "greatly",
        &["desiring", "see", "thee"],
        &["greatly", "desiring", "see", "thee", "mindful"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        1,
        5,
        8,
        "call await_value -> expression -> guard call remembrance unfeigned",
        &["call", "remembrance", "unfeigned", "faith", "thee"],
    );
    segment.await_value(
        5,
        "call",
        &["remembrance", "unfeigned", "faith"],
        &["call", "remembrance", "unfeigned", "faith", "thee"],
    )?;
    segment.expression(
        6,
        "god",
        &["wherefore", "put", "thee"],
        &["god", "wherefore", "put", "thee", "remembrance"],
    )?;
    segment.guard(
        7,
        "god",
        &["hath", "not", "given"],
        &["god", "hath", "not", "given", "spirit"],
    )?;
    segment.guard(
        8,
        "lord",
        &["not", "therefore", "ashamed"],
        &["lord", "not", "therefore", "ashamed", "testimony"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        1,
        9,
        12,
        "hath guard -> construct -> expression hath saved called",
        &["hath", "saved", "called", "holy", "calling"],
    );
    segment.guard(
        9,
        "hath",
        &["saved", "called", "holy"],
        &["hath", "saved", "called", "holy", "calling"],
    )?;
    segment.construct(
        10,
        "saviour",
        &["now", "made", "manifest"],
        &["saviour", "now", "made", "manifest", "appearing"],
    )?;
    segment.expression(
        11,
        "whereunto",
        &["appointed", "preacher", "apostle"],
        &["whereunto", "appointed", "preacher", "apostle", "teacher"],
    )?;
    segment.guard(
        12,
        "cause",
        &["suffer", "things", "nevertheless"],
        &["cause", "suffer", "things", "nevertheless", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        1,
        13,
        15,
        "hold await_value -> guard -> expression hold fast form",
        &["hold", "fast", "form", "sound", "words"],
    );
    segment.await_value(
        13,
        "hold",
        &["fast", "form", "sound"],
        &["hold", "fast", "form", "sound", "words"],
    )?;
    segment.guard(
        14,
        "good",
        &["thing", "committed", "unto"],
        &["good", "thing", "committed", "unto", "thee"],
    )?;
    segment.expression(
        15,
        "knowest",
        &["asia", "turned", "away"],
        &["knowest", "asia", "turned", "away", "phygellus"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        1,
        16,
        18,
        "lord guard -> expression -> borrow lord give mercy",
        &["lord", "give", "mercy", "unto", "house"],
    );
    segment.guard(
        16,
        "lord",
        &["give", "mercy", "unto"],
        &["lord", "give", "mercy", "unto", "house"],
    )?;
    segment.expression(
        17,
        "rome",
        &["sought", "very", "diligently"],
        &["rome", "sought", "very", "diligently", "found"],
    )?;
    segment.borrow(
        18,
        "lord",
        &["grant", "unto", "find"],
        &["lord", "grant", "unto", "find", "mercy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        2,
        1,
        4,
        "christ borrow -> call -> expression -> guard christ therefore son",
        &["christ", "therefore", "son", "strong", "grace"],
    );
    segment.borrow(
        1,
        "christ",
        &["therefore", "son", "strong"],
        &["christ", "therefore", "son", "strong", "grace"],
    )?;
    segment.call(
        2,
        "things",
        &["hast", "heard", "among"],
        &["things", "hast", "heard", "among", "many"],
    )?;
    segment.expression(
        3,
        "jesus",
        &["therefore", "endure", "hardness"],
        &["jesus", "therefore", "endure", "hardness", "good"],
    )?;
    segment.guard(
        4,
        "man",
        &["warreth", "entangleth", "himself"],
        &["man", "warreth", "entangleth", "himself", "affairs"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        2,
        5,
        8,
        "man guard -> call -> drop_value man strive masteries",
        &["man", "strive", "masteries", "yet", "not"],
    );
    segment.guard(
        5,
        "man",
        &["strive", "masteries", "yet"],
        &["man", "strive", "masteries", "yet", "not"],
    )?;
    segment.guard(
        6,
        "husbandman",
        &["laboureth", "must", "first"],
        &["husbandman", "laboureth", "must", "first", "partaker"],
    )?;
    segment.call(
        7,
        "lord",
        &["consider", "say", "give"],
        &["lord", "consider", "say", "give", "thee"],
    )?;
    segment.drop_value(
        8,
        "jesus",
        &["remember", "christ", "seed"],
        &["jesus", "remember", "christ", "seed", "david"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        2,
        9,
        12,
        "wherein guard -> trait_bound -> drop_value -> expression wherein suffer trouble",
        &["wherein", "suffer", "trouble", "evil", "doer"],
    );
    segment.guard(
        9,
        "wherein",
        &["suffer", "trouble", "evil"],
        &["wherein", "suffer", "trouble", "evil", "doer"],
    )?;
    segment.trait_bound(
        10,
        "christ",
        &["therefore", "endure", "things"],
        &["christ", "therefore", "endure", "things", "elect's"],
    )?;
    segment.drop_value(
        11,
        "faithful",
        &["saying", "dead", "shall"],
        &["faithful", "saying", "dead", "shall", "live"],
    )?;
    segment.expression(
        12,
        "suffer",
        &["shall", "reign", "deny"],
        &["suffer", "shall", "reign", "deny", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        2,
        13,
        16,
        "believe guard -> expression believe not yet",
        &["believe", "not", "yet", "abideth", "faithful"],
    );
    segment.guard(
        13,
        "believe",
        &["not", "yet", "abideth"],
        &["believe", "not", "yet", "abideth", "faithful"],
    )?;
    segment.guard(
        14,
        "lord",
        &["things", "put", "remembrance"],
        &["lord", "things", "put", "remembrance", "charging"],
    )?;
    segment.guard(
        15,
        "god",
        &["study", "shew", "thyself"],
        &["god", "study", "shew", "thyself", "approved"],
    )?;
    segment.expression(
        16,
        "shun",
        &["profane", "vain", "babblings"],
        &["shun", "profane", "vain", "babblings", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        2,
        17,
        20,
        "word call -> reinitialize -> move_value -> guard word will eat",
        &["word", "will", "eat", "doth", "canker"],
    );
    segment.call(
        17,
        "word",
        &["will", "eat", "doth"],
        &["word", "will", "eat", "doth", "canker"],
    )?;
    segment.reinitialize(
        18,
        "concerning",
        &["truth", "erred", "saying"],
        &["concerning", "truth", "erred", "saying", "resurrection"],
    )?;
    segment.move_value(
        19,
        "god",
        &["nevertheless", "foundation", "standeth"],
        &["god", "nevertheless", "foundation", "standeth", "sure"],
    )?;
    segment.guard(
        20,
        "great",
        &["house", "not", "vessels"],
        &["great", "house", "not", "vessels", "gold"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        2,
        21,
        23,
        "man iterate -> move_value -> expression man therefore purge",
        &["man", "therefore", "purge", "himself", "shall"],
    );
    segment.iterate(
        21,
        "man",
        &["therefore", "purge", "himself"],
        &["man", "therefore", "purge", "himself", "shall"],
    )?;
    segment.move_value(
        22,
        "flee",
        &["youthful", "lusts", "follow"],
        &["flee", "youthful", "lusts", "follow", "righteousness"],
    )?;
    segment.expression(
        23,
        "foolish",
        &["unlearned", "questions", "avoid"],
        &["foolish", "unlearned", "questions", "avoid", "knowing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        2,
        24,
        26,
        "lord guard -> expression lord servant must",
        &["lord", "servant", "must", "not", "strive"],
    );
    segment.guard(
        24,
        "lord",
        &["servant", "must", "not"],
        &["lord", "servant", "must", "not", "strive"],
    )?;
    segment.expression(
        25,
        "god",
        &["meekness", "instructing", "oppose"],
        &["god", "meekness", "instructing", "oppose", "themselves"],
    )?;
    segment.expression(
        26,
        "recover",
        &["themselves", "snare", "devil"],
        &["recover", "themselves", "snare", "devil", "taken"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        3,
        1,
        4,
        "know move_value -> expression know last days",
        &["know", "last", "days", "perilous", "times"],
    );
    segment.move_value(
        1,
        "know",
        &["last", "days", "perilous"],
        &["know", "last", "days", "perilous", "times"],
    )?;
    segment.expression(
        2,
        "men",
        &["shall", "lovers", "own"],
        &["men", "shall", "lovers", "own", "selves"],
    )?;
    segment.expression(
        3,
        "without",
        &["natural", "affection", "trucebreakers"],
        &["without", "natural", "affection", "trucebreakers", "false"],
    )?;
    segment.expression(
        4,
        "god",
        &["traitors", "heady", "highminded"],
        &["god", "traitors", "heady", "highminded", "lovers"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        3,
        5,
        8,
        "having expression -> move_value -> await_value having form godliness",
        &["having", "form", "godliness", "denying", "power"],
    );
    segment.expression(
        5,
        "having",
        &["form", "godliness", "denying"],
        &["having", "form", "godliness", "denying", "power"],
    )?;
    segment.expression(
        6,
        "women",
        &["sort", "creep", "houses"],
        &["women", "sort", "creep", "houses", "lead"],
    )?;
    segment.move_value(
        7,
        "ever",
        &["learning", "never", "able"],
        &["ever", "learning", "never", "able", "knowledge"],
    )?;
    segment.await_value(
        8,
        "moses",
        &["now", "jannes", "jambres"],
        &["moses", "now", "jannes", "jambres", "withstood"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        3,
        9,
        12,
        "men guard -> await_value -> move_value -> expression men shall proceed",
        &["men", "shall", "proceed", "further", "folly"],
    );
    segment.guard(
        9,
        "men",
        &["shall", "proceed", "further"],
        &["men", "shall", "proceed", "further", "folly"],
    )?;
    segment.await_value(
        10,
        "hast",
        &["fully", "known", "doctrine"],
        &["hast", "fully", "known", "doctrine", "manner"],
    )?;
    segment.move_value(
        11,
        "lord",
        &["persecutions", "afflictions", "unto"],
        &["lord", "persecutions", "afflictions", "unto", "antioch"],
    )?;
    segment.expression(
        12,
        "christ",
        &["yea", "will", "live"],
        &["christ", "yea", "will", "live", "godly"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        3,
        13,
        17,
        "men expression -> construct men evil seducers",
        &["men", "evil", "seducers", "shall", "wax"],
    );
    segment.expression(
        13,
        "men",
        &["evil", "seducers", "shall"],
        &["men", "evil", "seducers", "shall", "wax"],
    )?;
    segment.expression(
        14,
        "continue",
        &["things", "hast", "learned"],
        &["continue", "things", "hast", "learned", "assured"],
    )?;
    segment.construct(
        15,
        "child",
        &["hast", "known", "holy"],
        &["child", "hast", "known", "holy", "scriptures"],
    )?;
    segment.expression(
        16,
        "god",
        &["scripture", "given", "inspiration"],
        &["god", "scripture", "given", "inspiration", "profitable"],
    )?;
    segment.expression(
        17,
        "man",
        &["god", "perfect", "thoroughly"],
        &["man", "god", "perfect", "thoroughly", "furnished"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        4,
        1,
        4,
        "god drop_value -> call -> move_value -> expression god charge thee",
        &["god", "charge", "thee", "therefore", "lord"],
    );
    segment.drop_value(
        1,
        "god",
        &["charge", "thee", "therefore"],
        &["god", "charge", "thee", "therefore", "lord"],
    )?;
    segment.call(
        2,
        "preach",
        &["word", "instant", "season"],
        &["preach", "word", "instant", "season", "reprove"],
    )?;
    segment.move_value(
        3,
        "time",
        &["will", "not", "endure"],
        &["time", "will", "not", "endure", "sound"],
    )?;
    segment.expression(
        4,
        "shall",
        &["turn", "away", "ears"],
        &["shall", "turn", "away", "ears", "truth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        4,
        5,
        8,
        "watch construct -> trait_bound -> await_value -> guard watch things endure",
        &["watch", "things", "endure", "afflictions", "work"],
    );
    segment.construct(
        5,
        "watch",
        &["things", "endure", "afflictions"],
        &["watch", "things", "endure", "afflictions", "work"],
    )?;
    segment.trait_bound(
        6,
        "now",
        &["ready", "offered", "time"],
        &["now", "ready", "offered", "time", "departure"],
    )?;
    segment.await_value(
        7,
        "fought",
        &["good", "fight", "finished"],
        &["fought", "good", "fight", "finished", "course"],
    )?;
    segment.guard(
        8,
        "lord",
        &["henceforth", "laid", "crown"],
        &["lord", "henceforth", "laid", "crown", "righteousness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        4,
        9,
        12,
        "diligence move_value -> expression diligence shortly unto",
        &["diligence", "shortly", "unto", "demas", "hath"],
    );
    segment.move_value(
        9,
        "diligence",
        &["shortly", "unto"],
        &["diligence", "shortly", "unto"],
    )?;
    segment.expression(
        10,
        "demas",
        &["hath", "forsaken", "having"],
        &["demas", "hath", "forsaken", "having", "loved"],
    )?;
    segment.move_value(
        11,
        "luke",
        &["take", "mark", "bring"],
        &["luke", "take", "mark", "bring", "thee"],
    )?;
    segment.move_value(
        12,
        "tychicus",
        &["sent", "ephesus"],
        &["tychicus", "sent", "ephesus"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        4,
        13,
        16,
        "cloke move_value -> expression -> guard cloke left troas",
        &["cloke", "left", "troas", "carpus", "comest"],
    );
    segment.move_value(
        13,
        "cloke",
        &["left", "troas", "carpus"],
        &["cloke", "left", "troas", "carpus", "comest"],
    )?;
    segment.expression(
        14,
        "lord",
        &["alexander", "coppersmith", "much"],
        &["lord", "alexander", "coppersmith", "much", "evil"],
    )?;
    segment.expression(
        15,
        "ware",
        &["hath", "greatly", "withstood"],
        &["ware", "hath", "greatly", "withstood", "words"],
    )?;
    segment.guard(
        16,
        "man",
        &["first", "answer", "stood"],
        &["man", "first", "answer", "stood", "men"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        4,
        17,
        19,
        "lord expression -> trait_bound lord notwithstanding stood",
        &[
            "lord",
            "notwithstanding",
            "stood",
            "strengthened",
            "preaching",
        ],
    );
    segment.expression(
        17,
        "lord",
        &["notwithstanding", "stood", "strengthened"],
        &[
            "lord",
            "notwithstanding",
            "stood",
            "strengthened",
            "preaching",
        ],
    )?;
    segment.trait_bound(
        18,
        "lord",
        &["shall", "deliver", "every"],
        &["lord", "shall", "deliver", "every", "evil"],
    )?;
    segment.expression(
        19,
        "salute",
        &["prisca", "aquila", "household"],
        &["salute", "prisca", "aquila", "household", "onesiphorus"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_timothy_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Timothy,
        4,
        20,
        22,
        "erastus expression -> move_value -> borrow erastus abode corinth",
        &["erastus", "abode", "corinth", "trophimus", "left"],
    );
    segment.expression(
        20,
        "erastus",
        &["abode", "corinth", "trophimus"],
        &["erastus", "abode", "corinth", "trophimus", "left"],
    )?;
    segment.move_value(
        21,
        "diligence",
        &["winter", "eubulus", "greeteth"],
        &["diligence", "winter", "eubulus", "greeteth", "thee"],
    )?;
    segment.borrow(
        22,
        "lord",
        &["jesus", "christ", "spirit"],
        &["lord", "jesus", "christ", "spirit", "grace"],
    )?;
    segment.commit()?;
    Ok(())
}
