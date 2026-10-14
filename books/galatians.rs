use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    galatians(story)?;
    Ok(())
}

fn galatians(story: &mut Story) -> Result<(), StoryError> {
    galatians_1(story)?;
    galatians_2(story)?;
    galatians_3(story)?;
    galatians_4(story)?;
    galatians_5(story)?;
    galatians_6(story)?;
    Ok(())
}

fn galatians_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Galatians", 1, "Paul insists that his commission and gospel did not come from a human approval queue, recounting his former zeal and the sudden rerouting of his life.", "Even an angelic-looking patch must be rejected if it replaces the good news already received, because there is no second gospel with better branding.");
    galatians_1_group_01(story)?;
    galatians_1_group_02(story)?;
    galatians_1_group_03(story)?;
    galatians_1_group_04(story)?;
    galatians_1_group_05(story)?;
    galatians_1_group_06(story)?;
    Ok(())
}

fn galatians_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Galatians", 2, "Paul recounts the Jerusalem agreement and confronts Peter for withdrawing from Gentile table fellowship when certain visitors arrive.", "A person is made right through Christ's faithfulness and trust, not by reinstalling the law as a badge; Paul says he now lives from Christ's life.");
    galatians_2_group_01(story)?;
    galatians_2_group_02(story)?;
    galatians_2_group_03(story)?;
    galatians_2_group_04(story)?;
    galatians_2_group_05(story)?;
    Ok(())
}

fn galatians_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Galatians", 3, "The Galatians began by the Spirit but are tempted to finish by fleshly performance, as though a successful migration required reverting to the old schema.", "Abraham's promise precedes the law, which served as a temporary guardian until faith's arrival made all who trust children and heirs.");
    galatians_3_group_01(story)?;
    galatians_3_group_02(story)?;
    galatians_3_group_03(story)?;
    galatians_3_group_04(story)?;
    galatians_3_group_05(story)?;
    galatians_3_group_06(story)?;
    galatians_3_group_07(story)?;
    Ok(())
}

fn galatians_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Galatians", 4, "Heirs under guardians resemble servants until the appointed time, but God sends the Son and the Spirit so adoption becomes an intimate, lived reality.", "Paul fears the Galatians are returning to calendar-based bondage and compares his appeal to labor pains until Christ's shape appears in them.");
    galatians_4_group_01(story)?;
    galatians_4_group_02(story)?;
    galatians_4_group_03(story)?;
    galatians_4_group_04(story)?;
    galatians_4_group_05(story)?;
    galatians_4_group_06(story)?;
    galatians_4_group_07(story)?;
    galatians_4_group_08(story)?;
    Ok(())
}

fn galatians_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Galatians", 5, "Christ's freedom is not a license for self-indulgence or a reason to make others carry a yoke; love fulfills the law by serving a neighbor.", "The flesh and Spirit pull in opposite directions, while the Spirit's fruit grows as a coherent cluster that outshines rivalry, envy, and rage.");
    galatians_5_group_01(story)?;
    galatians_5_group_02(story)?;
    galatians_5_group_03(story)?;
    galatians_5_group_04(story)?;
    galatians_5_group_05(story)?;
    galatians_5_group_06(story)?;
    galatians_5_group_07(story)?;
    Ok(())
}

fn galatians_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Galatians", 6, "The community should restore a caught offender gently, carry burdens, and still take responsibility for its own load instead of competitive comparison.", "Sowing to the flesh harvests decay, but patient goodness reaches a due season; Paul signs off with the cross as his only boast.");
    galatians_6_group_01(story)?;
    galatians_6_group_02(story)?;
    galatians_6_group_03(story)?;
    galatians_6_group_04(story)?;
    galatians_6_group_05(story)?;
    Ok(())
}

