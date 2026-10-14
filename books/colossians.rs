use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    colossians(story)?;
    Ok(())
}

fn colossians(story: &mut Story) -> Result<(), StoryError> {
    colossians_1(story)?;
    colossians_2(story)?;
    colossians_3(story)?;
    colossians_4(story)?;
    Ok(())
}

fn colossians_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Colossians", 1, "Paul celebrates the Colossians' faith and presents Christ as the visible image of the invisible God, the one through whom creation holds together.", "The gospel reaches outsiders, and Paul's labor is to present everyone mature, empowered by the same Christ who lives among his people as hope.");
    colossians_1_group_01(story)?;
    colossians_1_group_02(story)?;
    colossians_1_group_03(story)?;
    colossians_1_group_04(story)?;
    colossians_1_group_05(story)?;
    colossians_1_group_06(story)?;
    colossians_1_group_07(story)?;
    Ok(())
}

fn colossians_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Colossians", 2, "The church is warned against persuasive philosophy, secret knowledge, food calendars, and angelic intermediaries that make shadows more important than Christ's body.", "In Christ the fullness of deity dwells, the old record is canceled, and believers share a new life rather than submitting again to humanly invented restrictions.");
    colossians_2_group_01(story)?;
    colossians_2_group_02(story)?;
    colossians_2_group_03(story)?;
    colossians_2_group_04(story)?;
    colossians_2_group_05(story)?;
    colossians_2_group_06(story)?;
    Ok(())
}

fn colossians_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Colossians", 3, "Since believers have been raised with Christ, their attention should move upward while they terminate rage, slander, lust, and deceit in the old process.", "The new self crosses old divisions, and household instructions are framed by compassion, patience, forgiveness, love, and work done for the Lord.");
    colossians_3_group_01(story)?;
    colossians_3_group_02(story)?;
    colossians_3_group_03(story)?;
    colossians_3_group_04(story)?;
    colossians_3_group_05(story)?;
    colossians_3_group_06(story)?;
    Ok(())
}

fn colossians_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Colossians", 4, "Paul asks for alert prayer and an open door for the message, while urging gracious, seasoned speech that answers each person wisely.", "A closing roster names coworkers, messengers, and household churches, and Archippus receives a direct reminder to complete the ministry he received.");
    colossians_4_group_01(story)?;
    colossians_4_group_02(story)?;
    colossians_4_group_03(story)?;
    colossians_4_group_04(story)?;
    colossians_4_group_05(story)?;
    Ok(())
}

