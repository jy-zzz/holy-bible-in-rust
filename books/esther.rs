use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    esther(story)?;
    Ok(())
}

fn esther(story: &mut Story) -> Result<(), StoryError> {
    esther_1(story)?;
    esther_2(story)?;
    esther_3(story)?;
    esther_4(story)?;
    esther_5(story)?;
    esther_6(story)?;
    esther_7(story)?;
    esther_8(story)?;
    esther_9(story)?;
    esther_10(story)?;
    Ok(())
}

fn esther_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 1, "Ahasuerus displays imperial wealth for a vast banquet, then deposes Vashti when she refuses the king's public summons.", "The court converts one queen's refusal into a legal policy for every household, showing how vanity can become administrative code.");
    esther_1_group_01(story)?;
    esther_1_group_02(story)?;
    esther_1_group_03(story)?;
    esther_1_group_04(story)?;
    esther_1_group_05(story)?;
    esther_1_group_06(story)?;
    Ok(())
}

fn esther_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 2, "Young women enter the royal selection process, and Esther wins favor while Mordecai quietly uncovers an assassination plot.", "Esther conceals her people as instructed, and the king's records preserve Mordecai's warning without immediate reward.");
    esther_2_group_01(story)?;
    esther_2_group_02(story)?;
    esther_2_group_03(story)?;
    esther_2_group_04(story)?;
    esther_2_group_05(story)?;
    esther_2_group_06(story)?;
    Ok(())
}

fn esther_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 3, "Haman rises, demands universal bowing, and turns Mordecai's refusal into a plan to delete the entire Jewish population.", "Lots choose the date, the king signs the irreversible decree, and the city reels while the palace treats catastrophe as routine paperwork.");
    esther_3_group_01(story)?;
    esther_3_group_02(story)?;
    esther_3_group_03(story)?;
    esther_3_group_04(story)?;
    Ok(())
}

fn esther_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 4, "Mordecai mourns in sackcloth and sends Esther the decree, asking whether her royal position exists for this exact crisis.", "Esther requests a fast and resolves to approach the king, accepting the possibility that courage may end her comfortable access.");
    esther_4_group_01(story)?;
    esther_4_group_02(story)?;
    esther_4_group_03(story)?;
    esther_4_group_04(story)?;
    Ok(())
}

fn esther_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 5, "Esther enters the inner court, receives the golden scepter, and invites the king and Haman to two private banquets.", "Haman leaves proud but burns with rage at Mordecai, then builds a towering stake for the man who will soon become his downfall.");
    esther_5_group_01(story)?;
    esther_5_group_02(story)?;
    esther_5_group_03(story)?;
    esther_5_group_04(story)?;
    Ok(())
}

fn esther_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 6, "The sleepless king reads the forgotten record of Mordecai's service and asks Haman how to honor a favored man.", "Haman prescribes royal parade language for himself, only to lead Mordecai through the streets before returning home humiliated.");
    esther_6_group_01(story)?;
    esther_6_group_02(story)?;
    esther_6_group_03(story)?;
    esther_6_group_04(story)?;
    Ok(())
}

fn esther_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 7, "At the second banquet Esther names Haman as the enemy, and the king's anger turns the room into a sudden judgment chamber.", "Haman falls on Esther's couch, the king misreads the scene, and the prepared stake becomes the execution site for its builder.");
    esther_7_group_01(story)?;
    esther_7_group_02(story)?;
    esther_7_group_03(story)?;
    Ok(())
}

fn esther_8(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 8, "Esther receives Haman's estate, Mordecai receives the signet, and a new decree permits the Jews to defend themselves.", "Couriers race through the provinces, grief becomes relief, and the people prepare for a lawful counter-release rather than helpless waiting.");
    esther_8_group_01(story)?;
    esther_8_group_02(story)?;
    esther_8_group_03(story)?;
    esther_8_group_04(story)?;
    Ok(())
}

fn esther_9(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 9, "The appointed day reverses the threat: Jewish communities defeat their attackers and establish Purim as a yearly memory of deliverance.", "Esther and Mordecai formalize the festival with letters, preserving joy as a repeatable calendar command.");
    esther_9_group_01(story)?;
    esther_9_group_02(story)?;
    esther_9_group_03(story)?;
    esther_9_group_04(story)?;
    esther_9_group_05(story)?;
    esther_9_group_06(story)?;
    esther_9_group_07(story)?;
    esther_9_group_08(story)?;
    Ok(())
}

fn esther_10(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Esther", 10, "Mordecai becomes second to the king, seeking the welfare of his people and speaking peace throughout the empire.", "The short closing record treats faithful administration as the final victory, with hidden providence visible in the shape of public good.");
    esther_10_group_01(story)?;
    Ok(())
}