fn galatians_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        1,
        1,
        4,
        "paul drop_value -> expression -> borrow paul apostle not",
        &["paul", "apostle", "not", "men", "neither"],
    );
    segment.drop_value(
        1,
        "paul",
        &["apostle", "not", "men"],
        &["paul", "apostle", "not", "men", "neither"],
    )?;
    segment.expression(
        2,
        "brethren",
        &["unto", "churches", "galatia"],
        &["brethren", "unto", "churches", "galatia"],
    )?;
    segment.borrow(
        3,
        "god",
        &["grace", "peace", "father"],
        &["god", "grace", "peace", "father", "lord"],
    )?;
    segment.expression(
        4,
        "gave",
        &["himself", "sins", "might"],
        &["gave", "himself", "sins", "might", "deliver"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        1,
        5,
        8,
        "glory trait_bound -> borrow -> guard -> call glory ever amen",
        &["glory", "ever", "amen", "christ", "marvel"],
    );
    segment.trait_bound(5, "glory", &["ever", "amen"], &["glory", "ever", "amen"])?;
    segment.borrow(
        6,
        "christ",
        &["marvel", "soon", "removed"],
        &["christ", "marvel", "soon", "removed", "called"],
    )?;
    segment.guard(
        7,
        "christ",
        &["not", "another", "trouble"],
        &["christ", "not", "another", "trouble", "would"],
    )?;
    segment.call(
        8,
        "angel",
        &["though", "heaven", "preach"],
        &["angel", "though", "heaven", "preach", "any"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        1,
        9,
        12,
        "man call -> guard man say now",
        &["man", "say", "now", "again", "any"],
    );
    segment.call(
        9,
        "man",
        &["say", "now", "again"],
        &["man", "say", "now", "again", "any"],
    )?;
    segment.guard(
        10,
        "men",
        &["now", "persuade", "god"],
        &["men", "now", "persuade", "god", "seek"],
    )?;
    segment.guard(
        11,
        "man",
        &["certify", "brethren", "gospel"],
        &["man", "certify", "brethren", "gospel", "preached"],
    )?;
    segment.guard(
        12,
        "man",
        &["neither", "received", "taught"],
        &["man", "neither", "received", "taught", "revelation"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        1,
        13,
        16,
        "heard expression -> borrow -> guard heard conversation time",
        &["heard", "conversation", "time", "past", "jews"],
    );
    segment.expression(
        13,
        "heard",
        &["conversation", "time", "past"],
        &["heard", "conversation", "time", "past", "jews"],
    )?;
    segment.expression(
        14,
        "profited",
        &["jews", "religion", "above"],
        &["profited", "jews", "religion", "above", "many"],
    )?;
    segment.borrow(
        15,
        "god",
        &["pleased", "separated", "mother's"],
        &["god", "pleased", "separated", "mother's", "womb"],
    )?;
    segment.guard(
        16,
        "reveal",
        &["son", "might", "preach"],
        &["reveal", "son", "might", "preach", "among"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        1,
        17,
        20,
        "neither move_value -> match_value -> guard neither went jerusalem",
        &["neither", "went", "jerusalem", "apostles", "arabia"],
    );
    segment.move_value(
        17,
        "neither",
        &["went", "jerusalem", "apostles"],
        &["neither", "went", "jerusalem", "apostles", "arabia"],
    )?;
    segment.move_value(
        18,
        "peter",
        &["three", "years", "went"],
        &["peter", "three", "years", "went", "jerusalem"],
    )?;
    segment.match_value(
        19,
        "other",
        &["apostles", "saw", "none"],
        &["other", "apostles", "saw", "none", "save"],
    )?;
    segment.guard(
        20,
        "god",
        &["now", "things", "write"],
        &["god", "now", "things", "write", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        1,
        21,
        24,
        "afterwards move_value -> expression -> drop_value afterwards regions syria",
        &["afterwards", "regions", "syria", "cilicia", "christ"],
    );
    segment.move_value(
        21,
        "afterwards",
        &["regions", "syria", "cilicia"],
        &["afterwards", "regions", "syria", "cilicia"],
    )?;
    segment.expression(
        22,
        "christ",
        &["unknown", "face", "unto"],
        &["christ", "unknown", "face", "unto", "churches"],
    )?;
    segment.drop_value(
        23,
        "heard",
        &["persecuted", "times", "past"],
        &["heard", "persecuted", "times", "past", "now"],
    )?;
    segment.expression(24, "god", &["glorified"], &["god", "glorified"])?;
    segment.commit()?;
    Ok(())
}

fn galatians_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        2,
        1,
        4,
        "fourteen move_value -> guard fourteen years went",
        &["fourteen", "years", "went", "again", "jerusalem"],
    );
    segment.move_value(
        1,
        "fourteen",
        &["years", "went", "again"],
        &["fourteen", "years", "went", "again", "jerusalem"],
    )?;
    segment.move_value(
        2,
        "went",
        &["revelation", "communicated", "unto"],
        &["went", "revelation", "communicated", "unto", "gospel"],
    )?;
    segment.guard(
        3,
        "neither",
        &["titus", "greek", "compelled"],
        &["neither", "titus", "greek", "compelled", "circumcised"],
    )?;
    segment.move_value(
        4,
        "christ",
        &["false", "brethren", "unawares"],
        &["christ", "false", "brethren", "unawares", "brought"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        2,
        5,
        8,
        "gave guard -> match_value -> expression gave place subjection",
        &["gave", "place", "subjection", "not", "hour"],
    );
    segment.guard(
        5,
        "gave",
        &["place", "subjection", "not"],
        &["gave", "place", "subjection", "not", "hour"],
    )?;
    segment.guard(
        6,
        "god",
        &["seemed", "somewhat", "whatsoever"],
        &["god", "seemed", "somewhat", "whatsoever", "maketh"],
    )?;
    segment.match_value(
        7,
        "peter",
        &["contrariwise", "saw", "gospel"],
        &["peter", "contrariwise", "saw", "gospel", "uncircumcision"],
    )?;
    segment.expression(
        8,
        "peter",
        &["wrought", "effectually", "apostleship"],
        &[
            "peter",
            "wrought",
            "effectually",
            "apostleship",
            "circumcision",
        ],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        2,
        9,
        12,
        "john move_value -> expression john james cephas",
        &["john", "james", "cephas", "seemed", "pillars"],
    );
    segment.move_value(
        9,
        "john",
        &["james", "cephas", "seemed"],
        &["john", "james", "cephas", "seemed", "pillars"],
    )?;
    segment.expression(
        10,
        "would",
        &["remember", "poor", "same"],
        &["would", "remember", "poor", "same", "forward"],
    )?;
    segment.move_value(
        11,
        "peter",
        &["antioch", "withstood", "face"],
        &["peter", "antioch", "withstood", "face", "blamed"],
    )?;
    segment.move_value(
        12,
        "certain",
        &["james", "eat", "gentiles"],
        &["certain", "james", "eat", "gentiles", "withdrew"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        2,
        13,
        16,
        "other expression -> guard other jews dissembled",
        &["other", "jews", "dissembled", "likewise", "insomuch"],
    );
    segment.expression(
        13,
        "other",
        &["jews", "dissembled", "likewise"],
        &["other", "jews", "dissembled", "likewise", "insomuch"],
    )?;
    segment.guard(
        14,
        "saw",
        &["walked", "not", "uprightly"],
        &["saw", "walked", "not", "uprightly", "according"],
    )?;
    segment.guard(
        15,
        "jews",
        &["nature", "not", "sinners"],
        &["jews", "nature", "not", "sinners", "gentiles"],
    )?;
    segment.guard(
        16,
        "man",
        &["knowing", "not", "justified"],
        &["man", "knowing", "not", "justified", "works"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        2,
        17,
        21,
        "christ guard -> construct -> drop_value -> move_value christ while seek",
        &["christ", "while", "seek", "justified", "ourselves"],
    );
    segment.guard(
        17,
        "christ",
        &["while", "seek", "justified"],
        &["christ", "while", "seek", "justified", "ourselves"],
    )?;
    segment.construct(
        18,
        "build",
        &["again", "things", "destroyed"],
        &["build", "again", "things", "destroyed", "make"],
    )?;
    segment.drop_value(
        19,
        "god",
        &["through", "law", "dead"],
        &["god", "through", "law", "dead", "might"],
    )?;
    segment.guard(
        20,
        "christ",
        &["crucified", "nevertheless", "live"],
        &["christ", "crucified", "nevertheless", "live", "yet"],
    )?;
    segment.move_value(
        21,
        "god",
        &["not", "frustrate", "grace"],
        &["god", "not", "frustrate", "grace", "righteousness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        3,
        1,
        4,
        "foolish guard -> construct -> expression foolish galatians hath",
        &["foolish", "galatians", "hath", "bewitched", "not"],
    );
    segment.guard(
        1,
        "foolish",
        &["galatians", "hath", "bewitched"],
        &["foolish", "galatians", "hath", "bewitched", "not"],
    )?;
    segment.guard(
        2,
        "spirit",
        &["would", "learn", "received"],
        &["spirit", "would", "learn", "received", "works"],
    )?;
    segment.construct(
        3,
        "spirit",
        &["foolish", "having", "begun"],
        &["spirit", "foolish", "having", "begun", "now"],
    )?;
    segment.expression(
        4,
        "suffered",
        &["many", "things", "vain"],
        &["suffered", "many", "things", "vain", "yet"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        3,
        5,
        8,
        "spirit guard -> await_value spirit therefore ministereth",
        &["spirit", "therefore", "ministereth", "worketh", "miracles"],
    );
    segment.guard(
        5,
        "spirit",
        &["therefore", "ministereth", "worketh"],
        &["spirit", "therefore", "ministereth", "worketh", "miracles"],
    )?;
    segment.await_value(
        6,
        "abraham",
        &["even", "believed", "god"],
        &["abraham", "even", "believed", "god", "accounted"],
    )?;
    segment.await_value(
        7,
        "children",
        &["know", "therefore", "faith"],
        &["children", "know", "therefore", "faith", "same"],
    )?;
    segment.await_value(
        8,
        "god",
        &["scripture", "foreseeing", "would"],
        &["god", "scripture", "foreseeing", "would", "justify"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        3,
        9,
        12,
        "abraham await_value -> guard abraham faith blessed",
        &["abraham", "faith", "blessed", "faithful", "many"],
    );
    segment.await_value(
        9,
        "abraham",
        &["faith", "blessed", "faithful"],
        &["abraham", "faith", "blessed", "faithful"],
    )?;
    segment.guard(
        10,
        "many",
        &["works", "law", "under"],
        &["many", "works", "law", "under", "curse"],
    )?;
    segment.guard(
        11,
        "man",
        &["justified", "law", "sight"],
        &["man", "justified", "law", "sight", "god"],
    )?;
    segment.guard(
        12,
        "man",
        &["law", "not", "faith"],
        &["man", "law", "not", "faith", "doeth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        3,
        13,
        16,
        "christ construct -> move_value -> guard christ hath redeemed",
        &["christ", "hath", "redeemed", "curse", "law"],
    );
    segment.construct(
        13,
        "christ",
        &["hath", "redeemed", "curse"],
        &["christ", "hath", "redeemed", "curse", "law"],
    )?;
    segment.move_value(
        14,
        "abraham",
        &["blessing", "might", "gentiles"],
        &["abraham", "blessing", "might", "gentiles", "through"],
    )?;
    segment.guard(
        15,
        "men",
        &["brethren", "speak", "manner"],
        &["men", "brethren", "speak", "manner", "though"],
    )?;
    segment.construct(
        16,
        "abraham",
        &["now", "seed", "promises"],
        &["abraham", "now", "seed", "promises", "made"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        3,
        17,
        20,
        "god construct -> guard god say covenant",
        &["god", "say", "covenant", "confirmed", "christ"],
    );
    segment.construct(
        17,
        "god",
        &["say", "covenant", "confirmed"],
        &["god", "say", "covenant", "confirmed", "christ"],
    )?;
    segment.guard(
        18,
        "god",
        &["inheritance", "law", "promise"],
        &["god", "inheritance", "law", "promise", "gave"],
    )?;
    segment.construct(
        19,
        "wherefore",
        &["serveth", "law", "added"],
        &["wherefore", "serveth", "law", "added", "transgressions"],
    )?;
    segment.guard(
        20,
        "god",
        &["now", "mediator", "not"],
        &["god", "now", "mediator", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        3,
        21,
        24,
        "god guard -> await_value -> move_value god law against",
        &["god", "law", "against", "promises", "forbid"],
    );
    segment.guard(
        21,
        "god",
        &["law", "against", "promises"],
        &["god", "law", "against", "promises", "forbid"],
    )?;
    segment.await_value(
        22,
        "jesus",
        &["scripture", "hath", "concluded"],
        &["jesus", "scripture", "hath", "concluded", "under"],
    )?;
    segment.move_value(
        23,
        "faith",
        &["kept", "under", "law"],
        &["faith", "kept", "under", "law", "shut"],
    )?;
    segment.move_value(
        24,
        "christ",
        &["wherefore", "law", "schoolmaster"],
        &["christ", "wherefore", "law", "schoolmaster", "bring"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_3_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        3,
        25,
        29,
        "faith move_value -> await_value -> expression -> guard -> return_value faith longer under",
        &["faith", "longer", "under", "schoolmaster", "children"],
    );
    segment.move_value(
        25,
        "faith",
        &["longer", "under", "schoolmaster"],
        &["faith", "longer", "under", "schoolmaster"],
    )?;
    segment.await_value(
        26,
        "children",
        &["god", "faith", "christ"],
        &["children", "god", "faith", "christ", "jesus"],
    )?;
    segment.expression(
        27,
        "christ",
        &["many", "baptized", "put"],
        &["christ", "many", "baptized", "put"],
    )?;
    segment.guard(
        28,
        "neither",
        &["jew", "nor", "greek"],
        &["neither", "jew", "nor", "greek", "bond"],
    )?;
    segment.return_value(
        29,
        "christ's",
        &["abraham's", "seed", "heirs"],
        &["christ's", "abraham's", "seed", "heirs", "according"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        1,
        4,
        "child call -> expression -> construct child now say",
        &["child", "now", "say", "heir", "long"],
    );
    segment.call(
        1,
        "child",
        &["now", "say", "heir"],
        &["child", "now", "say", "heir", "long"],
    )?;
    segment.expression(
        2,
        "under",
        &["tutors", "governors", "until"],
        &["under", "tutors", "governors", "until", "time"],
    )?;
    segment.expression(
        3,
        "children",
        &["even", "bondage", "under"],
        &["children", "even", "bondage", "under", "elements"],
    )?;
    segment.construct(
        4,
        "god",
        &["fulness", "time", "sent"],
        &["god", "fulness", "time", "sent", "forth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        5,
        8,
        "redeem guard -> move_value redeem under law",
        &["redeem", "under", "law", "might", "receive"],
    );
    segment.guard(
        5,
        "redeem",
        &["under", "law", "might"],
        &["redeem", "under", "law", "might", "receive"],
    )?;
    segment.move_value(
        6,
        "god",
        &["sons", "hath", "sent"],
        &["god", "sons", "hath", "sent", "forth"],
    )?;
    segment.guard(
        7,
        "god",
        &["wherefore", "art", "servant"],
        &["god", "wherefore", "art", "servant", "son"],
    )?;
    segment.guard(
        8,
        "god",
        &["howbeit", "knew", "not"],
        &["god", "howbeit", "knew", "not", "service"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        9,
        12,
        "god expression -> panic -> guard god now known",
        &["god", "now", "known", "rather", "turn"],
    );
    segment.expression(
        9,
        "god",
        &["now", "known", "rather"],
        &["god", "now", "known", "rather", "turn"],
    )?;
    segment.expression(
        10,
        "observe",
        &["days", "months", "times"],
        &["observe", "days", "months", "times", "years"],
    )?;
    segment.panic(
        11,
        "afraid",
        &["lest", "bestowed", "labour"],
        &["afraid", "lest", "bestowed", "labour", "vain"],
    )?;
    segment.guard(
        12,
        "brethren",
        &["beseech", "not", "injured"],
        &["brethren", "beseech", "not", "injured"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        13,
        16,
        "know expression -> guard -> call know through infirmity",
        &["know", "through", "infirmity", "flesh", "preached"],
    );
    segment.expression(
        13,
        "know",
        &["through", "infirmity", "flesh"],
        &["know", "through", "infirmity", "flesh", "preached"],
    )?;
    segment.guard(
        14,
        "angel",
        &["temptation", "flesh", "despised"],
        &["angel", "temptation", "flesh", "despised", "not"],
    )?;
    segment.call(
        15,
        "blessedness",
        &["spake", "bear", "record"],
        &["blessedness", "spake", "bear", "record", "possible"],
    )?;
    segment.expression(
        16,
        "therefore",
        &["become", "enemy", "tell"],
        &["therefore", "become", "enemy", "tell", "truth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        17,
        20,
        "zealously guard -> construct -> call zealously affect not",
        &["zealously", "affect", "not", "well", "yea"],
    );
    segment.guard(
        17,
        "zealously",
        &["affect", "not", "well"],
        &["zealously", "affect", "not", "well", "yea"],
    )?;
    segment.guard(
        18,
        "good",
        &["zealously", "affected", "always"],
        &["good", "zealously", "affected", "always", "thing"],
    )?;
    segment.construct(
        19,
        "children",
        &["little", "travail", "birth"],
        &["children", "little", "travail", "birth", "again"],
    )?;
    segment.call(
        20,
        "desire",
        &["present", "now", "change"],
        &["desire", "present", "now", "change", "voice"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        21,
        24,
        "tell guard -> iterate -> construct -> expression tell desire under",
        &["tell", "desire", "under", "law", "not"],
    );
    segment.guard(
        21,
        "tell",
        &["desire", "under", "law"],
        &["tell", "desire", "under", "law", "not"],
    )?;
    segment.iterate(
        22,
        "abraham",
        &["written", "two", "sons"],
        &["abraham", "written", "two", "sons", "bondmaid"],
    )?;
    segment.construct(
        23,
        "bondwoman",
        &["born", "flesh", "freewoman"],
        &["bondwoman", "born", "flesh", "freewoman", "promise"],
    )?;
    segment.expression(
        24,
        "things",
        &["allegory", "two", "covenants"],
        &["things", "allegory", "two", "covenants", "mount"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        25,
        28,
        "agar expression -> guard -> return_value agar mount sinai",
        &["agar", "mount", "sinai", "arabia", "answereth"],
    );
    segment.expression(
        25,
        "agar",
        &["mount", "sinai", "arabia"],
        &["agar", "mount", "sinai", "arabia", "answereth"],
    )?;
    segment.expression(
        26,
        "jerusalem",
        &["above", "free", "mother"],
        &["jerusalem", "above", "free", "mother"],
    )?;
    segment.guard(
        27,
        "written",
        &["rejoice", "barren", "bearest"],
        &["written", "rejoice", "barren", "bearest", "not"],
    )?;
    segment.return_value(
        28,
        "isaac",
        &["now", "brethren", "children"],
        &["isaac", "now", "brethren", "children", "promise"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_4_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        4,
        29,
        31,
        "spirit construct -> guard spirit born flesh",
        &["spirit", "born", "flesh", "persecuted", "even"],
    );
    segment.construct(
        29,
        "spirit",
        &["born", "flesh", "persecuted"],
        &["spirit", "born", "flesh", "persecuted", "even"],
    )?;
    segment.guard(
        30,
        "nevertheless",
        &["saith", "scripture", "cast"],
        &["nevertheless", "saith", "scripture", "cast", "bondwoman"],
    )?;
    segment.guard(
        31,
        "children",
        &["brethren", "not", "bondwoman"],
        &["children", "brethren", "not", "bondwoman", "free"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        5,
        1,
        4,
        "christ construct -> match_value -> guard christ stand fast",
        &["christ", "stand", "fast", "therefore", "liberty"],
    );
    segment.construct(
        1,
        "christ",
        &["stand", "fast", "therefore"],
        &["christ", "stand", "fast", "therefore", "liberty"],
    )?;
    segment.match_value(
        2,
        "paul",
        &["behold", "say", "unto"],
        &["paul", "behold", "say", "unto", "circumcised"],
    )?;
    segment.guard(
        3,
        "man",
        &["testify", "again", "every"],
        &["man", "testify", "again", "every", "circumcised"],
    )?;
    segment.guard(
        4,
        "christ",
        &["become", "effect", "unto"],
        &["christ", "become", "effect", "unto", "whosoever"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        5,
        5,
        8,
        "spirit await_value -> guard spirit through wait",
        &["spirit", "through", "wait", "hope", "righteousness"],
    );
    segment.await_value(
        5,
        "spirit",
        &["through", "wait", "hope"],
        &["spirit", "through", "wait", "hope", "righteousness"],
    )?;
    segment.guard(
        6,
        "jesus",
        &["christ", "neither", "circumcision"],
        &["jesus", "christ", "neither", "circumcision", "availeth"],
    )?;
    segment.guard(
        7,
        "run",
        &["well", "hinder", "not"],
        &["run", "well", "hinder", "not", "obey"],
    )?;
    segment.guard(
        8,
        "persuasion",
        &["cometh", "not", "calleth"],
        &["persuasion", "cometh", "not", "calleth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        5,
        9,
        12,
        "little expression -> await_value -> call little leaven leaveneth",
        &["little", "leaven", "leaveneth", "whole", "lump"],
    );
    segment.expression(
        9,
        "little",
        &["leaven", "leaveneth", "whole"],
        &["little", "leaven", "leaveneth", "whole", "lump"],
    )?;
    segment.await_value(
        10,
        "lord",
        &["confidence", "through", "will"],
        &["lord", "confidence", "through", "will", "none"],
    )?;
    segment.call(
        11,
        "brethren",
        &["yet", "preach", "circumcision"],
        &["brethren", "yet", "preach", "circumcision", "why"],
    )?;
    segment.expression(
        12,
        "would",
        &["even", "cut", "off"],
        &["would", "even", "cut", "off", "trouble"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        5,
        13,
        16,
        "brethren guard brethren called unto",
        &["brethren", "called", "unto", "liberty", "use"],
    );
    segment.guard(
        13,
        "brethren",
        &["called", "unto", "liberty"],
        &["brethren", "called", "unto", "liberty", "use"],
    )?;
    segment.guard(
        14,
        "law",
        &["fulfilled", "word", "even"],
        &["law", "fulfilled", "word", "even", "shalt"],
    )?;
    segment.guard(
        15,
        "bite",
        &["devour", "another", "take"],
        &["bite", "devour", "another", "take", "heed"],
    )?;
    segment.guard(
        16,
        "spirit",
        &["say", "walk", "shall"],
        &["spirit", "say", "walk", "shall", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        5,
        17,
        20,
        "spirit expression -> guard -> panic spirit flesh lusteth",
        &["spirit", "flesh", "lusteth", "against", "contrary"],
    );
    segment.expression(
        17,
        "spirit",
        &["flesh", "lusteth", "against"],
        &["spirit", "flesh", "lusteth", "against", "contrary"],
    )?;
    segment.guard(
        18,
        "spirit",
        &["led", "not", "under"],
        &["spirit", "led", "not", "under", "law"],
    )?;
    segment.expression(
        19,
        "now",
        &["works", "flesh", "manifest"],
        &["now", "works", "flesh", "manifest", "adultery"],
    )?;
    segment.panic(
        20,
        "idolatry",
        &["witchcraft", "hatred", "variance"],
        &["idolatry", "witchcraft", "hatred", "variance", "emulations"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_5_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        5,
        21,
        23,
        "envyings guard -> await_value envyings murders drunkenness",
        &["envyings", "murders", "drunkenness", "revellings", "such"],
    );
    segment.guard(
        21,
        "envyings",
        &["murders", "drunkenness", "revellings"],
        &["envyings", "murders", "drunkenness", "revellings", "such"],
    )?;
    segment.await_value(
        22,
        "spirit",
        &["fruit", "love", "joy"],
        &["spirit", "fruit", "love", "joy", "peace"],
    )?;
    segment.guard(
        23,
        "meekness",
        &["temperance", "against", "such"],
        &["meekness", "temperance", "against", "such", "law"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_5_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        5,
        24,
        26,
        "christ's expression -> guard christ's crucified flesh",
        &["christ's", "crucified", "flesh", "affections", "lusts"],
    );
    segment.expression(
        24,
        "christ's",
        &["crucified", "flesh", "affections"],
        &["christ's", "crucified", "flesh", "affections", "lusts"],
    )?;
    segment.expression(25, "spirit", &["live", "walk"], &["spirit", "live", "walk"])?;
    segment.guard(
        26,
        "not",
        &["desirous", "vain", "glory"],
        &["not", "desirous", "vain", "glory", "provoking"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        6,
        1,
        4,
        "man reinitialize -> guard -> expression man brethren overtaken",
        &["man", "brethren", "overtaken", "fault", "spiritual"],
    );
    segment.reinitialize(
        1,
        "man",
        &["brethren", "overtaken", "fault"],
        &["man", "brethren", "overtaken", "fault", "spiritual"],
    )?;
    segment.guard(
        2,
        "christ",
        &["bear", "another's", "burdens"],
        &["christ", "bear", "another's", "burdens", "fulfil"],
    )?;
    segment.expression(
        3,
        "man",
        &["think", "himself", "something"],
        &["man", "think", "himself", "something", "nothing"],
    )?;
    segment.guard(
        4,
        "man",
        &["every", "prove", "own"],
        &["man", "every", "prove", "own", "work"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        6,
        5,
        8,
        "man iterate -> call -> guard -> expression man every shall",
        &["man", "every", "shall", "bear", "own"],
    );
    segment.iterate(
        5,
        "man",
        &["every", "shall", "bear"],
        &["man", "every", "shall", "bear", "own"],
    )?;
    segment.call(
        6,
        "taught",
        &["word", "communicate", "unto"],
        &["taught", "word", "communicate", "unto", "teacheth"],
    )?;
    segment.guard(
        7,
        "god",
        &["not", "deceived", "mocked"],
        &["god", "not", "deceived", "mocked", "whatsoever"],
    )?;
    segment.expression(
        8,
        "spirit",
        &["soweth", "flesh", "shall"],
        &["spirit", "soweth", "flesh", "shall", "reap"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        6,
        9,
        12,
        "not guard -> await_value -> expression -> construct not weary well",
        &["not", "weary", "well", "doing", "due"],
    );
    segment.guard(
        9,
        "not",
        &["weary", "well", "doing"],
        &["not", "weary", "well", "doing", "due"],
    )?;
    segment.await_value(
        10,
        "men",
        &["therefore", "opportunity", "good"],
        &["men", "therefore", "opportunity", "good", "unto"],
    )?;
    segment.expression(
        11,
        "see",
        &["large", "letter", "written"],
        &["see", "large", "letter", "written", "unto"],
    )?;
    segment.construct(
        12,
        "many",
        &["desire", "make", "fair"],
        &["many", "desire", "make", "fair", "shew"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        6,
        13,
        15,
        "neither guard -> reinitialize neither themselves circumcised",
        &["neither", "themselves", "circumcised", "keep", "law"],
    );
    segment.guard(
        13,
        "neither",
        &["themselves", "circumcised", "keep"],
        &["neither", "themselves", "circumcised", "keep", "law"],
    )?;
    segment.guard(
        14,
        "god",
        &["forbid", "glory", "save"],
        &["god", "forbid", "glory", "save", "cross"],
    )?;
    segment.reinitialize(
        15,
        "christ",
        &["jesus", "neither", "circumcision"],
        &["christ", "jesus", "neither", "circumcision", "availeth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn galatians_6_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Galatians,
        6,
        16,
        18,
        "israel borrow -> guard israel many walk",
        &["israel", "many", "walk", "according", "rule"],
    );
    segment.borrow(
        16,
        "israel",
        &["many", "walk", "according"],
        &["israel", "many", "walk", "according", "rule"],
    )?;
    segment.guard(
        17,
        "man",
        &["henceforth", "trouble", "bear"],
        &["man", "henceforth", "trouble", "bear", "body"],
    )?;
    segment.borrow(
        18,
        "lord",
        &["brethren", "grace", "jesus"],
        &["lord", "brethren", "grace", "jesus", "christ"],
    )?;
    segment.commit()?;
    Ok(())
}
