use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    philippians(story)?;
    Ok(())
}

fn philippians(story: &mut Story) -> Result<(), StoryError> {
    philippians_1(story)?;
    philippians_2(story)?;
    philippians_3(story)?;
    philippians_4(story)?;
    Ok(())
}

fn philippians_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Philippians", 1, "Paul thanks the Philippians for partnership from the first commit and says his imprisonment has advanced the gospel through guards, coworkers, and public courage.", "Whether by life or death, Christ remains the central value; the church is urged to live as one team, standing firm without being frightened by opposition.");
    philippians_1_group_01(story)?;
    philippians_1_group_02(story)?;
    philippians_1_group_03(story)?;
    philippians_1_group_04(story)?;
    philippians_1_group_05(story)?;
    philippians_1_group_06(story)?;
    philippians_1_group_07(story)?;
    philippians_1_group_08(story)?;
    Ok(())
}

fn philippians_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Philippians", 2, "The community is called to the mind of Christ, who descends from status into obedient service and is then exalted above every name.", "Believers should work out their salvation with reverent seriousness, shining as lights while Timothy and Epaphroditus model dependable care.");
    philippians_2_group_01(story)?;
    philippians_2_group_02(story)?;
    philippians_2_group_03(story)?;
    philippians_2_group_04(story)?;
    philippians_2_group_05(story)?;
    philippians_2_group_06(story)?;
    philippians_2_group_07(story)?;
    philippians_2_group_08(story)?;
    Ok(())
}

fn philippians_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Philippians", 3, "Paul dumps his old credential stack, counting ancestry, law observance, and prestige as loss beside knowing Christ and sharing his resurrection life.", "He presses toward the prize, warns against appetites that make idols of the belly, and points to a citizenship whose future includes transformed bodies.");
    philippians_3_group_01(story)?;
    philippians_3_group_02(story)?;
    philippians_3_group_03(story)?;
    philippians_3_group_04(story)?;
    philippians_3_group_05(story)?;
    Ok(())
}

fn philippians_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Philippians", 4, "Paul asks Euodia and Syntyche to reconcile, then teaches joy, gentleness, prayer, and disciplined attention in an anxious world.", "The Philippians' gift is a fragrant act of partnership, and Paul has learned contentment in both scarcity and abundance through Christ's strength.");
    philippians_4_group_01(story)?;
    philippians_4_group_02(story)?;
    philippians_4_group_03(story)?;
    philippians_4_group_04(story)?;
    philippians_4_group_05(story)?;
    philippians_4_group_06(story)?;
    Ok(())
}

