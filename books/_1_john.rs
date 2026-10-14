use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    _1_john(story)?;
    Ok(())
}

fn _1_john(story: &mut Story) -> Result<(), StoryError> {
    _1_john_1(story)?;
    _1_john_2(story)?;
    _1_john_3(story)?;
    _1_john_4(story)?;
    _1_john_5(story)?;
    Ok(())
}

fn _1_john_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 John",
        1,
        "The witness began with what had been heard, seen, and touched: life made",
        "visible rather than an abstract interface discussed from a distance.",
        "God was light with no hidden error state, so fellowship required walking",
        "honestly, confessing failures, and trusting the advocate whose cleansing",
        "restores communication."
    );
    _1_john_1_group_01(story)?;
    _1_john_1_group_02(story)?;
    _1_john_1_group_03(story)?;
    Ok(())
}

fn _1_john_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 John",
        2,
        "The Son was the advocate for sin-stained users and the atoning provision",
        "whose reach was wider than one private team.",
        "Love for God showed itself in keeping the command, refusing love for the",
        "world's passing appetite, and remaining in the teaching received from the",
        "beginning instead of following the antichrist's counterfeit fork."
    );
    _1_john_2_group_01(story)?;
    _1_john_2_group_02(story)?;
    _1_john_2_group_03(story)?;
    _1_john_2_group_04(story)?;
    _1_john_2_group_05(story)?;
    _1_john_2_group_06(story)?;
    _1_john_2_group_07(story)?;
    Ok(())
}

fn _1_john_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 John",
        3,
        "The community's adoption was real even if the final output had not yet",
        "rendered, and that future likeness created a reason to purify present",
        "behavior.",
        "Love was not a comment in a readme: the Son's self-giving defined it, and",
        "believers had to share resources with a sibling in need rather than",
        "closing their compassion behind a permissions error."
    );
    _1_john_3_group_01(story)?;
    _1_john_3_group_02(story)?;
    _1_john_3_group_03(story)?;
    _1_john_3_group_04(story)?;
    _1_john_3_group_05(story)?;
    _1_john_3_group_06(story)?;
    Ok(())
}

fn _1_john_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 John",
        4,
        "Not every spirit passed review, so the church was told to test messages",
        "by whether they confess the Son truly rather than by how impressive they",
        "sound in a demo.",
        "God's love moved first and cast out fear, making love for unseen God",
        "inseparable from practical love for visible brothers and sisters."
    );
    _1_john_4_group_01(story)?;
    _1_john_4_group_02(story)?;
    _1_john_4_group_03(story)?;
    _1_john_4_group_04(story)?;
    _1_john_4_group_05(story)?;
    Ok(())
}

fn _1_john_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "1 John",
        5,
        "Faith in the Son became the victory that overcomes the world and the",
        "testimony that God had given life rather than merely promising a patch.",
        "The letter closed with confidence in prayer, a distinction between sins",
        "that lead toward death and those that do not, and a blunt warning to keep",
        "away from idols that impersonate the true God."
    );
    _1_john_5_group_01(story)?;
    _1_john_5_group_02(story)?;
    _1_john_5_group_03(story)?;
    _1_john_5_group_04(story)?;
    _1_john_5_group_05(story)?;
    Ok(())
}

