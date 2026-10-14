use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _1_peter(story)?;
    Ok(())
}

fn _1_peter(story: &mut Story) -> Result<(), StoryError> {
    _1_peter_1(story)?;
    _1_peter_2(story)?;
    _1_peter_3(story)?;
    _1_peter_4(story)?;
    _1_peter_5(story)?;
    Ok(())
}

fn _1_peter_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 Peter",
        1,
        "Exiles received a living hope through the resurrection, an inheritance",
        "stored beyond corruption, and joy that could survive a temporary trial.",
        "Their faith was refined like metal in a furnace, so they were summoned to",
        "holy conduct, sincere love, and obedience to the living word that had",
        "generated their new life."
    );
    _1_peter_1_group_01(story)?;
    _1_peter_1_group_02(story)?;
    _1_peter_1_group_03(story)?;
    _1_peter_1_group_04(story)?;
    _1_peter_1_group_05(story)?;
    _1_peter_1_group_06(story)?;
    Ok(())
}

fn _1_peter_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 Peter",
        2,
        "The believers were living stones assembled into a spiritual house, with",
        "Christ as the selected cornerstone that exposed every faulty foundation.",
        "They were also told to live honorably among outsiders, endure unjust",
        "treatment without returning evil, and follow the suffering servant who",
        "carried wounds toward healing."
    );
    _1_peter_2_group_01(story)?;
    _1_peter_2_group_02(story)?;
    _1_peter_2_group_03(story)?;
    _1_peter_2_group_04(story)?;
    _1_peter_2_group_05(story)?;
    _1_peter_2_group_06(story)?;
    Ok(())
}

fn _1_peter_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 Peter",
        3,
        "Households were instructed to practice patient honor, with beauty measured",
        "by a calm and courageous inner configuration rather than surface polish.",
        "The community was told to bless enemies, keep a clear conscience, and",
        "answer for its hope gently, remembering that the righteous one suffered",
        "once and now reigns over every hidden power."
    );
    _1_peter_3_group_01(story)?;
    _1_peter_3_group_02(story)?;
    _1_peter_3_group_03(story)?;
    _1_peter_3_group_04(story)?;
    _1_peter_3_group_05(story)?;
    _1_peter_3_group_06(story)?;
    Ok(())
}

fn _1_peter_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 Peter",
        4,
        "Since Christ suffered in the flesh, the readers were urged to stop living",
        "by the old request queue of drunkenness, rivalry, and idolized excess.",
        "They should expect strange hostility, serve one another with gifted",
        "stewardship, and rejoice when hardship arrives as a test rather than",
        "treating the trial as evidence that the project has been abandoned."
    );
    _1_peter_4_group_01(story)?;
    _1_peter_4_group_02(story)?;
    _1_peter_4_group_03(story)?;
    _1_peter_4_group_04(story)?;
    _1_peter_4_group_05(story)?;
    Ok(())
}

fn _1_peter_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 Peter",
        5,
        "Elders were told to shepherd without coercion, greed, or vanity, and the",
        "younger members were told to receive guidance while everyone wore humility.",
        "The adversary prowled like a hostile process, but the community could",
        "resist in faith, trusting that the God of grace would restore and",
        "strengthen them after the season of suffering."
    );
    _1_peter_5_group_01(story)?;
    _1_peter_5_group_02(story)?;
    _1_peter_5_group_03(story)?;
    _1_peter_5_group_04(story)?;
    Ok(())
}