fn esther_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        1,
        1,
        4,
        "now move_value -> expression -> construct now pass days",
        &["now", "pass", "days", "ahasuerus", "reigned"],
    );
    segment.move_value(
        1,
        "now",
        &["pass", "days", "ahasuerus"],
        &["now", "pass", "days", "ahasuerus", "reigned"],
    )?;
    segment.expression(
        2,
        "king",
        &["days", "ahasuerus", "sat"],
        &["king", "days", "ahasuerus", "sat", "throne"],
    )?;
    segment.construct(
        3,
        "third",
        &["year", "reign", "made"],
        &["third", "year", "reign", "made", "feast"],
    )?;
    segment.expression(
        4,
        "shewed",
        &["riches", "glorious", "kingdom"],
        &["shewed", "riches", "glorious", "kingdom", "honour"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        1,
        5,
        8,
        "king construct -> expression -> guard king days expired",
        &["king", "days", "expired", "made", "feast"],
    );
    segment.construct(
        5,
        "king",
        &["days", "expired", "made"],
        &["king", "days", "expired", "made", "feast"],
    )?;
    segment.expression(
        6,
        "white",
        &["green", "blue", "hangings"],
        &["white", "green", "blue", "hangings", "fastened"],
    )?;
    segment.expression(
        7,
        "gave",
        &["drink", "vessels", "gold"],
        &["gave", "drink", "vessels", "gold", "diverse"],
    )?;
    segment.guard(
        8,
        "king",
        &["drinking", "according", "law"],
        &["king", "drinking", "according", "law", "none"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        1,
        9,
        12,
        "queen construct -> expression -> move_value queen vashti made",
        &["queen", "vashti", "made", "feast", "women"],
    );
    segment.construct(
        9,
        "queen",
        &["vashti", "made", "feast"],
        &["queen", "vashti", "made", "feast", "women"],
    )?;
    segment.expression(
        10,
        "king",
        &["seventh", "day", "heart"],
        &["king", "seventh", "day", "heart", "merry"],
    )?;
    segment.move_value(
        11,
        "queen",
        &["bring", "vashti", "king"],
        &["queen", "bring", "vashti", "king", "crown"],
    )?;
    segment.move_value(
        12,
        "queen",
        &["vashti", "refused", "king's"],
        &["queen", "vashti", "refused", "king's", "commandment"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        1,
        13,
        16,
        "king guard -> match_value king wise men",
        &["king", "wise", "men", "knew", "times"],
    );
    segment.guard(
        13,
        "king",
        &["wise", "men", "knew"],
        &["king", "wise", "men", "knew", "times"],
    )?;
    segment.match_value(
        14,
        "next",
        &["unto", "carshena", "shethar"],
        &["next", "unto", "carshena", "shethar", "admatha"],
    )?;
    segment.guard(
        15,
        "queen",
        &["shall", "unto", "vashti"],
        &["queen", "shall", "unto", "vashti", "according"],
    )?;
    segment.guard(
        16,
        "king",
        &["memucan", "answered", "princes"],
        &["king", "memucan", "answered", "princes", "vashti"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        1,
        17,
        19,
        "queen move_value -> call queen deed shall",
        &["queen", "deed", "shall", "abroad", "unto"],
    );
    segment.move_value(
        17,
        "queen",
        &["deed", "shall", "abroad"],
        &["queen", "deed", "shall", "abroad", "unto"],
    )?;
    segment.call(
        18,
        "likewise",
        &["shall", "ladies", "persia"],
        &["likewise", "shall", "ladies", "persia", "media"],
    )?;
    segment.move_value(
        19,
        "king",
        &["please", "royal", "commandment"],
        &["king", "please", "royal", "commandment", "written"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        1,
        20,
        22,
        "king's construct -> call -> move_value king's decree shall",
        &["king's", "decree", "shall", "make", "published"],
    );
    segment.construct(
        20,
        "king's",
        &["decree", "shall", "make"],
        &["king's", "decree", "shall", "make", "published"],
    )?;
    segment.call(
        21,
        "king",
        &["saying", "pleased", "princes"],
        &["king", "saying", "pleased", "princes", "according"],
    )?;
    segment.move_value(
        22,
        "sent",
        &["letters", "king's", "provinces"],
        &["sent", "letters", "king's", "provinces", "every"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        2,
        1,
        4,
        "king panic -> call -> expression king things wrath",
        &["king", "things", "wrath", "ahasuerus", "appeased"],
    );
    segment.panic(
        1,
        "king",
        &["things", "wrath", "ahasuerus"],
        &["king", "things", "wrath", "ahasuerus", "appeased"],
    )?;
    segment.call(
        2,
        "king's",
        &["servants", "ministered", "unto"],
        &["king's", "servants", "ministered", "unto", "fair"],
    )?;
    segment.expression(
        3,
        "king",
        &["appoint", "officers", "provinces"],
        &["king", "appoint", "officers", "provinces", "kingdom"],
    )?;
    segment.expression(
        4,
        "king",
        &["maiden", "pleaseth", "queen"],
        &["king", "maiden", "pleaseth", "queen", "instead"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        2,
        5,
        8,
        "now expression -> move_value now shushan palace",
        &["now", "shushan", "palace", "certain", "jew"],
    );
    segment.expression(
        5,
        "now",
        &["shushan", "palace", "certain"],
        &["now", "shushan", "palace", "certain", "jew"],
    )?;
    segment.expression(
        6,
        "king",
        &["carried", "away", "jerusalem"],
        &["king", "carried", "away", "jerusalem", "captivity"],
    )?;
    segment.move_value(
        7,
        "brought",
        &["hadassah", "esther", "uncle's"],
        &["brought", "hadassah", "esther", "uncle's", "daughter"],
    )?;
    segment.move_value(
        8,
        "pass",
        &["king's", "commandment", "decree"],
        &["pass", "king's", "commandment", "decree", "heard"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        2,
        9,
        12,
        "maiden borrow -> guard -> iterate -> move_value maiden pleased obtained",
        &["maiden", "pleased", "obtained", "kindness", "speedily"],
    );
    segment.borrow(
        9,
        "maiden",
        &["pleased", "obtained", "kindness"],
        &["maiden", "pleased", "obtained", "kindness", "speedily"],
    )?;
    segment.guard(
        10,
        "people",
        &["esther", "not", "shewed"],
        &["people", "esther", "not", "shewed", "nor"],
    )?;
    segment.iterate(
        11,
        "mordecai",
        &["walked", "every", "day"],
        &["mordecai", "walked", "every", "day", "court"],
    )?;
    segment.move_value(
        12,
        "king",
        &["now", "every", "maid's"],
        &["king", "now", "every", "maid's", "turn"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        2,
        13,
        16,
        "king move_value -> expression king thus every",
        &["king", "thus", "every", "maiden", "unto"],
    );
    segment.move_value(
        13,
        "king",
        &["thus", "every", "maiden"],
        &["king", "thus", "every", "maiden", "unto"],
    )?;
    segment.move_value(
        14,
        "women",
        &["evening", "went", "morrow"],
        &["women", "evening", "went", "morrow", "returned"],
    )?;
    segment.move_value(
        15,
        "now",
        &["turn", "esther", "daughter"],
        &["now", "turn", "esther", "daughter", "abihail"],
    )?;
    segment.expression(
        16,
        "king",
        &["esther", "taken", "unto"],
        &["king", "esther", "taken", "unto", "ahasuerus"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        2,
        17,
        20,
        "king construct -> expression -> move_value king loved esther",
        &["king", "loved", "esther", "above", "women"],
    );
    segment.construct(
        17,
        "king",
        &["loved", "esther", "above"],
        &["king", "loved", "esther", "above", "women"],
    )?;
    segment.construct(
        18,
        "king",
        &["made", "great", "feast"],
        &["king", "made", "great", "feast", "unto"],
    )?;
    segment.expression(
        19,
        "virgins",
        &["gathered", "together", "second"],
        &["virgins", "gathered", "together", "second", "time"],
    )?;
    segment.move_value(
        20,
        "people",
        &["esther", "not", "yet"],
        &["people", "esther", "not", "yet", "shewed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        2,
        21,
        23,
        "days expression -> construct days while mordecai",
        &["days", "while", "mordecai", "sat", "king's"],
    );
    segment.expression(
        21,
        "days",
        &["while", "mordecai", "sat"],
        &["days", "while", "mordecai", "sat", "king's"],
    )?;
    segment.expression(
        22,
        "queen",
        &["thing", "known", "mordecai"],
        &["queen", "thing", "known", "mordecai", "told"],
    )?;
    segment.construct(
        23,
        "inquisition",
        &["made", "matter", "found"],
        &["inquisition", "made", "matter", "found", "therefore"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        3,
        1,
        4,
        "king expression -> guard -> move_value king things ahasuerus",
        &["king", "things", "ahasuerus", "promote", "haman"],
    );
    segment.expression(
        1,
        "king",
        &["things", "ahasuerus", "promote"],
        &["king", "things", "ahasuerus", "promote", "haman"],
    )?;
    segment.guard(
        2,
        "king",
        &["king's", "servants", "gate"],
        &["king", "king's", "servants", "gate", "bowed"],
    )?;
    segment.guard(
        3,
        "king's",
        &["servants", "gate", "unto"],
        &["king's", "servants", "gate", "unto", "mordecai"],
    )?;
    segment.move_value(
        4,
        "now",
        &["pass", "spake", "daily"],
        &["now", "pass", "spake", "daily", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        3,
        5,
        8,
        "haman guard -> drop_value -> expression haman saw mordecai",
        &["haman", "saw", "mordecai", "bowed", "not"],
    );
    segment.guard(
        5,
        "haman",
        &["saw", "mordecai", "bowed"],
        &["haman", "saw", "mordecai", "bowed", "not"],
    )?;
    segment.drop_value(
        6,
        "people",
        &["thought", "scorn", "lay"],
        &["people", "thought", "scorn", "lay", "hands"],
    )?;
    segment.expression(
        7,
        "king",
        &["first", "month", "nisan"],
        &["king", "first", "month", "nisan", "twelfth"],
    )?;
    segment.guard(
        8,
        "king",
        &["haman", "unto", "ahasuerus"],
        &["king", "haman", "unto", "ahasuerus", "certain"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        3,
        9,
        12,
        "king move_value -> expression -> call -> iterate king please written",
        &["king", "please", "written", "destroyed", "will"],
    );
    segment.move_value(
        9,
        "king",
        &["please", "written", "destroyed"],
        &["king", "please", "written", "destroyed", "will"],
    )?;
    segment.expression(
        10,
        "king",
        &["took", "ring", "hand"],
        &["king", "took", "ring", "hand", "gave"],
    )?;
    segment.call(
        11,
        "king",
        &["unto", "haman", "silver"],
        &["king", "unto", "haman", "silver", "given"],
    )?;
    segment.iterate(
        12,
        "king's",
        &["scribes", "called", "thirteenth"],
        &["king's", "scribes", "called", "thirteenth", "day"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        3,
        13,
        15,
        "letters move_value -> guard letters sent posts",
        &["letters", "sent", "posts", "king's", "provinces"],
    );
    segment.move_value(
        13,
        "letters",
        &["sent", "posts", "king's"],
        &["letters", "sent", "posts", "king's", "provinces"],
    )?;
    segment.guard(
        14,
        "copy",
        &["writing", "commandment", "given"],
        &["copy", "writing", "commandment", "given", "every"],
    )?;
    segment.move_value(
        15,
        "posts",
        &["went", "hastened", "king's"],
        &["posts", "went", "hastened", "king's", "commandment"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        4,
        1,
        4,
        "mordecai move_value mordecai perceived done",
        &["mordecai", "perceived", "done", "rent", "clothes"],
    );
    segment.move_value(
        1,
        "mordecai",
        &["perceived", "done", "rent"],
        &["mordecai", "perceived", "done", "rent", "clothes"],
    )?;
    segment.move_value(
        2,
        "even",
        &["king's", "gate", "none"],
        &["even", "king's", "gate", "none", "might"],
    )?;
    segment.move_value(
        3,
        "every",
        &["province", "whithersoever", "king's"],
        &[
            "every",
            "province",
            "whithersoever",
            "king's",
            "commandment",
        ],
    )?;
    segment.move_value(
        4,
        "queen",
        &["esther's", "maids", "chamberlains"],
        &["queen", "esther's", "maids", "chamberlains", "told"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        4,
        5,
        8,
        "called guard -> move_value -> drop_value -> construct called esther hatach",
        &["called", "esther", "hatach", "king's", "chamberlains"],
    );
    segment.guard(
        5,
        "called",
        &["esther", "hatach", "king's"],
        &["called", "esther", "hatach", "king's", "chamberlains"],
    )?;
    segment.move_value(
        6,
        "hatach",
        &["went", "forth", "mordecai"],
        &["hatach", "went", "forth", "mordecai", "unto"],
    )?;
    segment.drop_value(
        7,
        "mordecai",
        &["told", "happened", "unto"],
        &["mordecai", "told", "happened", "unto", "sum"],
    )?;
    segment.construct(
        8,
        "gave",
        &["copy", "writing", "decree"],
        &["gave", "copy", "writing", "decree", "given"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        4,
        9,
        12,
        "hatach move_value -> guard -> expression hatach told esther",
        &["hatach", "told", "esther", "words", "mordecai"],
    );
    segment.move_value(
        9,
        "hatach",
        &["told", "esther", "words"],
        &["hatach", "told", "esther", "words", "mordecai"],
    )?;
    segment.guard(
        10,
        "again",
        &["esther", "spake", "unto"],
        &["again", "esther", "spake", "unto", "hatach"],
    )?;
    segment.move_value(
        11,
        "people",
        &["king's", "servants", "provinces"],
        &["people", "king's", "servants", "provinces", "know"],
    )?;
    segment.expression(
        12,
        "told",
        &["mordecai", "esther's", "words"],
        &["told", "mordecai", "esther's", "words"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        4,
        13,
        17,
        "mordecai guard -> move_value -> return_value mordecai commanded answer",
        &["mordecai", "commanded", "answer", "esther", "think"],
    );
    segment.guard(
        13,
        "mordecai",
        &["commanded", "answer", "esther"],
        &["mordecai", "commanded", "answer", "esther", "think"],
    )?;
    segment.move_value(
        14,
        "altogether",
        &["holdest", "peace", "time"],
        &["altogether", "holdest", "peace", "time", "shall"],
    )?;
    segment.return_value(
        15,
        "esther",
        &["bade", "return", "mordecai"],
        &["esther", "bade", "return", "mordecai", "answer"],
    )?;
    segment.move_value(
        16,
        "gather",
        &["together", "jews", "present"],
        &["gather", "together", "jews", "present", "shushan"],
    )?;
    segment.move_value(
        17,
        "mordecai",
        &["went", "way", "according"],
        &["mordecai", "went", "way", "according", "esther"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        5,
        1,
        4,
        "now move_value -> match_value -> result now pass third",
        &["now", "pass", "third", "day", "esther"],
    );
    segment.move_value(
        1,
        "now",
        &["pass", "third", "day"],
        &["now", "pass", "third", "day", "esther"],
    )?;
    segment.match_value(
        2,
        "king",
        &["saw", "esther", "queen"],
        &["king", "saw", "esther", "queen", "standing"],
    )?;
    segment.result(
        3,
        "king",
        &["unto", "wilt", "queen"],
        &["king", "unto", "wilt", "queen", "esther"],
    )?;
    segment.move_value(
        4,
        "king",
        &["esther", "answered", "seem"],
        &["king", "esther", "answered", "seem", "good"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        5,
        5,
        8,
        "king construct -> result -> move_value king cause haman",
        &["king", "cause", "haman", "make", "haste"],
    );
    segment.construct(
        5,
        "king",
        &["cause", "haman", "make"],
        &["king", "cause", "haman", "make", "haste"],
    )?;
    segment.result(
        6,
        "king",
        &["unto", "esther", "banquet"],
        &["king", "unto", "esther", "banquet", "wine"],
    )?;
    segment.result(
        7,
        "answered",
        &["esther", "petition", "request"],
        &["answered", "esther", "petition", "request"],
    )?;
    segment.move_value(
        8,
        "king",
        &["found", "favour", "sight"],
        &["king", "found", "favour", "sight", "please"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        5,
        9,
        11,
        "went move_value -> trait_bound went haman forth",
        &["went", "haman", "forth", "day", "joyful"],
    );
    segment.move_value(
        9,
        "went",
        &["haman", "forth", "day"],
        &["went", "haman", "forth", "day", "joyful"],
    )?;
    segment.move_value(
        10,
        "nevertheless",
        &["haman", "refrained", "himself"],
        &["nevertheless", "haman", "refrained", "himself", "home"],
    )?;
    segment.trait_bound(
        11,
        "children",
        &["haman", "told", "glory"],
        &["children", "haman", "told", "glory", "riches"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        5,
        12,
        14,
        "queen move_value -> expression -> construct queen haman moreover",
        &["queen", "haman", "moreover", "yea", "esther"],
    );
    segment.move_value(
        12,
        "queen",
        &["haman", "moreover", "yea"],
        &["queen", "haman", "moreover", "yea", "esther"],
    )?;
    segment.expression(
        13,
        "yet",
        &["availeth", "nothing", "long"],
        &["yet", "availeth", "nothing", "long", "see"],
    )?;
    segment.construct(
        14,
        "zeresh",
        &["wife", "friends", "unto"],
        &["zeresh", "wife", "friends", "unto", "gallows"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        6,
        1,
        4,
        "king move_value -> expression -> call king night not",
        &["king", "night", "not", "sleep", "commanded"],
    );
    segment.move_value(
        1,
        "king",
        &["night", "not", "sleep"],
        &["king", "night", "not", "sleep", "commanded"],
    )?;
    segment.expression(
        2,
        "found",
        &["written", "mordecai", "told"],
        &["found", "written", "mordecai", "told", "bigthana"],
    )?;
    segment.call(
        3,
        "king",
        &["honour", "dignity", "hath"],
        &["king", "honour", "dignity", "hath", "done"],
    )?;
    segment.move_value(
        4,
        "king",
        &["court", "now", "haman"],
        &["king", "court", "now", "haman", "outward"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        6,
        5,
        8,
        "king move_value -> call king king's servants",
        &["king", "king's", "servants", "unto", "behold"],
    );
    segment.move_value(
        5,
        "king",
        &["king's", "servants", "unto"],
        &["king", "king's", "servants", "unto", "behold"],
    )?;
    segment.move_value(
        6,
        "king",
        &["haman", "unto", "shall"],
        &["king", "haman", "unto", "shall", "done"],
    )?;
    segment.call(
        7,
        "king",
        &["haman", "answered", "man"],
        &["king", "haman", "answered", "man", "delighteth"],
    )?;
    segment.move_value(
        8,
        "king",
        &["royal", "apparel", "brought"],
        &["king", "royal", "apparel", "brought", "useth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        6,
        9,
        11,
        "apparel move_value -> construct apparel horse delivered",
        &["apparel", "horse", "delivered", "hand", "king's"],
    );
    segment.move_value(
        9,
        "apparel",
        &["horse", "delivered", "hand"],
        &["apparel", "horse", "delivered", "hand", "king's"],
    )?;
    segment.construct(
        10,
        "king",
        &["haman", "make", "haste"],
        &["king", "haman", "make", "haste", "take"],
    )?;
    segment.move_value(
        11,
        "took",
        &["haman", "apparel", "horse"],
        &["took", "haman", "apparel", "horse", "arrayed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        6,
        12,
        14,
        "mordecai move_value -> guard mordecai again king's",
        &["mordecai", "again", "king's", "gate", "haman"],
    );
    segment.move_value(
        12,
        "mordecai",
        &["again", "king's", "gate"],
        &["mordecai", "again", "king's", "gate", "haman"],
    )?;
    segment.guard(
        13,
        "haman",
        &["told", "zeresh", "wife"],
        &["haman", "told", "zeresh", "wife", "friends"],
    )?;
    segment.move_value(
        14,
        "while",
        &["yet", "talking", "king's"],
        &["while", "yet", "talking", "king's", "chamberlains"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        7,
        1,
        4,
        "king move_value -> result -> drop_value king haman banquet",
        &["king", "haman", "banquet", "esther", "queen"],
    );
    segment.move_value(
        1,
        "king",
        &["haman", "banquet", "esther"],
        &["king", "haman", "banquet", "esther", "queen"],
    )?;
    segment.result(
        2,
        "king",
        &["again", "unto", "esther"],
        &["king", "again", "unto", "esther", "second"],
    )?;
    segment.result(
        3,
        "queen",
        &["esther", "answered", "found"],
        &["queen", "esther", "answered", "found", "favour"],
    )?;
    segment.drop_value(
        4,
        "people",
        &["sold", "destroyed", "slain"],
        &["people", "sold", "destroyed", "slain", "perish"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        7,
        5,
        7,
        "king call -> construct king ahasuerus answered",
        &["king", "ahasuerus", "answered", "unto", "esther"],
    );
    segment.call(
        5,
        "king",
        &["ahasuerus", "answered", "unto"],
        &["king", "ahasuerus", "answered", "unto", "esther"],
    )?;
    segment.call(
        6,
        "king",
        &["esther", "adversary", "enemy"],
        &["king", "esther", "adversary", "enemy", "wicked"],
    )?;
    segment.construct(
        7,
        "king",
        &["arising", "banquet", "wine"],
        &["king", "arising", "banquet", "wine", "wrath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        7,
        8,
        10,
        "king move_value -> construct -> panic king returned palace",
        &["king", "returned", "palace", "garden", "place"],
    );
    segment.move_value(
        8,
        "king",
        &["returned", "palace", "garden"],
        &["king", "returned", "palace", "garden", "place"],
    )?;
    segment.construct(
        9,
        "king",
        &["harbonah", "chamberlains", "behold"],
        &["king", "harbonah", "chamberlains", "behold", "gallows"],
    )?;
    segment.panic(
        10,
        "hanged",
        &["haman", "gallows", "prepared"],
        &["hanged", "haman", "gallows", "prepared", "mordecai"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_8_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        8,
        1,
        4,
        "king move_value -> expression -> call -> reinitialize king day ahasuerus",
        &["king", "day", "ahasuerus", "give", "house"],
    );
    segment.move_value(
        1,
        "king",
        &["day", "ahasuerus", "give"],
        &["king", "day", "ahasuerus", "give", "house"],
    )?;
    segment.expression(
        2,
        "king",
        &["took", "off", "ring"],
        &["king", "took", "off", "ring", "taken"],
    )?;
    segment.call(
        3,
        "king",
        &["esther", "spake", "yet"],
        &["king", "esther", "spake", "yet", "again"],
    )?;
    segment.reinitialize(
        4,
        "king",
        &["held", "golden", "sceptre"],
        &["king", "held", "golden", "sceptre", "toward"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_8_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        8,
        5,
        8,
        "king drop_value -> move_value -> match_value -> guard king please favour",
        &["king", "please", "favour", "sight", "thing"],
    );
    segment.drop_value(
        5,
        "king",
        &["please", "favour", "sight"],
        &["king", "please", "favour", "sight", "thing"],
    )?;
    segment.move_value(
        6,
        "people",
        &["endure", "see", "evil"],
        &["people", "endure", "see", "evil", "shall"],
    )?;
    segment.match_value(
        7,
        "king",
        &["ahasuerus", "unto", "esther"],
        &["king", "ahasuerus", "unto", "esther", "queen"],
    )?;
    segment.guard(
        8,
        "write",
        &["jews", "liketh", "king's"],
        &["write", "jews", "liketh", "king's", "name"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_8_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        8,
        9,
        12,
        "king's iterate -> move_value -> drop_value -> expression king's scribes called",
        &["king's", "scribes", "called", "time", "third"],
    );
    segment.iterate(
        9,
        "king's",
        &["scribes", "called", "time"],
        &["king's", "scribes", "called", "time", "third"],
    )?;
    segment.move_value(
        10,
        "king",
        &["wrote", "ahasuerus", "name"],
        &["king", "wrote", "ahasuerus", "name", "sealed"],
    )?;
    segment.drop_value(
        11,
        "king",
        &["wherein", "granted", "jews"],
        &["king", "wherein", "granted", "jews", "every"],
    )?;
    segment.expression(
        12,
        "king",
        &["day", "provinces", "ahasuerus"],
        &["king", "day", "provinces", "ahasuerus", "namely"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_8_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        8,
        13,
        17,
        "copy guard -> move_value -> expression copy writing commandment",
        &["copy", "writing", "commandment", "given", "every"],
    );
    segment.guard(
        13,
        "copy",
        &["writing", "commandment", "given"],
        &["copy", "writing", "commandment", "given", "every"],
    )?;
    segment.move_value(
        14,
        "posts",
        &["rode", "mules", "camels"],
        &["posts", "rode", "mules", "camels", "went"],
    )?;
    segment.move_value(
        15,
        "king",
        &["mordecai", "went", "presence"],
        &["king", "mordecai", "went", "presence", "royal"],
    )?;
    segment.expression(
        16,
        "jews",
        &["light", "gladness", "joy"],
        &["jews", "light", "gladness", "joy", "honour"],
    )?;
    segment.move_value(
        17,
        "every",
        &["province", "city", "whithersoever"],
        &["every", "province", "city", "whithersoever", "king's"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        1,
        4,
        "now guard -> panic -> move_value now twelfth month",
        &["now", "twelfth", "month", "adar", "thirteenth"],
    );
    segment.guard(
        1,
        "now",
        &["twelfth", "month", "adar"],
        &["now", "twelfth", "month", "adar", "thirteenth"],
    )?;
    segment.guard(
        2,
        "king",
        &["jews", "gathered", "themselves"],
        &["king", "jews", "gathered", "themselves", "together"],
    )?;
    segment.panic(
        3,
        "king",
        &["rulers", "provinces", "lieutenants"],
        &["king", "rulers", "provinces", "lieutenants", "deputies"],
    )?;
    segment.move_value(
        4,
        "mordecai",
        &["great", "king's", "house"],
        &["mordecai", "great", "king's", "house", "fame"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        5,
        8,
        "thus expression -> drop_value thus jews smote",
        &["thus", "jews", "smote", "enemies", "stroke"],
    );
    segment.expression(
        5,
        "thus",
        &["jews", "smote", "enemies"],
        &["thus", "jews", "smote", "enemies", "stroke"],
    )?;
    segment.drop_value(
        6,
        "men",
        &["shushan", "palace", "jews"],
        &["men", "shushan", "palace", "jews", "slew"],
    )?;
    segment.expression(
        7,
        "parshandatha",
        &["dalphon", "aspatha"],
        &["parshandatha", "dalphon", "aspatha"],
    )?;
    segment.expression(
        8,
        "poratha",
        &["adalia", "aridatha"],
        &["poratha", "adalia", "aridatha"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        9,
        12,
        "parmashta expression -> guard -> move_value -> drop_value parmashta arisai aridai",
        &["parmashta", "arisai", "aridai", "vajezatha", "ten"],
    );
    segment.expression(
        9,
        "parmashta",
        &["arisai", "aridai", "vajezatha"],
        &["parmashta", "arisai", "aridai", "vajezatha"],
    )?;
    segment.guard(
        10,
        "ten",
        &["sons", "haman", "son"],
        &["ten", "sons", "haman", "son", "hammedatha"],
    )?;
    segment.move_value(
        11,
        "king",
        &["day", "number", "slain"],
        &["king", "day", "number", "slain", "shushan"],
    )?;
    segment.drop_value(
        12,
        "king",
        &["unto", "esther", "queen"],
        &["king", "unto", "esther", "queen", "jews"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        13,
        16,
        "king call -> iterate -> guard king esther please",
        &["king", "esther", "please", "granted", "jews"],
    );
    segment.call(
        13,
        "king",
        &["esther", "please", "granted"],
        &["king", "esther", "please", "granted", "jews"],
    )?;
    segment.iterate(
        14,
        "king",
        &["commanded", "done", "decree"],
        &["king", "commanded", "done", "decree", "given"],
    )?;
    segment.guard(
        15,
        "jews",
        &["shushan", "gathered", "themselves"],
        &["jews", "shushan", "gathered", "themselves", "together"],
    )?;
    segment.guard(
        16,
        "other",
        &["jews", "king's", "provinces"],
        &["other", "jews", "king's", "provinces", "gathered"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        17,
        20,
        "thirteenth construct -> move_value thirteenth day month",
        &["thirteenth", "day", "month", "adar", "fourteenth"],
    );
    segment.construct(
        17,
        "thirteenth",
        &["day", "month", "adar"],
        &["thirteenth", "day", "month", "adar", "fourteenth"],
    )?;
    segment.construct(
        18,
        "jews",
        &["shushan", "assembled", "together"],
        &["jews", "shushan", "assembled", "together", "thirteenth"],
    )?;
    segment.construct(
        19,
        "therefore",
        &["jews", "villages", "dwelt"],
        &["therefore", "jews", "villages", "dwelt", "unwalled"],
    )?;
    segment.move_value(
        20,
        "mordecai",
        &["wrote", "things", "sent"],
        &["mordecai", "wrote", "things", "sent", "letters"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        21,
        24,
        "stablish guard -> construct -> expression -> drop_value stablish among keep",
        &["stablish", "among", "keep", "fourteenth", "day"],
    );
    segment.guard(
        21,
        "stablish",
        &["among", "keep", "fourteenth"],
        &["stablish", "among", "keep", "fourteenth", "day"],
    )?;
    segment.construct(
        22,
        "days",
        &["wherein", "jews", "rested"],
        &["days", "wherein", "jews", "rested", "enemies"],
    )?;
    segment.expression(
        23,
        "jews",
        &["undertook", "begun", "mordecai"],
        &["jews", "undertook", "begun", "mordecai", "written"],
    )?;
    segment.drop_value(
        24,
        "haman",
        &["son", "hammedatha", "agagite"],
        &["haman", "son", "hammedatha", "agagite", "enemy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        25,
        28,
        "king move_value -> guard king esther commanded",
        &["king", "esther", "commanded", "letters", "wicked"],
    );
    segment.move_value(
        25,
        "king",
        &["esther", "commanded", "letters"],
        &["king", "esther", "commanded", "letters", "wicked"],
    )?;
    segment.move_value(
        26,
        "wherefore",
        &["called", "days", "purim"],
        &["wherefore", "called", "days", "purim", "name"],
    )?;
    segment.guard(
        27,
        "jews",
        &["ordained", "took", "seed"],
        &["jews", "ordained", "took", "seed", "such"],
    )?;
    segment.guard(
        28,
        "days",
        &["remembered", "kept", "throughout"],
        &["days", "remembered", "kept", "throughout", "every"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_9_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        9,
        29,
        32,
        "queen expression -> move_value -> result queen esther daughter",
        &["queen", "esther", "daughter", "abihail", "mordecai"],
    );
    segment.expression(
        29,
        "queen",
        &["esther", "daughter", "abihail"],
        &["queen", "esther", "daughter", "abihail", "mordecai"],
    )?;
    segment.move_value(
        30,
        "sent",
        &["letters", "unto", "jews"],
        &["sent", "letters", "unto", "jews", "hundred"],
    )?;
    segment.result(
        31,
        "confirm",
        &["days", "purim", "times"],
        &["confirm", "days", "purim", "times", "appointed"],
    )?;
    segment.expression(
        32,
        "decree",
        &["esther", "confirmed", "matters"],
        &["decree", "esther", "confirmed", "matters", "purim"],
    )?;
    segment.commit()?;
    Ok(())
}

fn esther_10_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Esther,
        10,
        1,
        3,
        "king expression -> guard king ahasuerus laid",
        &["king", "ahasuerus", "laid", "tribute", "land"],
    );
    segment.expression(
        1,
        "king",
        &["ahasuerus", "laid", "tribute"],
        &["king", "ahasuerus", "laid", "tribute", "land"],
    )?;
    segment.guard(
        2,
        "king",
        &["acts", "power", "might"],
        &["king", "acts", "power", "might", "declaration"],
    )?;
    segment.expression(
        3,
        "king",
        &["mordecai", "jew", "next"],
        &["king", "mordecai", "jew", "next", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}
