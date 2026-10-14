use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    ephesians(story)?;
    Ok(())
}

fn ephesians(story: &mut Story) -> Result<(), StoryError> {
    ephesians_1(story)?;
    ephesians_2(story)?;
    ephesians_3(story)?;
    ephesians_4(story)?;
    ephesians_5(story)?;
    ephesians_6(story)?;
    Ok(())
}

fn ephesians_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ephesians", 1, "Paul blesses God for choosing and adopting believers in Christ, where redemption, forgiveness, and a vast inheritance are disclosed.", "He prays that the church will know the hope, wealth, and resurrection power already directed toward Christ and shared with his body.");
    ephesians_1_group_01(story)?;
    ephesians_1_group_02(story)?;
    ephesians_1_group_03(story)?;
    ephesians_1_group_04(story)?;
    ephesians_1_group_05(story)?;
    ephesians_1_group_06(story)?;
    Ok(())
}

fn ephesians_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ephesians", 2, "Those once dead in trespasses are made alive by grace, not by a résumé of good works, and are remade for a life of prepared goodness.", "Christ dismantles the wall between Jew and Gentile, creating one new humanity and one household with access to the Father through the Spirit.");
    ephesians_2_group_01(story)?;
    ephesians_2_group_02(story)?;
    ephesians_2_group_03(story)?;
    ephesians_2_group_04(story)?;
    ephesians_2_group_05(story)?;
    ephesians_2_group_06(story)?;
    Ok(())
}

fn ephesians_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ephesians", 3, "Paul explains the revealed mystery that Gentiles share the promise, then describes his imprisonment as service for this wide-open inclusion.", "He prays for inner strength and for Christ's love to exceed every measurement, trusting God to do more than the church can request or imagine.");
    ephesians_3_group_01(story)?;
    ephesians_3_group_02(story)?;
    ephesians_3_group_03(story)?;
    ephesians_3_group_04(story)?;
    ephesians_3_group_05(story)?;
    Ok(())
}

fn ephesians_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ephesians", 4, "The church is urged to walk worthy of its calling through humility, patience, unity, and diverse gifts that mature the whole body.", "Believers must discard the old deceitful runtime, speak truth, work honestly, forgive quickly, and avoid grieving the Spirit who sealed them.");
    ephesians_4_group_01(story)?;
    ephesians_4_group_02(story)?;
    ephesians_4_group_03(story)?;
    ephesians_4_group_04(story)?;
    ephesians_4_group_05(story)?;
    ephesians_4_group_06(story)?;
    ephesians_4_group_07(story)?;
    ephesians_4_group_08(story)?;
    Ok(())
}

fn ephesians_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ephesians", 5, "Paul calls the community to imitate God's self-giving love, reject hidden works of darkness, and walk wisely while the days are difficult.", "Marriage becomes a metaphor of sacrificial union, with mutual honor replacing domination and the church pictured as cherished rather than exploited.");
    ephesians_5_group_01(story)?;
    ephesians_5_group_02(story)?;
    ephesians_5_group_03(story)?;
    ephesians_5_group_04(story)?;
    ephesians_5_group_05(story)?;
    ephesians_5_group_06(story)?;
    ephesians_5_group_07(story)?;
    ephesians_5_group_08(story)?;
    Ok(())
}

fn ephesians_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ephesians", 6, "Household relationships are addressed with responsibility on every side, while children are guided and parents warned against provoking them into resentment.", "The final deployment equips believers with truth, righteousness, peace, faith, salvation, and the Spirit's word for a struggle against unseen powers.");
    ephesians_6_group_01(story)?;
    ephesians_6_group_02(story)?;
    ephesians_6_group_03(story)?;
    ephesians_6_group_04(story)?;
    ephesians_6_group_05(story)?;
    ephesians_6_group_06(story)?;
    Ok(())
}