fn _1_peter_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        1,
        1,
        4,
        "peter expression -> borrow -> construct -> guard peter apostle jesus",
        &["peter", "apostle", "jesus", "christ", "strangers"],
    );
    segment.expression(
        1,
        "peter",
        &["apostle", "jesus", "christ"],
        &["peter", "apostle", "jesus", "christ", "strangers"],
    )?;
    segment.borrow(
        2,
        "god",
        &["elect", "according", "foreknowledge"],
        &["god", "elect", "according", "foreknowledge", "father"],
    )?;
    segment.construct(
        3,
        "god",
        &["blessed", "father", "lord"],
        &["god", "blessed", "father", "lord", "jesus"],
    )?;
    segment.guard(
        4,
        "inheritance",
        &["incorruptible", "undefiled", "fadeth"],
        &["inheritance", "incorruptible", "undefiled", "fadeth", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        1,
        5,
        8,
        "god match_value -> expression -> trait_bound -> guard god kept power",
        &["god", "kept", "power", "through", "faith"],
    );
    segment.match_value(
        5,
        "god",
        &["kept", "power", "through"],
        &["god", "kept", "power", "through", "faith"],
    )?;
    segment.expression(
        6,
        "wherein",
        &["greatly", "rejoice", "though"],
        &["wherein", "greatly", "rejoice", "though", "now"],
    )?;
    segment.trait_bound(
        7,
        "trial",
        &["faith", "much", "precious"],
        &["trial", "faith", "much", "precious", "gold"],
    )?;
    segment.guard(
        8,
        "having",
        &["not", "seen", "love"],
        &["having", "not", "seen", "love", "though"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        1,
        9,
        12,
        "receiving await_value -> move_value -> trait_bound receiving end faith",
        &["receiving", "end", "faith", "even", "salvation"],
    );
    segment.await_value(
        9,
        "receiving",
        &["end", "faith", "even"],
        &["receiving", "end", "faith", "even", "salvation"],
    )?;
    segment.move_value(
        10,
        "salvation",
        &["prophets", "enquired", "searched"],
        &[
            "salvation",
            "prophets",
            "enquired",
            "searched",
            "diligently",
        ],
    )?;
    segment.trait_bound(
        11,
        "spirit",
        &["searching", "manner", "time"],
        &["spirit", "searching", "manner", "time", "christ"],
    )?;
    segment.move_value(
        12,
        "unto",
        &["revealed", "not", "themselves"],
        &["unto", "revealed", "not", "themselves", "minister"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        1,
        13,
        16,
        "wherefore move_value -> guard -> expression wherefore gird loins",
        &["wherefore", "gird", "loins", "mind", "sober"],
    );
    segment.move_value(
        13,
        "wherefore",
        &["gird", "loins", "mind"],
        &["wherefore", "gird", "loins", "mind", "sober"],
    )?;
    segment.guard(
        14,
        "children",
        &["obedient", "not", "fashioning"],
        &["children", "obedient", "not", "fashioning", "yourselves"],
    )?;
    segment.expression(
        15,
        "hath",
        &["called", "holy", "manner"],
        &["hath", "called", "holy", "manner", "conversation"],
    )?;
    segment.expression(16, "written", &["holy"], &["written", "holy"])?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        1,
        17,
        20,
        "call iterate -> guard -> expression call father without",
        &["call", "father", "without", "respect", "persons"],
    );
    segment.iterate(
        17,
        "call",
        &["father", "without", "respect"],
        &["call", "father", "without", "respect", "persons"],
    )?;
    segment.guard(
        18,
        "forasmuch",
        &["know", "not", "redeemed"],
        &["forasmuch", "know", "not", "redeemed", "corruptible"],
    )?;
    segment.expression(
        19,
        "christ",
        &["precious", "blood", "lamb"],
        &["christ", "precious", "blood", "lamb", "without"],
    )?;
    segment.expression(
        20,
        "verily",
        &["foreordained", "foundation", "world"],
        &["verily", "foreordained", "foundation", "world", "manifest"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        1,
        21,
        25,
        "god drop_value -> borrow -> construct -> trait_bound -> call god believe raised",
        &["god", "believe", "raised", "dead", "gave"],
    );
    segment.drop_value(
        21,
        "god",
        &["believe", "raised", "dead"],
        &["god", "believe", "raised", "dead", "gave"],
    )?;
    segment.borrow(
        22,
        "spirit",
        &["seeing", "purified", "souls"],
        &["spirit", "seeing", "purified", "souls", "obeying"],
    )?;
    segment.construct(
        23,
        "god",
        &["born", "again", "not"],
        &["god", "born", "again", "not", "corruptible"],
    )?;
    segment.trait_bound(
        24,
        "man",
        &["flesh", "grass", "glory"],
        &["man", "flesh", "grass", "glory", "flower"],
    )?;
    segment.call(
        25,
        "lord",
        &["word", "endureth", "ever"],
        &["lord", "word", "endureth", "ever", "gospel"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        2,
        1,
        4,
        "wherefore expression -> call wherefore laying aside",
        &["wherefore", "laying", "aside", "malice", "guile"],
    );
    segment.expression(
        1,
        "wherefore",
        &["laying", "aside", "malice"],
        &["wherefore", "laying", "aside", "malice", "guile"],
    )?;
    segment.call(
        2,
        "newborn",
        &["babes", "desire", "sincere"],
        &["newborn", "babes", "desire", "sincere", "milk"],
    )?;
    segment.expression(
        3,
        "lord",
        &["tasted", "gracious"],
        &["lord", "tasted", "gracious"],
    )?;
    segment.expression(
        4,
        "men",
        &["coming", "unto", "living"],
        &["men", "coming", "unto", "living", "stone"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        2,
        5,
        8,
        "lively construct -> guard -> call lively stones built",
        &["lively", "stones", "built", "spiritual", "house"],
    );
    segment.construct(
        5,
        "lively",
        &["stones", "built", "spiritual"],
        &["lively", "stones", "built", "spiritual", "house"],
    )?;
    segment.guard(
        6,
        "wherefore",
        &["contained", "scripture", "behold"],
        &["wherefore", "contained", "scripture", "behold", "lay"],
    )?;
    segment.construct(
        7,
        "unto",
        &["therefore", "believe", "precious"],
        &["unto", "therefore", "believe", "precious", "disobedient"],
    )?;
    segment.call(
        8,
        "stone",
        &["stumbling", "rock", "offence"],
        &["stone", "stumbling", "rock", "offence", "even"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        2,
        9,
        12,
        "nation expression -> guard -> borrow -> match_value nation chosen generation",
        &["nation", "chosen", "generation", "royal", "priesthood"],
    );
    segment.expression(
        9,
        "nation",
        &["chosen", "generation", "royal"],
        &["nation", "chosen", "generation", "royal", "priesthood"],
    )?;
    segment.guard(
        10,
        "people",
        &["time", "past", "not"],
        &["people", "time", "past", "not", "now"],
    )?;
    segment.borrow(
        11,
        "dearly",
        &["beloved", "beseech", "strangers"],
        &["dearly", "beloved", "beseech", "strangers", "pilgrims"],
    )?;
    segment.match_value(
        12,
        "having",
        &["conversation", "honest", "among"],
        &["having", "conversation", "honest", "among", "gentiles"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        2,
        13,
        16,
        "man guard -> move_value -> expression man submit yourselves",
        &["man", "submit", "yourselves", "every", "ordinance"],
    );
    segment.guard(
        13,
        "man",
        &["submit", "yourselves", "every"],
        &["man", "submit", "yourselves", "every", "ordinance"],
    )?;
    segment.move_value(
        14,
        "unto",
        &["governors", "sent", "punishment"],
        &["unto", "governors", "sent", "punishment", "evildoers"],
    )?;
    segment.expression(
        15,
        "god",
        &["will", "well", "doing"],
        &["god", "will", "well", "doing", "put"],
    )?;
    segment.guard(
        16,
        "god",
        &["free", "not", "using"],
        &["god", "free", "not", "using", "liberty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        2,
        17,
        20,
        "men borrow -> guard -> expression -> trait_bound men honour love",
        &["men", "honour", "love", "brotherhood", "fear"],
    );
    segment.borrow(
        17,
        "men",
        &["honour", "love", "brotherhood"],
        &["men", "honour", "love", "brotherhood", "fear"],
    )?;
    segment.guard(
        18,
        "servants",
        &["subject", "masters", "fear"],
        &["servants", "subject", "masters", "fear", "not"],
    )?;
    segment.expression(
        19,
        "man",
        &["thankworthy", "conscience", "toward"],
        &["man", "thankworthy", "conscience", "toward", "god"],
    )?;
    segment.trait_bound(
        20,
        "glory",
        &["buffeted", "faults", "shall"],
        &["glory", "buffeted", "faults", "shall", "take"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        2,
        21,
        25,
        "christ expression -> guard -> drop_value -> return_value christ even hereunto",
        &["christ", "even", "hereunto", "called", "suffered"],
    );
    segment.expression(
        21,
        "christ",
        &["even", "hereunto", "called"],
        &["christ", "even", "hereunto", "called", "suffered"],
    )?;
    segment.guard(
        22,
        "sin",
        &["neither", "guile", "found"],
        &["sin", "neither", "guile", "found", "mouth"],
    )?;
    segment.guard(
        23,
        "reviled",
        &["not", "again", "suffered"],
        &["reviled", "not", "again", "suffered", "threatened"],
    )?;
    segment.drop_value(
        24,
        "own",
        &["self", "bare", "sins"],
        &["own", "self", "bare", "sins", "body"],
    )?;
    segment.return_value(
        25,
        "sheep",
        &["going", "astray", "now"],
        &["sheep", "going", "astray", "now", "returned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        3,
        1,
        4,
        "likewise guard -> match_value likewise wives subjection",
        &["likewise", "wives", "subjection", "own", "husbands"],
    );
    segment.guard(
        1,
        "likewise",
        &["wives", "subjection", "own"],
        &["likewise", "wives", "subjection", "own", "husbands"],
    )?;
    segment.match_value(
        2,
        "while",
        &["behold", "chaste", "conversation"],
        &["while", "behold", "chaste", "conversation", "coupled"],
    )?;
    segment.guard(
        3,
        "whose",
        &["adorning", "not", "outward"],
        &["whose", "adorning", "not", "outward", "plaiting"],
    )?;
    segment.guard(
        4,
        "man",
        &["hidden", "heart", "not"],
        &["man", "hidden", "heart", "not", "corruptible"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        3,
        5,
        8,
        "women expression -> guard -> borrow women manner old",
        &["women", "manner", "old", "time", "holy"],
    );
    segment.expression(
        5,
        "women",
        &["manner", "old", "time"],
        &["women", "manner", "old", "time", "holy"],
    )?;
    segment.guard(
        6,
        "abraham",
        &["even", "sara", "obeyed"],
        &["abraham", "even", "sara", "obeyed", "calling"],
    )?;
    segment.guard(
        7,
        "likewise",
        &["husbands", "dwell", "according"],
        &["likewise", "husbands", "dwell", "according", "knowledge"],
    )?;
    segment.borrow(
        8,
        "finally",
        &["mind", "having", "compassion"],
        &["finally", "mind", "having", "compassion", "another"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        3,
        9,
        12,
        "not guard -> expression not rendering evil",
        &["not", "rendering", "evil", "railing", "contrariwise"],
    );
    segment.guard(
        9,
        "not",
        &["rendering", "evil", "railing"],
        &["not", "rendering", "evil", "railing", "contrariwise"],
    )?;
    segment.guard(
        10,
        "will",
        &["love", "life", "see"],
        &["will", "love", "life", "see", "good"],
    )?;
    segment.expression(
        11,
        "eschew",
        &["evil", "good", "seek"],
        &["eschew", "evil", "good", "seek", "peace"],
    )?;
    segment.expression(
        12,
        "lord",
        &["eyes", "over", "righteous"],
        &["lord", "eyes", "over", "righteous", "ears"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        3,
        13,
        16,
        "will expression -> guard -> await_value -> call will harm followers",
        &["will", "harm", "followers", "good", "suffer"],
    );
    segment.expression(
        13,
        "will",
        &["harm", "followers", "good"],
        &["will", "harm", "followers", "good"],
    )?;
    segment.guard(
        14,
        "suffer",
        &["righteousness", "sake", "happy"],
        &["suffer", "righteousness", "sake", "happy", "not"],
    )?;
    segment.await_value(
        15,
        "lord",
        &["sanctify", "god", "hearts"],
        &["lord", "sanctify", "god", "hearts", "ready"],
    )?;
    segment.call(
        16,
        "having",
        &["good", "conscience", "whereas"],
        &["having", "good", "conscience", "whereas", "speak"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        3,
        17,
        19,
        "god expression -> move_value god better will",
        &["god", "better", "will", "suffer", "well"],
    );
    segment.expression(
        17,
        "god",
        &["better", "will", "suffer"],
        &["god", "better", "will", "suffer", "well"],
    )?;
    segment.move_value(
        18,
        "christ",
        &["hath", "once", "suffered"],
        &["christ", "hath", "once", "suffered", "sins"],
    )?;
    segment.move_value(
        19,
        "went",
        &["preached", "unto", "spirits"],
        &["went", "preached", "unto", "spirits", "prison"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        3,
        20,
        22,
        "god expression -> reinitialize -> construct god sometime disobedient",
        &["god", "sometime", "disobedient", "once", "longsuffering"],
    );
    segment.expression(
        20,
        "god",
        &["sometime", "disobedient", "once"],
        &["god", "sometime", "disobedient", "once", "longsuffering"],
    )?;
    segment.reinitialize(
        21,
        "like",
        &["figure", "whereunto", "even"],
        &["like", "figure", "whereunto", "even", "baptism"],
    )?;
    segment.construct(
        22,
        "god",
        &["gone", "heaven", "right"],
        &["god", "gone", "heaven", "right", "hand"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        4,
        1,
        4,
        "christ expression -> guard christ forasmuch hath",
        &["christ", "forasmuch", "hath", "suffered", "flesh"],
    );
    segment.expression(
        1,
        "christ",
        &["forasmuch", "hath", "suffered"],
        &["christ", "forasmuch", "hath", "suffered", "flesh"],
    )?;
    segment.guard(
        2,
        "men",
        &["longer", "live", "rest"],
        &["men", "longer", "live", "rest", "time"],
    )?;
    segment.expression(
        3,
        "time",
        &["past", "life", "suffice"],
        &["time", "past", "life", "suffice", "wrought"],
    )?;
    segment.guard(
        4,
        "wherein",
        &["think", "strange", "run"],
        &["wherein", "think", "strange", "run", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        4,
        5,
        8,
        "shall drop_value -> result -> borrow shall give account",
        &["shall", "give", "account", "ready", "judge"],
    );
    segment.drop_value(
        5,
        "shall",
        &["give", "account", "ready"],
        &["shall", "give", "account", "ready", "judge"],
    )?;
    segment.drop_value(
        6,
        "men",
        &["cause", "gospel", "preached"],
        &["men", "cause", "gospel", "preached", "dead"],
    )?;
    segment.result(
        7,
        "end",
        &["things", "hand", "therefore"],
        &["end", "things", "hand", "therefore", "sober"],
    )?;
    segment.borrow(
        8,
        "above",
        &["things", "fervent", "charity"],
        &["above", "things", "fervent", "charity", "among"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        4,
        9,
        12,
        "use expression -> borrow -> call -> guard use hospitality another",
        &["use", "hospitality", "another", "without", "grudging"],
    );
    segment.expression(
        9,
        "use",
        &["hospitality", "another", "without"],
        &["use", "hospitality", "another", "without", "grudging"],
    )?;
    segment.borrow(
        10,
        "man",
        &["every", "hath", "received"],
        &["man", "every", "hath", "received", "gift"],
    )?;
    segment.call(
        11,
        "man",
        &["any", "speak", "oracles"],
        &["man", "any", "speak", "oracles", "god"],
    )?;
    segment.guard(
        12,
        "beloved",
        &["think", "not", "strange"],
        &["beloved", "think", "not", "strange", "concerning"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        4,
        13,
        16,
        "rejoice match_value -> trait_bound -> expression -> guard rejoice inasmuch partakers",
        &["rejoice", "inasmuch", "partakers", "christ's", "sufferings"],
    );
    segment.match_value(
        13,
        "rejoice",
        &["inasmuch", "partakers", "christ's"],
        &["rejoice", "inasmuch", "partakers", "christ's", "sufferings"],
    )?;
    segment.trait_bound(
        14,
        "christ",
        &["reproached", "name", "happy"],
        &["christ", "reproached", "name", "happy", "spirit"],
    )?;
    segment.expression(
        15,
        "none",
        &["suffer", "murderer", "thief"],
        &["none", "suffer", "murderer", "thief", "evildoer"],
    )?;
    segment.guard(
        16,
        "man",
        &["yet", "any", "suffer"],
        &["man", "yet", "any", "suffer", "christian"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        4,
        17,
        19,
        "god move_value -> expression god time judgment",
        &["god", "time", "judgment", "must", "begin"],
    );
    segment.move_value(
        17,
        "god",
        &["time", "judgment", "must"],
        &["god", "time", "judgment", "must", "begin"],
    )?;
    segment.expression(
        18,
        "righteous",
        &["scarcely", "saved", "shall"],
        &["righteous", "scarcely", "saved", "shall", "ungodly"],
    )?;
    segment.expression(
        19,
        "god",
        &["wherefore", "suffer", "according"],
        &["god", "wherefore", "suffer", "according", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        5,
        1,
        4,
        "christ match_value -> guard christ elders among",
        &["christ", "elders", "among", "exhort", "elder"],
    );
    segment.match_value(
        1,
        "christ",
        &["elders", "among", "exhort"],
        &["christ", "elders", "among", "exhort", "elder"],
    )?;
    segment.guard(
        2,
        "god",
        &["feed", "flock", "among"],
        &["god", "feed", "flock", "among", "taking"],
    )?;
    segment.guard(
        3,
        "neither",
        &["lords", "over", "god's"],
        &["neither", "lords", "over", "god's", "heritage"],
    )?;
    segment.guard(
        4,
        "chief",
        &["shepherd", "shall", "appear"],
        &["chief", "shepherd", "shall", "appear", "receive"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        5,
        5,
        8,
        "likewise borrow -> expression likewise younger submit",
        &["likewise", "younger", "submit", "yourselves", "unto"],
    );
    segment.borrow(
        5,
        "likewise",
        &["younger", "submit", "yourselves"],
        &["likewise", "younger", "submit", "yourselves", "unto"],
    )?;
    segment.expression(
        6,
        "god",
        &["humble", "yourselves", "therefore"],
        &["god", "humble", "yourselves", "therefore", "under"],
    )?;
    segment.expression(
        7,
        "casting",
        &["care", "careth"],
        &["casting", "care", "careth"],
    )?;
    segment.expression(
        8,
        "sober",
        &["vigilant", "adversary", "devil"],
        &["sober", "vigilant", "adversary", "devil", "roaring"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        5,
        9,
        11,
        "resist await_value -> construct -> trait_bound resist stedfast faith",
        &["resist", "stedfast", "faith", "knowing", "same"],
    );
    segment.await_value(
        9,
        "resist",
        &["stedfast", "faith", "knowing"],
        &["resist", "stedfast", "faith", "knowing", "same"],
    )?;
    segment.construct(
        10,
        "god",
        &["grace", "hath", "called"],
        &["god", "grace", "hath", "called", "unto"],
    )?;
    segment.trait_bound(
        11,
        "glory",
        &["dominion", "ever", "amen"],
        &["glory", "dominion", "ever", "amen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_peter_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_Peter,
        5,
        12,
        14,
        "silvanus borrow -> expression silvanus faithful brother",
        &["silvanus", "faithful", "brother", "unto", "suppose"],
    );
    segment.borrow(
        12,
        "silvanus",
        &["faithful", "brother", "unto"],
        &["silvanus", "faithful", "brother", "unto", "suppose"],
    )?;
    segment.expression(
        13,
        "church",
        &["babylon", "elected", "together"],
        &["church", "babylon", "elected", "together", "saluteth"],
    )?;
    segment.borrow(
        14,
        "christ",
        &["greet", "another", "kiss"],
        &["christ", "greet", "another", "kiss", "charity"],
    )?;
    segment.commit()?;
    Ok(())
}
