use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    romans(story)?;
    Ok(())
}

fn romans(story: &mut Story) -> Result<(), StoryError> {
    romans_1(story)?;
    romans_2(story)?;
    romans_3(story)?;
    romans_4(story)?;
    romans_5(story)?;
    romans_6(story)?;
    romans_7(story)?;
    romans_8(story)?;
    romans_9(story)?;
    romans_10(story)?;
    romans_11(story)?;
    romans_12(story)?;
    romans_13(story)?;
    romans_14(story)?;
    romans_15(story)?;
    romans_16(story)?;
    Ok(())
}

fn romans_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 1, "Paul introduces himself as a commissioned messenger and says the good news reveals a rightness that runs on trust rather than status.", "He watches humanity exchange the Creator's glory for carved dependencies, and the resulting codebase spirals into pride, envy, violence, and approval of every bad patch.");
    romans_1_group_01(story)?;
    romans_1_group_02(story)?;
    romans_1_group_03(story)?;
    romans_1_group_04(story)?;
    romans_1_group_05(story)?;
    romans_1_group_06(story)?;
    romans_1_group_07(story)?;
    romans_1_group_08(story)?;
    Ok(())
}

fn romans_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 2, "The reviewer who condemns everyone else discovers the same warnings in their own build log, because God's audit examines hidden motives as well as public commits.", "Possessing a law, a badge, or a repository label cannot substitute for a changed heart; the true mark is an inward work of the Spirit.");
    romans_2_group_01(story)?;
    romans_2_group_02(story)?;
    romans_2_group_03(story)?;
    romans_2_group_04(story)?;
    romans_2_group_05(story)?;
    romans_2_group_06(story)?;
    romans_2_group_07(story)?;
    Ok(())
}

fn romans_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 3, "Every faction has failed its tests, yet God's faithfulness is not canceled by human unfaithfulness or by the loudest list of exceptions.", "The new verdict comes through trust in Jesus rather than boasting in rule-keeping, leaving the whole team equally dependent on grace.");
    romans_3_group_01(story)?;
    romans_3_group_02(story)?;
    romans_3_group_03(story)?;
    romans_3_group_04(story)?;
    romans_3_group_05(story)?;
    romans_3_group_06(story)?;
    romans_3_group_07(story)?;
    romans_3_group_08(story)?;
    Ok(())
}

fn romans_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 4, "Abraham is presented as the ancient case study: counted right before circumcision, before the badge, and before any performance metric could be claimed.", "His trust treats God's promise as a reliable return value, bringing life from a body that looked past its normal operating window.");
    romans_4_group_01(story)?;
    romans_4_group_02(story)?;
    romans_4_group_03(story)?;
    romans_4_group_04(story)?;
    romans_4_group_05(story)?;
    romans_4_group_06(story)?;
    Ok(())
}

fn romans_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 5, "Being made right opens peace with God, and suffering is reframed as a chain that can compile perseverance, character, and hope.", "Adam's trespass spreads like a corrupted base commit, but the gift through Christ overflows farther, bringing life where the old failure brought death.");
    romans_5_group_01(story)?;
    romans_5_group_02(story)?;
    romans_5_group_03(story)?;
    romans_5_group_04(story)?;
    romans_5_group_05(story)?;
    Ok(())
}

fn romans_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 6, "Grace is not permission to keep shipping the same destructive release; those joined to Christ are pictured as having died to the old ownership model.", "The former master paid in death, while the new master gives a life that is not earned as wages but received as a gift.");
    romans_6_group_01(story)?;
    romans_6_group_02(story)?;
    romans_6_group_03(story)?;
    romans_6_group_04(story)?;
    romans_6_group_05(story)?;
    romans_6_group_06(story)?;
    Ok(())
}

fn romans_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 7, "The law can expose the bug without possessing the power to fix it, leaving a conflicted developer wanting the good while finding the old process running again.", "The cry for rescue is answered by Jesus Christ, because diagnosis alone cannot free a person from the body of death.");
    romans_7_group_01(story)?;
    romans_7_group_02(story)?;
    romans_7_group_03(story)?;
    romans_7_group_04(story)?;
    romans_7_group_05(story)?;
    romans_7_group_06(story)?;
    Ok(())
}

fn romans_8(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 8, "There is no condemnation for those in Christ, whose new runtime is governed by the Spirit rather than by the old sin process.", "Creation groans, believers wait, and the Spirit intercedes; through every crash and unknown state, God's love refuses to release its hold.");
    romans_8_group_01(story)?;
    romans_8_group_02(story)?;
    romans_8_group_03(story)?;
    romans_8_group_04(story)?;
    romans_8_group_05(story)?;
    romans_8_group_06(story)?;
    romans_8_group_07(story)?;
    romans_8_group_08(story)?;
    romans_8_group_09(story)?;
    romans_8_group_10(story)?;
    Ok(())
}

fn romans_9(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 9, "Paul grieves over Israel and distinguishes ancestry from the deeper freedom of God's calling, using a potter and clay image to discuss mercy and patience.", "Gentiles who trusted found the promised righteousness, while Israel's pursuit through self-made credentials stumbled over the stone God provided.");
    romans_9_group_01(story)?;
    romans_9_group_02(story)?;
    romans_9_group_03(story)?;
    romans_9_group_04(story)?;
    romans_9_group_05(story)?;
    romans_9_group_06(story)?;
    romans_9_group_07(story)?;
    romans_9_group_08(story)?;
    Ok(())
}

fn romans_10(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 10, "Zeal without knowledge is an overconfident build, and righteousness cannot be uploaded by climbing into heaven or digging into the abyss.", "The message is near enough to confess and trust, while the good news travels outward through messengers whose feet carry the deployment.");
    romans_10_group_01(story)?;
    romans_10_group_02(story)?;
    romans_10_group_03(story)?;
    romans_10_group_04(story)?;
    romans_10_group_05(story)?;
    Ok(())
}

fn romans_11(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 11, "Israel's present stumbling is not a final deletion: Gentiles are grafted in like branches warned against boasting over the root.", "Paul traces a mystery of mercy, ends in astonished praise, and reminds every branch that the whole tree depends on God's generosity.");
    romans_11_group_01(story)?;
    romans_11_group_02(story)?;
    romans_11_group_03(story)?;
    romans_11_group_04(story)?;
    romans_11_group_05(story)?;
    romans_11_group_06(story)?;
    romans_11_group_07(story)?;
    romans_11_group_08(story)?;
    romans_11_group_09(story)?;
    Ok(())
}

fn romans_12(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 12, "The community is urged to offer its whole life as a living service and to stop copying the world's default configuration.", "Many members receive different gifts, but sincere love, patient hope, practical hospitality, and blessing enemies are the shared interface.");
    romans_12_group_01(story)?;
    romans_12_group_02(story)?;
    romans_12_group_03(story)?;
    romans_12_group_04(story)?;
    romans_12_group_05(story)?;
    Ok(())
}

fn romans_13(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 13, "Paul addresses public authority, taxes, and civic obligations, then compresses the law's neighbor-facing work into love.", "The night is far spent, so believers are told to put on the Lord's character and stop provisioning the appetites that belong to darkness.");
    romans_13_group_01(story)?;
    romans_13_group_02(story)?;
    romans_13_group_03(story)?;
    romans_13_group_04(story)?;
    Ok(())
}

fn romans_14(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 14, "Disputed diets and special days become a lesson in not turning personal settings into a universal gate, since each servant answers to the Lord.", "Stronger members must avoid making a sibling stumble; the kingdom's core values are righteousness, peace, and joy rather than menu arguments.");
    romans_14_group_01(story)?;
    romans_14_group_02(story)?;
    romans_14_group_03(story)?;
    romans_14_group_04(story)?;
    romans_14_group_05(story)?;
    romans_14_group_06(story)?;
    Ok(())
}

fn romans_15(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 15, "The strong are told to carry the weak, and Christ is offered as the pattern for welcoming people across old boundary conditions.", "Paul describes his mission to unreached regions, asks for prayer about danger in Jerusalem, and hopes to visit Rome on the way toward Spain.");
    romans_15_group_01(story)?;
    romans_15_group_02(story)?;
    romans_15_group_03(story)?;
    romans_15_group_04(story)?;
    romans_15_group_05(story)?;
    romans_15_group_06(story)?;
    romans_15_group_07(story)?;
    romans_15_group_08(story)?;
    Ok(())
}

fn romans_16(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 16, "Paul greets a long registry of coworkers, naming women, household churches, prisoners, hosts, and laborers whose names keep the mission from becoming a solo project.", "He warns against smooth teachers who divide communities, then closes with confidence that God's wisdom will outlast every deceptive fork.");
    romans_16_group_01(story)?;
    romans_16_group_02(story)?;
    romans_16_group_03(story)?;
    romans_16_group_04(story)?;
    romans_16_group_05(story)?;
    romans_16_group_06(story)?;
    romans_16_group_07(story)?;
    Ok(())
}