fn colossians_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        1,
        1,
        4,
        "paul expression -> borrow -> await_value paul apostle jesus",
        &["paul", "apostle", "jesus", "christ", "will"],
    );
    segment.expression(
        1,
        "paul",
        &["apostle", "jesus", "christ"],
        &["paul", "apostle", "jesus", "christ", "will"],
    )?;
    segment.borrow(
        2,
        "christ",
        &["saints", "faithful", "brethren"],
        &["christ", "saints", "faithful", "brethren", "colosse"],
    )?;
    segment.expression(
        3,
        "god",
        &["give", "thanks", "father"],
        &["god", "give", "thanks", "father", "lord"],
    )?;
    segment.await_value(
        4,
        "christ",
        &["since", "heard", "faith"],
        &["christ", "since", "heard", "faith", "jesus"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        1,
        5,
        8,
        "hope call -> move_value -> expression -> borrow hope laid heaven",
        &["hope", "laid", "heaven", "whereof", "heard"],
    );
    segment.call(
        5,
        "hope",
        &["laid", "heaven", "whereof"],
        &["hope", "laid", "heaven", "whereof", "heard"],
    )?;
    segment.move_value(
        6,
        "unto",
        &["world", "bringeth", "forth"],
        &["unto", "world", "bringeth", "forth", "fruit"],
    )?;
    segment.expression(
        7,
        "christ",
        &["learned", "epaphras", "dear"],
        &["christ", "learned", "epaphras", "dear", "fellowservant"],
    )?;
    segment.borrow(
        8,
        "spirit",
        &["declared", "unto", "love"],
        &["spirit", "declared", "unto", "love"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        1,
        9,
        12,
        "cause guard -> iterate -> expression -> construct cause since day",
        &["cause", "since", "day", "heard", "not"],
    );
    segment.guard(
        9,
        "cause",
        &["since", "day", "heard"],
        &["cause", "since", "day", "heard", "not"],
    )?;
    segment.iterate(
        10,
        "lord",
        &["might", "walk", "worthy"],
        &["lord", "might", "walk", "worthy", "unto"],
    )?;
    segment.expression(
        11,
        "strengthened",
        &["might", "according", "glorious"],
        &["strengthened", "might", "according", "glorious", "power"],
    )?;
    segment.construct(
        12,
        "giving",
        &["thanks", "unto", "father"],
        &["giving", "thanks", "unto", "father", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        1,
        13,
        16,
        "hath expression -> iterate -> construct hath delivered power",
        &["hath", "delivered", "power", "darkness", "translated"],
    );
    segment.expression(
        13,
        "hath",
        &["delivered", "power", "darkness"],
        &["hath", "delivered", "power", "darkness", "translated"],
    )?;
    segment.expression(
        14,
        "redemption",
        &["through", "blood", "even"],
        &["redemption", "through", "blood", "even", "forgiveness"],
    )?;
    segment.iterate(
        15,
        "god",
        &["image", "invisible", "firstborn"],
        &["god", "image", "invisible", "firstborn", "every"],
    )?;
    segment.construct(
        16,
        "things",
        &["created", "heaven", "earth"],
        &["things", "created", "heaven", "earth", "visible"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        1,
        17,
        20,
        "things expression -> drop_value -> construct things consist head",
        &["things", "consist", "head", "body", "church"],
    );
    segment.expression(17, "things", &["consist"], &["things", "consist"])?;
    segment.drop_value(
        18,
        "head",
        &["body", "church", "beginning"],
        &["head", "body", "church", "beginning", "firstborn"],
    )?;
    segment.expression(
        19,
        "pleased",
        &["father", "fulness", "dwell"],
        &["pleased", "father", "fulness", "dwell"],
    )?;
    segment.construct(
        20,
        "having",
        &["made", "peace", "through"],
        &["having", "made", "peace", "through", "blood"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        1,
        21,
        24,
        "sometime expression -> drop_value -> construct sometime alienated enemies",
        &["sometime", "alienated", "enemies", "mind", "wicked"],
    );
    segment.expression(
        21,
        "sometime",
        &["alienated", "enemies", "mind"],
        &["sometime", "alienated", "enemies", "mind", "wicked"],
    )?;
    segment.drop_value(
        22,
        "body",
        &["flesh", "through", "death"],
        &["body", "flesh", "through", "death", "present"],
    )?;
    segment.construct(
        23,
        "continue",
        &["faith", "grounded", "settled"],
        &["continue", "faith", "grounded", "settled", "not"],
    )?;
    segment.expression(
        24,
        "christ",
        &["now", "rejoice", "sufferings"],
        &["christ", "now", "rejoice", "sufferings", "fill"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_1_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        1,
        25,
        29,
        "god construct -> call -> expression god whereof made",
        &["god", "whereof", "made", "minister", "according"],
    );
    segment.construct(
        25,
        "god",
        &["whereof", "made", "minister"],
        &["god", "whereof", "made", "minister", "according"],
    )?;
    segment.construct(
        26,
        "even",
        &["mystery", "hath", "hid"],
        &["even", "mystery", "hath", "hid", "ages"],
    )?;
    segment.construct(
        27,
        "god",
        &["would", "make", "known"],
        &["god", "would", "make", "known", "riches"],
    )?;
    segment.call(
        28,
        "man",
        &["preach", "warning", "every"],
        &["man", "preach", "warning", "every", "teaching"],
    )?;
    segment.expression(
        29,
        "whereunto",
        &["labour", "striving", "according"],
        &["whereunto", "labour", "striving", "according", "working"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        2,
        1,
        4,
        "would guard -> borrow -> expression -> call would knew great",
        &["would", "knew", "great", "conflict", "laodicea"],
    );
    segment.guard(
        1,
        "would",
        &["knew", "great", "conflict"],
        &["would", "knew", "great", "conflict", "laodicea"],
    )?;
    segment.borrow(
        2,
        "hearts",
        &["might", "comforted", "knit"],
        &["hearts", "might", "comforted", "knit", "together"],
    )?;
    segment.expression(
        3,
        "hid",
        &["treasures", "wisdom", "knowledge"],
        &["hid", "treasures", "wisdom", "knowledge"],
    )?;
    segment.call(
        4,
        "man",
        &["say", "lest", "any"],
        &["man", "say", "lest", "any", "beguile"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        2,
        5,
        8,
        "spirit await_value -> expression -> construct -> guard spirit though absent",
        &["spirit", "though", "absent", "flesh", "yet"],
    );
    segment.await_value(
        5,
        "spirit",
        &["though", "absent", "flesh"],
        &["spirit", "though", "absent", "flesh", "yet"],
    )?;
    segment.expression(
        6,
        "christ",
        &["therefore", "received", "jesus"],
        &["christ", "therefore", "received", "jesus", "lord"],
    )?;
    segment.construct(
        7,
        "rooted",
        &["built", "stablished", "faith"],
        &["rooted", "built", "stablished", "faith", "taught"],
    )?;
    segment.guard(
        8,
        "man",
        &["beware", "lest", "any"],
        &["man", "beware", "lest", "any", "spoil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        2,
        9,
        12,
        "dwelleth expression -> construct -> drop_value dwelleth fulness godhead",
        &["dwelleth", "fulness", "godhead", "bodily", "complete"],
    );
    segment.expression(
        9,
        "dwelleth",
        &["fulness", "godhead", "bodily"],
        &["dwelleth", "fulness", "godhead", "bodily"],
    )?;
    segment.expression(
        10,
        "complete",
        &["head", "principality", "power"],
        &["complete", "head", "principality", "power"],
    )?;
    segment.construct(
        11,
        "circumcised",
        &["circumcision", "made", "without"],
        &["circumcised", "circumcision", "made", "without", "hands"],
    )?;
    segment.drop_value(
        12,
        "god",
        &["buried", "baptism", "wherein"],
        &["god", "buried", "baptism", "wherein", "risen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        2,
        13,
        16,
        "dead drop_value -> expression -> construct -> reinitialize dead sins uncircumcision",
        &["dead", "sins", "uncircumcision", "flesh", "hath"],
    );
    segment.drop_value(
        13,
        "dead",
        &["sins", "uncircumcision", "flesh"],
        &["dead", "sins", "uncircumcision", "flesh", "hath"],
    )?;
    segment.expression(
        14,
        "blotting",
        &["handwriting", "ordinances", "against"],
        &[
            "blotting",
            "handwriting",
            "ordinances",
            "against",
            "contrary",
        ],
    )?;
    segment.construct(
        15,
        "having",
        &["spoiled", "principalities", "powers"],
        &["having", "spoiled", "principalities", "powers", "made"],
    )?;
    segment.reinitialize(
        16,
        "man",
        &["therefore", "judge", "meat"],
        &["man", "therefore", "judge", "meat", "drink"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        2,
        17,
        20,
        "christ move_value -> guard -> drop_value christ shadow things",
        &["christ", "shadow", "things", "body", "man"],
    );
    segment.move_value(
        17,
        "christ",
        &["shadow", "things", "body"],
        &["christ", "shadow", "things", "body"],
    )?;
    segment.guard(
        18,
        "man",
        &["beguile", "reward", "voluntary"],
        &["man", "beguile", "reward", "voluntary", "humility"],
    )?;
    segment.guard(
        19,
        "not",
        &["holding", "head", "body"],
        &["not", "holding", "head", "body", "joints"],
    )?;
    segment.drop_value(
        20,
        "christ",
        &["wherefore", "dead", "rudiments"],
        &["christ", "wherefore", "dead", "rudiments", "world"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        2,
        21,
        23,
        "touch guard -> expression touch not taste",
        &["touch", "not", "taste", "handle", "men"],
    );
    segment.guard(
        21,
        "touch",
        &["not", "taste", "handle"],
        &["touch", "not", "taste", "handle"],
    )?;
    segment.expression(
        22,
        "men",
        &["perish", "using", "commandments"],
        &["men", "perish", "using", "commandments", "doctrines"],
    )?;
    segment.guard(
        23,
        "things",
        &["indeed", "shew", "wisdom"],
        &["things", "indeed", "shew", "wisdom", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        3,
        1,
        4,
        "christ reinitialize -> guard -> drop_value -> trait_bound christ risen seek",
        &["christ", "risen", "seek", "things", "above"],
    );
    segment.reinitialize(
        1,
        "christ",
        &["risen", "seek", "things"],
        &["christ", "risen", "seek", "things", "above"],
    )?;
    segment.guard(
        2,
        "set",
        &["affection", "things", "above"],
        &["set", "affection", "things", "above", "not"],
    )?;
    segment.drop_value(
        3,
        "christ",
        &["dead", "life", "hid"],
        &["christ", "dead", "life", "hid", "god"],
    )?;
    segment.trait_bound(
        4,
        "christ",
        &["life", "shall", "appear"],
        &["christ", "life", "shall", "appear", "glory"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        3,
        5,
        8,
        "mortify expression -> panic mortify therefore members",
        &["mortify", "therefore", "members", "earth", "fornication"],
    );
    segment.expression(
        5,
        "mortify",
        &["therefore", "members", "earth"],
        &["mortify", "therefore", "members", "earth", "fornication"],
    )?;
    segment.panic(
        6,
        "god",
        &["things", "sake", "wrath"],
        &["god", "things", "sake", "wrath", "cometh"],
    )?;
    segment.expression(
        7,
        "walked",
        &["time", "lived"],
        &["walked", "time", "lived"],
    )?;
    segment.panic(
        8,
        "now",
        &["put", "off", "anger"],
        &["now", "put", "off", "anger", "wrath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        3,
        9,
        12,
        "man guard -> construct -> borrow man lie not",
        &["man", "lie", "not", "another", "seeing"],
    );
    segment.guard(
        9,
        "man",
        &["lie", "not", "another"],
        &["man", "lie", "not", "another", "seeing"],
    )?;
    segment.construct(
        10,
        "man",
        &["put", "new", "renewed"],
        &["man", "put", "new", "renewed", "knowledge"],
    )?;
    segment.guard(
        11,
        "neither",
        &["greek", "nor", "jew"],
        &["neither", "greek", "nor", "jew", "circumcision"],
    )?;
    segment.borrow(
        12,
        "god",
        &["put", "therefore", "elect"],
        &["god", "put", "therefore", "elect", "holy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        3,
        13,
        16,
        "man expression -> borrow -> call man forbearing another",
        &["man", "forbearing", "another", "forgiving", "any"],
    );
    segment.expression(
        13,
        "man",
        &["forbearing", "another", "forgiving"],
        &["man", "forbearing", "another", "forgiving", "any"],
    )?;
    segment.borrow(
        14,
        "above",
        &["things", "put", "charity"],
        &["above", "things", "put", "charity", "bond"],
    )?;
    segment.expression(
        15,
        "god",
        &["peace", "rule", "hearts"],
        &["god", "peace", "rule", "hearts", "called"],
    )?;
    segment.call(
        16,
        "christ",
        &["word", "dwell", "richly"],
        &["christ", "word", "dwell", "richly", "wisdom"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        3,
        17,
        20,
        "lord call -> expression -> guard lord whatsoever word",
        &["lord", "whatsoever", "word", "deed", "name"],
    );
    segment.call(
        17,
        "lord",
        &["whatsoever", "word", "deed"],
        &["lord", "whatsoever", "word", "deed", "name"],
    )?;
    segment.expression(
        18,
        "lord",
        &["wives", "submit", "yourselves"],
        &["lord", "wives", "submit", "yourselves", "unto"],
    )?;
    segment.guard(
        19,
        "husbands",
        &["love", "wives", "not"],
        &["husbands", "love", "wives", "not", "bitter"],
    )?;
    segment.expression(
        20,
        "children",
        &["obey", "parents", "things"],
        &["children", "obey", "parents", "things", "well"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        3,
        21,
        25,
        "children guard -> return_value children fathers provoke",
        &["children", "fathers", "provoke", "not", "anger"],
    );
    segment.guard(
        21,
        "children",
        &["fathers", "provoke", "not"],
        &["children", "fathers", "provoke", "not", "anger"],
    )?;
    segment.guard(
        22,
        "servants",
        &["obey", "things", "masters"],
        &["servants", "obey", "things", "masters", "according"],
    )?;
    segment.guard(
        23,
        "lord",
        &["whatsoever", "heartily", "not"],
        &["lord", "whatsoever", "heartily", "not", "unto"],
    )?;
    segment.return_value(
        24,
        "lord",
        &["knowing", "shall", "receive"],
        &["lord", "knowing", "shall", "receive", "reward"],
    )?;
    segment.guard(
        25,
        "doeth",
        &["wrong", "shall", "receive"],
        &["doeth", "wrong", "shall", "receive", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        4,
        1,
        4,
        "masters expression -> result -> call -> construct masters give unto",
        &["masters", "give", "unto", "servants", "just"],
    );
    segment.expression(
        1,
        "masters",
        &["give", "unto", "servants"],
        &["masters", "give", "unto", "servants", "just"],
    )?;
    segment.result(
        2,
        "continue",
        &["prayer", "watch", "same"],
        &["continue", "prayer", "watch", "same", "thanksgiving"],
    )?;
    segment.call(
        3,
        "god",
        &["withal", "praying", "would"],
        &["god", "withal", "praying", "would", "open"],
    )?;
    segment.construct(
        4,
        "make",
        &["manifest", "ought", "speak"],
        &["make", "manifest", "ought", "speak"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        4,
        5,
        8,
        "walk expression -> borrow -> call -> move_value walk wisdom toward",
        &["walk", "wisdom", "toward", "without", "redeeming"],
    );
    segment.expression(
        5,
        "walk",
        &["wisdom", "toward", "without"],
        &["walk", "wisdom", "toward", "without", "redeeming"],
    )?;
    segment.borrow(
        6,
        "speech",
        &["always", "grace", "seasoned"],
        &["speech", "always", "grace", "seasoned", "salt"],
    )?;
    segment.call(
        7,
        "state",
        &["shall", "tychicus", "declare"],
        &["state", "shall", "tychicus", "declare", "unto"],
    )?;
    segment.move_value(
        8,
        "sent",
        &["unto", "same", "purpose"],
        &["sent", "unto", "same", "purpose", "might"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        4,
        9,
        12,
        "onesimus construct -> move_value -> expression onesimus faithful beloved",
        &["onesimus", "faithful", "beloved", "brother", "shall"],
    );
    segment.construct(
        9,
        "onesimus",
        &["faithful", "beloved", "brother"],
        &["onesimus", "faithful", "beloved", "brother", "shall"],
    )?;
    segment.move_value(
        10,
        "aristarchus",
        &["fellowprisoner", "saluteth", "marcus"],
        &[
            "aristarchus",
            "fellowprisoner",
            "saluteth",
            "marcus",
            "sister's",
        ],
    )?;
    segment.expression(
        11,
        "jesus",
        &["called", "justus", "circumcision"],
        &["jesus", "called", "justus", "circumcision", "fellowworkers"],
    )?;
    segment.expression(
        12,
        "christ",
        &["epaphras", "servant", "saluteth"],
        &["christ", "epaphras", "servant", "saluteth", "always"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        4,
        13,
        15,
        "bear expression -> borrow bear record hath",
        &["bear", "record", "hath", "great", "zeal"],
    );
    segment.expression(
        13,
        "bear",
        &["record", "hath", "great"],
        &["bear", "record", "hath", "great", "zeal"],
    )?;
    segment.borrow(
        14,
        "luke",
        &["beloved", "physician", "demas"],
        &["luke", "beloved", "physician", "demas", "greet"],
    )?;
    segment.expression(
        15,
        "salute",
        &["brethren", "laodicea", "nymphas"],
        &["salute", "brethren", "laodicea", "nymphas", "church"],
    )?;
    segment.commit()?;
    Ok(())
}

fn colossians_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Colossians,
        4,
        16,
        18,
        "epistle expression -> call -> borrow epistle read among",
        &["epistle", "read", "among", "cause", "church"],
    );
    segment.expression(
        16,
        "epistle",
        &["read", "among", "cause"],
        &["epistle", "read", "among", "cause", "church"],
    )?;
    segment.call(
        17,
        "lord",
        &["say", "archippus", "take"],
        &["lord", "say", "archippus", "take", "heed"],
    )?;
    segment.borrow(
        18,
        "paul",
        &["salutation", "hand", "remember"],
        &["paul", "salutation", "hand", "remember", "bonds"],
    )?;
    segment.commit()?;
    Ok(())
}