fn _1_john_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        1,
        1,
        4,
        "beginning call -> expression beginning heard seen",
        &["beginning", "heard", "seen", "eyes", "looked"],
    );
    segment.call(
        1,
        "beginning",
        &["heard", "seen", "eyes"],
        &["beginning", "heard", "seen", "eyes", "looked"],
    )?;
    segment.expression(
        2,
        "life",
        &["manifested", "seen", "bear"],
        &["life", "manifested", "seen", "bear", "witness"],
    )?;
    segment.call(
        3,
        "seen",
        &["heard", "declare", "unto"],
        &["seen", "heard", "declare", "unto", "fellowship"],
    )?;
    segment.expression(
        4,
        "things",
        &["write", "unto", "joy"],
        &["things", "write", "unto", "joy", "full"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        1,
        5,
        7,
        "god guard -> expression god message heard",
        &["god", "message", "heard", "declare", "unto"],
    );
    segment.guard(
        5,
        "god",
        &["message", "heard", "declare"],
        &["god", "message", "heard", "declare", "unto"],
    )?;
    segment.guard(
        6,
        "say",
        &["fellowship", "walk", "darkness"],
        &["say", "fellowship", "walk", "darkness", "lie"],
    )?;
    segment.expression(
        7,
        "jesus",
        &["walk", "light", "fellowship"],
        &["jesus", "walk", "light", "fellowship", "another"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        1,
        8,
        10,
        "say guard -> expression -> construct say sin deceive",
        &["say", "sin", "deceive", "ourselves", "truth"],
    );
    segment.guard(
        8,
        "say",
        &["sin", "deceive", "ourselves"],
        &["say", "sin", "deceive", "ourselves", "truth"],
    )?;
    segment.expression(
        9,
        "confess",
        &["sins", "faithful", "just"],
        &["confess", "sins", "faithful", "just", "forgive"],
    )?;
    segment.construct(
        10,
        "say",
        &["not", "sinned", "make"],
        &["say", "not", "sinned", "make", "liar"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        2,
        1,
        4,
        "children guard children little things",
        &["children", "little", "things", "write", "unto"],
    );
    segment.guard(
        1,
        "children",
        &["little", "things", "write"],
        &["children", "little", "things", "write", "unto"],
    )?;
    segment.guard(
        2,
        "propitiation",
        &["sins", "not", "ours"],
        &["propitiation", "sins", "not", "ours", "whole"],
    )?;
    segment.guard(
        3,
        "hereby",
        &["know", "keep", "commandments"],
        &["hereby", "know", "keep", "commandments"],
    )?;
    segment.guard(
        4,
        "saith",
        &["know", "keepeth", "not"],
        &["saith", "know", "keepeth", "not", "commandments"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        2,
        5,
        8,
        "god call -> expression -> reinitialize god whoso keepeth",
        &["god", "whoso", "keepeth", "word", "verily"],
    );
    segment.call(
        5,
        "god",
        &["whoso", "keepeth", "word"],
        &["god", "whoso", "keepeth", "word", "verily"],
    )?;
    segment.expression(
        6,
        "saith",
        &["abideth", "ought", "himself"],
        &["saith", "abideth", "ought", "himself", "walk"],
    )?;
    segment.reinitialize(
        7,
        "brethren",
        &["write", "new", "commandment"],
        &["brethren", "write", "new", "commandment", "unto"],
    )?;
    segment.reinitialize(
        8,
        "again",
        &["new", "commandment", "write"],
        &["again", "new", "commandment", "write", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        2,
        9,
        12,
        "saith expression -> guard saith light hateth",
        &["saith", "light", "hateth", "brother", "darkness"],
    );
    segment.expression(
        9,
        "saith",
        &["light", "hateth", "brother"],
        &["saith", "light", "hateth", "brother", "darkness"],
    )?;
    segment.expression(
        10,
        "loveth",
        &["brother", "abideth", "light"],
        &["loveth", "brother", "abideth", "light", "none"],
    )?;
    segment.guard(
        11,
        "hateth",
        &["brother", "darkness", "walketh"],
        &["hateth", "brother", "darkness", "walketh", "knoweth"],
    )?;
    segment.expression(
        12,
        "children",
        &["write", "unto", "little"],
        &["children", "write", "unto", "little", "sins"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        2,
        13,
        16,
        "men expression -> call -> guard men write unto",
        &["men", "write", "unto", "fathers", "known"],
    );
    segment.expression(
        13,
        "men",
        &["write", "unto", "fathers"],
        &["men", "write", "unto", "fathers", "known"],
    )?;
    segment.call(
        14,
        "men",
        &["written", "unto", "fathers"],
        &["men", "written", "unto", "fathers", "known"],
    )?;
    segment.guard(
        15,
        "man",
        &["love", "not", "world"],
        &["man", "love", "not", "world", "neither"],
    )?;
    segment.guard(
        16,
        "world",
        &["lust", "flesh", "eyes"],
        &["world", "lust", "flesh", "eyes", "pride"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        2,
        17,
        20,
        "god expression -> move_value -> construct god world passeth",
        &["god", "world", "passeth", "away", "lust"],
    );
    segment.expression(
        17,
        "god",
        &["world", "passeth", "away"],
        &["god", "world", "passeth", "away", "lust"],
    )?;
    segment.move_value(
        18,
        "children",
        &["little", "last", "time"],
        &["children", "little", "last", "time", "heard"],
    )?;
    segment.construct(
        19,
        "went",
        &["not", "would", "doubt"],
        &["went", "not", "would", "doubt", "continued"],
    )?;
    segment.expression(
        20,
        "unction",
        &["holy", "know", "things"],
        &["unction", "holy", "know", "things"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        2,
        21,
        24,
        "not guard -> expression not written unto",
        &["not", "written", "unto", "know", "truth"],
    );
    segment.guard(
        21,
        "not",
        &["written", "unto", "know"],
        &["not", "written", "unto", "know", "truth"],
    )?;
    segment.expression(
        22,
        "jesus",
        &["liar", "denieth", "christ"],
        &["jesus", "liar", "denieth", "christ", "antichrist"],
    )?;
    segment.guard(
        23,
        "whosoever",
        &["denieth", "son", "same"],
        &["whosoever", "denieth", "son", "same", "hath"],
    )?;
    segment.expression(
        24,
        "therefore",
        &["abide", "heard", "beginning"],
        &["therefore", "abide", "heard", "beginning", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        2,
        25,
        29,
        "promise return_value -> expression -> guard -> construct promise hath promised",
        &["promise", "hath", "promised", "even", "eternal"],
    );
    segment.return_value(
        25,
        "promise",
        &["hath", "promised", "even"],
        &["promise", "hath", "promised", "even", "eternal"],
    )?;
    segment.expression(
        26,
        "things",
        &["written", "unto", "concerning"],
        &["things", "written", "unto", "concerning", "seduce"],
    )?;
    segment.guard(
        27,
        "man",
        &["anointing", "received", "abideth"],
        &["man", "anointing", "received", "abideth", "need"],
    )?;
    segment.guard(
        28,
        "children",
        &["now", "little", "abide"],
        &["children", "now", "little", "abide", "shall"],
    )?;
    segment.construct(
        29,
        "know",
        &["righteous", "every", "doeth"],
        &["know", "righteous", "every", "doeth", "righteousness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        3,
        1,
        4,
        "behold guard -> await_value behold manner love",
        &["behold", "manner", "love", "father", "hath"],
    );
    segment.guard(
        1,
        "behold",
        &["manner", "love", "father"],
        &["behold", "manner", "love", "father", "hath"],
    )?;
    segment.guard(
        2,
        "god",
        &["beloved", "now", "sons"],
        &["god", "beloved", "now", "sons", "doth"],
    )?;
    segment.await_value(
        3,
        "man",
        &["every", "hath", "hope"],
        &["man", "every", "hath", "hope", "purifieth"],
    )?;
    segment.guard(
        4,
        "whosoever",
        &["committeth", "sin", "transgresseth"],
        &["whosoever", "committeth", "sin", "transgresseth", "law"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        3,
        5,
        8,
        "know guard -> drop_value know manifested take",
        &["know", "manifested", "take", "away", "sins"],
    );
    segment.guard(
        5,
        "know",
        &["manifested", "take", "away"],
        &["know", "manifested", "take", "away", "sins"],
    )?;
    segment.guard(
        6,
        "whosoever",
        &["abideth", "sinneth", "not"],
        &["whosoever", "abideth", "sinneth", "not", "hath"],
    )?;
    segment.guard(
        7,
        "children",
        &["little", "man", "deceive"],
        &["children", "little", "man", "deceive", "doeth"],
    )?;
    segment.drop_value(
        8,
        "god",
        &["committeth", "sin", "devil"],
        &["god", "committeth", "sin", "devil", "sinneth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        3,
        9,
        12,
        "god construct -> guard -> borrow god whosoever born",
        &["god", "whosoever", "born", "doth", "not"],
    );
    segment.construct(
        9,
        "god",
        &["whosoever", "born", "doth"],
        &["god", "whosoever", "born", "doth", "not"],
    )?;
    segment.guard(
        10,
        "children",
        &["god", "manifest", "devil"],
        &["children", "god", "manifest", "devil", "whosoever"],
    )?;
    segment.borrow(
        11,
        "message",
        &["heard", "beginning", "love"],
        &["message", "heard", "beginning", "love", "another"],
    )?;
    segment.guard(
        12,
        "not",
        &["cain", "wicked", "slew"],
        &["not", "cain", "wicked", "slew", "brother"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        3,
        13,
        16,
        "marvel guard -> drop_value -> borrow marvel not brethren",
        &["marvel", "not", "brethren", "world", "hate"],
    );
    segment.guard(
        13,
        "marvel",
        &["not", "brethren", "world"],
        &["marvel", "not", "brethren", "world", "hate"],
    )?;
    segment.drop_value(
        14,
        "know",
        &["passed", "death", "unto"],
        &["know", "passed", "death", "unto", "life"],
    )?;
    segment.guard(
        15,
        "whosoever",
        &["hateth", "brother", "murderer"],
        &["whosoever", "hateth", "brother", "murderer", "know"],
    )?;
    segment.borrow(
        16,
        "god",
        &["hereby", "perceive", "love"],
        &["god", "hereby", "perceive", "love", "laid"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        3,
        17,
        20,
        "whoso borrow -> guard -> expression whoso hath world's",
        &["whoso", "hath", "world's", "good", "seeth"],
    );
    segment.borrow(
        17,
        "whoso",
        &["hath", "world's", "good"],
        &["whoso", "hath", "world's", "good", "seeth"],
    )?;
    segment.guard(
        18,
        "children",
        &["little", "not", "love"],
        &["children", "little", "not", "love", "word"],
    )?;
    segment.expression(
        19,
        "hereby",
        &["know", "truth", "shall"],
        &["hereby", "know", "truth", "shall", "assure"],
    )?;
    segment.expression(
        20,
        "god",
        &["heart", "condemn", "greater"],
        &["god", "heart", "condemn", "greater", "knoweth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        3,
        21,
        24,
        "god guard -> expression god beloved heart",
        &["god", "beloved", "heart", "condemn", "not"],
    );
    segment.guard(
        21,
        "god",
        &["beloved", "heart", "condemn"],
        &["god", "beloved", "heart", "condemn", "not"],
    )?;
    segment.guard(
        22,
        "whatsoever",
        &["ask", "receive", "keep"],
        &["whatsoever", "ask", "receive", "keep", "commandments"],
    )?;
    segment.guard(
        23,
        "jesus",
        &["commandment", "believe", "name"],
        &["jesus", "commandment", "believe", "name", "son"],
    )?;
    segment.expression(
        24,
        "spirit",
        &["keepeth", "commandments", "dwelleth"],
        &["spirit", "keepeth", "commandments", "dwelleth", "hereby"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        4,
        1,
        4,
        "spirit move_value -> expression spirit beloved believe",
        &["spirit", "beloved", "believe", "not", "every"],
    );
    segment.move_value(
        1,
        "spirit",
        &["beloved", "believe", "not"],
        &["spirit", "beloved", "believe", "not", "every"],
    )?;
    segment.move_value(
        2,
        "spirit",
        &["hereby", "know", "god"],
        &["spirit", "hereby", "know", "god", "every"],
    )?;
    segment.move_value(
        3,
        "spirit",
        &["every", "confesseth", "not"],
        &["spirit", "every", "confesseth", "not", "jesus"],
    )?;
    segment.expression(
        4,
        "god",
        &["little", "children", "overcome"],
        &["god", "little", "children", "overcome", "greater"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        4,
        5,
        8,
        "world call -> guard -> construct world therefore speak",
        &["world", "therefore", "speak", "heareth", "god"],
    );
    segment.call(
        5,
        "world",
        &["therefore", "speak", "heareth"],
        &["world", "therefore", "speak", "heareth"],
    )?;
    segment.guard(
        6,
        "god",
        &["knoweth", "heareth", "not"],
        &["god", "knoweth", "heareth", "not", "hereby"],
    )?;
    segment.construct(
        7,
        "god",
        &["beloved", "love", "another"],
        &["god", "beloved", "love", "another", "every"],
    )?;
    segment.guard(
        8,
        "god",
        &["loveth", "not", "knoweth"],
        &["god", "loveth", "not", "knoweth", "love"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        4,
        9,
        12,
        "god construct -> move_value -> borrow -> guard god manifested love",
        &["god", "manifested", "love", "toward", "sent"],
    );
    segment.construct(
        9,
        "god",
        &["manifested", "love", "toward"],
        &["god", "manifested", "love", "toward", "sent"],
    )?;
    segment.move_value(
        10,
        "god",
        &["herein", "love", "not"],
        &["god", "herein", "love", "not", "loved"],
    )?;
    segment.borrow(
        11,
        "god",
        &["beloved", "loved", "ought"],
        &["god", "beloved", "loved", "ought", "love"],
    )?;
    segment.guard(
        12,
        "man",
        &["hath", "seen", "god"],
        &["man", "hath", "seen", "god", "any"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        4,
        13,
        16,
        "spirit expression -> move_value -> await_value spirit hereby know",
        &["spirit", "hereby", "know", "dwell", "hath"],
    );
    segment.expression(
        13,
        "spirit",
        &["hereby", "know", "dwell"],
        &["spirit", "hereby", "know", "dwell", "hath"],
    )?;
    segment.move_value(
        14,
        "saviour",
        &["seen", "testify", "father"],
        &["saviour", "seen", "testify", "father", "sent"],
    )?;
    segment.expression(
        15,
        "jesus",
        &["whosoever", "shall", "confess"],
        &["jesus", "whosoever", "shall", "confess", "son"],
    )?;
    segment.await_value(
        16,
        "god",
        &["known", "believed", "love"],
        &["god", "known", "believed", "love", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        4,
        17,
        21,
        "herein construct -> borrow -> guard herein love made",
        &["herein", "love", "made", "perfect", "boldness"],
    );
    segment.construct(
        17,
        "herein",
        &["love", "made", "perfect"],
        &["herein", "love", "made", "perfect", "boldness"],
    )?;
    segment.construct(
        18,
        "fear",
        &["love", "perfect", "casteth"],
        &["fear", "love", "perfect", "casteth", "hath"],
    )?;
    segment.borrow(19, "love", &["first", "loved"], &["love", "first", "loved"])?;
    segment.guard(
        20,
        "man",
        &["say", "love", "god"],
        &["man", "say", "love", "god", "hateth"],
    )?;
    segment.guard(
        21,
        "god",
        &["commandment", "loveth", "love"],
        &["god", "commandment", "loveth", "love", "brother"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        5,
        1,
        4,
        "jesus construct -> guard jesus whosoever believeth",
        &["jesus", "whosoever", "believeth", "christ", "born"],
    );
    segment.construct(
        1,
        "jesus",
        &["whosoever", "believeth", "christ"],
        &["jesus", "whosoever", "believeth", "christ", "born"],
    )?;
    segment.guard(
        2,
        "children",
        &["know", "love", "god"],
        &["children", "know", "love", "god", "keep"],
    )?;
    segment.guard(
        3,
        "god",
        &["love", "keep", "commandments"],
        &["god", "love", "keep", "commandments", "not"],
    )?;
    segment.construct(
        4,
        "god",
        &["whatsoever", "born", "overcometh"],
        &["god", "whatsoever", "born", "overcometh", "world"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        5,
        5,
        8,
        "jesus expression -> move_value -> call jesus overcometh world",
        &["jesus", "overcometh", "world", "believeth", "son"],
    );
    segment.expression(
        5,
        "jesus",
        &["overcometh", "world", "believeth"],
        &["jesus", "overcometh", "world", "believeth", "son"],
    )?;
    segment.move_value(
        6,
        "jesus",
        &["water", "blood", "even"],
        &["jesus", "water", "blood", "even", "christ"],
    )?;
    segment.call(
        7,
        "three",
        &["bear", "record", "heaven"],
        &["three", "bear", "record", "heaven", "father"],
    )?;
    segment.expression(
        8,
        "spirit",
        &["three", "bear", "witness"],
        &["spirit", "three", "bear", "witness", "earth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        5,
        9,
        12,
        "men expression -> construct -> guard men receive witness",
        &["men", "receive", "witness", "god", "greater"],
    );
    segment.expression(
        9,
        "men",
        &["receive", "witness", "god"],
        &["men", "receive", "witness", "god", "greater"],
    )?;
    segment.construct(
        10,
        "god",
        &["believeth", "son", "hath"],
        &["god", "believeth", "son", "hath", "witness"],
    )?;
    segment.expression(
        11,
        "god",
        &["record", "hath", "given"],
        &["god", "record", "hath", "given", "eternal"],
    )?;
    segment.guard(
        12,
        "god",
        &["hath", "son", "life"],
        &["god", "hath", "son", "life", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::_1_John,
        5,
        13,
        16,
        "god await_value -> result -> drop_value god things written",
        &["god", "things", "written", "unto", "believe"],
    );
    segment.await_value(
        13,
        "god",
        &["things", "written", "unto"],
        &["god", "things", "written", "unto", "believe"],
    )?;
    segment.result(
        14,
        "confidence",
        &["ask", "any", "thing"],
        &["confidence", "ask", "any", "thing", "according"],
    )?;
    segment.result(
        15,
        "know",
        &["hear", "whatsoever", "ask"],
        &["know", "hear", "whatsoever", "ask", "petitions"],
    )?;
    segment.drop_value(
        16,
        "man",
        &["any", "see", "brother"],
        &["man", "any", "see", "brother", "sin"],
    )?;
    segment.commit()?;
    Ok(())
}

fn _1_john_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(Book::_1_John, 5, 17, 21, "unrighteousness drop_value -> construct -> expression -> move_value -> guard unrighteousness sin not", &["unrighteousness", "sin", "not", "unto", "death"]);
    segment.drop_value(
        17,
        "unrighteousness",
        &["sin", "not", "unto"],
        &["unrighteousness", "sin", "not", "unto", "death"],
    )?;
    segment.construct(
        18,
        "god",
        &["know", "whosoever", "born"],
        &["god", "know", "whosoever", "born", "sinneth"],
    )?;
    segment.expression(
        19,
        "god",
        &["know", "whole", "world"],
        &["god", "know", "whole", "world", "lieth"],
    )?;
    segment.move_value(
        20,
        "god",
        &["know", "son", "hath"],
        &["god", "know", "son", "hath", "given"],
    )?;
    segment.guard(
        21,
        "children",
        &["little", "keep", "yourselves"],
        &["children", "little", "keep", "yourselves", "idols"],
    )?;
    segment.commit()?;
    Ok(())
}
