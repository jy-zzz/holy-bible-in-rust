use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    lamentations(story)?;
    Ok(())
}

fn lamentations(story: &mut Story) -> Result<(), StoryError> {
    lamentations_1(story)?;
    lamentations_2(story)?;
    lamentations_3(story)?;
    lamentations_4(story)?;
    lamentations_5(story)?;
    Ok(())
}

fn lamentations_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Lamentations", 1, "Jerusalem sits alone like a widow after the siege, her former companions gone and her streets carrying only remembered festivals.", "She confesses that the Lord is righteous while asking every passerby to notice the pain of a city whose lovers abandoned her.");
    lamentations_1_group_01(story)?;
    lamentations_1_group_02(story)?;
    lamentations_1_group_03(story)?;
    lamentations_1_group_04(story)?;
    lamentations_1_group_05(story)?;
    lamentations_1_group_06(story)?;
    Ok(())
}

fn lamentations_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Lamentations", 2, "The Lord's anger covers Zion like a storm, tearing down sanctuary, king, wall, and festival in one devastating release.", "The poet tells the walls to cry without pause and urges the fainting children to pour out their hearts before the Lord.");
    lamentations_2_group_01(story)?;
    lamentations_2_group_02(story)?;
    lamentations_2_group_03(story)?;
    lamentations_2_group_04(story)?;
    lamentations_2_group_05(story)?;
    lamentations_2_group_06(story)?;
    Ok(())
}

fn lamentations_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Lamentations", 3, "The speaker walks through darkness, bitterness, and broken teeth, yet remembers that the Lord's mercies are not exhausted.", "Hope becomes a morning routine: wait quietly, bear the yoke, and search the heart even while the city remains under judgment.");
    lamentations_3_group_01(story)?;
    lamentations_3_group_02(story)?;
    lamentations_3_group_03(story)?;
    lamentations_3_group_04(story)?;
    lamentations_3_group_05(story)?;
    lamentations_3_group_06(story)?;
    lamentations_3_group_07(story)?;
    lamentations_3_group_08(story)?;
    lamentations_3_group_09(story)?;
    lamentations_3_group_10(story)?;
    lamentations_3_group_11(story)?;
    lamentations_3_group_12(story)?;
    lamentations_3_group_13(story)?;
    lamentations_3_group_14(story)?;
    lamentations_3_group_15(story)?;
    lamentations_3_group_16(story)?;
    lamentations_3_group_17(story)?;
    Ok(())
}

fn lamentations_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Lamentations", 4, "Holy stones are scattered, princes grow dim, and mothers cook what no mother should cook as famine erases every ordinary boundary.", "Edom is told to rejoice only briefly, because Zion's punishment will end while the neighbor's own cup is still compiling.");
    lamentations_4_group_01(story)?;
    lamentations_4_group_02(story)?;
    lamentations_4_group_03(story)?;
    lamentations_4_group_04(story)?;
    lamentations_4_group_05(story)?;
    lamentations_4_group_06(story)?;
    Ok(())
}

fn lamentations_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Lamentations", 5, "The community lists stolen inheritance, forced labor, violated women, missing elders, and a crown fallen from its head.", "The final prayer asks the Lord to remember and restore, ending not with an easy answer but with a plea that refuses to stop knocking.");
    lamentations_5_group_01(story)?;
    lamentations_5_group_02(story)?;
    lamentations_5_group_03(story)?;
    lamentations_5_group_04(story)?;
    lamentations_5_group_05(story)?;
    lamentations_5_group_06(story)?;
    Ok(())
}

