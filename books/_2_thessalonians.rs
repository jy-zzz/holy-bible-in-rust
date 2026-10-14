use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _2_thessalonians(story)?;
    Ok(())
}

fn _2_thessalonians(story: &mut Story) -> Result<(), StoryError> {
    _2_thessalonians_1(story)?;
    _2_thessalonians_2(story)?;
    _2_thessalonians_3(story)?;
    Ok(())
}

fn _2_thessalonians_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Thessalonians", 1, "Paul praises the church's growing faith and endurance under persecution, describing righteous judgment and relief when Jesus is revealed.", "He prays that God will make them worthy of the calling and empower every good purpose, so Christ's name is honored in them.");
    _2_thessalonians_1_group_01(story)?;
    _2_thessalonians_1_group_02(story)?;
    _2_thessalonians_1_group_03(story)?;
    Ok(())
}

fn _2_thessalonians_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Thessalonians", 2, "Paul corrects rumors that the day of the Lord has already arrived, pointing to a rebellion and a lawless figure before the final unveiling.", "The church must stand firm in the received teaching while God strengthens its hope, comfort, and every good work.");
    _2_thessalonians_2_group_01(story)?;
    _2_thessalonians_2_group_02(story)?;
    _2_thessalonians_2_group_03(story)?;
    _2_thessalonians_2_group_04(story)?;
    Ok(())
}

fn _2_thessalonians_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Thessalonians", 3, "Paul asks for prayer that the message will run freely and that the church will be guarded from hostile and unreliable people.", "He confronts idleness directly: those unwilling to work should not consume the community's shared table, though correction must remain brotherly rather than hateful.");
    _2_thessalonians_3_group_01(story)?;
    _2_thessalonians_3_group_02(story)?;
    _2_thessalonians_3_group_03(story)?;
    _2_thessalonians_3_group_04(story)?;
    _2_thessalonians_3_group_05(story)?;
    Ok(())
}