fn ephesians_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        1,
        1,
        4,
        "paul expression -> borrow -> return_value paul apostle jesus",
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
        "god",
        &["grace", "peace", "father"],
        &["god", "grace", "peace", "father", "lord"],
    )?;
    segment.return_value(
        3,
        "god",
        &["blessed", "father", "lord"],
        &["god", "blessed", "father", "lord", "jesus"],
    )?;
    segment.borrow(
        4,
        "according",
        &["hath", "chosen", "foundation"],
        &["according", "hath", "chosen", "foundation", "world"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        1,
        5,
        8,
        "children expression -> construct -> borrow children having predestinated",
        &["children", "having", "predestinated", "unto", "adoption"],
    );
    segment.expression(
        5,
        "children",
        &["having", "predestinated", "unto"],
        &["children", "having", "predestinated", "unto", "adoption"],
    )?;
    segment.construct(
        6,
        "praise",
        &["glory", "grace", "wherein"],
        &["praise", "glory", "grace", "wherein", "hath"],
    )?;
    segment.borrow(
        7,
        "redemption",
        &["through", "blood", "forgiveness"],
        &["redemption", "through", "blood", "forgiveness", "sins"],
    )?;
    segment.expression(
        8,
        "wherein",
        &["hath", "abounded", "toward"],
        &["wherein", "hath", "abounded", "toward", "wisdom"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        1,
        9,
        12,
        "having construct -> expression -> return_value -> trait_bound having made known",
        &["having", "made", "known", "unto", "mystery"],
    );
    segment.construct(
        9,
        "having",
        &["made", "known", "unto"],
        &["having", "made", "known", "unto", "mystery"],
    )?;
    segment.expression(
        10,
        "christ",
        &["dispensation", "fulness", "times"],
        &["christ", "dispensation", "fulness", "times", "might"],
    )?;
    segment.return_value(
        11,
        "obtained",
        &["inheritance", "predestinated", "according"],
        &[
            "obtained",
            "inheritance",
            "predestinated",
            "according",
            "purpose",
        ],
    )?;
    segment.trait_bound(
        12,
        "christ",
        &["praise", "glory", "first"],
        &["christ", "praise", "glory", "first", "trusted"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        1,
        13,
        16,
        "trusted call -> trait_bound -> await_value -> guard trusted heard word",
        &["trusted", "heard", "word", "truth", "gospel"],
    );
    segment.call(
        13,
        "trusted",
        &["heard", "word", "truth"],
        &["trusted", "heard", "word", "truth", "gospel"],
    )?;
    segment.trait_bound(
        14,
        "earnest",
        &["inheritance", "until", "redemption"],
        &["earnest", "inheritance", "until", "redemption", "purchased"],
    )?;
    segment.await_value(
        15,
        "lord",
        &["wherefore", "heard", "faith"],
        &["lord", "wherefore", "heard", "faith", "jesus"],
    )?;
    segment.guard(
        16,
        "cease",
        &["not", "give", "thanks"],
        &["cease", "not", "give", "thanks", "making"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        1,
        17,
        20,
        "god match_value -> trait_bound -> await_value -> drop_value god lord jesus",
        &["god", "lord", "jesus", "christ", "father"],
    );
    segment.match_value(
        17,
        "god",
        &["lord", "jesus", "christ"],
        &["god", "lord", "jesus", "christ", "father"],
    )?;
    segment.trait_bound(
        18,
        "eyes",
        &["understanding", "enlightened", "know"],
        &["eyes", "understanding", "enlightened", "know", "hope"],
    )?;
    segment.await_value(
        19,
        "exceeding",
        &["greatness", "power", "ward"],
        &["exceeding", "greatness", "power", "ward", "believe"],
    )?;
    segment.drop_value(
        20,
        "christ",
        &["wrought", "raised", "dead"],
        &["christ", "wrought", "raised", "dead", "set"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        1,
        21,
        23,
        "far move_value -> expression far above principality",
        &["far", "above", "principality", "power", "might"],
    );
    segment.move_value(
        21,
        "far",
        &["above", "principality", "power"],
        &["far", "above", "principality", "power", "might"],
    )?;
    segment.expression(
        22,
        "hath",
        &["put", "things", "under"],
        &["hath", "put", "things", "under", "feet"],
    )?;
    segment.expression(
        23,
        "body",
        &["fulness", "filleth"],
        &["body", "fulness", "filleth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        2,
        1,
        4,
        "hath drop_value -> expression -> panic -> borrow hath quickened dead",
        &["hath", "quickened", "dead", "trespasses", "sins"],
    );
    segment.drop_value(
        1,
        "hath",
        &["quickened", "dead", "trespasses"],
        &["hath", "quickened", "dead", "trespasses", "sins"],
    )?;
    segment.expression(
        2,
        "wherein",
        &["time", "past", "walked"],
        &["wherein", "time", "past", "walked", "according"],
    )?;
    segment.panic(
        3,
        "among",
        &["conversation", "times", "past"],
        &["among", "conversation", "times", "past", "lusts"],
    )?;
    segment.borrow(
        4,
        "god",
        &["rich", "mercy", "great"],
        &["god", "rich", "mercy", "great", "love"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        2,
        5,
        8,
        "christ drop_value -> construct -> move_value -> guard christ even dead",
        &["christ", "even", "dead", "sins", "hath"],
    );
    segment.drop_value(
        5,
        "christ",
        &["even", "dead", "sins"],
        &["christ", "even", "dead", "sins", "hath"],
    )?;
    segment.construct(
        6,
        "christ",
        &["hath", "raised", "together"],
        &["christ", "hath", "raised", "together", "made"],
    )?;
    segment.move_value(
        7,
        "ages",
        &["might", "shew", "exceeding"],
        &["ages", "might", "shew", "exceeding", "riches"],
    )?;
    segment.guard(
        8,
        "god",
        &["grace", "saved", "through"],
        &["god", "grace", "saved", "through", "faith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        2,
        9,
        12,
        "man guard -> construct man not works",
        &["man", "not", "works", "lest", "any"],
    );
    segment.guard(
        9,
        "man",
        &["not", "works", "lest"],
        &["man", "not", "works", "lest", "any"],
    )?;
    segment.construct(
        10,
        "christ",
        &["workmanship", "created", "jesus"],
        &["christ", "workmanship", "created", "jesus", "unto"],
    )?;
    segment.construct(
        11,
        "wherefore",
        &["remember", "time", "past"],
        &["wherefore", "remember", "time", "past", "gentiles"],
    )?;
    segment.guard(
        12,
        "christ",
        &["time", "without", "aliens"],
        &["christ", "time", "without", "aliens", "commonwealth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        2,
        13,
        16,
        "christ construct -> drop_value christ now jesus",
        &["christ", "now", "jesus", "sometimes", "far"],
    );
    segment.construct(
        13,
        "christ",
        &["now", "jesus", "sometimes"],
        &["christ", "now", "jesus", "sometimes", "far"],
    )?;
    segment.construct(
        14,
        "peace",
        &["hath", "made", "both"],
        &["peace", "hath", "made", "both", "broken"],
    )?;
    segment.construct(
        15,
        "having",
        &["abolished", "flesh", "enmity"],
        &["having", "abolished", "flesh", "enmity", "even"],
    )?;
    segment.drop_value(
        16,
        "god",
        &["might", "reconcile", "both"],
        &["god", "might", "reconcile", "both", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        2,
        17,
        19,
        "preached move_value -> expression -> guard preached peace afar",
        &["preached", "peace", "afar", "off", "nigh"],
    );
    segment.move_value(
        17,
        "preached",
        &["peace", "afar", "off"],
        &["preached", "peace", "afar", "off", "nigh"],
    )?;
    segment.expression(
        18,
        "spirit",
        &["through", "both", "access"],
        &["spirit", "through", "both", "access", "unto"],
    )?;
    segment.guard(
        19,
        "god",
        &["now", "therefore", "strangers"],
        &["god", "now", "therefore", "strangers", "foreigners"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        2,
        20,
        22,
        "jesus construct -> trait_bound -> expression jesus built foundation",
        &["jesus", "built", "foundation", "apostles", "prophets"],
    );
    segment.construct(
        20,
        "jesus",
        &["built", "foundation", "apostles"],
        &["jesus", "built", "foundation", "apostles", "prophets"],
    )?;
    segment.trait_bound(
        21,
        "building",
        &["fitly", "framed", "together"],
        &["building", "fitly", "framed", "together", "groweth"],
    )?;
    segment.expression(
        22,
        "god",
        &["builded", "together", "habitation"],
        &["god", "builded", "together", "habitation", "through"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        3,
        1,
        4,
        "paul expression -> borrow -> construct paul cause prisoner",
        &["paul", "cause", "prisoner", "jesus", "christ"],
    );
    segment.expression(
        1,
        "paul",
        &["cause", "prisoner", "jesus"],
        &["paul", "cause", "prisoner", "jesus", "christ"],
    )?;
    segment.borrow(
        2,
        "god",
        &["heard", "dispensation", "grace"],
        &["god", "heard", "dispensation", "grace", "given"],
    )?;
    segment.construct(
        3,
        "revelation",
        &["made", "known", "unto"],
        &["revelation", "made", "known", "unto", "mystery"],
    )?;
    segment.expression(
        4,
        "christ",
        &["whereby", "read", "understand"],
        &["christ", "whereby", "read", "understand", "knowledge"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        3,
        5,
        8,
        "men construct -> return_value -> call men other ages",
        &["men", "other", "ages", "not", "made"],
    );
    segment.construct(
        5,
        "men",
        &["other", "ages", "not"],
        &["men", "other", "ages", "not", "made"],
    )?;
    segment.return_value(
        6,
        "christ",
        &["gentiles", "fellowheirs", "same"],
        &["christ", "gentiles", "fellowheirs", "same", "body"],
    )?;
    segment.construct(
        7,
        "god",
        &["whereof", "made", "minister"],
        &["god", "whereof", "made", "minister", "according"],
    )?;
    segment.call(
        8,
        "unto",
        &["less", "least", "saints"],
        &["unto", "less", "least", "saints", "grace"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        3,
        9,
        12,
        "men construct -> expression -> await_value men make see",
        &["men", "make", "see", "fellowship", "mystery"],
    );
    segment.construct(
        9,
        "men",
        &["make", "see", "fellowship"],
        &["men", "make", "see", "fellowship", "mystery"],
    )?;
    segment.expression(
        10,
        "intent",
        &["now", "unto", "principalities"],
        &["intent", "now", "unto", "principalities", "powers"],
    )?;
    segment.expression(
        11,
        "christ",
        &["according", "eternal", "purpose"],
        &["christ", "according", "eternal", "purpose", "purposed"],
    )?;
    segment.await_value(
        12,
        "boldness",
        &["access", "confidence", "faith"],
        &["boldness", "access", "confidence", "faith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        3,
        13,
        16,
        "wherefore guard -> expression -> trait_bound wherefore desire faint",
        &["wherefore", "desire", "faint", "not", "tribulations"],
    );
    segment.guard(
        13,
        "wherefore",
        &["desire", "faint", "not"],
        &["wherefore", "desire", "faint", "not", "tribulations"],
    )?;
    segment.expression(
        14,
        "lord",
        &["cause", "bow", "knees"],
        &["lord", "cause", "bow", "knees", "unto"],
    )?;
    segment.expression(
        15,
        "whole",
        &["family", "heaven", "earth"],
        &["whole", "family", "heaven", "earth", "named"],
    )?;
    segment.trait_bound(
        16,
        "spirit",
        &["would", "grant", "according"],
        &["spirit", "would", "grant", "according", "riches"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        3,
        17,
        21,
        "christ await_value -> expression -> borrow -> result -> trait_bound christ dwell hearts",
        &["christ", "dwell", "hearts", "faith", "rooted"],
    );
    segment.await_value(
        17,
        "christ",
        &["dwell", "hearts", "faith"],
        &["christ", "dwell", "hearts", "faith", "rooted"],
    )?;
    segment.expression(
        18,
        "able",
        &["comprehend", "saints", "breadth"],
        &["able", "comprehend", "saints", "breadth", "length"],
    )?;
    segment.borrow(
        19,
        "christ",
        &["know", "love", "passeth"],
        &["christ", "know", "love", "passeth", "knowledge"],
    )?;
    segment.result(
        20,
        "now",
        &["unto", "able", "exceeding"],
        &["now", "unto", "able", "exceeding", "abundantly"],
    )?;
    segment.trait_bound(
        21,
        "christ",
        &["unto", "glory", "church"],
        &["christ", "unto", "glory", "church", "jesus"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        1,
        4,
        "lord expression -> borrow -> guard -> await_value lord therefore prisoner",
        &["lord", "therefore", "prisoner", "beseech", "walk"],
    );
    segment.expression(
        1,
        "lord",
        &["therefore", "prisoner", "beseech"],
        &["lord", "therefore", "prisoner", "beseech", "walk"],
    )?;
    segment.borrow(
        2,
        "lowliness",
        &["meekness", "longsuffering", "forbearing"],
        &[
            "lowliness",
            "meekness",
            "longsuffering",
            "forbearing",
            "another",
        ],
    )?;
    segment.guard(
        3,
        "spirit",
        &["endeavouring", "keep", "unity"],
        &["spirit", "endeavouring", "keep", "unity", "bond"],
    )?;
    segment.await_value(
        4,
        "spirit",
        &["body", "even", "called"],
        &["spirit", "body", "even", "called", "hope"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        5,
        8,
        "lord await_value -> expression -> borrow -> reinitialize lord faith baptism",
        &["lord", "faith", "baptism", "god", "father"],
    );
    segment.await_value(
        5,
        "lord",
        &["faith", "baptism"],
        &["lord", "faith", "baptism"],
    )?;
    segment.expression(
        6,
        "god",
        &["father", "above", "through"],
        &["god", "father", "above", "through"],
    )?;
    segment.borrow(
        7,
        "christ",
        &["unto", "every", "given"],
        &["christ", "unto", "every", "given", "grace"],
    )?;
    segment.reinitialize(
        8,
        "wherefore",
        &["saith", "ascended", "high"],
        &["wherefore", "saith", "ascended", "high", "led"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        9,
        12,
        "now reinitialize -> expression now ascended descended",
        &["now", "ascended", "descended", "first", "lower"],
    );
    segment.reinitialize(
        9,
        "now",
        &["ascended", "descended", "first"],
        &["now", "ascended", "descended", "first", "lower"],
    )?;
    segment.reinitialize(
        10,
        "descended",
        &["same", "ascended", "far"],
        &["descended", "same", "ascended", "far", "above"],
    )?;
    segment.expression(
        11,
        "gave",
        &["apostles", "prophets", "evangelists"],
        &["gave", "apostles", "prophets", "evangelists", "pastors"],
    )?;
    segment.expression(
        12,
        "christ",
        &["perfecting", "saints", "work"],
        &["christ", "perfecting", "saints", "work", "ministry"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        13,
        16,
        "god move_value -> guard -> borrow god till unity",
        &["god", "till", "unity", "faith", "knowledge"],
    );
    segment.move_value(
        13,
        "god",
        &["till", "unity", "faith"],
        &["god", "till", "unity", "faith", "knowledge"],
    )?;
    segment.guard(
        14,
        "children",
        &["henceforth", "tossed", "fro"],
        &["children", "henceforth", "tossed", "fro", "carried"],
    )?;
    segment.borrow(
        15,
        "christ",
        &["speaking", "truth", "love"],
        &["christ", "speaking", "truth", "love", "grow"],
    )?;
    segment.borrow(
        16,
        "whole",
        &["body", "fitly", "joined"],
        &["whole", "body", "fitly", "joined", "together"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        17,
        20,
        "lord guard -> expression lord say therefore",
        &["lord", "say", "therefore", "testify", "henceforth"],
    );
    segment.guard(
        17,
        "lord",
        &["say", "therefore", "testify"],
        &["lord", "say", "therefore", "testify", "henceforth"],
    )?;
    segment.expression(
        18,
        "god",
        &["having", "understanding", "darkened"],
        &["god", "having", "understanding", "darkened", "alienated"],
    )?;
    segment.expression(
        19,
        "past",
        &["feeling", "given", "themselves"],
        &["past", "feeling", "given", "themselves", "over"],
    )?;
    segment.guard(
        20,
        "christ",
        &["not", "learned"],
        &["christ", "not", "learned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        21,
        24,
        "jesus call -> expression -> reinitialize -> construct jesus heard taught",
        &["jesus", "heard", "taught", "truth", "man"],
    );
    segment.call(
        21,
        "jesus",
        &["heard", "taught", "truth"],
        &["jesus", "heard", "taught", "truth"],
    )?;
    segment.expression(
        22,
        "man",
        &["put", "off", "concerning"],
        &["man", "put", "off", "concerning", "former"],
    )?;
    segment.reinitialize(
        23,
        "spirit",
        &["renewed", "mind"],
        &["spirit", "renewed", "mind"],
    )?;
    segment.construct(
        24,
        "man",
        &["put", "new", "god"],
        &["man", "put", "new", "god", "created"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        25,
        28,
        "man call -> move_value -> guard man wherefore putting",
        &["man", "wherefore", "putting", "away", "lying"],
    );
    segment.call(
        25,
        "man",
        &["wherefore", "putting", "away"],
        &["man", "wherefore", "putting", "away", "lying"],
    )?;
    segment.move_value(
        26,
        "angry",
        &["sin", "not", "sun"],
        &["angry", "sin", "not", "sun", "down"],
    )?;
    segment.guard(
        27,
        "neither",
        &["give", "place", "devil"],
        &["neither", "give", "place", "devil"],
    )?;
    segment.guard(
        28,
        "stole",
        &["steal", "rather", "labour"],
        &["stole", "steal", "rather", "labour", "working"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_4_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        4,
        29,
        32,
        "corrupt guard -> panic -> expression corrupt communication proceed",
        &["corrupt", "communication", "proceed", "mouth", "good"],
    );
    segment.guard(
        29,
        "corrupt",
        &["communication", "proceed", "mouth"],
        &["corrupt", "communication", "proceed", "mouth", "good"],
    )?;
    segment.guard(
        30,
        "spirit",
        &["grieve", "not", "holy"],
        &["spirit", "grieve", "not", "holy", "god"],
    )?;
    segment.panic(
        31,
        "bitterness",
        &["wrath", "anger", "clamour"],
        &["bitterness", "wrath", "anger", "clamour", "evil"],
    )?;
    segment.expression(
        32,
        "god",
        &["kind", "another", "tenderhearted"],
        &["god", "kind", "another", "tenderhearted", "forgiving"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        1,
        4,
        "god expression -> trait_bound -> guard god therefore followers",
        &["god", "therefore", "followers", "dear", "children"],
    );
    segment.expression(
        1,
        "god",
        &["therefore", "followers", "dear"],
        &["god", "therefore", "followers", "dear", "children"],
    )?;
    segment.trait_bound(
        2,
        "christ",
        &["walk", "love", "hath"],
        &["christ", "walk", "love", "hath", "loved"],
    )?;
    segment.guard(
        3,
        "fornication",
        &["uncleanness", "covetousness", "not"],
        &["fornication", "uncleanness", "covetousness", "not", "once"],
    )?;
    segment.guard(
        4,
        "neither",
        &["filthiness", "nor", "foolish"],
        &["neither", "filthiness", "nor", "foolish", "talking"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        5,
        8,
        "man guard -> expression man know whoremonger",
        &["man", "know", "whoremonger", "nor", "unclean"],
    );
    segment.guard(
        5,
        "man",
        &["know", "whoremonger", "nor"],
        &["man", "know", "whoremonger", "nor", "unclean"],
    )?;
    segment.guard(
        6,
        "man",
        &["deceive", "vain", "words"],
        &["man", "deceive", "vain", "words", "things"],
    )?;
    segment.guard(
        7,
        "not",
        &["therefore", "partakers"],
        &["not", "therefore", "partakers"],
    )?;
    segment.expression(
        8,
        "lord",
        &["sometimes", "darkness", "now"],
        &["lord", "sometimes", "darkness", "now", "light"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        9,
        12,
        "spirit expression -> guard -> call spirit fruit goodness",
        &["spirit", "fruit", "goodness", "righteousness", "truth"],
    );
    segment.expression(
        9,
        "spirit",
        &["fruit", "goodness", "righteousness"],
        &["spirit", "fruit", "goodness", "righteousness", "truth"],
    )?;
    segment.expression(
        10,
        "lord",
        &["proving", "acceptable", "unto"],
        &["lord", "proving", "acceptable", "unto"],
    )?;
    segment.guard(
        11,
        "fellowship",
        &["unfruitful", "works", "darkness"],
        &["fellowship", "unfruitful", "works", "darkness", "rather"],
    )?;
    segment.call(
        12,
        "shame",
        &["even", "speak", "things"],
        &["shame", "even", "speak", "things", "done"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        13,
        16,
        "things construct -> drop_value -> guard -> expression things reproved made",
        &["things", "reproved", "made", "manifest", "light"],
    );
    segment.construct(
        13,
        "things",
        &["reproved", "made", "manifest"],
        &["things", "reproved", "made", "manifest", "light"],
    )?;
    segment.drop_value(
        14,
        "christ",
        &["wherefore", "saith", "awake"],
        &["christ", "wherefore", "saith", "awake", "sleepest"],
    )?;
    segment.guard(
        15,
        "see",
        &["walk", "circumspectly", "not"],
        &["see", "walk", "circumspectly", "not", "fools"],
    )?;
    segment.expression(
        16,
        "redeeming",
        &["time", "days", "evil"],
        &["redeeming", "time", "days", "evil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        17,
        20,
        "lord guard -> expression lord wherefore not",
        &["lord", "wherefore", "not", "unwise", "understanding"],
    );
    segment.guard(
        17,
        "lord",
        &["wherefore", "not", "unwise"],
        &["lord", "wherefore", "not", "unwise", "understanding"],
    )?;
    segment.guard(
        18,
        "spirit",
        &["not", "drunk", "wine"],
        &["spirit", "not", "drunk", "wine", "wherein"],
    )?;
    segment.expression(
        19,
        "speaking",
        &["yourselves", "psalms", "hymns"],
        &["speaking", "yourselves", "psalms", "hymns", "spiritual"],
    )?;
    segment.expression(
        20,
        "god",
        &["giving", "thanks", "always"],
        &["god", "giving", "thanks", "always", "things"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        21,
        24,
        "god panic -> expression -> iterate god submitting yourselves",
        &["god", "submitting", "yourselves", "another", "fear"],
    );
    segment.panic(
        21,
        "god",
        &["submitting", "yourselves", "another"],
        &["god", "submitting", "yourselves", "another", "fear"],
    )?;
    segment.expression(
        22,
        "lord",
        &["wives", "submit", "yourselves"],
        &["lord", "wives", "submit", "yourselves", "unto"],
    )?;
    segment.expression(
        23,
        "christ",
        &["husband", "head", "wife"],
        &["christ", "husband", "head", "wife", "even"],
    )?;
    segment.iterate(
        24,
        "christ",
        &["therefore", "church", "subject"],
        &["christ", "therefore", "church", "subject", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        25,
        28,
        "christ borrow -> call -> guard christ husbands love",
        &["christ", "husbands", "love", "wives", "even"],
    );
    segment.borrow(
        25,
        "christ",
        &["husbands", "love", "wives"],
        &["christ", "husbands", "love", "wives", "even"],
    )?;
    segment.call(
        26,
        "might",
        &["sanctify", "cleanse", "washing"],
        &["might", "sanctify", "cleanse", "washing", "water"],
    )?;
    segment.guard(
        27,
        "might",
        &["present", "himself", "glorious"],
        &["might", "present", "himself", "glorious", "church"],
    )?;
    segment.borrow(
        28,
        "men",
        &["ought", "love", "wives"],
        &["men", "ought", "love", "wives", "own"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_5_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        5,
        29,
        33,
        "man guard -> expression -> call -> borrow man ever yet",
        &["man", "ever", "yet", "hated", "own"],
    );
    segment.guard(
        29,
        "man",
        &["ever", "yet", "hated"],
        &["man", "ever", "yet", "hated", "own"],
    )?;
    segment.expression(
        30,
        "members",
        &["body", "flesh", "bones"],
        &["members", "body", "flesh", "bones"],
    )?;
    segment.expression(
        31,
        "man",
        &["cause", "shall", "leave"],
        &["man", "cause", "shall", "leave", "father"],
    )?;
    segment.call(
        32,
        "christ",
        &["great", "mystery", "speak"],
        &["christ", "great", "mystery", "speak", "concerning"],
    )?;
    segment.borrow(
        33,
        "nevertheless",
        &["every", "particular", "love"],
        &["nevertheless", "every", "particular", "love", "wife"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        6,
        1,
        4,
        "children expression -> guard -> move_value children obey parents",
        &["children", "obey", "parents", "lord", "right"],
    );
    segment.expression(
        1,
        "children",
        &["obey", "parents", "lord"],
        &["children", "obey", "parents", "lord", "right"],
    )?;
    segment.guard(
        2,
        "honour",
        &["father", "mother", "first"],
        &["honour", "father", "mother", "first", "commandment"],
    )?;
    segment.expression(
        3,
        "well",
        &["thee", "mayest", "live"],
        &["well", "thee", "mayest", "live", "long"],
    )?;
    segment.move_value(
        4,
        "children",
        &["fathers", "provoke", "not"],
        &["children", "fathers", "provoke", "not", "wrath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        6,
        5,
        8,
        "servants panic -> guard -> match_value servants obedient masters",
        &["servants", "obedient", "masters", "according", "flesh"],
    );
    segment.panic(
        5,
        "servants",
        &["obedient", "masters", "according"],
        &["servants", "obedient", "masters", "according", "flesh"],
    )?;
    segment.guard(
        6,
        "christ",
        &["not", "eyeservice", "menpleasers"],
        &["christ", "not", "eyeservice", "menpleasers", "servants"],
    )?;
    segment.guard(
        7,
        "lord",
        &["good", "will", "doing"],
        &["lord", "good", "will", "doing", "service"],
    )?;
    segment.match_value(
        8,
        "man",
        &["knowing", "whatsoever", "good"],
        &["man", "knowing", "whatsoever", "good", "thing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        6,
        9,
        12,
        "masters guard -> expression masters same things",
        &["masters", "same", "things", "unto", "forbearing"],
    );
    segment.guard(
        9,
        "masters",
        &["same", "things", "unto"],
        &["masters", "same", "things", "unto", "forbearing"],
    )?;
    segment.expression(
        10,
        "lord",
        &["finally", "brethren", "strong"],
        &["lord", "finally", "brethren", "strong", "power"],
    )?;
    segment.expression(
        11,
        "god",
        &["put", "whole", "armour"],
        &["god", "put", "whole", "armour", "able"],
    )?;
    segment.guard(
        12,
        "wrestle",
        &["not", "against", "flesh"],
        &["wrestle", "not", "against", "flesh", "blood"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        6,
        13,
        16,
        "god expression -> await_value god wherefore take",
        &["god", "wherefore", "take", "unto", "whole"],
    );
    segment.expression(
        13,
        "god",
        &["wherefore", "take", "unto"],
        &["god", "wherefore", "take", "unto", "whole"],
    )?;
    segment.expression(
        14,
        "stand",
        &["therefore", "having", "loins"],
        &["stand", "therefore", "having", "loins", "girt"],
    )?;
    segment.expression(
        15,
        "feet",
        &["shod", "preparation", "gospel"],
        &["feet", "shod", "preparation", "gospel", "peace"],
    )?;
    segment.await_value(
        16,
        "above",
        &["taking", "shield", "faith"],
        &["above", "taking", "shield", "faith", "wherewith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_6_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        6,
        17,
        20,
        "spirit call -> result -> construct spirit take helmet",
        &["spirit", "take", "helmet", "salvation", "sword"],
    );
    segment.call(
        17,
        "spirit",
        &["take", "helmet", "salvation"],
        &["spirit", "take", "helmet", "salvation", "sword"],
    )?;
    segment.result(
        18,
        "spirit",
        &["praying", "always", "prayer"],
        &["spirit", "praying", "always", "prayer", "supplication"],
    )?;
    segment.construct(
        19,
        "utterance",
        &["given", "unto", "open"],
        &["utterance", "given", "unto", "open", "mouth"],
    )?;
    segment.call(
        20,
        "ambassador",
        &["bonds", "therein", "speak"],
        &["ambassador", "bonds", "therein", "speak", "boldly"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ephesians_6_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ephesians,
        6,
        21,
        24,
        "lord construct -> move_value -> await_value -> borrow lord know affairs",
        &["lord", "know", "affairs", "tychicus", "beloved"],
    );
    segment.construct(
        21,
        "lord",
        &["know", "affairs", "tychicus"],
        &["lord", "know", "affairs", "tychicus", "beloved"],
    )?;
    segment.move_value(
        22,
        "sent",
        &["unto", "same", "purpose"],
        &["sent", "unto", "same", "purpose", "might"],
    )?;
    segment.await_value(
        23,
        "god",
        &["peace", "brethren", "love"],
        &["god", "peace", "brethren", "love", "faith"],
    )?;
    segment.borrow(
        24,
        "lord",
        &["grace", "love", "jesus"],
        &["lord", "grace", "love", "jesus", "christ"],
    )?;
    segment.commit()?;
    Ok(())
}