fn lamentations_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        1,
        1,
        4,
        "people expression -> move_value people doth city",
        &["people", "doth", "city", "sit", "solitary"],
    );
    segment.expression(
        1,
        "people",
        &["doth", "city", "sit"],
        &["people", "doth", "city", "sit", "solitary"],
    )?;
    segment.expression(
        2,
        "weepeth",
        &["sore", "night", "tears"],
        &["weepeth", "sore", "night", "tears", "cheeks"],
    )?;
    segment.move_value(
        3,
        "judah",
        &["gone", "captivity", "affliction"],
        &["judah", "gone", "captivity", "affliction", "great"],
    )?;
    segment.move_value(
        4,
        "ways",
        &["zion", "mourn", "none"],
        &["ways", "zion", "mourn", "none", "solemn"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        1,
        5,
        8,
        "lord move_value -> match_value -> expression lord adversaries chief",
        &["lord", "adversaries", "chief", "enemies", "prosper"],
    );
    segment.move_value(
        5,
        "lord",
        &["adversaries", "chief", "enemies"],
        &["lord", "adversaries", "chief", "enemies", "prosper"],
    )?;
    segment.move_value(
        6,
        "daughter",
        &["zion", "beauty", "departed"],
        &["daughter", "zion", "beauty", "departed", "princes"],
    )?;
    segment.match_value(
        7,
        "jerusalem",
        &["remembered", "days", "affliction"],
        &["jerusalem", "remembered", "days", "affliction", "miseries"],
    )?;
    segment.expression(
        8,
        "jerusalem",
        &["hath", "grievously", "sinned"],
        &["jerusalem", "hath", "grievously", "sinned", "therefore"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        1,
        9,
        12,
        "filthiness move_value -> guard -> expression -> match_value filthiness skirts remembereth",
        &["filthiness", "skirts", "remembereth", "not", "last"],
    );
    segment.move_value(
        9,
        "filthiness",
        &["skirts", "remembereth", "not"],
        &["filthiness", "skirts", "remembereth", "not", "last"],
    )?;
    segment.guard(
        10,
        "adversary",
        &["hath", "spread", "hand"],
        &["adversary", "hath", "spread", "hand", "pleasant"],
    )?;
    segment.expression(
        11,
        "people",
        &["sigh", "seek", "bread"],
        &["people", "sigh", "seek", "bread", "given"],
    )?;
    segment.match_value(
        12,
        "nothing",
        &["pass", "behold", "see"],
        &["nothing", "pass", "behold", "see", "any"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        1,
        13,
        16,
        "above construct -> expression -> result above hath sent",
        &["above", "hath", "sent", "fire", "bones"],
    );
    segment.construct(
        13,
        "above",
        &["hath", "sent", "fire"],
        &["above", "hath", "sent", "fire", "bones"],
    )?;
    segment.construct(
        14,
        "yoke",
        &["transgressions", "bound", "hand"],
        &["yoke", "transgressions", "bound", "hand", "wreathed"],
    )?;
    segment.expression(
        15,
        "lord",
        &["hath", "trodden", "under"],
        &["lord", "hath", "trodden", "under", "foot"],
    )?;
    segment.result(
        16,
        "things",
        &["weep", "mine", "eye"],
        &["things", "weep", "mine", "eye", "runneth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        1,
        17,
        19,
        "lord expression -> move_value lord zion spreadeth",
        &["lord", "zion", "spreadeth", "forth", "hands"],
    );
    segment.expression(
        17,
        "lord",
        &["zion", "spreadeth", "forth"],
        &["lord", "zion", "spreadeth", "forth", "hands"],
    )?;
    segment.move_value(
        18,
        "lord",
        &["righteous", "rebelled", "against"],
        &["lord", "righteous", "rebelled", "against", "commandment"],
    )?;
    segment.expression(
        19,
        "called",
        &["lovers", "deceived", "priests"],
        &["called", "lovers", "deceived", "priests", "mine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        1,
        20,
        22,
        "lord drop_value -> move_value lord behold distress",
        &["lord", "behold", "distress", "bowels", "troubled"],
    );
    segment.drop_value(
        20,
        "lord",
        &["behold", "distress", "bowels"],
        &["lord", "behold", "distress", "bowels", "troubled"],
    )?;
    segment.move_value(
        21,
        "heard",
        &["sigh", "none", "comfort"],
        &["heard", "sigh", "none", "comfort", "mine"],
    )?;
    segment.move_value(
        22,
        "wickedness",
        &["thee", "unto", "hast"],
        &["wickedness", "thee", "unto", "hast", "done"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        2,
        1,
        4,
        "lord guard -> move_value -> expression -> trait_bound lord hath covered",
        &["lord", "hath", "covered", "daughter", "zion"],
    );
    segment.guard(
        1,
        "lord",
        &["hath", "covered", "daughter"],
        &["lord", "hath", "covered", "daughter", "zion"],
    )?;
    segment.move_value(
        2,
        "lord",
        &["hath", "swallowed", "habitations"],
        &["lord", "hath", "swallowed", "habitations", "jacob"],
    )?;
    segment.expression(
        3,
        "israel",
        &["hath", "cut", "off"],
        &["israel", "hath", "cut", "off", "fierce"],
    )?;
    segment.trait_bound(
        4,
        "hath",
        &["bent", "bow", "like"],
        &["hath", "bent", "bow", "like", "enemy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        2,
        5,
        8,
        "lord drop_value -> construct lord enemy hath",
        &["lord", "enemy", "hath", "swallowed", "israel"],
    );
    segment.drop_value(
        5,
        "lord",
        &["enemy", "hath", "swallowed"],
        &["lord", "enemy", "hath", "swallowed", "israel"],
    )?;
    segment.drop_value(
        6,
        "hath",
        &["violently", "taken", "away"],
        &["hath", "violently", "taken", "away", "tabernacle"],
    )?;
    segment.construct(
        7,
        "lord",
        &["hath", "cast", "off"],
        &["lord", "hath", "cast", "off", "altar"],
    )?;
    segment.construct(
        8,
        "lord",
        &["hath", "purposed", "destroy"],
        &["lord", "hath", "purposed", "destroy", "wall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        2,
        9,
        12,
        "king drop_value -> guard -> expression -> call king gates sunk",
        &["king", "gates", "sunk", "ground", "hath"],
    );
    segment.drop_value(
        9,
        "king",
        &["gates", "sunk", "ground"],
        &["king", "gates", "sunk", "ground", "hath"],
    )?;
    segment.guard(
        10,
        "elders",
        &["daughter", "zion", "sit"],
        &["elders", "daughter", "zion", "sit", "ground"],
    )?;
    segment.expression(
        11,
        "mine",
        &["eyes", "fail", "tears"],
        &["mine", "eyes", "fail", "tears", "bowels"],
    )?;
    segment.call(
        12,
        "say",
        &["mothers", "corn", "wine"],
        &["say", "mothers", "corn", "wine", "swooned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        2,
        13,
        16,
        "thing expression -> guard -> call thing shall take",
        &["thing", "shall", "take", "witness", "thee"],
    );
    segment.expression(
        13,
        "thing",
        &["shall", "take", "witness"],
        &["thing", "shall", "take", "witness", "thee"],
    )?;
    segment.guard(
        14,
        "prophets",
        &["seen", "vain", "foolish"],
        &["prophets", "seen", "vain", "foolish", "things"],
    )?;
    segment.expression(
        15,
        "pass",
        &["clap", "hands", "thee"],
        &["pass", "clap", "hands", "thee", "hiss"],
    )?;
    segment.call(
        16,
        "thine",
        &["enemies", "opened", "mouth"],
        &["thine", "enemies", "opened", "mouth", "against"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        2,
        17,
        19,
        "lord guard -> result lord hath done",
        &["lord", "hath", "done", "devised", "fulfilled"],
    );
    segment.guard(
        17,
        "lord",
        &["hath", "done", "devised"],
        &["lord", "hath", "done", "devised", "fulfilled"],
    )?;
    segment.guard(
        18,
        "lord",
        &["heart", "cried", "unto"],
        &["lord", "heart", "cried", "unto", "wall"],
    )?;
    segment.result(
        19,
        "arise",
        &["cry", "night", "beginning"],
        &["arise", "cry", "night", "beginning", "watches"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        2,
        20,
        22,
        "lord drop_value -> move_value lord behold consider",
        &["lord", "behold", "consider", "hast", "done"],
    );
    segment.drop_value(
        20,
        "lord",
        &["behold", "consider", "hast"],
        &["lord", "behold", "consider", "hast", "done"],
    )?;
    segment.drop_value(
        21,
        "men",
        &["young", "old", "lie"],
        &["men", "young", "old", "lie", "ground"],
    )?;
    segment.move_value(
        22,
        "hast",
        &["called", "solemn", "day"],
        &["hast", "called", "solemn", "day", "terrors"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        1,
        4,
        "man panic -> move_value -> expression -> construct man hath seen",
        &["man", "hath", "seen", "affliction", "rod"],
    );
    segment.panic(
        1,
        "man",
        &["hath", "seen", "affliction"],
        &["man", "hath", "seen", "affliction", "rod"],
    )?;
    segment.move_value(
        2,
        "hath",
        &["led", "brought", "darkness"],
        &["hath", "led", "brought", "darkness", "not"],
    )?;
    segment.expression(
        3,
        "surely",
        &["against", "turned", "turneth"],
        &["surely", "against", "turned", "turneth", "hand"],
    )?;
    segment.construct(
        4,
        "flesh",
        &["skin", "hath", "made"],
        &["flesh", "skin", "hath", "made", "old"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        5,
        8,
        "hath expression -> drop_value -> construct -> result hath builded against",
        &["hath", "builded", "against", "compassed", "gall"],
    );
    segment.expression(
        5,
        "hath",
        &["builded", "against", "compassed"],
        &["hath", "builded", "against", "compassed", "gall"],
    )?;
    segment.drop_value(
        6,
        "hath",
        &["set", "dark", "places"],
        &["hath", "set", "dark", "places", "dead"],
    )?;
    segment.construct(
        7,
        "hath",
        &["hedged", "cannot", "get"],
        &["hath", "hedged", "cannot", "get", "made"],
    )?;
    segment.result(
        8,
        "cry",
        &["shout", "shutteth", "prayer"],
        &["cry", "shout", "shutteth", "prayer"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        9,
        12,
        "hath construct -> await_value -> expression hath inclosed ways",
        &["hath", "inclosed", "ways", "hewn", "stone"],
    );
    segment.construct(
        9,
        "hath",
        &["inclosed", "ways", "hewn"],
        &["hath", "inclosed", "ways", "hewn", "stone"],
    )?;
    segment.await_value(
        10,
        "unto",
        &["bear", "lying", "wait"],
        &["unto", "bear", "lying", "wait", "lion"],
    )?;
    segment.construct(
        11,
        "hath",
        &["turned", "aside", "ways"],
        &["hath", "turned", "aside", "ways", "pulled"],
    )?;
    segment.expression(
        12,
        "hath",
        &["bent", "bow", "set"],
        &["hath", "bent", "bow", "set", "mark"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        13,
        16,
        "hath expression -> trait_bound -> construct hath caused arrows",
        &["hath", "caused", "arrows", "quiver", "enter"],
    );
    segment.expression(
        13,
        "hath",
        &["caused", "arrows", "quiver"],
        &["hath", "caused", "arrows", "quiver", "enter"],
    )?;
    segment.trait_bound(
        14,
        "people",
        &["derision", "song", "day"],
        &["people", "derision", "song", "day"],
    )?;
    segment.construct(
        15,
        "hath",
        &["filled", "bitterness", "made"],
        &["hath", "filled", "bitterness", "made", "drunken"],
    )?;
    segment.expression(
        16,
        "hath",
        &["broken", "teeth", "gravel"],
        &["hath", "broken", "teeth", "gravel", "stones"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        17,
        20,
        "hast expression -> call hast removed soul",
        &["hast", "removed", "soul", "far", "off"],
    );
    segment.expression(
        17,
        "hast",
        &["removed", "soul", "far"],
        &["hast", "removed", "soul", "far", "off"],
    )?;
    segment.call(
        18,
        "lord",
        &["strength", "hope", "perished"],
        &["lord", "strength", "hope", "perished"],
    )?;
    segment.expression(
        19,
        "remembering",
        &["mine", "affliction", "misery"],
        &["remembering", "mine", "affliction", "misery", "wormwood"],
    )?;
    segment.expression(
        20,
        "soul",
        &["hath", "still", "remembrance"],
        &["soul", "hath", "still", "remembrance", "humbled"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        21,
        24,
        "recall await_value -> guard -> reinitialize recall mind therefore",
        &["recall", "mind", "therefore", "hope", "lord's"],
    );
    segment.await_value(
        21,
        "recall",
        &["mind", "therefore", "hope"],
        &["recall", "mind", "therefore", "hope"],
    )?;
    segment.guard(
        22,
        "lord's",
        &["mercies", "not", "consumed"],
        &["lord's", "mercies", "not", "consumed", "compassions"],
    )?;
    segment.reinitialize(
        23,
        "new",
        &["every", "morning", "great"],
        &["new", "every", "morning", "great", "faithfulness"],
    )?;
    segment.await_value(
        24,
        "lord",
        &["portion", "saith", "soul"],
        &["lord", "portion", "saith", "soul", "therefore"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        25,
        28,
        "lord await_value -> expression lord good unto",
        &["lord", "good", "unto", "wait", "soul"],
    );
    segment.await_value(
        25,
        "lord",
        &["good", "unto", "wait"],
        &["lord", "good", "unto", "wait", "soul"],
    )?;
    segment.await_value(
        26,
        "man",
        &["good", "both", "hope"],
        &["man", "good", "both", "hope", "quietly"],
    )?;
    segment.expression(
        27,
        "man",
        &["good", "bear", "yoke"],
        &["man", "good", "bear", "yoke", "youth"],
    )?;
    segment.expression(
        28,
        "sitteth",
        &["alone", "keepeth", "silence"],
        &["sitteth", "alone", "keepeth", "silence", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        29,
        32,
        "putteth await_value -> expression -> guard -> borrow putteth mouth dust",
        &["putteth", "mouth", "dust", "hope", "giveth"],
    );
    segment.await_value(
        29,
        "putteth",
        &["mouth", "dust", "hope"],
        &["putteth", "mouth", "dust", "hope"],
    )?;
    segment.expression(
        30,
        "giveth",
        &["cheek", "smiteth", "filled"],
        &["giveth", "cheek", "smiteth", "filled", "full"],
    )?;
    segment.guard(
        31,
        "lord",
        &["will", "not", "cast"],
        &["lord", "will", "not", "cast", "off"],
    )?;
    segment.borrow(
        32,
        "though",
        &["cause", "grief", "yet"],
        &["though", "cause", "grief", "yet", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        33,
        36,
        "children guard -> expression children doth not",
        &["children", "doth", "not", "afflict", "willingly"],
    );
    segment.guard(
        33,
        "children",
        &["doth", "not", "afflict"],
        &["children", "doth", "not", "afflict", "willingly"],
    )?;
    segment.expression(
        34,
        "crush",
        &["under", "feet", "prisoners"],
        &["crush", "under", "feet", "prisoners", "earth"],
    )?;
    segment.expression(
        35,
        "man",
        &["turn", "aside", "right"],
        &["man", "turn", "aside", "right", "face"],
    )?;
    segment.guard(
        36,
        "man",
        &["subvert", "cause", "lord"],
        &["man", "subvert", "cause", "lord", "approveth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        37,
        40,
        "lord guard -> expression lord saith cometh",
        &["lord", "saith", "cometh", "pass", "commandeth"],
    );
    segment.guard(
        37,
        "lord",
        &["saith", "cometh", "pass"],
        &["lord", "saith", "cometh", "pass", "commandeth"],
    )?;
    segment.guard(
        38,
        "mouth",
        &["most", "high", "proceedeth"],
        &["mouth", "most", "high", "proceedeth", "not"],
    )?;
    segment.expression(
        39,
        "man",
        &["wherefore", "doth", "living"],
        &["man", "wherefore", "doth", "living", "complain"],
    )?;
    segment.expression(
        40,
        "lord",
        &["search", "try", "ways"],
        &["lord", "search", "try", "ways", "turn"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        41,
        44,
        "god expression -> guard -> drop_value god lift heart",
        &["god", "lift", "heart", "hands", "unto"],
    );
    segment.expression(
        41,
        "god",
        &["lift", "heart", "hands"],
        &["god", "lift", "heart", "hands", "unto"],
    )?;
    segment.guard(
        42,
        "transgressed",
        &["rebelled", "hast", "not"],
        &["transgressed", "rebelled", "hast", "not", "pardoned"],
    )?;
    segment.drop_value(
        43,
        "hast",
        &["covered", "anger", "persecuted"],
        &["hast", "covered", "anger", "persecuted", "slain"],
    )?;
    segment.guard(
        44,
        "hast",
        &["covered", "thyself", "cloud"],
        &["hast", "covered", "thyself", "cloud", "prayer"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        45,
        48,
        "people construct -> expression -> move_value people hast made",
        &["people", "hast", "made", "offscouring", "refuse"],
    );
    segment.construct(
        45,
        "people",
        &["hast", "made", "offscouring"],
        &["people", "hast", "made", "offscouring", "refuse"],
    )?;
    segment.expression(
        46,
        "enemies",
        &["opened", "mouths", "against"],
        &["enemies", "opened", "mouths", "against"],
    )?;
    segment.move_value(
        47,
        "fear",
        &["snare", "desolation", "destruction"],
        &["fear", "snare", "desolation", "destruction"],
    )?;
    segment.expression(
        48,
        "mine",
        &["eye", "runneth", "down"],
        &["mine", "eye", "runneth", "down", "rivers"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_13(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        49,
        52,
        "mine guard -> match_value -> expression mine eye trickleth",
        &["mine", "eye", "trickleth", "down", "ceaseth"],
    );
    segment.guard(
        49,
        "mine",
        &["eye", "trickleth", "down"],
        &["mine", "eye", "trickleth", "down", "ceaseth"],
    )?;
    segment.match_value(
        50,
        "lord",
        &["till", "look", "down"],
        &["lord", "till", "look", "down", "behold"],
    )?;
    segment.expression(
        51,
        "mine",
        &["eye", "affecteth", "heart"],
        &["mine", "eye", "affecteth", "heart", "daughters"],
    )?;
    segment.expression(
        52,
        "mine",
        &["enemies", "chased", "sore"],
        &["mine", "enemies", "chased", "sore", "like"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_14(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        53,
        56,
        "cut expression -> call -> guard cut off life",
        &["cut", "off", "life", "dungeon", "cast"],
    );
    segment.expression(
        53,
        "cut",
        &["off", "life", "dungeon"],
        &["cut", "off", "life", "dungeon", "cast"],
    )?;
    segment.call(
        54,
        "waters",
        &["flowed", "over", "mine"],
        &["waters", "flowed", "over", "mine", "head"],
    )?;
    segment.expression(
        55,
        "lord",
        &["called", "name", "low"],
        &["lord", "called", "name", "low", "dungeon"],
    )?;
    segment.guard(
        56,
        "hast",
        &["heard", "voice", "hide"],
        &["hast", "heard", "voice", "hide", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_15(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        57,
        60,
        "drewest guard -> expression drewest near day",
        &["drewest", "near", "day", "called", "thee"],
    );
    segment.guard(
        57,
        "drewest",
        &["near", "day", "called"],
        &["drewest", "near", "day", "called", "thee"],
    )?;
    segment.expression(
        58,
        "lord",
        &["hast", "pleaded", "causes"],
        &["lord", "hast", "pleaded", "causes", "soul"],
    )?;
    segment.expression(
        59,
        "lord",
        &["hast", "seen", "wrong"],
        &["lord", "hast", "seen", "wrong", "judge"],
    )?;
    segment.expression(
        60,
        "hast",
        &["seen", "vengeance", "imaginations"],
        &["hast", "seen", "vengeance", "imaginations", "against"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_16(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        61,
        63,
        "lord expression -> match_value lord hast heard",
        &["lord", "hast", "heard", "reproach", "imaginations"],
    );
    segment.expression(
        61,
        "lord",
        &["hast", "heard", "reproach"],
        &["lord", "hast", "heard", "reproach", "imaginations"],
    )?;
    segment.expression(
        62,
        "lips",
        &["rose", "against", "device"],
        &["lips", "rose", "against", "device", "day"],
    )?;
    segment.match_value(
        63,
        "behold",
        &["sitting", "down", "rising"],
        &["behold", "sitting", "down", "rising", "musick"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_3_group_17(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        3,
        64,
        66,
        "lord expression -> drop_value lord render unto",
        &["lord", "render", "unto", "recompence", "according"],
    );
    segment.expression(
        64,
        "lord",
        &["render", "unto", "recompence"],
        &["lord", "render", "unto", "recompence", "according"],
    )?;
    segment.expression(
        65,
        "give",
        &["sorrow", "heart", "curse"],
        &["give", "sorrow", "heart", "curse", "unto"],
    )?;
    segment.drop_value(
        66,
        "lord",
        &["persecute", "destroy", "anger"],
        &["lord", "persecute", "destroy", "anger", "under"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        4,
        1,
        4,
        "gold iterate -> expression -> guard gold become dim",
        &["gold", "become", "dim", "most", "fine"],
    );
    segment.iterate(
        1,
        "gold",
        &["become", "dim", "most"],
        &["gold", "become", "dim", "most", "fine"],
    )?;
    segment.iterate(
        2,
        "precious",
        &["sons", "zion", "comparable"],
        &["precious", "sons", "zion", "comparable", "fine"],
    )?;
    segment.expression(
        3,
        "even",
        &["sea", "monsters", "draw"],
        &["even", "sea", "monsters", "draw", "breast"],
    )?;
    segment.guard(
        4,
        "child",
        &["tongue", "sucking", "cleaveth"],
        &["child", "tongue", "sucking", "cleaveth", "roof"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        4,
        5,
        8,
        "feed move_value -> guard -> expression feed delicately desolate",
        &["feed", "delicately", "desolate", "streets", "brought"],
    );
    segment.move_value(
        5,
        "feed",
        &["delicately", "desolate", "streets"],
        &["feed", "delicately", "desolate", "streets", "brought"],
    )?;
    segment.guard(
        6,
        "people",
        &["punishment", "iniquity", "daughter"],
        &["people", "punishment", "iniquity", "daughter", "greater"],
    )?;
    segment.expression(
        7,
        "nazarites",
        &["purer", "snow", "whiter"],
        &["nazarites", "purer", "snow", "whiter", "milk"],
    )?;
    segment.guard(
        8,
        "visage",
        &["blacker", "coal", "not"],
        &["visage", "blacker", "coal", "not", "known"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        4,
        9,
        12,
        "slain drop_value -> expression -> guard slain sword better",
        &["slain", "sword", "better", "hunger", "pine"],
    );
    segment.drop_value(
        9,
        "slain",
        &["sword", "better", "hunger"],
        &["slain", "sword", "better", "hunger", "pine"],
    )?;
    segment.expression(
        10,
        "women",
        &["hands", "pitiful", "sodden"],
        &["women", "hands", "pitiful", "sodden", "own"],
    )?;
    segment.expression(
        11,
        "lord",
        &["hath", "accomplished", "fury"],
        &["lord", "hath", "accomplished", "fury", "poured"],
    )?;
    segment.guard(
        12,
        "kings",
        &["earth", "inhabitants", "world"],
        &["kings", "earth", "inhabitants", "world", "would"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        4,
        13,
        16,
        "sins expression -> guard -> move_value sins prophets iniquities",
        &["sins", "prophets", "iniquities", "priests", "shed"],
    );
    segment.expression(
        13,
        "sins",
        &["prophets", "iniquities", "priests"],
        &["sins", "prophets", "iniquities", "priests", "shed"],
    )?;
    segment.guard(
        14,
        "men",
        &["wandered", "blind", "streets"],
        &["men", "wandered", "blind", "streets", "polluted"],
    )?;
    segment.move_value(
        15,
        "cried",
        &["unto", "depart", "unclean"],
        &["cried", "unto", "depart", "unclean", "touch"],
    )?;
    segment.guard(
        16,
        "lord",
        &["anger", "hath", "divided"],
        &["lord", "anger", "hath", "divided", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        4,
        17,
        19,
        "nation guard -> move_value -> await_value nation eyes yet",
        &["nation", "eyes", "yet", "failed", "vain"],
    );
    segment.guard(
        17,
        "nation",
        &["eyes", "yet", "failed"],
        &["nation", "eyes", "yet", "failed", "vain"],
    )?;
    segment.move_value(
        18,
        "hunt",
        &["steps", "cannot", "streets"],
        &["hunt", "steps", "cannot", "streets", "end"],
    )?;
    segment.await_value(
        19,
        "persecutors",
        &["swifter", "eagles", "heaven"],
        &["persecutors", "swifter", "eagles", "heaven", "pursued"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        4,
        20,
        22,
        "lord call -> construct -> guard lord breath nostrils",
        &["lord", "breath", "nostrils", "anointed", "taken"],
    );
    segment.call(
        20,
        "lord",
        &["breath", "nostrils", "anointed"],
        &["lord", "breath", "nostrils", "anointed", "taken"],
    )?;
    segment.construct(
        21,
        "rejoice",
        &["glad", "daughter", "edom"],
        &["rejoice", "glad", "daughter", "edom", "dwellest"],
    )?;
    segment.guard(
        22,
        "punishment",
        &["thine", "iniquity", "accomplished"],
        &[
            "punishment",
            "thine",
            "iniquity",
            "accomplished",
            "daughter",
        ],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        5,
        1,
        4,
        "lord move_value -> return_value -> expression lord remember consider",
        &["lord", "remember", "consider", "behold", "reproach"],
    );
    segment.move_value(
        1,
        "lord",
        &["remember", "consider", "behold"],
        &["lord", "remember", "consider", "behold", "reproach"],
    )?;
    segment.return_value(
        2,
        "inheritance",
        &["turned", "strangers", "houses"],
        &["inheritance", "turned", "strangers", "houses", "aliens"],
    )?;
    segment.expression(
        3,
        "orphans",
        &["fatherless", "mothers", "widows"],
        &["orphans", "fatherless", "mothers", "widows"],
    )?;
    segment.expression(
        4,
        "drunken",
        &["water", "money", "wood"],
        &["drunken", "water", "money", "wood", "sold"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        5,
        5,
        8,
        "necks guard -> expression necks under persecution",
        &["necks", "under", "persecution", "labour", "rest"],
    );
    segment.guard(
        5,
        "necks",
        &["under", "persecution", "labour"],
        &["necks", "under", "persecution", "labour", "rest"],
    )?;
    segment.expression(
        6,
        "given",
        &["hand", "egyptians", "assyrians"],
        &["given", "hand", "egyptians", "assyrians", "satisfied"],
    )?;
    segment.guard(
        7,
        "fathers",
        &["sinned", "not", "borne"],
        &["fathers", "sinned", "not", "borne", "iniquities"],
    )?;
    segment.expression(
        8,
        "servants",
        &["ruled", "over", "none"],
        &["servants", "ruled", "over", "none", "doth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        5,
        9,
        12,
        "gat expression -> guard gat bread peril",
        &["gat", "bread", "peril", "lives", "sword"],
    );
    segment.expression(
        9,
        "gat",
        &["bread", "peril", "lives"],
        &["gat", "bread", "peril", "lives", "sword"],
    )?;
    segment.expression(
        10,
        "skin",
        &["black", "like", "oven"],
        &["skin", "black", "like", "oven", "terrible"],
    )?;
    segment.expression(
        11,
        "women",
        &["ravished", "zion", "maids"],
        &["women", "ravished", "zion", "maids", "cities"],
    )?;
    segment.guard(
        12,
        "princes",
        &["hanged", "hand", "faces"],
        &["princes", "hanged", "hand", "faces", "elders"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        5,
        13,
        16,
        "men expression -> panic men took young",
        &["men", "took", "young", "grind", "children"],
    );
    segment.expression(
        13,
        "men",
        &["took", "young", "grind"],
        &["men", "took", "young", "grind", "children"],
    )?;
    segment.expression(
        14,
        "men",
        &["elders", "ceased", "gate"],
        &["men", "elders", "ceased", "gate", "young"],
    )?;
    segment.expression(
        15,
        "joy",
        &["heart", "ceased", "dance"],
        &["joy", "heart", "ceased", "dance", "turned"],
    )?;
    segment.panic(
        16,
        "crown",
        &["fallen", "head", "woe"],
        &["crown", "fallen", "head", "woe", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        5,
        17,
        19,
        "heart expression heart faint things",
        &["heart", "faint", "things", "eyes", "dim"],
    );
    segment.expression(
        17,
        "heart",
        &["faint", "things", "eyes"],
        &["heart", "faint", "things", "eyes", "dim"],
    )?;
    segment.expression(
        18,
        "mountain",
        &["zion", "desolate", "foxes"],
        &["mountain", "zion", "desolate", "foxes", "walk"],
    )?;
    segment.expression(
        19,
        "lord",
        &["remainest", "ever", "throne"],
        &["lord", "remainest", "ever", "throne", "generation"],
    )?;
    segment.commit()?;
    Ok(())
}

fn lamentations_5_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Lamentations,
        5,
        20,
        22,
        "wherefore expression wherefore dost forget",
        &["wherefore", "dost", "forget", "ever", "forsake"],
    );
    segment.expression(
        20,
        "wherefore",
        &["dost", "forget", "ever"],
        &["wherefore", "dost", "forget", "ever", "forsake"],
    )?;
    segment.expression(
        21,
        "lord",
        &["turn", "unto", "thee"],
        &["lord", "turn", "unto", "thee", "shall"],
    )?;
    segment.expression(
        22,
        "hast",
        &["utterly", "rejected", "art"],
        &["hast", "utterly", "rejected", "art", "very"],
    )?;
    segment.commit()?;
    Ok(())
}