fn philippians_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        1,
        4,
        "paul expression -> borrow -> iterate -> result paul timotheus servants",
        &["paul", "timotheus", "servants", "jesus", "christ"],
    );
    segment.expression(
        1,
        "paul",
        &["timotheus", "servants", "jesus"],
        &["paul", "timotheus", "servants", "jesus", "christ"],
    )?;
    segment.borrow(
        2,
        "god",
        &["grace", "unto", "peace"],
        &["god", "grace", "unto", "peace", "father"],
    )?;
    segment.iterate(
        3,
        "god",
        &["thank", "every", "remembrance"],
        &["god", "thank", "every", "remembrance"],
    )?;
    segment.result(
        4,
        "always",
        &["every", "prayer", "mine"],
        &["always", "every", "prayer", "mine", "making"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        5,
        8,
        "fellowship expression -> borrow fellowship gospel first",
        &["fellowship", "gospel", "first", "day", "until"],
    );
    segment.expression(
        5,
        "fellowship",
        &["gospel", "first", "day"],
        &["fellowship", "gospel", "first", "day", "until"],
    )?;
    segment.expression(
        6,
        "confident",
        &["very", "thing", "hath"],
        &["confident", "very", "thing", "hath", "begun"],
    )?;
    segment.borrow(
        7,
        "even",
        &["meet", "think", "heart"],
        &["even", "meet", "think", "heart", "inasmuch"],
    )?;
    segment.expression(
        8,
        "god",
        &["record", "greatly", "long"],
        &["god", "record", "greatly", "long", "bowels"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        9,
        12,
        "pray result -> expression -> trait_bound pray love abound",
        &["pray", "love", "abound", "yet", "knowledge"],
    );
    segment.result(
        9,
        "pray",
        &["love", "abound", "yet"],
        &["pray", "love", "abound", "yet", "knowledge"],
    )?;
    segment.expression(
        10,
        "approve",
        &["things", "excellent", "sincere"],
        &["approve", "things", "excellent", "sincere", "without"],
    )?;
    segment.trait_bound(
        11,
        "jesus",
        &["filled", "fruits", "righteousness"],
        &["jesus", "filled", "fruits", "righteousness", "christ"],
    )?;
    segment.expression(
        12,
        "would",
        &["understand", "brethren", "things"],
        &["would", "understand", "brethren", "things", "happened"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        13,
        16,
        "christ expression -> call -> guard christ bonds manifest",
        &["christ", "bonds", "manifest", "palace", "other"],
    );
    segment.expression(
        13,
        "christ",
        &["bonds", "manifest", "palace"],
        &["christ", "bonds", "manifest", "palace", "other"],
    )?;
    segment.call(
        14,
        "lord",
        &["many", "brethren", "waxing"],
        &["lord", "many", "brethren", "waxing", "confident"],
    )?;
    segment.call(
        15,
        "christ",
        &["indeed", "preach", "even"],
        &["christ", "indeed", "preach", "even", "envy"],
    )?;
    segment.guard(
        16,
        "christ",
        &["preach", "contention", "not"],
        &["christ", "preach", "contention", "not", "sincerely"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        17,
        20,
        "other borrow -> match_value -> result -> drop_value other love knowing",
        &["other", "love", "knowing", "set", "defence"],
    );
    segment.borrow(
        17,
        "other",
        &["love", "knowing", "set"],
        &["other", "love", "knowing", "set", "defence"],
    )?;
    segment.match_value(
        18,
        "christ",
        &["notwithstanding", "every", "way"],
        &["christ", "notwithstanding", "every", "way", "whether"],
    )?;
    segment.result(
        19,
        "spirit",
        &["know", "shall", "turn"],
        &["spirit", "know", "shall", "turn", "salvation"],
    )?;
    segment.drop_value(
        20,
        "according",
        &["earnest", "expectation", "hope"],
        &["according", "earnest", "expectation", "hope", "nothing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        21,
        24,
        "christ expression -> guard -> move_value christ live die",
        &["christ", "live", "die", "gain", "flesh"],
    );
    segment.expression(
        21,
        "christ",
        &["live", "die", "gain"],
        &["christ", "live", "die", "gain"],
    )?;
    segment.guard(
        22,
        "live",
        &["flesh", "fruit", "labour"],
        &["live", "flesh", "fruit", "labour", "yet"],
    )?;
    segment.move_value(
        23,
        "christ",
        &["strait", "betwixt", "two"],
        &["christ", "strait", "betwixt", "two", "having"],
    )?;
    segment.expression(
        24,
        "nevertheless",
        &["abide", "flesh", "needful"],
        &["nevertheless", "abide", "flesh", "needful"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_1_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        25,
        27,
        "having await_value -> expression -> move_value having confidence know",
        &["having", "confidence", "know", "shall", "abide"],
    );
    segment.await_value(
        25,
        "having",
        &["confidence", "know", "shall"],
        &["having", "confidence", "know", "shall", "abide"],
    )?;
    segment.expression(
        26,
        "jesus",
        &["rejoicing", "abundant", "christ"],
        &["jesus", "rejoicing", "abundant", "christ", "coming"],
    )?;
    segment.move_value(
        27,
        "christ",
        &["conversation", "becometh", "gospel"],
        &["christ", "conversation", "becometh", "gospel", "whether"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_1_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        1,
        28,
        30,
        "god expression -> guard -> match_value god nothing terrified",
        &["god", "nothing", "terrified", "adversaries", "evident"],
    );
    segment.expression(
        28,
        "god",
        &["nothing", "terrified", "adversaries"],
        &["god", "nothing", "terrified", "adversaries", "evident"],
    )?;
    segment.guard(
        29,
        "christ",
        &["unto", "given", "behalf"],
        &["christ", "unto", "given", "behalf", "not"],
    )?;
    segment.match_value(
        30,
        "having",
        &["same", "conflict", "saw"],
        &["having", "same", "conflict", "saw", "now"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        1,
        4,
        "christ borrow -> iterate -> guard christ therefore any",
        &["christ", "therefore", "any", "consolation", "comfort"],
    );
    segment.borrow(
        1,
        "christ",
        &["therefore", "any", "consolation"],
        &["christ", "therefore", "any", "consolation", "comfort"],
    )?;
    segment.borrow(
        2,
        "fulfil",
        &["joy", "likeminded", "having"],
        &["fulfil", "joy", "likeminded", "having", "same"],
    )?;
    segment.iterate(
        3,
        "nothing",
        &["done", "through", "strife"],
        &["nothing", "done", "through", "strife", "vainglory"],
    )?;
    segment.guard(
        4,
        "man",
        &["look", "not", "every"],
        &["man", "look", "not", "every", "own"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        5,
        8,
        "christ expression -> guard -> construct -> drop_value christ mind jesus",
        &["christ", "mind", "jesus", "god", "form"],
    );
    segment.expression(
        5,
        "christ",
        &["mind", "jesus"],
        &["christ", "mind", "jesus"],
    )?;
    segment.guard(
        6,
        "god",
        &["form", "thought", "not"],
        &["god", "form", "thought", "not", "robbery"],
    )?;
    segment.construct(
        7,
        "men",
        &["made", "himself", "reputation"],
        &["men", "made", "himself", "reputation", "took"],
    )?;
    segment.drop_value(
        8,
        "man",
        &["found", "fashion", "humbled"],
        &["man", "found", "fashion", "humbled", "himself"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        9,
        12,
        "god iterate -> trait_bound -> guard god wherefore hath",
        &["god", "wherefore", "hath", "highly", "exalted"],
    );
    segment.iterate(
        9,
        "god",
        &["wherefore", "hath", "highly"],
        &["god", "wherefore", "hath", "highly", "exalted"],
    )?;
    segment.iterate(
        10,
        "jesus",
        &["name", "every", "knee"],
        &["jesus", "name", "every", "knee", "bow"],
    )?;
    segment.trait_bound(
        11,
        "jesus",
        &["every", "tongue", "confess"],
        &["jesus", "every", "tongue", "confess", "christ"],
    )?;
    segment.guard(
        12,
        "wherefore",
        &["beloved", "always", "obeyed"],
        &["wherefore", "beloved", "always", "obeyed", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        13,
        16,
        "god expression -> iterate -> guard god worketh both",
        &["god", "worketh", "both", "will", "good"],
    );
    segment.expression(
        13,
        "god",
        &["worketh", "both", "will"],
        &["god", "worketh", "both", "will", "good"],
    )?;
    segment.expression(
        14,
        "things",
        &["without", "murmurings", "disputings"],
        &["things", "without", "murmurings", "disputings"],
    )?;
    segment.iterate(
        15,
        "god",
        &["blameless", "harmless", "sons"],
        &["god", "blameless", "harmless", "sons", "without"],
    )?;
    segment.guard(
        16,
        "christ",
        &["holding", "forth", "word"],
        &["christ", "holding", "forth", "word", "life"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        17,
        20,
        "yea trait_bound -> expression -> move_value -> guard yea offered sacrifice",
        &["yea", "offered", "sacrifice", "service", "faith"],
    );
    segment.trait_bound(
        17,
        "yea",
        &["offered", "sacrifice", "service"],
        &["yea", "offered", "sacrifice", "service", "faith"],
    )?;
    segment.expression(
        18,
        "same",
        &["cause", "joy", "rejoice"],
        &["same", "cause", "joy", "rejoice"],
    )?;
    segment.move_value(
        19,
        "lord",
        &["trust", "jesus", "send"],
        &["lord", "trust", "jesus", "send", "timotheus"],
    )?;
    segment.guard(
        20,
        "man",
        &["likeminded", "will", "naturally"],
        &["man", "likeminded", "will", "naturally", "care"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        21,
        24,
        "jesus guard -> expression -> move_value jesus seek own",
        &["jesus", "seek", "own", "not", "things"],
    );
    segment.guard(
        21,
        "jesus",
        &["seek", "own", "not"],
        &["jesus", "seek", "own", "not", "things"],
    )?;
    segment.expression(
        22,
        "know",
        &["proof", "son", "father"],
        &["know", "proof", "son", "father", "hath"],
    )?;
    segment.move_value(
        23,
        "therefore",
        &["hope", "send", "presently"],
        &["therefore", "hope", "send", "presently", "soon"],
    )?;
    segment.move_value(
        24,
        "lord",
        &["trust", "myself", "shall"],
        &["lord", "trust", "myself", "shall", "shortly"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        25,
        27,
        "yet move_value -> expression -> drop_value yet supposed necessary",
        &["yet", "supposed", "necessary", "send", "epaphroditus"],
    );
    segment.move_value(
        25,
        "yet",
        &["supposed", "necessary", "send"],
        &["yet", "supposed", "necessary", "send", "epaphroditus"],
    )?;
    segment.expression(
        26,
        "longed",
        &["full", "heaviness", "heard"],
        &["longed", "full", "heaviness", "heard", "sick"],
    )?;
    segment.drop_value(
        27,
        "god",
        &["indeed", "sick", "nigh"],
        &["god", "indeed", "sick", "nigh", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_2_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        2,
        28,
        30,
        "sent move_value -> expression -> drop_value sent therefore carefully",
        &["sent", "therefore", "carefully", "see", "again"],
    );
    segment.move_value(
        28,
        "sent",
        &["therefore", "carefully", "see"],
        &["sent", "therefore", "carefully", "see", "again"],
    )?;
    segment.expression(
        29,
        "lord",
        &["receive", "therefore", "gladness"],
        &["lord", "receive", "therefore", "gladness", "hold"],
    )?;
    segment.drop_value(
        30,
        "christ",
        &["work", "nigh", "unto"],
        &["christ", "work", "nigh", "unto", "death"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        3,
        1,
        4,
        "lord guard -> expression -> await_value lord finally brethren",
        &["lord", "finally", "brethren", "rejoice", "write"],
    );
    segment.guard(
        1,
        "lord",
        &["finally", "brethren", "rejoice"],
        &["lord", "finally", "brethren", "rejoice", "write"],
    )?;
    segment.expression(
        2,
        "beware",
        &["dogs", "evil", "workers"],
        &["beware", "dogs", "evil", "workers", "concision"],
    )?;
    segment.guard(
        3,
        "god",
        &["circumcision", "worship", "spirit"],
        &["god", "circumcision", "worship", "spirit", "rejoice"],
    )?;
    segment.await_value(
        4,
        "man",
        &["though", "might", "confidence"],
        &["man", "though", "might", "confidence", "flesh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        3,
        5,
        8,
        "israel guard -> iterate -> expression israel circumcised eighth",
        &["israel", "circumcised", "eighth", "day", "stock"],
    );
    segment.guard(
        5,
        "israel",
        &["circumcised", "eighth", "day"],
        &["israel", "circumcised", "eighth", "day", "stock"],
    )?;
    segment.guard(
        6,
        "concerning",
        &["zeal", "persecuting", "church"],
        &["concerning", "zeal", "persecuting", "church", "touching"],
    )?;
    segment.iterate(
        7,
        "christ",
        &["things", "gain", "counted"],
        &["christ", "things", "gain", "counted", "loss"],
    )?;
    segment.expression(
        8,
        "christ",
        &["yea", "doubtless", "count"],
        &["christ", "yea", "doubtless", "count", "things"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        3,
        9,
        12,
        "found guard -> construct -> drop_value found not having",
        &["found", "not", "having", "mine", "own"],
    );
    segment.guard(
        9,
        "found",
        &["not", "having", "mine"],
        &["found", "not", "having", "mine", "own"],
    )?;
    segment.construct(
        10,
        "know",
        &["power", "resurrection", "fellowship"],
        &["know", "power", "resurrection", "fellowship", "sufferings"],
    )?;
    segment.drop_value(
        11,
        "any",
        &["means", "might", "attain"],
        &["any", "means", "might", "attain", "unto"],
    )?;
    segment.guard(
        12,
        "not",
        &["though", "already", "attained"],
        &["not", "though", "already", "attained", "either"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        3,
        13,
        16,
        "brethren guard -> expression brethren count not",
        &["brethren", "count", "not", "myself", "apprehended"],
    );
    segment.guard(
        13,
        "brethren",
        &["count", "not", "myself"],
        &["brethren", "count", "not", "myself", "apprehended"],
    )?;
    segment.expression(
        14,
        "god",
        &["press", "toward", "mark"],
        &["god", "press", "toward", "mark", "prize"],
    )?;
    segment.expression(
        15,
        "therefore",
        &["many", "perfect", "thus"],
        &["therefore", "many", "perfect", "thus", "minded"],
    )?;
    segment.expression(
        16,
        "nevertheless",
        &["whereto", "already", "attained"],
        &["nevertheless", "whereto", "already", "attained", "walk"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        3,
        17,
        21,
        "brethren expression -> trait_bound -> construct brethren followers together",
        &["brethren", "followers", "together", "mark", "walk"],
    );
    segment.expression(
        17,
        "brethren",
        &["followers", "together", "mark"],
        &["brethren", "followers", "together", "mark", "walk"],
    )?;
    segment.expression(
        18,
        "many",
        &["walk", "told", "often"],
        &["many", "walk", "told", "often", "now"],
    )?;
    segment.trait_bound(
        19,
        "god",
        &["whose", "end", "destruction"],
        &["god", "whose", "end", "destruction", "belly"],
    )?;
    segment.expression(
        20,
        "saviour",
        &["conversation", "heaven", "whence"],
        &["saviour", "conversation", "heaven", "whence", "look"],
    )?;
    segment.construct(
        21,
        "shall",
        &["change", "vile", "body"],
        &["shall", "change", "vile", "body", "fashioned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        4,
        1,
        4,
        "therefore borrow -> expression -> call therefore brethren dearly",
        &["therefore", "brethren", "dearly", "beloved", "longed"],
    );
    segment.borrow(
        1,
        "therefore",
        &["brethren", "dearly", "beloved"],
        &["therefore", "brethren", "dearly", "beloved", "longed"],
    )?;
    segment.expression(
        2,
        "lord",
        &["beseech", "euodias", "syntyche"],
        &["lord", "beseech", "euodias", "syntyche", "same"],
    )?;
    segment.expression(
        3,
        "women",
        &["intreat", "thee", "true"],
        &["women", "intreat", "thee", "true", "yokefellow"],
    )?;
    segment.call(
        4,
        "lord",
        &["rejoice", "always", "again"],
        &["lord", "rejoice", "always", "again", "say"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        4,
        5,
        8,
        "men expression -> construct -> guard -> trait_bound men moderation known",
        &["men", "moderation", "known", "unto", "lord"],
    );
    segment.expression(
        5,
        "men",
        &["moderation", "known", "unto"],
        &["men", "moderation", "known", "unto", "lord"],
    )?;
    segment.construct(
        6,
        "careful",
        &["nothing", "every", "thing"],
        &["careful", "nothing", "every", "thing", "prayer"],
    )?;
    segment.guard(
        7,
        "god",
        &["peace", "passeth", "understanding"],
        &["god", "peace", "passeth", "understanding", "shall"],
    )?;
    segment.trait_bound(
        8,
        "finally",
        &["brethren", "whatsoever", "things"],
        &["finally", "brethren", "whatsoever", "things", "true"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        4,
        9,
        12,
        "god expression -> guard -> iterate god things both",
        &["god", "things", "both", "learned", "received"],
    );
    segment.expression(
        9,
        "god",
        &["things", "both", "learned"],
        &["god", "things", "both", "learned", "received"],
    )?;
    segment.expression(
        10,
        "lord",
        &["rejoiced", "greatly", "now"],
        &["lord", "rejoiced", "greatly", "now", "last"],
    )?;
    segment.guard(
        11,
        "not",
        &["speak", "respect", "want"],
        &["not", "speak", "respect", "want", "learned"],
    )?;
    segment.iterate(
        12,
        "know",
        &["both", "abased", "abound"],
        &["know", "both", "abased", "abound", "every"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        4,
        13,
        16,
        "christ expression -> guard -> move_value christ things through",
        &[
            "christ",
            "things",
            "through",
            "strengtheneth",
            "notwithstanding",
        ],
    );
    segment.expression(
        13,
        "christ",
        &["things", "through", "strengtheneth"],
        &["christ", "things", "through", "strengtheneth"],
    )?;
    segment.expression(
        14,
        "notwithstanding",
        &["well", "done", "communicate"],
        &[
            "notwithstanding",
            "well",
            "done",
            "communicate",
            "affliction",
        ],
    )?;
    segment.guard(
        15,
        "now",
        &["philippians", "know", "beginning"],
        &["now", "philippians", "know", "beginning", "gospel"],
    )?;
    segment.move_value(
        16,
        "even",
        &["thessalonica", "sent", "once"],
        &["even", "thessalonica", "sent", "once", "again"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        4,
        17,
        20,
        "not guard -> move_value -> trait_bound not desire gift",
        &["not", "desire", "gift", "fruit", "abound"],
    );
    segment.guard(
        17,
        "not",
        &["desire", "gift", "fruit"],
        &["not", "desire", "gift", "fruit", "abound"],
    )?;
    segment.move_value(
        18,
        "abound",
        &["full", "having", "received"],
        &["abound", "full", "having", "received", "epaphroditus"],
    )?;
    segment.trait_bound(
        19,
        "god",
        &["shall", "supply", "need"],
        &["god", "shall", "supply", "need", "according"],
    )?;
    segment.trait_bound(
        20,
        "god",
        &["now", "unto", "father"],
        &["god", "now", "unto", "father", "glory"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philippians_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philippians,
        4,
        21,
        23,
        "christ iterate -> expression -> borrow christ salute every",
        &["christ", "salute", "every", "saint", "jesus"],
    );
    segment.iterate(
        21,
        "christ",
        &["salute", "every", "saint"],
        &["christ", "salute", "every", "saint", "jesus"],
    )?;
    segment.expression(
        22,
        "saints",
        &["salute", "chiefly", "caesar's"],
        &["saints", "salute", "chiefly", "caesar's", "household"],
    )?;
    segment.borrow(
        23,
        "lord",
        &["grace", "jesus", "christ"],
        &["lord", "grace", "jesus", "christ", "amen"],
    )?;
    segment.commit()?;
    Ok(())
}