fn romans_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        1,
        4,
        "paul expression -> construct -> drop_value paul servant jesus",
        &["paul", "servant", "jesus", "christ", "called"],
    );
    segment.expression(
        1,
        "paul",
        &["servant", "jesus", "christ"],
        &["paul", "servant", "jesus", "christ", "called"],
    )?;
    segment.expression(
        2,
        "promised",
        &["afore", "prophets", "holy"],
        &["promised", "afore", "prophets", "holy", "scriptures"],
    )?;
    segment.construct(
        3,
        "jesus",
        &["concerning", "son", "christ"],
        &["jesus", "concerning", "son", "christ", "lord"],
    )?;
    segment.drop_value(
        4,
        "god",
        &["declared", "son", "power"],
        &["god", "declared", "son", "power", "according"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        5,
        8,
        "nations await_value -> expression -> borrow nations received grace",
        &["nations", "received", "grace", "apostleship", "obedience"],
    );
    segment.await_value(
        5,
        "nations",
        &["received", "grace", "apostleship"],
        &["nations", "received", "grace", "apostleship", "obedience"],
    )?;
    segment.expression(
        6,
        "jesus",
        &["among", "called", "christ"],
        &["jesus", "among", "called", "christ"],
    )?;
    segment.borrow(
        7,
        "god",
        &["rome", "beloved", "called"],
        &["god", "rome", "beloved", "called", "saints"],
    )?;
    segment.await_value(
        8,
        "god",
        &["first", "thank", "through"],
        &["god", "first", "thank", "through", "jesus"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        9,
        12,
        "god construct -> move_value -> expression -> await_value god witness serve",
        &["god", "witness", "serve", "spirit", "gospel"],
    );
    segment.construct(
        9,
        "god",
        &["witness", "serve", "spirit"],
        &["god", "witness", "serve", "spirit", "gospel"],
    )?;
    segment.move_value(
        10,
        "making",
        &["request", "any", "means"],
        &["making", "request", "any", "means", "now"],
    )?;
    segment.expression(
        11,
        "long",
        &["see", "impart", "unto"],
        &["long", "see", "impart", "unto", "spiritual"],
    )?;
    segment.await_value(
        12,
        "comforted",
        &["together", "mutual", "faith"],
        &["comforted", "together", "mutual", "faith", "both"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        13,
        16,
        "now move_value -> expression -> call -> guard now would not",
        &["now", "would", "not", "ignorant", "brethren"],
    );
    segment.move_value(
        13,
        "now",
        &["would", "not", "ignorant"],
        &["now", "would", "not", "ignorant", "brethren"],
    )?;
    segment.expression(
        14,
        "debtor",
        &["both", "greeks", "barbarians"],
        &["debtor", "both", "greeks", "barbarians", "wise"],
    )?;
    segment.call(
        15,
        "much",
        &["ready", "preach", "gospel"],
        &["much", "ready", "preach", "gospel", "rome"],
    )?;
    segment.guard(
        16,
        "christ",
        &["not", "ashamed", "gospel"],
        &["christ", "not", "ashamed", "gospel", "power"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        17,
        20,
        "god match_value -> expression -> construct god therein righteousness",
        &["god", "therein", "righteousness", "revealed", "faith"],
    );
    segment.match_value(
        17,
        "god",
        &["therein", "righteousness", "revealed"],
        &["god", "therein", "righteousness", "revealed", "faith"],
    )?;
    segment.match_value(
        18,
        "god",
        &["wrath", "revealed", "heaven"],
        &["god", "wrath", "revealed", "heaven", "against"],
    )?;
    segment.expression(
        19,
        "god",
        &["known", "manifest", "hath"],
        &["god", "known", "manifest", "hath", "shewed"],
    )?;
    segment.construct(
        20,
        "invisible",
        &["things", "creation", "world"],
        &["invisible", "things", "creation", "world", "clearly"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        21,
        24,
        "god guard -> expression -> construct god knew glorified",
        &["god", "knew", "glorified", "not", "neither"],
    );
    segment.guard(
        21,
        "god",
        &["knew", "glorified", "not"],
        &["god", "knew", "glorified", "not", "neither"],
    )?;
    segment.expression(
        22,
        "professing",
        &["themselves", "wise", "became"],
        &["professing", "themselves", "wise", "became", "fools"],
    )?;
    segment.construct(
        23,
        "god",
        &["changed", "glory", "uncorruptible"],
        &["god", "changed", "glory", "uncorruptible", "image"],
    )?;
    segment.expression(
        24,
        "god",
        &["wherefore", "gave", "uncleanness"],
        &["god", "wherefore", "gave", "uncleanness", "through"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_1_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        25,
        28,
        "god return_value -> expression -> guard god changed truth",
        &["god", "changed", "truth", "lie", "worshipped"],
    );
    segment.return_value(
        25,
        "god",
        &["changed", "truth", "lie"],
        &["god", "changed", "truth", "lie", "worshipped"],
    )?;
    segment.expression(
        26,
        "god",
        &["cause", "gave", "unto"],
        &["god", "cause", "gave", "unto", "vile"],
    )?;
    segment.expression(
        27,
        "men",
        &["likewise", "leaving", "natural"],
        &["men", "likewise", "leaving", "natural", "use"],
    )?;
    segment.guard(
        28,
        "god",
        &["even", "not", "like"],
        &["god", "even", "not", "like", "retain"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_1_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        1,
        29,
        32,
        "filled expression -> drop_value filled unrighteousness fornication",
        &[
            "filled",
            "unrighteousness",
            "fornication",
            "wickedness",
            "covetousness",
        ],
    );
    segment.expression(
        29,
        "filled",
        &["unrighteousness", "fornication", "wickedness"],
        &[
            "filled",
            "unrighteousness",
            "fornication",
            "wickedness",
            "covetousness",
        ],
    )?;
    segment.expression(
        30,
        "god",
        &["backbiters", "haters", "despiteful"],
        &["god", "backbiters", "haters", "despiteful", "proud"],
    )?;
    segment.expression(
        31,
        "without",
        &["understanding", "covenantbreakers", "natural"],
        &[
            "without",
            "understanding",
            "covenantbreakers",
            "natural",
            "affection",
        ],
    )?;
    segment.drop_value(
        32,
        "god",
        &["knowing", "judgment", "commit"],
        &["god", "knowing", "judgment", "commit", "such"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        2,
        1,
        4,
        "man expression -> guard man therefore art",
        &["man", "therefore", "art", "inexcusable", "whosoever"],
    );
    segment.expression(
        1,
        "man",
        &["therefore", "art", "inexcusable"],
        &["man", "therefore", "art", "inexcusable", "whosoever"],
    )?;
    segment.expression(
        2,
        "god",
        &["sure", "judgment", "according"],
        &["god", "sure", "judgment", "according", "truth"],
    )?;
    segment.guard(
        3,
        "man",
        &["thinkest", "judgest", "such"],
        &["man", "thinkest", "judgest", "such", "things"],
    )?;
    segment.guard(
        4,
        "god",
        &["despisest", "riches", "goodness"],
        &["god", "despisest", "riches", "goodness", "forbearance"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        2,
        5,
        8,
        "hardness match_value -> iterate -> trait_bound -> guard hardness impenitent heart",
        &["hardness", "impenitent", "heart", "treasurest", "unto"],
    );
    segment.match_value(
        5,
        "hardness",
        &["impenitent", "heart", "treasurest"],
        &["hardness", "impenitent", "heart", "treasurest", "unto"],
    )?;
    segment.iterate(
        6,
        "man",
        &["will", "render", "every"],
        &["man", "will", "render", "every", "according"],
    )?;
    segment.trait_bound(
        7,
        "patient",
        &["continuance", "well", "doing"],
        &["patient", "continuance", "well", "doing", "seek"],
    )?;
    segment.guard(
        8,
        "unto",
        &["contentious", "not", "obey"],
        &["unto", "contentious", "not", "obey", "truth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        2,
        9,
        12,
        "man iterate -> trait_bound -> guard man tribulation anguish",
        &["man", "tribulation", "anguish", "every", "soul"],
    );
    segment.iterate(
        9,
        "man",
        &["tribulation", "anguish", "every"],
        &["man", "tribulation", "anguish", "every", "soul"],
    )?;
    segment.trait_bound(
        10,
        "man",
        &["glory", "honour", "peace"],
        &["man", "glory", "honour", "peace", "every"],
    )?;
    segment.guard(
        11,
        "god",
        &["respect", "persons"],
        &["god", "respect", "persons"],
    )?;
    segment.guard(
        12,
        "many",
        &["sinned", "without", "law"],
        &["many", "sinned", "without", "law", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        2,
        13,
        16,
        "god guard -> expression god not hearers",
        &["god", "not", "hearers", "law", "just"],
    );
    segment.guard(
        13,
        "god",
        &["not", "hearers", "law"],
        &["god", "not", "hearers", "law", "just"],
    )?;
    segment.guard(
        14,
        "gentiles",
        &["not", "law", "nature"],
        &["gentiles", "not", "law", "nature", "things"],
    )?;
    segment.guard(
        15,
        "shew",
        &["work", "law", "written"],
        &["shew", "work", "law", "written", "hearts"],
    )?;
    segment.expression(
        16,
        "god",
        &["day", "shall", "judge"],
        &["god", "day", "shall", "judge", "secrets"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        2,
        17,
        20,
        "behold guard -> expression behold art called",
        &["behold", "art", "called", "jew", "restest"],
    );
    segment.guard(
        17,
        "behold",
        &["art", "called", "jew"],
        &["behold", "art", "called", "jew", "restest"],
    )?;
    segment.guard(
        18,
        "knowest",
        &["will", "approvest", "things"],
        &["knowest", "will", "approvest", "things", "excellent"],
    )?;
    segment.expression(
        19,
        "art",
        &["confident", "thyself", "guide"],
        &["art", "confident", "thyself", "guide", "blind"],
    )?;
    segment.guard(
        20,
        "instructor",
        &["foolish", "teacher", "babes"],
        &["instructor", "foolish", "teacher", "babes", "hast"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        2,
        21,
        24,
        "man guard -> expression man therefore teachest",
        &["man", "therefore", "teachest", "another", "not"],
    );
    segment.guard(
        21,
        "man",
        &["therefore", "teachest", "another"],
        &["man", "therefore", "teachest", "another", "not"],
    )?;
    segment.guard(
        22,
        "man",
        &["sayest", "not", "commit"],
        &["man", "sayest", "not", "commit", "adultery"],
    )?;
    segment.guard(
        23,
        "god",
        &["makest", "boast", "law"],
        &["god", "makest", "boast", "law", "through"],
    )?;
    segment.expression(
        24,
        "god",
        &["name", "blasphemed", "among"],
        &["god", "name", "blasphemed", "among", "gentiles"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        2,
        25,
        29,
        "circumcision construct -> guard circumcision verily profiteth",
        &["circumcision", "verily", "profiteth", "keep", "law"],
    );
    segment.construct(
        25,
        "circumcision",
        &["verily", "profiteth", "keep"],
        &["circumcision", "verily", "profiteth", "keep", "law"],
    )?;
    segment.guard(
        26,
        "therefore",
        &["uncircumcision", "keep", "righteousness"],
        &[
            "therefore",
            "uncircumcision",
            "keep",
            "righteousness",
            "law",
        ],
    )?;
    segment.guard(
        27,
        "shall",
        &["not", "uncircumcision", "nature"],
        &["shall", "not", "uncircumcision", "nature", "fulfil"],
    )?;
    segment.guard(
        28,
        "not",
        &["jew", "outwardly", "neither"],
        &["not", "jew", "outwardly", "neither", "circumcision"],
    )?;
    segment.guard(
        29,
        "spirit",
        &["jew", "inwardly", "circumcision"],
        &["spirit", "jew", "inwardly", "circumcision", "heart"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        1,
        4,
        "advantage expression -> iterate -> construct -> guard advantage hath jew",
        &["advantage", "hath", "jew", "profit", "circumcision"],
    );
    segment.expression(
        1,
        "advantage",
        &["hath", "jew", "profit"],
        &["advantage", "hath", "jew", "profit", "circumcision"],
    )?;
    segment.iterate(
        2,
        "god",
        &["much", "every", "way"],
        &["god", "much", "every", "way", "chiefly"],
    )?;
    segment.construct(
        3,
        "god",
        &["not", "believe", "shall"],
        &["god", "not", "believe", "shall", "unbelief"],
    )?;
    segment.guard(
        4,
        "god",
        &["forbid", "yea", "true"],
        &["god", "forbid", "yea", "true", "every"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        5,
        8,
        "god call -> guard -> trait_bound -> move_value god unrighteousness commend",
        &[
            "god",
            "unrighteousness",
            "commend",
            "righteousness",
            "shall",
        ],
    );
    segment.call(
        5,
        "god",
        &["unrighteousness", "commend", "righteousness"],
        &[
            "god",
            "unrighteousness",
            "commend",
            "righteousness",
            "shall",
        ],
    )?;
    segment.guard(
        6,
        "god",
        &["forbid", "shall", "judge"],
        &["god", "forbid", "shall", "judge", "world"],
    )?;
    segment.trait_bound(
        7,
        "god",
        &["truth", "hath", "abounded"],
        &["god", "truth", "hath", "abounded", "through"],
    )?;
    segment.move_value(
        8,
        "not",
        &["rather", "slanderously", "reported"],
        &["not", "rather", "slanderously", "reported", "affirm"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        9,
        12,
        "better guard -> expression -> move_value better wise proved",
        &["better", "wise", "proved", "both", "jews"],
    );
    segment.guard(
        9,
        "better",
        &["wise", "proved", "both"],
        &["better", "wise", "proved", "both", "jews"],
    )?;
    segment.guard(
        10,
        "written",
        &["none", "righteous", "not"],
        &["written", "none", "righteous", "not"],
    )?;
    segment.expression(
        11,
        "god",
        &["none", "understandeth", "seeketh"],
        &["god", "none", "understandeth", "seeketh"],
    )?;
    segment.move_value(
        12,
        "gone",
        &["way", "together", "become"],
        &["gone", "way", "together", "become", "unprofitable"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        13,
        16,
        "throat expression throat open sepulchre",
        &["throat", "open", "sepulchre", "tongues", "used"],
    );
    segment.expression(
        13,
        "throat",
        &["open", "sepulchre", "tongues"],
        &["throat", "open", "sepulchre", "tongues", "used"],
    )?;
    segment.expression(
        14,
        "whose",
        &["mouth", "full", "cursing"],
        &["whose", "mouth", "full", "cursing", "bitterness"],
    )?;
    segment.expression(
        15,
        "feet",
        &["swift", "shed", "blood"],
        &["feet", "swift", "shed", "blood"],
    )?;
    segment.expression(
        16,
        "destruction",
        &["misery", "ways"],
        &["destruction", "misery", "ways"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        17,
        20,
        "way guard way peace not",
        &["way", "peace", "not", "known", "god"],
    );
    segment.guard(
        17,
        "way",
        &["peace", "not", "known"],
        &["way", "peace", "not", "known"],
    )?;
    segment.guard(18, "god", &["fear", "eyes"], &["god", "fear", "eyes"])?;
    segment.guard(
        19,
        "now",
        &["know", "things", "soever"],
        &["now", "know", "things", "soever", "law"],
    )?;
    segment.guard(
        20,
        "therefore",
        &["deeds", "law", "shall"],
        &["therefore", "deeds", "law", "shall", "flesh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        21,
        24,
        "god guard -> move_value -> borrow god now righteousness",
        &["god", "now", "righteousness", "without", "law"],
    );
    segment.guard(
        21,
        "god",
        &["now", "righteousness", "without"],
        &["god", "now", "righteousness", "without", "law"],
    )?;
    segment.guard(
        22,
        "god",
        &["even", "righteousness", "faith"],
        &["god", "even", "righteousness", "faith", "jesus"],
    )?;
    segment.move_value(
        23,
        "god",
        &["sinned", "short", "glory"],
        &["god", "sinned", "short", "glory"],
    )?;
    segment.borrow(
        24,
        "christ",
        &["justified", "freely", "grace"],
        &["christ", "justified", "freely", "grace", "through"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        25,
        28,
        "god call -> guard god hath set",
        &["god", "hath", "set", "forth", "propitiation"],
    );
    segment.call(
        25,
        "god",
        &["hath", "set", "forth"],
        &["god", "hath", "set", "forth", "propitiation"],
    )?;
    segment.call(
        26,
        "declare",
        &["say", "time", "righteousness"],
        &["declare", "say", "time", "righteousness", "might"],
    )?;
    segment.guard(
        27,
        "boasting",
        &["excluded", "law", "works"],
        &["boasting", "excluded", "law", "works", "nay"],
    )?;
    segment.guard(
        28,
        "man",
        &["therefore", "conclude", "justified"],
        &["man", "therefore", "conclude", "justified", "faith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_3_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        3,
        29,
        31,
        "god guard -> await_value -> construct god jews not",
        &["god", "jews", "not", "gentiles", "yes"],
    );
    segment.guard(
        29,
        "god",
        &["jews", "not", "gentiles"],
        &["god", "jews", "not", "gentiles", "yes"],
    )?;
    segment.await_value(
        30,
        "god",
        &["seeing", "shall", "justify"],
        &["god", "seeing", "shall", "justify", "circumcision"],
    )?;
    segment.construct(
        31,
        "god",
        &["make", "void", "law"],
        &["god", "make", "void", "law", "through"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        4,
        1,
        4,
        "abraham call -> guard -> await_value abraham shall say",
        &["abraham", "shall", "say", "father", "pertaining"],
    );
    segment.call(
        1,
        "abraham",
        &["shall", "say", "father"],
        &["abraham", "shall", "say", "father", "pertaining"],
    )?;
    segment.guard(
        2,
        "abraham",
        &["justified", "works", "hath"],
        &["abraham", "justified", "works", "hath", "whereof"],
    )?;
    segment.await_value(
        3,
        "abraham",
        &["saith", "scripture", "believed"],
        &["abraham", "saith", "scripture", "believed", "god"],
    )?;
    segment.guard(
        4,
        "now",
        &["worketh", "reward", "not"],
        &["now", "worketh", "reward", "not", "reckoned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        4,
        5,
        8,
        "worketh guard -> expression -> return_value worketh not believeth",
        &["worketh", "not", "believeth", "justifieth", "ungodly"],
    );
    segment.guard(
        5,
        "worketh",
        &["not", "believeth", "justifieth"],
        &["worketh", "not", "believeth", "justifieth", "ungodly"],
    )?;
    segment.expression(
        6,
        "david",
        &["even", "describeth", "blessedness"],
        &["david", "even", "describeth", "blessedness", "man"],
    )?;
    segment.return_value(
        7,
        "saying",
        &["blessed", "whose", "iniquities"],
        &["saying", "blessed", "whose", "iniquities", "forgiven"],
    )?;
    segment.guard(
        8,
        "man",
        &["blessed", "lord", "will"],
        &["man", "blessed", "lord", "will", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        4,
        9,
        12,
        "abraham call -> guard abraham cometh blessedness",
        &[
            "abraham",
            "cometh",
            "blessedness",
            "circumcision",
            "uncircumcision",
        ],
    );
    segment.call(
        9,
        "abraham",
        &["cometh", "blessedness", "circumcision"],
        &[
            "abraham",
            "cometh",
            "blessedness",
            "circumcision",
            "uncircumcision",
        ],
    )?;
    segment.guard(
        10,
        "reckoned",
        &["circumcision", "uncircumcision", "not"],
        &["reckoned", "circumcision", "uncircumcision", "not"],
    )?;
    segment.guard(
        11,
        "received",
        &["sign", "circumcision", "seal"],
        &["received", "sign", "circumcision", "seal", "righteousness"],
    )?;
    segment.guard(
        12,
        "abraham",
        &["father", "circumcision", "not"],
        &["abraham", "father", "circumcision", "not", "walk"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        4,
        13,
        16,
        "abraham guard -> construct abraham promise heir",
        &["abraham", "promise", "heir", "world", "not"],
    );
    segment.guard(
        13,
        "abraham",
        &["promise", "heir", "world"],
        &["abraham", "promise", "heir", "world", "not"],
    )?;
    segment.construct(
        14,
        "law",
        &["heirs", "faith", "made"],
        &["law", "heirs", "faith", "made", "void"],
    )?;
    segment.guard(
        15,
        "law",
        &["worketh", "wrath", "transgression"],
        &["law", "worketh", "wrath", "transgression"],
    )?;
    segment.guard(
        16,
        "therefore",
        &["faith", "might", "grace"],
        &["therefore", "faith", "might", "grace", "end"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        4,
        17,
        20,
        "nations construct -> await_value -> drop_value -> guard nations written made",
        &["nations", "written", "made", "thee", "father"],
    );
    segment.construct(
        17,
        "nations",
        &["written", "made", "thee"],
        &["nations", "written", "made", "thee", "father"],
    )?;
    segment.await_value(
        18,
        "nations",
        &["against", "hope", "believed"],
        &["nations", "against", "hope", "believed", "might"],
    )?;
    segment.drop_value(
        19,
        "not",
        &["weak", "faith", "considered"],
        &["not", "weak", "faith", "considered", "own"],
    )?;
    segment.guard(
        20,
        "god",
        &["staggered", "not", "promise"],
        &["god", "staggered", "not", "promise", "through"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        4,
        21,
        25,
        "fully expression -> guard -> drop_value -> reinitialize fully persuaded promised",
        &["fully", "persuaded", "promised", "able", "perform"],
    );
    segment.expression(
        21,
        "fully",
        &["persuaded", "promised", "able"],
        &["fully", "persuaded", "promised", "able", "perform"],
    )?;
    segment.expression(
        22,
        "therefore",
        &["imputed", "righteousness"],
        &["therefore", "imputed", "righteousness"],
    )?;
    segment.guard(
        23,
        "now",
        &["not", "written", "sake"],
        &["now", "not", "written", "sake", "alone"],
    )?;
    segment.drop_value(
        24,
        "jesus",
        &["shall", "imputed", "believe"],
        &["jesus", "shall", "imputed", "believe", "raised"],
    )?;
    segment.reinitialize(
        25,
        "delivered",
        &["offences", "raised", "again"],
        &["delivered", "offences", "raised", "again", "justification"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        5,
        1,
        4,
        "god await_value -> trait_bound -> guard god therefore justified",
        &["god", "therefore", "justified", "faith", "peace"],
    );
    segment.await_value(
        1,
        "god",
        &["therefore", "justified", "faith"],
        &["god", "therefore", "justified", "faith", "peace"],
    )?;
    segment.trait_bound(
        2,
        "access",
        &["faith", "grace", "wherein"],
        &["access", "faith", "grace", "wherein", "stand"],
    )?;
    segment.guard(
        3,
        "not",
        &["glory", "tribulations", "knowing"],
        &["not", "glory", "tribulations", "knowing", "tribulation"],
    )?;
    segment.await_value(
        4,
        "patience",
        &["experience", "hope"],
        &["patience", "experience", "hope"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        5,
        5,
        8,
        "god guard -> drop_value -> expression god hope maketh",
        &["god", "hope", "maketh", "not", "ashamed"],
    );
    segment.guard(
        5,
        "god",
        &["hope", "maketh", "not"],
        &["god", "hope", "maketh", "not", "ashamed"],
    )?;
    segment.drop_value(
        6,
        "christ",
        &["yet", "without", "strength"],
        &["christ", "yet", "without", "strength", "due"],
    )?;
    segment.expression(
        7,
        "man",
        &["scarcely", "righteous", "will"],
        &["man", "scarcely", "righteous", "will", "die"],
    )?;
    segment.drop_value(
        8,
        "god",
        &["commendeth", "love", "toward"],
        &["god", "commendeth", "love", "toward", "while"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        5,
        9,
        12,
        "much panic -> drop_value -> guard much now justified",
        &["much", "now", "justified", "blood", "shall"],
    );
    segment.panic(
        9,
        "much",
        &["now", "justified", "blood"],
        &["much", "now", "justified", "blood", "shall"],
    )?;
    segment.drop_value(
        10,
        "god",
        &["enemies", "reconciled", "death"],
        &["god", "enemies", "reconciled", "death", "son"],
    )?;
    segment.guard(
        11,
        "god",
        &["not", "joy", "through"],
        &["god", "not", "joy", "through", "lord"],
    )?;
    segment.drop_value(
        12,
        "man",
        &["wherefore", "sin", "entered"],
        &["man", "wherefore", "sin", "entered", "world"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        5,
        13,
        16,
        "until guard -> move_value -> drop_value until law sin",
        &["until", "law", "sin", "world", "not"],
    );
    segment.guard(
        13,
        "until",
        &["law", "sin", "world"],
        &["until", "law", "sin", "world", "not"],
    )?;
    segment.move_value(
        14,
        "moses",
        &["nevertheless", "death", "reigned"],
        &["moses", "nevertheless", "death", "reigned", "adam"],
    )?;
    segment.drop_value(
        15,
        "not",
        &["offence", "free", "gift"],
        &["not", "offence", "free", "gift", "through"],
    )?;
    segment.guard(
        16,
        "not",
        &["sinned", "gift", "judgment"],
        &["not", "sinned", "gift", "judgment", "condemnation"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        5,
        17,
        21,
        "man's drop_value -> move_value -> construct -> guard man's offence death",
        &["man's", "offence", "death", "reigned", "much"],
    );
    segment.drop_value(
        17,
        "man's",
        &["offence", "death", "reigned"],
        &["man's", "offence", "death", "reigned", "much"],
    )?;
    segment.move_value(
        18,
        "men",
        &["therefore", "offence", "judgment"],
        &["men", "therefore", "offence", "judgment", "condemnation"],
    )?;
    segment.construct(
        19,
        "man's",
        &["disobedience", "many", "made"],
        &["man's", "disobedience", "many", "made", "sinners"],
    )?;
    segment.guard(
        20,
        "moreover",
        &["law", "entered", "offence"],
        &["moreover", "law", "entered", "offence", "might"],
    )?;
    segment.drop_value(
        21,
        "sin",
        &["hath", "reigned", "unto"],
        &["sin", "hath", "reigned", "unto", "death"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        6,
        1,
        4,
        "shall call -> drop_value shall say continue",
        &["shall", "say", "continue", "sin", "grace"],
    );
    segment.call(
        1,
        "shall",
        &["say", "continue", "sin"],
        &["shall", "say", "continue", "sin", "grace"],
    )?;
    segment.drop_value(
        2,
        "god",
        &["forbid", "shall", "dead"],
        &["god", "forbid", "shall", "dead", "sin"],
    )?;
    segment.drop_value(
        3,
        "jesus",
        &["know", "not", "many"],
        &["jesus", "know", "not", "many", "baptized"],
    )?;
    segment.drop_value(
        4,
        "christ",
        &["therefore", "buried", "baptism"],
        &["christ", "therefore", "buried", "baptism", "death"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        6,
        5,
        8,
        "planted drop_value planted together likeness",
        &["planted", "together", "likeness", "death", "shall"],
    );
    segment.drop_value(
        5,
        "planted",
        &["together", "likeness", "death"],
        &["planted", "together", "likeness", "death", "shall"],
    )?;
    segment.drop_value(
        6,
        "man",
        &["knowing", "old", "crucified"],
        &["man", "knowing", "old", "crucified", "body"],
    )?;
    segment.drop_value(7, "dead", &["freed", "sin"], &["dead", "freed", "sin"])?;
    segment.drop_value(
        8,
        "christ",
        &["now", "dead", "believe"],
        &["christ", "now", "dead", "believe", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        6,
        9,
        12,
        "christ drop_value -> guard christ knowing raised",
        &["christ", "knowing", "raised", "dead", "dieth"],
    );
    segment.drop_value(
        9,
        "christ",
        &["knowing", "raised", "dead"],
        &["christ", "knowing", "raised", "dead", "dieth"],
    )?;
    segment.drop_value(
        10,
        "god",
        &["died", "unto", "sin"],
        &["god", "died", "unto", "sin", "once"],
    )?;
    segment.drop_value(
        11,
        "likewise",
        &["reckon", "yourselves", "dead"],
        &["likewise", "reckon", "yourselves", "dead", "indeed"],
    )?;
    segment.guard(
        12,
        "not",
        &["sin", "therefore", "reign"],
        &["not", "sin", "therefore", "reign", "mortal"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        6,
        13,
        16,
        "neither drop_value -> guard neither yield members",
        &[
            "neither",
            "yield",
            "members",
            "instruments",
            "unrighteousness",
        ],
    );
    segment.drop_value(
        13,
        "neither",
        &["yield", "members", "instruments"],
        &[
            "neither",
            "yield",
            "members",
            "instruments",
            "unrighteousness",
        ],
    )?;
    segment.guard(
        14,
        "sin",
        &["shall", "not", "dominion"],
        &["sin", "shall", "not", "dominion", "over"],
    )?;
    segment.guard(
        15,
        "god",
        &["shall", "sin", "not"],
        &["god", "shall", "sin", "not", "under"],
    )?;
    segment.drop_value(
        16,
        "know",
        &["not", "yield", "yourselves"],
        &["know", "not", "yield", "yourselves", "servants"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_6_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        6,
        17,
        20,
        "god expression -> construct -> call god thanked servants",
        &["god", "thanked", "servants", "sin", "obeyed"],
    );
    segment.expression(
        17,
        "god",
        &["thanked", "servants", "sin"],
        &["god", "thanked", "servants", "sin", "obeyed"],
    )?;
    segment.construct(
        18,
        "made",
        &["free", "sin", "became"],
        &["made", "free", "sin", "became", "servants"],
    )?;
    segment.call(
        19,
        "men",
        &["speak", "manner", "infirmity"],
        &["men", "speak", "manner", "infirmity", "flesh"],
    )?;
    segment.expression(
        20,
        "servants",
        &["sin", "free", "righteousness"],
        &["servants", "sin", "free", "righteousness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_6_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        6,
        21,
        23,
        "fruit drop_value -> construct fruit things whereof",
        &["fruit", "things", "whereof", "now", "ashamed"],
    );
    segment.drop_value(
        21,
        "fruit",
        &["things", "whereof", "now"],
        &["fruit", "things", "whereof", "now", "ashamed"],
    )?;
    segment.construct(
        22,
        "god",
        &["now", "made", "free"],
        &["god", "now", "made", "free", "sin"],
    )?;
    segment.drop_value(
        23,
        "god",
        &["wages", "sin", "death"],
        &["god", "wages", "sin", "death", "gift"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        7,
        1,
        4,
        "know guard -> drop_value -> move_value know not brethren",
        &["know", "not", "brethren", "speak", "law"],
    );
    segment.guard(
        1,
        "know",
        &["not", "brethren", "speak"],
        &["know", "not", "brethren", "speak", "law"],
    )?;
    segment.drop_value(
        2,
        "woman",
        &["hath", "husband", "bound"],
        &["woman", "hath", "husband", "bound", "law"],
    )?;
    segment.drop_value(
        3,
        "man",
        &["while", "husband", "liveth"],
        &["man", "while", "husband", "liveth", "married"],
    )?;
    segment.move_value(
        4,
        "christ",
        &["wherefore", "brethren", "become"],
        &["christ", "wherefore", "brethren", "become", "dead"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        7,
        5,
        8,
        "flesh move_value -> drop_value -> guard flesh motions sins",
        &["flesh", "motions", "sins", "law", "work"],
    );
    segment.move_value(
        5,
        "flesh",
        &["motions", "sins", "law"],
        &["flesh", "motions", "sins", "law", "work"],
    )?;
    segment.drop_value(
        6,
        "now",
        &["delivered", "law", "dead"],
        &["now", "delivered", "law", "dead", "wherein"],
    )?;
    segment.guard(
        7,
        "god",
        &["shall", "say", "law"],
        &["god", "shall", "say", "law", "sin"],
    )?;
    segment.drop_value(
        8,
        "sin",
        &["taking", "occasion", "commandment"],
        &["sin", "taking", "occasion", "commandment", "wrought"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        7,
        9,
        12,
        "alive move_value -> drop_value -> guard alive without law",
        &["alive", "without", "law", "once", "commandment"],
    );
    segment.move_value(
        9,
        "alive",
        &["without", "law", "once"],
        &["alive", "without", "law", "once", "commandment"],
    )?;
    segment.drop_value(
        10,
        "commandment",
        &["ordained", "life", "found"],
        &["commandment", "ordained", "life", "found", "unto"],
    )?;
    segment.guard(
        11,
        "sin",
        &["taking", "occasion", "commandment"],
        &["sin", "taking", "occasion", "commandment", "deceived"],
    )?;
    segment.guard(
        12,
        "wherefore",
        &["law", "holy", "commandment"],
        &["wherefore", "law", "holy", "commandment", "just"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_7_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        7,
        13,
        16,
        "god construct -> guard god good made",
        &["god", "good", "made", "death", "unto"],
    );
    segment.construct(
        13,
        "god",
        &["good", "made", "death"],
        &["god", "good", "made", "death", "unto"],
    )?;
    segment.guard(
        14,
        "know",
        &["law", "spiritual", "carnal"],
        &["know", "law", "spiritual", "carnal", "sold"],
    )?;
    segment.guard(
        15,
        "allow",
        &["not", "would", "hate"],
        &["allow", "not", "would", "hate"],
    )?;
    segment.guard(
        16,
        "would",
        &["not", "consent", "unto"],
        &["would", "not", "consent", "unto", "law"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_7_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        7,
        17,
        20,
        "now guard now sin dwelleth",
        &["now", "sin", "dwelleth", "know", "flesh"],
    );
    segment.guard(17, "now", &["sin", "dwelleth"], &["now", "sin", "dwelleth"])?;
    segment.guard(
        18,
        "know",
        &["flesh", "dwelleth", "good"],
        &["know", "flesh", "dwelleth", "good", "thing"],
    )?;
    segment.guard(
        19,
        "good",
        &["would", "not", "evil"],
        &["good", "would", "not", "evil"],
    )?;
    segment.guard(
        20,
        "now",
        &["would", "not", "sin"],
        &["now", "would", "not", "sin", "dwelleth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_7_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        7,
        21,
        25,
        "find guard -> drop_value find law would",
        &["find", "law", "would", "good", "evil"],
    );
    segment.guard(
        21,
        "find",
        &["law", "would", "good"],
        &["find", "law", "would", "good", "evil"],
    )?;
    segment.guard(
        22,
        "god",
        &["delight", "law", "inward"],
        &["god", "delight", "law", "inward", "man"],
    )?;
    segment.guard(
        23,
        "see",
        &["another", "law", "members"],
        &["see", "another", "law", "members", "warring"],
    )?;
    segment.drop_value(
        24,
        "man",
        &["wretched", "shall", "deliver"],
        &["man", "wretched", "shall", "deliver", "body"],
    )?;
    segment.guard(
        25,
        "god",
        &["thank", "through", "jesus"],
        &["god", "thank", "through", "jesus", "christ"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        1,
        4,
        "christ guard -> construct christ therefore now",
        &["christ", "therefore", "now", "condemnation", "jesus"],
    );
    segment.guard(
        1,
        "christ",
        &["therefore", "now", "condemnation"],
        &["christ", "therefore", "now", "condemnation", "jesus"],
    )?;
    segment.construct(
        2,
        "spirit",
        &["law", "life", "christ"],
        &["spirit", "law", "life", "christ", "jesus"],
    )?;
    segment.guard(
        3,
        "god",
        &["law", "not", "weak"],
        &["god", "law", "not", "weak", "through"],
    )?;
    segment.guard(
        4,
        "spirit",
        &["righteousness", "law", "might"],
        &["spirit", "righteousness", "law", "might", "fulfilled"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        5,
        8,
        "spirit expression -> drop_value -> guard spirit flesh mind",
        &["spirit", "flesh", "mind", "things", "carnally"],
    );
    segment.expression(
        5,
        "spirit",
        &["flesh", "mind", "things"],
        &["spirit", "flesh", "mind", "things"],
    )?;
    segment.drop_value(
        6,
        "carnally",
        &["minded", "death", "spiritually"],
        &["carnally", "minded", "death", "spiritually", "life"],
    )?;
    segment.guard(
        7,
        "god",
        &["carnal", "mind", "enmity"],
        &["god", "carnal", "mind", "enmity", "against"],
    )?;
    segment.expression(
        8,
        "god",
        &["flesh", "cannot", "please"],
        &["god", "flesh", "cannot", "please"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        9,
        12,
        "spirit guard -> drop_value spirit not flesh",
        &["spirit", "not", "flesh", "god", "dwell"],
    );
    segment.guard(
        9,
        "spirit",
        &["not", "flesh", "god"],
        &["spirit", "not", "flesh", "god", "dwell"],
    )?;
    segment.drop_value(
        10,
        "christ",
        &["body", "dead", "sin"],
        &["christ", "body", "dead", "sin", "spirit"],
    )?;
    segment.drop_value(
        11,
        "spirit",
        &["raised", "jesus", "dead"],
        &["spirit", "raised", "jesus", "dead", "dwell"],
    )?;
    segment.guard(
        12,
        "therefore",
        &["brethren", "debtors", "not"],
        &["therefore", "brethren", "debtors", "not", "flesh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        13,
        16,
        "spirit expression -> iterate -> guard spirit live flesh",
        &["spirit", "live", "flesh", "shall", "die"],
    );
    segment.expression(
        13,
        "spirit",
        &["live", "flesh", "shall"],
        &["spirit", "live", "flesh", "shall", "die"],
    )?;
    segment.iterate(
        14,
        "spirit",
        &["many", "led", "god"],
        &["spirit", "many", "led", "god", "sons"],
    )?;
    segment.guard(
        15,
        "spirit",
        &["not", "received", "bondage"],
        &["spirit", "not", "received", "bondage", "again"],
    )?;
    segment.expression(
        16,
        "spirit",
        &["itself", "beareth", "witness"],
        &["spirit", "itself", "beareth", "witness", "children"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        17,
        20,
        "children expression -> guard -> iterate -> construct children heirs god",
        &["children", "heirs", "god", "joint", "christ"],
    );
    segment.expression(
        17,
        "children",
        &["heirs", "god", "joint"],
        &["children", "heirs", "god", "joint", "christ"],
    )?;
    segment.guard(
        18,
        "reckon",
        &["sufferings", "present", "time"],
        &["reckon", "sufferings", "present", "time", "not"],
    )?;
    segment.iterate(
        19,
        "god",
        &["earnest", "expectation", "creature"],
        &["god", "earnest", "expectation", "creature", "waiteth"],
    )?;
    segment.construct(
        20,
        "creature",
        &["made", "subject", "vanity"],
        &["creature", "made", "subject", "vanity", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        21,
        24,
        "creature expression -> guard creature itself shall",
        &["creature", "itself", "shall", "delivered", "bondage"],
    );
    segment.expression(
        21,
        "creature",
        &["itself", "shall", "delivered"],
        &["creature", "itself", "shall", "delivered", "bondage"],
    )?;
    segment.expression(
        22,
        "know",
        &["whole", "creation", "groaneth"],
        &["know", "whole", "creation", "groaneth", "travaileth"],
    )?;
    segment.guard(
        23,
        "spirit",
        &["not", "ourselves", "firstfruits"],
        &["spirit", "not", "ourselves", "firstfruits", "even"],
    )?;
    segment.guard(
        24,
        "man",
        &["saved", "hope", "seen"],
        &["man", "saved", "hope", "seen", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        25,
        28,
        "hope guard -> expression -> borrow hope see not",
        &["hope", "see", "not", "patience", "wait"],
    );
    segment.guard(
        25,
        "hope",
        &["see", "not", "patience"],
        &["hope", "see", "not", "patience", "wait"],
    )?;
    segment.guard(
        26,
        "spirit",
        &["likewise", "helpeth", "infirmities"],
        &["spirit", "likewise", "helpeth", "infirmities", "know"],
    )?;
    segment.expression(
        27,
        "spirit",
        &["searcheth", "hearts", "knoweth"],
        &["spirit", "searcheth", "hearts", "knoweth", "mind"],
    )?;
    segment.borrow(
        28,
        "god",
        &["know", "things", "work"],
        &["god", "know", "things", "work", "together"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        29,
        32,
        "foreknow expression -> call -> guard foreknow predestinate conformed",
        &["foreknow", "predestinate", "conformed", "image", "son"],
    );
    segment.expression(
        29,
        "foreknow",
        &["predestinate", "conformed", "image"],
        &["foreknow", "predestinate", "conformed", "image", "son"],
    )?;
    segment.expression(
        30,
        "moreover",
        &["predestinate", "called", "justified"],
        &[
            "moreover",
            "predestinate",
            "called",
            "justified",
            "glorified",
        ],
    )?;
    segment.call(
        31,
        "god",
        &["shall", "say", "things"],
        &["god", "shall", "say", "things", "against"],
    )?;
    segment.guard(
        32,
        "spared",
        &["not", "own", "son"],
        &["spared", "not", "own", "son", "delivered"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        33,
        36,
        "god expression -> drop_value -> borrow god shall lay",
        &["god", "shall", "lay", "any", "thing"],
    );
    segment.expression(
        33,
        "god",
        &["shall", "lay", "any"],
        &["god", "shall", "lay", "any", "thing"],
    )?;
    segment.drop_value(
        34,
        "christ",
        &["condemneth", "died", "yea"],
        &["christ", "condemneth", "died", "yea", "rather"],
    )?;
    segment.borrow(
        35,
        "christ",
        &["shall", "separate", "love"],
        &["christ", "shall", "separate", "love", "tribulation"],
    )?;
    segment.drop_value(
        36,
        "written",
        &["sake", "killed", "day"],
        &["written", "sake", "killed", "day", "long"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_8_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        8,
        37,
        39,
        "nay expression -> move_value -> guard nay things conquerors",
        &["nay", "things", "conquerors", "through", "loved"],
    );
    segment.expression(
        37,
        "nay",
        &["things", "conquerors", "through"],
        &["nay", "things", "conquerors", "through", "loved"],
    )?;
    segment.move_value(
        38,
        "persuaded",
        &["neither", "death", "nor"],
        &["persuaded", "neither", "death", "nor", "life"],
    )?;
    segment.guard(
        39,
        "nor",
        &["height", "depth", "any"],
        &["nor", "height", "depth", "any", "other"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        1,
        4,
        "christ guard -> expression christ say truth",
        &["christ", "say", "truth", "lie", "not"],
    );
    segment.guard(
        1,
        "christ",
        &["say", "truth", "lie"],
        &["christ", "say", "truth", "lie", "not"],
    )?;
    segment.expression(
        2,
        "great",
        &["heaviness", "continual", "sorrow"],
        &["great", "heaviness", "continual", "sorrow", "heart"],
    )?;
    segment.expression(
        3,
        "christ",
        &["wish", "myself", "accursed"],
        &["christ", "wish", "myself", "accursed", "brethren"],
    )?;
    segment.guard(
        4,
        "israelites",
        &["pertaineth", "adoption", "glory"],
        &["israelites", "pertaineth", "adoption", "glory", "covenants"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        5,
        8,
        "christ move_value -> guard christ whose fathers",
        &["christ", "whose", "fathers", "concerning", "flesh"],
    );
    segment.move_value(
        5,
        "christ",
        &["whose", "fathers", "concerning"],
        &["christ", "whose", "fathers", "concerning", "flesh"],
    )?;
    segment.guard(
        6,
        "god",
        &["not", "though", "word"],
        &["god", "not", "though", "word", "hath"],
    )?;
    segment.guard(
        7,
        "abraham",
        &["neither", "seed", "children"],
        &["abraham", "neither", "seed", "children", "isaac"],
    )?;
    segment.guard(
        8,
        "children",
        &["flesh", "not", "god"],
        &["children", "flesh", "not", "god", "promise"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        9,
        12,
        "word move_value -> guard -> construct -> call word promise time",
        &["word", "promise", "time", "will", "sarah"],
    );
    segment.move_value(
        9,
        "word",
        &["promise", "time", "will"],
        &["word", "promise", "time", "will", "sarah"],
    )?;
    segment.guard(
        10,
        "isaac",
        &["not", "rebecca", "conceived"],
        &["isaac", "not", "rebecca", "conceived", "even"],
    )?;
    segment.construct(
        11,
        "children",
        &["not", "yet", "born"],
        &["children", "not", "yet", "born", "neither"],
    )?;
    segment.call(
        12,
        "unto",
        &["elder", "shall", "serve"],
        &["unto", "elder", "shall", "serve", "younger"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        13,
        16,
        "jacob expression -> guard -> borrow jacob written loved",
        &["jacob", "written", "loved", "esau", "hated"],
    );
    segment.expression(
        13,
        "jacob",
        &["written", "loved", "esau"],
        &["jacob", "written", "loved", "esau", "hated"],
    )?;
    segment.guard(
        14,
        "god",
        &["shall", "say", "unrighteousness"],
        &["god", "shall", "say", "unrighteousness", "forbid"],
    )?;
    segment.borrow(
        15,
        "moses",
        &["saith", "will", "mercy"],
        &["moses", "saith", "will", "mercy", "compassion"],
    )?;
    segment.guard(
        16,
        "god",
        &["not", "willeth", "nor"],
        &["god", "not", "willeth", "nor", "runneth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        17,
        20,
        "pharaoh reinitialize -> borrow -> call -> construct pharaoh scripture saith",
        &["pharaoh", "scripture", "saith", "unto", "even"],
    );
    segment.reinitialize(
        17,
        "pharaoh",
        &["scripture", "saith", "unto"],
        &["pharaoh", "scripture", "saith", "unto", "even"],
    )?;
    segment.borrow(
        18,
        "therefore",
        &["hath", "mercy", "will"],
        &["therefore", "hath", "mercy", "will", "hardeneth"],
    )?;
    segment.call(
        19,
        "wilt",
        &["say", "unto", "why"],
        &["wilt", "say", "unto", "why", "doth"],
    )?;
    segment.construct(
        20,
        "man",
        &["nay", "art", "repliest"],
        &["man", "nay", "art", "repliest", "against"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        21,
        24,
        "hath construct -> guard hath not potter",
        &["hath", "not", "potter", "power", "over"],
    );
    segment.construct(
        21,
        "hath",
        &["not", "potter", "power"],
        &["hath", "not", "potter", "power", "over"],
    )?;
    segment.construct(
        22,
        "god",
        &["willing", "shew", "wrath"],
        &["god", "willing", "shew", "wrath", "make"],
    )?;
    segment.construct(
        23,
        "might",
        &["make", "known", "riches"],
        &["might", "make", "known", "riches", "glory"],
    )?;
    segment.guard(
        24,
        "even",
        &["hath", "called", "not"],
        &["even", "hath", "called", "not", "jews"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        25,
        28,
        "people guard -> move_value -> expression -> construct people saith osee",
        &["people", "saith", "osee", "will", "call"],
    );
    segment.guard(
        25,
        "people",
        &["saith", "osee", "will"],
        &["people", "saith", "osee", "will", "call"],
    )?;
    segment.move_value(
        26,
        "people",
        &["shall", "pass", "place"],
        &["people", "shall", "pass", "place", "unto"],
    )?;
    segment.expression(
        27,
        "israel",
        &["esaias", "crieth", "concerning"],
        &["israel", "esaias", "crieth", "concerning", "though"],
    )?;
    segment.construct(
        28,
        "lord",
        &["will", "finish", "work"],
        &["lord", "will", "finish", "work", "cut"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_9_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        9,
        29,
        33,
        "lord construct -> guard lord esaias except",
        &["lord", "esaias", "except", "sabaoth", "left"],
    );
    segment.construct(
        29,
        "lord",
        &["esaias", "except", "sabaoth"],
        &["lord", "esaias", "except", "sabaoth", "left"],
    )?;
    segment.guard(
        30,
        "shall",
        &["say", "gentiles", "followed"],
        &["shall", "say", "gentiles", "followed", "not"],
    )?;
    segment.guard(
        31,
        "israel",
        &["followed", "law", "righteousness"],
        &["israel", "followed", "law", "righteousness", "hath"],
    )?;
    segment.guard(
        32,
        "wherefore",
        &["sought", "not", "faith"],
        &["wherefore", "sought", "not", "faith", "works"],
    )?;
    segment.guard(
        33,
        "written",
        &["behold", "lay", "sion"],
        &["written", "behold", "lay", "sion", "stumblingstone"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_10_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        10,
        1,
        4,
        "god result -> guard god brethren heart's",
        &["god", "brethren", "heart's", "desire", "prayer"],
    );
    segment.result(
        1,
        "god",
        &["brethren", "heart's", "desire"],
        &["god", "brethren", "heart's", "desire", "prayer"],
    )?;
    segment.guard(
        2,
        "god",
        &["bear", "record", "zeal"],
        &["god", "bear", "record", "zeal", "not"],
    )?;
    segment.guard(
        3,
        "ignorant",
        &["god's", "righteousness", "going"],
        &["ignorant", "god's", "righteousness", "going", "establish"],
    )?;
    segment.guard(
        4,
        "christ",
        &["end", "law", "righteousness"],
        &["christ", "end", "law", "righteousness", "every"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_10_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        10,
        5,
        8,
        "moses guard -> move_value -> call moses describeth righteousness",
        &["moses", "describeth", "righteousness", "law", "man"],
    );
    segment.guard(
        5,
        "moses",
        &["describeth", "righteousness", "law"],
        &["moses", "describeth", "righteousness", "law", "man"],
    )?;
    segment.move_value(
        6,
        "righteousness",
        &["faith", "speaketh", "wise"],
        &["righteousness", "faith", "speaketh", "wise", "say"],
    )?;
    segment.move_value(
        7,
        "christ",
        &["shall", "descend", "deep"],
        &["christ", "shall", "descend", "deep", "bring"],
    )?;
    segment.call(
        8,
        "saith",
        &["word", "nigh", "thee"],
        &["saith", "word", "nigh", "thee", "even"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_10_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        10,
        9,
        12,
        "lord drop_value -> construct -> guard lord shalt confess",
        &["lord", "shalt", "confess", "mouth", "jesus"],
    );
    segment.drop_value(
        9,
        "lord",
        &["shalt", "confess", "mouth"],
        &["lord", "shalt", "confess", "mouth", "jesus"],
    )?;
    segment.construct(
        10,
        "man",
        &["heart", "believeth", "unto"],
        &["man", "heart", "believeth", "unto", "righteousness"],
    )?;
    segment.guard(
        11,
        "scripture",
        &["saith", "whosoever", "believeth"],
        &["scripture", "saith", "whosoever", "believeth", "shall"],
    )?;
    segment.guard(
        12,
        "lord",
        &["difference", "between", "jew"],
        &["lord", "difference", "between", "jew", "greek"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_10_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        10,
        13,
        16,
        "lord expression -> guard -> move_value lord whosoever shall",
        &["lord", "whosoever", "shall", "call", "name"],
    );
    segment.expression(
        13,
        "lord",
        &["whosoever", "shall", "call"],
        &["lord", "whosoever", "shall", "call", "name"],
    )?;
    segment.guard(
        14,
        "shall",
        &["call", "not", "believed"],
        &["shall", "call", "not", "believed", "believe"],
    )?;
    segment.move_value(
        15,
        "shall",
        &["preach", "except", "sent"],
        &["shall", "preach", "except", "sent", "written"],
    )?;
    segment.guard(
        16,
        "lord",
        &["not", "obeyed", "gospel"],
        &["lord", "not", "obeyed", "gospel", "esaias"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_10_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        10,
        17,
        21,
        "god call -> move_value -> guard -> construct -> expression god faith cometh",
        &["god", "faith", "cometh", "hearing", "word"],
    );
    segment.call(
        17,
        "god",
        &["faith", "cometh", "hearing"],
        &["god", "faith", "cometh", "hearing", "word"],
    )?;
    segment.move_value(
        18,
        "say",
        &["not", "heard", "yes"],
        &["say", "not", "heard", "yes", "verily"],
    )?;
    segment.guard(
        19,
        "israel",
        &["say", "not", "know"],
        &["israel", "say", "not", "know", "first"],
    )?;
    segment.construct(
        20,
        "esaias",
        &["very", "bold", "saith"],
        &["esaias", "very", "bold", "saith", "found"],
    )?;
    segment.expression(
        21,
        "israel",
        &["saith", "day", "long"],
        &["israel", "saith", "day", "long", "stretched"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        1,
        4,
        "god guard -> drop_value god say hath",
        &["god", "say", "hath", "cast", "away"],
    );
    segment.guard(
        1,
        "god",
        &["say", "hath", "cast"],
        &["god", "say", "hath", "cast", "away"],
    )?;
    segment.guard(
        2,
        "god",
        &["hath", "not", "cast"],
        &["god", "hath", "not", "cast", "away"],
    )?;
    segment.drop_value(
        3,
        "lord",
        &["killed", "prophets", "digged"],
        &["lord", "killed", "prophets", "digged", "down"],
    )?;
    segment.guard(
        4,
        "god",
        &["saith", "answer", "unto"],
        &["god", "saith", "answer", "unto", "reserved"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        5,
        8,
        "even borrow -> guard even present time",
        &["even", "present", "time", "remnant", "according"],
    );
    segment.borrow(
        5,
        "even",
        &["present", "time", "remnant"],
        &["even", "present", "time", "remnant", "according"],
    )?;
    segment.guard(
        6,
        "grace",
        &["works", "otherwise", "work"],
        &["grace", "works", "otherwise", "work"],
    )?;
    segment.guard(
        7,
        "israel",
        &["hath", "not", "obtained"],
        &["israel", "hath", "not", "obtained", "seeketh"],
    )?;
    segment.guard(
        8,
        "god",
        &["according", "written", "hath"],
        &["god", "according", "written", "hath", "given"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        9,
        12,
        "david construct -> guard -> move_value -> expression david saith table",
        &["david", "saith", "table", "made", "snare"],
    );
    segment.construct(
        9,
        "david",
        &["saith", "table", "made"],
        &["david", "saith", "table", "made", "snare"],
    )?;
    segment.guard(
        10,
        "eyes",
        &["darkened", "not", "see"],
        &["eyes", "darkened", "not", "see", "bow"],
    )?;
    segment.move_value(
        11,
        "god",
        &["say", "stumbled", "fall"],
        &["god", "say", "stumbled", "fall", "forbid"],
    )?;
    segment.expression(
        12,
        "now",
        &["fall", "riches", "world"],
        &["now", "fall", "riches", "world", "diminishing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        13,
        16,
        "speak call -> expression -> drop_value speak gentiles inasmuch",
        &["speak", "gentiles", "inasmuch", "apostle", "magnify"],
    );
    segment.call(
        13,
        "speak",
        &["gentiles", "inasmuch", "apostle"],
        &["speak", "gentiles", "inasmuch", "apostle", "magnify"],
    )?;
    segment.expression(
        14,
        "any",
        &["means", "provoke", "emulation"],
        &["any", "means", "provoke", "emulation", "flesh"],
    )?;
    segment.drop_value(
        15,
        "casting",
        &["away", "reconciling", "world"],
        &["casting", "away", "reconciling", "world", "shall"],
    )?;
    segment.expression(
        16,
        "firstfruit",
        &["holy", "lump", "root"],
        &["firstfruit", "holy", "lump", "root", "branches"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        17,
        20,
        "branches expression -> guard -> call branches broken off",
        &["branches", "broken", "off", "wild", "olive"],
    );
    segment.expression(
        17,
        "branches",
        &["broken", "off", "wild"],
        &["branches", "broken", "off", "wild", "olive"],
    )?;
    segment.guard(
        18,
        "boast",
        &["not", "against", "branches"],
        &["boast", "not", "against", "branches", "bearest"],
    )?;
    segment.call(
        19,
        "wilt",
        &["say", "branches", "broken"],
        &["wilt", "say", "branches", "broken", "off"],
    )?;
    segment.guard(
        20,
        "well",
        &["unbelief", "broken", "off"],
        &["well", "unbelief", "broken", "off", "standest"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        21,
        24,
        "god guard -> expression god spared not",
        &["god", "spared", "not", "natural", "branches"],
    );
    segment.guard(
        21,
        "god",
        &["spared", "not", "natural"],
        &["god", "spared", "not", "natural", "branches"],
    )?;
    segment.guard(
        22,
        "god",
        &["behold", "therefore", "goodness"],
        &["god", "behold", "therefore", "goodness", "severity"],
    )?;
    segment.guard(
        23,
        "god",
        &["abide", "not", "still"],
        &["god", "abide", "not", "still", "unbelief"],
    )?;
    segment.expression(
        24,
        "wert",
        &["cut", "olive", "tree"],
        &["wert", "cut", "olive", "tree", "wild"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        25,
        28,
        "would move_value -> return_value -> borrow would not brethren",
        &["would", "not", "brethren", "ignorant", "mystery"],
    );
    segment.move_value(
        25,
        "would",
        &["not", "brethren", "ignorant"],
        &["would", "not", "brethren", "ignorant", "mystery"],
    )?;
    segment.move_value(
        26,
        "israel",
        &["shall", "saved", "written"],
        &["israel", "shall", "saved", "written", "sion"],
    )?;
    segment.return_value(
        27,
        "covenant",
        &["unto", "shall", "take"],
        &["covenant", "unto", "shall", "take", "away"],
    )?;
    segment.borrow(
        28,
        "concerning",
        &["gospel", "enemies", "sakes"],
        &["concerning", "gospel", "enemies", "sakes", "touching"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        29,
        32,
        "god expression -> guard -> borrow god gifts calling",
        &["god", "gifts", "calling", "without", "repentance"],
    );
    segment.expression(
        29,
        "god",
        &["gifts", "calling", "without"],
        &["god", "gifts", "calling", "without", "repentance"],
    )?;
    segment.guard(
        30,
        "god",
        &["times", "past", "not"],
        &["god", "times", "past", "not", "believed"],
    )?;
    segment.guard(
        31,
        "even",
        &["now", "not", "believed"],
        &["even", "now", "not", "believed", "through"],
    )?;
    segment.borrow(
        32,
        "god",
        &["hath", "concluded", "unbelief"],
        &["god", "hath", "concluded", "unbelief", "might"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_11_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        11,
        33,
        36,
        "god expression -> trait_bound god depth riches",
        &["god", "depth", "riches", "both", "wisdom"],
    );
    segment.expression(
        33,
        "god",
        &["depth", "riches", "both"],
        &["god", "depth", "riches", "both", "wisdom"],
    )?;
    segment.expression(
        34,
        "lord",
        &["hath", "known", "mind"],
        &["lord", "hath", "known", "mind", "counsellor"],
    )?;
    segment.expression(
        35,
        "hath",
        &["first", "given", "shall"],
        &["hath", "first", "given", "shall", "recompensed"],
    )?;
    segment.trait_bound(
        36,
        "through",
        &["things", "glory", "ever"],
        &["through", "things", "glory", "ever", "amen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_12_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        12,
        1,
        4,
        "god trait_bound -> guard god beseech therefore",
        &["god", "beseech", "therefore", "brethren", "mercies"],
    );
    segment.trait_bound(
        1,
        "god",
        &["beseech", "therefore", "brethren"],
        &["god", "beseech", "therefore", "brethren", "mercies"],
    )?;
    segment.guard(
        2,
        "not",
        &["conformed", "world", "transformed"],
        &["not", "conformed", "world", "transformed", "renewing"],
    )?;
    segment.guard(
        3,
        "man",
        &["say", "through", "grace"],
        &["man", "say", "through", "grace", "given"],
    )?;
    segment.guard(
        4,
        "many",
        &["members", "body", "not"],
        &["many", "members", "body", "not", "same"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_12_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        12,
        5,
        8,
        "christ iterate -> match_value -> await_value -> borrow christ many body",
        &["christ", "many", "body", "every", "members"],
    );
    segment.iterate(
        5,
        "christ",
        &["many", "body", "every"],
        &["christ", "many", "body", "every", "members"],
    )?;
    segment.match_value(
        6,
        "having",
        &["gifts", "differing", "according"],
        &["having", "gifts", "differing", "according", "grace"],
    )?;
    segment.await_value(
        7,
        "ministry",
        &["wait", "ministering", "teacheth"],
        &["ministry", "wait", "ministering", "teacheth", "teaching"],
    )?;
    segment.borrow(
        8,
        "exhorteth",
        &["exhortation", "giveth", "simplicity"],
        &["exhorteth", "exhortation", "giveth", "simplicity", "ruleth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_12_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        12,
        9,
        12,
        "love borrow -> guard -> result love without dissimulation",
        &["love", "without", "dissimulation", "abhor", "evil"],
    );
    segment.borrow(
        9,
        "love",
        &["without", "dissimulation", "abhor"],
        &["love", "without", "dissimulation", "abhor", "evil"],
    )?;
    segment.borrow(
        10,
        "kindly",
        &["affectioned", "another", "brotherly"],
        &["kindly", "affectioned", "another", "brotherly", "love"],
    )?;
    segment.guard(
        11,
        "spirit",
        &["not", "slothful", "business"],
        &["spirit", "not", "slothful", "business", "fervent"],
    )?;
    segment.result(
        12,
        "rejoicing",
        &["hope", "patient", "tribulation"],
        &["rejoicing", "hope", "patient", "tribulation", "continuing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_12_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        12,
        13,
        16,
        "distributing expression -> guard -> result distributing necessity saints",
        &[
            "distributing",
            "necessity",
            "saints",
            "given",
            "hospitality",
        ],
    );
    segment.expression(
        13,
        "distributing",
        &["necessity", "saints", "given"],
        &[
            "distributing",
            "necessity",
            "saints",
            "given",
            "hospitality",
        ],
    )?;
    segment.guard(
        14,
        "bless",
        &["persecute", "curse", "not"],
        &["bless", "persecute", "curse", "not"],
    )?;
    segment.result(15, "rejoice", &["weep"], &["rejoice", "weep"])?;
    segment.guard(
        16,
        "same",
        &["mind", "toward", "another"],
        &["same", "mind", "toward", "another", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_12_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        12,
        17,
        21,
        "man guard -> expression man recompense evil",
        &["man", "recompense", "evil", "provide", "things"],
    );
    segment.guard(
        17,
        "man",
        &["recompense", "evil", "provide"],
        &["man", "recompense", "evil", "provide", "things"],
    )?;
    segment.expression(
        18,
        "men",
        &["possible", "much", "lieth"],
        &["men", "possible", "much", "lieth", "live"],
    )?;
    segment.guard(
        19,
        "dearly",
        &["beloved", "avenge", "not"],
        &["dearly", "beloved", "avenge", "not", "yourselves"],
    )?;
    segment.guard(
        20,
        "therefore",
        &["thine", "enemy", "hunger"],
        &["therefore", "thine", "enemy", "hunger", "feed"],
    )?;
    segment.guard(
        21,
        "not",
        &["overcome", "evil", "good"],
        &["not", "overcome", "evil", "good"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_13_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        13,
        1,
        4,
        "god guard god every soul",
        &["god", "every", "soul", "subject", "unto"],
    );
    segment.guard(
        1,
        "god",
        &["every", "soul", "subject"],
        &["god", "every", "soul", "subject", "unto"],
    )?;
    segment.guard(
        2,
        "god",
        &["whosoever", "therefore", "resisteth"],
        &["god", "whosoever", "therefore", "resisteth", "power"],
    )?;
    segment.guard(
        3,
        "rulers",
        &["not", "terror", "good"],
        &["rulers", "not", "terror", "good", "works"],
    )?;
    segment.guard(
        4,
        "god",
        &["minister", "thee", "good"],
        &["god", "minister", "thee", "good", "evil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_13_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        13,
        5,
        8,
        "wherefore guard -> expression -> panic wherefore must needs",
        &["wherefore", "must", "needs", "subject", "not"],
    );
    segment.guard(
        5,
        "wherefore",
        &["must", "needs", "subject"],
        &["wherefore", "must", "needs", "subject", "not"],
    )?;
    segment.expression(
        6,
        "cause",
        &["pay", "tribute", "god's"],
        &["cause", "pay", "tribute", "god's", "ministers"],
    )?;
    segment.panic(
        7,
        "render",
        &["therefore", "dues", "tribute"],
        &["render", "therefore", "dues", "tribute", "due"],
    )?;
    segment.guard(
        8,
        "man",
        &["owe", "any", "thing"],
        &["man", "owe", "any", "thing", "love"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_13_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        13,
        9,
        11,
        "shalt drop_value -> guard -> await_value shalt not commit",
        &["shalt", "not", "commit", "adultery", "kill"],
    );
    segment.drop_value(
        9,
        "shalt",
        &["not", "commit", "adultery"],
        &["shalt", "not", "commit", "adultery", "kill"],
    )?;
    segment.guard(
        10,
        "love",
        &["worketh", "ill", "neighbour"],
        &["love", "worketh", "ill", "neighbour", "therefore"],
    )?;
    segment.await_value(
        11,
        "knowing",
        &["time", "now", "high"],
        &["knowing", "time", "now", "high", "awake"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_13_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        13,
        12,
        14,
        "night expression -> guard -> construct night far spent",
        &["night", "far", "spent", "day", "hand"],
    );
    segment.expression(
        12,
        "night",
        &["far", "spent", "day"],
        &["night", "far", "spent", "day", "hand"],
    )?;
    segment.guard(
        13,
        "walk",
        &["honestly", "day", "not"],
        &["walk", "honestly", "day", "not", "rioting"],
    )?;
    segment.construct(
        14,
        "lord",
        &["put", "jesus", "christ"],
        &["lord", "put", "jesus", "christ", "make"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_14_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        14,
        1,
        4,
        "weak guard -> expression -> construct weak faith receive",
        &["weak", "faith", "receive", "not", "doubtful"],
    );
    segment.guard(
        1,
        "weak",
        &["faith", "receive", "not"],
        &["weak", "faith", "receive", "not", "doubtful"],
    )?;
    segment.expression(
        2,
        "believeth",
        &["eat", "things", "another"],
        &["believeth", "eat", "things", "another", "weak"],
    )?;
    segment.guard(
        3,
        "god",
        &["not", "eateth", "despise"],
        &["god", "not", "eateth", "despise", "judge"],
    )?;
    segment.construct(
        4,
        "art",
        &["judgest", "another", "man's"],
        &["art", "judgest", "another", "man's", "servant"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_14_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        14,
        5,
        8,
        "man iterate -> guard -> match_value man esteemeth day",
        &["man", "esteemeth", "day", "above", "another"],
    );
    segment.iterate(
        5,
        "man",
        &["esteemeth", "day", "above"],
        &["man", "esteemeth", "day", "above", "another"],
    )?;
    segment.guard(
        6,
        "lord",
        &["regardeth", "day", "unto"],
        &["lord", "regardeth", "day", "unto", "not"],
    )?;
    segment.guard(
        7,
        "man",
        &["none", "liveth", "himself"],
        &["man", "none", "liveth", "himself", "dieth"],
    )?;
    segment.match_value(
        8,
        "lord",
        &["whether", "live", "unto"],
        &["lord", "whether", "live", "unto", "die"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_14_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        14,
        9,
        12,
        "christ drop_value -> expression -> iterate christ end both",
        &["christ", "end", "both", "died", "rose"],
    );
    segment.drop_value(
        9,
        "christ",
        &["end", "both", "died"],
        &["christ", "end", "both", "died", "rose"],
    )?;
    segment.expression(
        10,
        "why",
        &["dost", "judge", "brother"],
        &["why", "dost", "judge", "brother", "set"],
    )?;
    segment.iterate(
        11,
        "lord",
        &["written", "live", "saith"],
        &["lord", "written", "live", "saith", "every"],
    )?;
    segment.iterate(
        12,
        "god",
        &["every", "shall", "give"],
        &["god", "every", "shall", "give", "account"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_14_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        14,
        13,
        16,
        "man guard -> expression -> drop_value man not therefore",
        &["man", "not", "therefore", "judge", "another"],
    );
    segment.guard(
        13,
        "man",
        &["not", "therefore", "judge"],
        &["man", "not", "therefore", "judge", "another"],
    )?;
    segment.expression(
        14,
        "lord",
        &["know", "persuaded", "jesus"],
        &["lord", "know", "persuaded", "jesus", "nothing"],
    )?;
    segment.drop_value(
        15,
        "brother",
        &["grieved", "meat", "now"],
        &["brother", "grieved", "meat", "now", "walkest"],
    )?;
    segment.guard(
        16,
        "not",
        &["good", "evil", "spoken"],
        &["not", "good", "evil", "spoken"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_14_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        14,
        17,
        20,
        "god guard -> expression -> construct -> drop_value god kingdom not",
        &["god", "kingdom", "not", "meat", "drink"],
    );
    segment.guard(
        17,
        "god",
        &["kingdom", "not", "meat"],
        &["god", "kingdom", "not", "meat", "drink"],
    )?;
    segment.expression(
        18,
        "christ",
        &["things", "serveth", "acceptable"],
        &["christ", "things", "serveth", "acceptable", "god"],
    )?;
    segment.construct(
        19,
        "therefore",
        &["follow", "things", "make"],
        &["therefore", "follow", "things", "make", "peace"],
    )?;
    segment.drop_value(
        20,
        "god",
        &["meat", "destroy", "not"],
        &["god", "meat", "destroy", "not", "work"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_14_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        14,
        21,
        23,
        "good construct -> guard good neither eat",
        &["good", "neither", "eat", "flesh", "nor"],
    );
    segment.construct(
        21,
        "good",
        &["neither", "eat", "flesh"],
        &["good", "neither", "eat", "flesh", "nor"],
    )?;
    segment.guard(
        22,
        "god",
        &["hast", "faith", "thyself"],
        &["god", "hast", "faith", "thyself", "happy"],
    )?;
    segment.guard(
        23,
        "doubteth",
        &["damned", "eat", "eateth"],
        &["doubteth", "damned", "eat", "eateth", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        1,
        4,
        "strong guard -> iterate -> await_value strong ought bear",
        &["strong", "ought", "bear", "infirmities", "weak"],
    );
    segment.guard(
        1,
        "strong",
        &["ought", "bear", "infirmities"],
        &["strong", "ought", "bear", "infirmities", "weak"],
    )?;
    segment.iterate(
        2,
        "every",
        &["please", "neighbour", "good"],
        &["every", "please", "neighbour", "good", "edification"],
    )?;
    segment.guard(
        3,
        "christ",
        &["even", "pleased", "not"],
        &["christ", "even", "pleased", "not", "himself"],
    )?;
    segment.await_value(
        4,
        "whatsoever",
        &["things", "written", "aforetime"],
        &["whatsoever", "things", "written", "aforetime", "learning"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        5,
        8,
        "god expression -> trait_bound -> construct god now patience",
        &["god", "now", "patience", "consolation", "grant"],
    );
    segment.expression(
        5,
        "god",
        &["now", "patience", "consolation"],
        &["god", "now", "patience", "consolation", "grant"],
    )?;
    segment.expression(
        6,
        "god",
        &["mind", "mouth", "glorify"],
        &["god", "mind", "mouth", "glorify", "even"],
    )?;
    segment.trait_bound(
        7,
        "christ",
        &["wherefore", "receive", "another"],
        &["christ", "wherefore", "receive", "another", "received"],
    )?;
    segment.construct(
        8,
        "jesus",
        &["now", "say", "christ"],
        &["jesus", "now", "say", "christ", "minister"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        9,
        12,
        "god trait_bound -> expression -> await_value god gentiles might",
        &["god", "gentiles", "might", "glorify", "mercy"],
    );
    segment.trait_bound(
        9,
        "god",
        &["gentiles", "might", "glorify"],
        &["god", "gentiles", "might", "glorify", "mercy"],
    )?;
    segment.expression(
        10,
        "people",
        &["again", "saith", "rejoice"],
        &["people", "again", "saith", "rejoice", "gentiles"],
    )?;
    segment.trait_bound(
        11,
        "lord",
        &["again", "praise", "gentiles"],
        &["lord", "again", "praise", "gentiles", "laud"],
    )?;
    segment.await_value(
        12,
        "again",
        &["esaias", "saith", "shall"],
        &["again", "esaias", "saith", "shall", "root"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        13,
        16,
        "god await_value -> expression -> borrow -> trait_bound god now hope",
        &["god", "now", "hope", "fill", "joy"],
    );
    segment.await_value(
        13,
        "god",
        &["now", "hope", "fill"],
        &["god", "now", "hope", "fill", "joy"],
    )?;
    segment.expression(
        14,
        "myself",
        &["persuaded", "brethren", "full"],
        &["myself", "persuaded", "brethren", "full", "goodness"],
    )?;
    segment.borrow(
        15,
        "nevertheless",
        &["brethren", "written", "boldly"],
        &["nevertheless", "brethren", "written", "boldly", "unto"],
    )?;
    segment.trait_bound(
        16,
        "jesus",
        &["minister", "christ", "gentiles"],
        &["jesus", "minister", "christ", "gentiles", "ministering"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        17,
        20,
        "jesus trait_bound -> construct -> expression jesus therefore whereof",
        &["jesus", "therefore", "whereof", "glory", "through"],
    );
    segment.trait_bound(
        17,
        "jesus",
        &["therefore", "whereof", "glory"],
        &["jesus", "therefore", "whereof", "glory", "through"],
    )?;
    segment.construct(
        18,
        "christ",
        &["will", "not", "dare"],
        &["christ", "will", "not", "dare", "speak"],
    )?;
    segment.expression(
        19,
        "spirit",
        &["through", "mighty", "signs"],
        &["spirit", "through", "mighty", "signs", "wonders"],
    )?;
    segment.construct(
        20,
        "christ",
        &["yea", "strived", "preach"],
        &["christ", "yea", "strived", "preach", "gospel"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        21,
        24,
        "written guard -> expression -> move_value written not spoken",
        &["written", "not", "spoken", "shall", "see"],
    );
    segment.guard(
        21,
        "written",
        &["not", "spoken", "shall"],
        &["written", "not", "spoken", "shall", "see"],
    )?;
    segment.expression(
        22,
        "cause",
        &["much", "hindered", "coming"],
        &["cause", "much", "hindered", "coming"],
    )?;
    segment.move_value(
        23,
        "now",
        &["having", "place", "parts"],
        &["now", "having", "place", "parts", "great"],
    )?;
    segment.move_value(
        24,
        "whensoever",
        &["take", "journey", "spain"],
        &["whensoever", "take", "journey", "spain", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        25,
        28,
        "now move_value -> construct now unto jerusalem",
        &["now", "unto", "jerusalem", "minister", "saints"],
    );
    segment.move_value(
        25,
        "now",
        &["unto", "jerusalem", "minister"],
        &["now", "unto", "jerusalem", "minister", "saints"],
    )?;
    segment.construct(
        26,
        "hath",
        &["pleased", "macedonia", "achaia"],
        &["hath", "pleased", "macedonia", "achaia", "make"],
    )?;
    segment.construct(
        27,
        "hath",
        &["pleased", "verily", "debtors"],
        &["hath", "pleased", "verily", "debtors", "gentiles"],
    )?;
    segment.move_value(
        28,
        "therefore",
        &["performed", "sealed", "fruit"],
        &["therefore", "performed", "sealed", "fruit", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_15_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        15,
        29,
        33,
        "christ move_value -> borrow -> guard -> expression christ sure unto",
        &["christ", "sure", "unto", "shall", "fulness"],
    );
    segment.move_value(
        29,
        "christ",
        &["sure", "unto", "shall"],
        &["christ", "sure", "unto", "shall", "fulness"],
    )?;
    segment.borrow(
        30,
        "lord",
        &["now", "beseech", "brethren"],
        &["lord", "now", "beseech", "brethren", "jesus"],
    )?;
    segment.guard(
        31,
        "delivered",
        &["not", "believe", "judaea"],
        &["delivered", "not", "believe", "judaea", "service"],
    )?;
    segment.move_value(
        32,
        "god",
        &["unto", "joy", "will"],
        &["god", "unto", "joy", "will", "refreshed"],
    )?;
    segment.expression(
        33,
        "god",
        &["now", "peace", "amen"],
        &["god", "now", "peace", "amen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_16_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        16,
        1,
        4,
        "commend expression -> guard commend unto phebe",
        &["commend", "unto", "phebe", "sister", "servant"],
    );
    segment.expression(
        1,
        "commend",
        &["unto", "phebe", "sister"],
        &["commend", "unto", "phebe", "sister", "servant"],
    )?;
    segment.expression(
        2,
        "lord",
        &["receive", "becometh", "saints"],
        &["lord", "receive", "becometh", "saints", "assist"],
    )?;
    segment.expression(
        3,
        "christ",
        &["greet", "priscilla", "aquila"],
        &["christ", "greet", "priscilla", "aquila", "helpers"],
    )?;
    segment.guard(
        4,
        "life",
        &["laid", "down", "own"],
        &["life", "laid", "down", "own", "necks"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_16_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        16,
        5,
        8,
        "likewise borrow -> expression likewise greet church",
        &["likewise", "greet", "church", "house", "salute"],
    );
    segment.borrow(
        5,
        "likewise",
        &["greet", "church", "house"],
        &["likewise", "greet", "church", "house", "salute"],
    )?;
    segment.expression(
        6,
        "greet",
        &["mary", "bestowed", "much"],
        &["greet", "mary", "bestowed", "much", "labour"],
    )?;
    segment.expression(
        7,
        "salute",
        &["andronicus", "junia", "kinsmen"],
        &["salute", "andronicus", "junia", "kinsmen", "fellow"],
    )?;
    segment.borrow(
        8,
        "lord",
        &["greet", "amplias", "beloved"],
        &["lord", "greet", "amplias", "beloved"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_16_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        16,
        9,
        12,
        "christ borrow -> expression christ salute urbane",
        &["christ", "salute", "urbane", "helper", "stachys"],
    );
    segment.borrow(
        9,
        "christ",
        &["salute", "urbane", "helper"],
        &["christ", "salute", "urbane", "helper", "stachys"],
    )?;
    segment.expression(
        10,
        "christ",
        &["salute", "apelles", "approved"],
        &["christ", "salute", "apelles", "approved", "aristobulus"],
    )?;
    segment.expression(
        11,
        "lord",
        &["salute", "herodion", "kinsman"],
        &["lord", "salute", "herodion", "kinsman", "greet"],
    )?;
    segment.borrow(
        12,
        "lord",
        &["salute", "tryphena", "tryphosa"],
        &["lord", "salute", "tryphena", "tryphosa", "labour"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_16_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        16,
        13,
        16,
        "lord expression lord salute rufus",
        &["lord", "salute", "rufus", "chosen", "mother"],
    );
    segment.expression(
        13,
        "lord",
        &["salute", "rufus", "chosen"],
        &["lord", "salute", "rufus", "chosen", "mother"],
    )?;
    segment.expression(
        14,
        "salute",
        &["asyncritus", "phlegon", "hermas"],
        &["salute", "asyncritus", "phlegon", "hermas", "patrobas"],
    )?;
    segment.expression(
        15,
        "salute",
        &["philologus", "julia", "nereus"],
        &["salute", "philologus", "julia", "nereus", "sister"],
    )?;
    segment.expression(
        16,
        "christ",
        &["salute", "another", "holy"],
        &["christ", "salute", "another", "holy", "kiss"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_16_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        16,
        17,
        20,
        "now expression -> guard -> move_value -> borrow now beseech brethren",
        &["now", "beseech", "brethren", "mark", "cause"],
    );
    segment.expression(
        17,
        "now",
        &["beseech", "brethren", "mark"],
        &["now", "beseech", "brethren", "mark", "cause"],
    )?;
    segment.guard(
        18,
        "lord",
        &["such", "serve", "not"],
        &["lord", "such", "serve", "not", "jesus"],
    )?;
    segment.move_value(
        19,
        "men",
        &["obedience", "abroad", "unto"],
        &["men", "obedience", "abroad", "unto", "glad"],
    )?;
    segment.borrow(
        20,
        "god",
        &["peace", "shall", "bruise"],
        &["god", "peace", "shall", "bruise", "satan"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_16_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        16,
        21,
        24,
        "timotheus expression -> borrow timotheus workfellow lucius",
        &["timotheus", "workfellow", "lucius", "jason", "sosipater"],
    );
    segment.expression(
        21,
        "timotheus",
        &["workfellow", "lucius", "jason"],
        &["timotheus", "workfellow", "lucius", "jason", "sosipater"],
    )?;
    segment.expression(
        22,
        "lord",
        &["tertius", "wrote", "epistle"],
        &["lord", "tertius", "wrote", "epistle", "salute"],
    )?;
    segment.expression(
        23,
        "gaius",
        &["mine", "host", "whole"],
        &["gaius", "mine", "host", "whole", "church"],
    )?;
    segment.borrow(
        24,
        "lord",
        &["grace", "jesus", "christ"],
        &["lord", "grace", "jesus", "christ", "amen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn romans_16_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Romans,
        16,
        25,
        27,
        "jesus match_value -> construct -> trait_bound jesus now power",
        &["jesus", "now", "power", "stablish", "according"],
    );
    segment.match_value(
        25,
        "jesus",
        &["now", "power", "stablish"],
        &["jesus", "now", "power", "stablish", "according"],
    )?;
    segment.construct(
        26,
        "now",
        &["made", "manifest", "scriptures"],
        &["now", "made", "manifest", "scriptures", "prophets"],
    )?;
    segment.trait_bound(
        27,
        "god",
        &["wise", "glory", "through"],
        &["god", "wise", "glory", "through", "jesus"],
    )?;
    segment.commit()?;
    Ok(())
}