fn _2_thessalonians_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        1,
        1,
        4,
        "paul expression -> borrow -> await_value -> trait_bound paul silvanus timotheus",
        &["paul", "silvanus", "timotheus", "unto", "church"],
    );
    segment.expression(
        1,
        "paul",
        &["silvanus", "timotheus", "unto"],
        &["paul", "silvanus", "timotheus", "unto", "church"],
    )?;
    segment.borrow(
        2,
        "god",
        &["grace", "unto", "peace"],
        &["god", "grace", "unto", "peace", "father"],
    )?;
    segment.await_value(
        3,
        "god",
        &["bound", "thank", "always"],
        &["god", "bound", "thank", "always", "brethren"],
    )?;
    segment.trait_bound(
        4,
        "god",
        &["ourselves", "glory", "churches"],
        &["god", "ourselves", "glory", "churches", "patience"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        1,
        5,
        8,
        "god iterate -> expression -> match_value -> guard god manifest token",
        &["god", "manifest", "token", "righteous", "judgment"],
    );
    segment.iterate(
        5,
        "god",
        &["manifest", "token", "righteous"],
        &["god", "manifest", "token", "righteous", "judgment"],
    )?;
    segment.expression(
        6,
        "god",
        &["seeing", "righteous", "thing"],
        &["god", "seeing", "righteous", "thing", "recompense"],
    )?;
    segment.match_value(
        7,
        "lord",
        &["troubled", "rest", "jesus"],
        &["lord", "troubled", "rest", "jesus", "shall"],
    )?;
    segment.guard(
        8,
        "god",
        &["flaming", "fire", "taking"],
        &["god", "flaming", "fire", "taking", "vengeance"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        1,
        9,
        12,
        "lord trait_bound -> move_value -> result -> borrow lord shall punished",
        &["lord", "shall", "punished", "everlasting", "destruction"],
    );
    segment.trait_bound(
        9,
        "lord",
        &["shall", "punished", "everlasting"],
        &["lord", "shall", "punished", "everlasting", "destruction"],
    )?;
    segment.move_value(
        10,
        "shall",
        &["glorified", "saints", "admired"],
        &["shall", "glorified", "saints", "admired", "believe"],
    )?;
    segment.result(
        11,
        "god",
        &["wherefore", "pray", "always"],
        &["god", "wherefore", "pray", "always", "would"],
    )?;
    segment.borrow(
        12,
        "lord",
        &["name", "jesus", "christ"],
        &["lord", "name", "jesus", "christ", "glorified"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        2,
        1,
        4,
        "lord expression -> guard -> move_value -> trait_bound lord now beseech",
        &["lord", "now", "beseech", "brethren", "coming"],
    );
    segment.expression(
        1,
        "lord",
        &["now", "beseech", "brethren"],
        &["lord", "now", "beseech", "brethren", "coming"],
    )?;
    segment.guard(
        2,
        "spirit",
        &["not", "soon", "shaken"],
        &["spirit", "not", "soon", "shaken", "mind"],
    )?;
    segment.move_value(
        3,
        "man",
        &["deceive", "any", "means"],
        &["man", "deceive", "any", "means", "day"],
    )?;
    segment.trait_bound(
        4,
        "god",
        &["opposeth", "exalteth", "himself"],
        &["god", "opposeth", "exalteth", "himself", "above"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        2,
        5,
        8,
        "remember guard -> match_value -> expression -> drop_value remember not yet",
        &["remember", "not", "yet", "told", "things"],
    );
    segment.guard(
        5,
        "remember",
        &["not", "yet", "told"],
        &["remember", "not", "yet", "told", "things"],
    )?;
    segment.match_value(
        6,
        "now",
        &["know", "withholdeth", "might"],
        &["now", "know", "withholdeth", "might", "revealed"],
    )?;
    segment.expression(
        7,
        "mystery",
        &["iniquity", "doth", "already"],
        &["mystery", "iniquity", "doth", "already", "work"],
    )?;
    segment.drop_value(
        8,
        "lord",
        &["shall", "wicked", "revealed"],
        &["lord", "shall", "wicked", "revealed", "consume"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        2,
        9,
        12,
        "even expression -> guard -> move_value even whose coming",
        &["even", "whose", "coming", "working", "satan"],
    );
    segment.expression(
        9,
        "even",
        &["whose", "coming", "working"],
        &["even", "whose", "coming", "working", "satan"],
    )?;
    segment.guard(
        10,
        "deceivableness",
        &["unrighteousness", "perish", "received"],
        &[
            "deceivableness",
            "unrighteousness",
            "perish",
            "received",
            "not",
        ],
    )?;
    segment.move_value(
        11,
        "god",
        &["cause", "shall", "send"],
        &["god", "cause", "shall", "send", "strong"],
    )?;
    segment.guard(
        12,
        "might",
        &["damned", "believed", "not"],
        &["might", "damned", "believed", "not", "truth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        2,
        13,
        17,
        "god borrow -> trait_bound -> match_value -> await_value -> call god bound give",
        &["god", "bound", "give", "thanks", "alway"],
    );
    segment.borrow(
        13,
        "god",
        &["bound", "give", "thanks"],
        &["god", "bound", "give", "thanks", "alway"],
    )?;
    segment.trait_bound(
        14,
        "lord",
        &["whereunto", "called", "gospel"],
        &["lord", "whereunto", "called", "gospel", "obtaining"],
    )?;
    segment.match_value(
        15,
        "therefore",
        &["brethren", "stand", "fast"],
        &["therefore", "brethren", "stand", "fast", "hold"],
    )?;
    segment.await_value(
        16,
        "lord",
        &["now", "jesus", "christ"],
        &["lord", "now", "jesus", "christ", "himself"],
    )?;
    segment.call(
        17,
        "comfort",
        &["hearts", "stablish", "every"],
        &["comfort", "hearts", "stablish", "every", "good"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        3,
        1,
        4,
        "lord result -> guard lord finally brethren",
        &["lord", "finally", "brethren", "pray", "word"],
    );
    segment.result(
        1,
        "lord",
        &["finally", "brethren", "pray"],
        &["lord", "finally", "brethren", "pray", "word"],
    )?;
    segment.guard(
        2,
        "men",
        &["delivered", "unreasonable", "wicked"],
        &["men", "delivered", "unreasonable", "wicked", "not"],
    )?;
    segment.guard(
        3,
        "lord",
        &["faithful", "shall", "stablish"],
        &["lord", "faithful", "shall", "stablish", "keep"],
    )?;
    segment.guard(
        4,
        "lord",
        &["confidence", "touching", "both"],
        &["lord", "confidence", "touching", "both", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        3,
        5,
        8,
        "lord await_value -> guard lord direct hearts",
        &["lord", "direct", "hearts", "love", "god"],
    );
    segment.await_value(
        5,
        "lord",
        &["direct", "hearts", "love"],
        &["lord", "direct", "hearts", "love", "god"],
    )?;
    segment.guard(
        6,
        "lord",
        &["now", "command", "brethren"],
        &["lord", "now", "command", "brethren", "name"],
    )?;
    segment.guard(
        7,
        "yourselves",
        &["know", "ought", "follow"],
        &["yourselves", "know", "ought", "follow", "behaved"],
    )?;
    segment.guard(
        8,
        "neither",
        &["eat", "any", "man's"],
        &["neither", "eat", "any", "man's", "bread"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        3,
        9,
        12,
        "not construct -> guard not power make",
        &["not", "power", "make", "ourselves", "ensample"],
    );
    segment.construct(
        9,
        "not",
        &["power", "make", "ourselves"],
        &["not", "power", "make", "ourselves", "ensample"],
    )?;
    segment.guard(
        10,
        "even",
        &["commanded", "any", "would"],
        &["even", "commanded", "any", "would", "not"],
    )?;
    segment.guard(
        11,
        "hear",
        &["walk", "among", "disorderly"],
        &["hear", "walk", "among", "disorderly", "working"],
    )?;
    segment.guard(
        12,
        "lord",
        &["now", "such", "command"],
        &["lord", "now", "such", "command", "exhort"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        3,
        13,
        15,
        "brethren guard brethren not weary",
        &["brethren", "not", "weary", "well", "doing"],
    );
    segment.guard(
        13,
        "brethren",
        &["not", "weary", "well"],
        &["brethren", "not", "weary", "well", "doing"],
    )?;
    segment.guard(
        14,
        "man",
        &["any", "obey", "not"],
        &["man", "any", "obey", "not", "word"],
    )?;
    segment.guard(
        15,
        "yet",
        &["count", "not", "enemy"],
        &["yet", "count", "not", "enemy", "admonish"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _2_thessalonians_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_2_Thessalonians,
        3,
        16,
        18,
        "lord expression -> iterate -> borrow lord now peace",
        &["lord", "now", "peace", "himself", "give"],
    );
    segment.expression(
        16,
        "lord",
        &["now", "peace", "himself"],
        &["lord", "now", "peace", "himself", "give"],
    )?;
    segment.iterate(
        17,
        "paul",
        &["salutation", "mine", "own"],
        &["paul", "salutation", "mine", "own", "hand"],
    )?;
    segment.borrow(
        18,
        "lord",
        &["grace", "jesus", "christ"],
        &["lord", "grace", "jesus", "christ", "amen"],
    )?;
    segment.commit()?;
    Ok(())
}
