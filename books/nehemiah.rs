use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    nehemiah(story)?;
    Ok(())
}

fn nehemiah(story: &mut Story) -> Result<(), StoryError> {
    nehemiah_1(story)?;
    nehemiah_2(story)?;
    nehemiah_3(story)?;
    nehemiah_4(story)?;
    nehemiah_5(story)?;
    nehemiah_6(story)?;
    nehemiah_7(story)?;
    nehemiah_8(story)?;
    nehemiah_9(story)?;
    nehemiah_10(story)?;
    nehemiah_11(story)?;
    nehemiah_12(story)?;
    nehemiah_13(story)?;
    Ok(())
}

fn nehemiah_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 1, "Nehemiah hears that Jerusalem's wall is broken, sits down in grief, fasts, and confesses the nation's long disobedience.", "His prayer remembers the promise of gathering scattered people, then asks for favor before the king who controls his next move.");
    nehemiah_1_group_01(story)?;
    nehemiah_1_group_02(story)?;
    nehemiah_1_group_03(story)?;
    Ok(())
}

fn nehemiah_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 2, "Artaxerxes grants Nehemiah leave, letters, timber, and an escort, while enemies react with alarm to the rebuilding mission.", "Nehemiah surveys the ruined wall at night and calls the people to rise, framing restoration as a shared construction commit.");
    nehemiah_2_group_01(story)?;
    nehemiah_2_group_02(story)?;
    nehemiah_2_group_03(story)?;
    nehemiah_2_group_04(story)?;
    nehemiah_2_group_05(story)?;
    Ok(())
}

fn nehemiah_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 3, "Priests, goldsmiths, merchants, rulers, and families repair assigned wall sections beside one another.", "The chapter's detailed work log honors small gates and short stretches, proving that a city is restored by many adjacent acts of ownership.");
    nehemiah_3_group_01(story)?;
    nehemiah_3_group_02(story)?;
    nehemiah_3_group_03(story)?;
    nehemiah_3_group_04(story)?;
    nehemiah_3_group_05(story)?;
    nehemiah_3_group_06(story)?;
    nehemiah_3_group_07(story)?;
    nehemiah_3_group_08(story)?;
    Ok(())
}

fn nehemiah_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 4, "Mockery becomes a threat, so the builders pray, set guards, and work with one hand while holding a weapon in the other.", "The people keep listening for the trumpet, ready to converge wherever the wall comes under attack.");
    nehemiah_4_group_01(story)?;
    nehemiah_4_group_02(story)?;
    nehemiah_4_group_03(story)?;
    nehemiah_4_group_04(story)?;
    nehemiah_4_group_05(story)?;
    nehemiah_4_group_06(story)?;
    Ok(())
}

fn nehemiah_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 5, "Poor families cry out over debt, mortgage, and food shortages while nobles profit from the crisis.", "Nehemiah rebukes the lenders, cancels interest claims, restores fields, and refuses the governor's allowance to model clean leadership.");
    nehemiah_5_group_01(story)?;
    nehemiah_5_group_02(story)?;
    nehemiah_5_group_03(story)?;
    nehemiah_5_group_04(story)?;
    nehemiah_5_group_05(story)?;
    Ok(())
}

fn nehemiah_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 6, "Sanballat and Geshem invite Nehemiah to a valley meeting, but he recognizes the distraction and repeats that he is doing a great work.", "False prophecy and a forged letter fail to frighten him, and the wall finishes in fifty-two days despite the rumor campaign.");
    nehemiah_6_group_01(story)?;
    nehemiah_6_group_02(story)?;
    nehemiah_6_group_03(story)?;
    nehemiah_6_group_04(story)?;
    nehemiah_6_group_05(story)?;
    Ok(())
}

fn nehemiah_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 7, "Nehemiah appoints Hanani and Hananiah to guard Jerusalem, then opens the genealogical registry of the first return.", "The population roll echoes Ezra's list, turning civic security and ancestral memory into the foundation for the next phase.");
    nehemiah_7_group_01(story)?;
    nehemiah_7_group_02(story)?;
    nehemiah_7_group_03(story)?;
    nehemiah_7_group_04(story)?;
    nehemiah_7_group_05(story)?;
    nehemiah_7_group_06(story)?;
    nehemiah_7_group_07(story)?;
    nehemiah_7_group_08(story)?;
    nehemiah_7_group_09(story)?;
    nehemiah_7_group_10(story)?;
    nehemiah_7_group_11(story)?;
    nehemiah_7_group_12(story)?;
    nehemiah_7_group_13(story)?;
    nehemiah_7_group_14(story)?;
    nehemiah_7_group_15(story)?;
    nehemiah_7_group_16(story)?;
    nehemiah_7_group_17(story)?;
    nehemiah_7_group_18(story)?;
    Ok(())
}

fn nehemiah_8(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 8, "Ezra reads the law from a raised platform, and the people listen for hours until they weep at what they hear.", "Levites explain the text, Nehemiah redirects grief toward joy, and the community keeps the Feast of Booths with renewed understanding.");
    nehemiah_8_group_01(story)?;
    nehemiah_8_group_02(story)?;
    nehemiah_8_group_03(story)?;
    nehemiah_8_group_04(story)?;
    nehemiah_8_group_05(story)?;
    Ok(())
}

fn nehemiah_9(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 9, "The people fast, separate from foreign influences, and recite a long history of creation, exodus, rebellion, patience, and exile.", "The prayer calls God righteous and the nation stubborn, then asks for a covenant document that records the new commitment.");
    nehemiah_9_group_01(story)?;
    nehemiah_9_group_02(story)?;
    nehemiah_9_group_03(story)?;
    nehemiah_9_group_04(story)?;
    nehemiah_9_group_05(story)?;
    nehemiah_9_group_06(story)?;
    nehemiah_9_group_07(story)?;
    nehemiah_9_group_08(story)?;
    nehemiah_9_group_09(story)?;
    nehemiah_9_group_10(story)?;
    Ok(())
}

fn nehemiah_10(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 10, "Leaders, priests, Levites, and households seal the covenant with specific promises about marriage, Sabbath commerce, offerings, and temple service.", "The reform becomes contractual, because a public return needs more than emotion; it needs repeatable practices.");
    nehemiah_10_group_01(story)?;
    nehemiah_10_group_02(story)?;
    nehemiah_10_group_03(story)?;
    nehemiah_10_group_04(story)?;
    nehemiah_10_group_05(story)?;
    nehemiah_10_group_06(story)?;
    nehemiah_10_group_07(story)?;
    nehemiah_10_group_08(story)?;
    nehemiah_10_group_09(story)?;
    nehemiah_10_group_10(story)?;
    Ok(())
}

fn nehemiah_11(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 11, "Leaders live in Jerusalem, and lots select one tenth of the wider population to relocate into the holy city.", "The registry names priests, Levites, gatekeepers, and singers, giving the rebuilt capital a population and service schedule.");
    nehemiah_11_group_01(story)?;
    nehemiah_11_group_02(story)?;
    nehemiah_11_group_03(story)?;
    nehemiah_11_group_04(story)?;
    nehemiah_11_group_05(story)?;
    nehemiah_11_group_06(story)?;
    nehemiah_11_group_07(story)?;
    nehemiah_11_group_08(story)?;
    nehemiah_11_group_09(story)?;
    Ok(())
}

fn nehemiah_12(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 12, "Priestly and Levitical genealogies are recorded, then two great thanksgiving choirs walk the wall in opposite directions.", "The dedication fills Jerusalem with joy, singers, and appointed contributions, turning masonry into a public celebration.");
    nehemiah_12_group_01(story)?;
    nehemiah_12_group_02(story)?;
    nehemiah_12_group_03(story)?;
    nehemiah_12_group_04(story)?;
    nehemiah_12_group_05(story)?;
    nehemiah_12_group_06(story)?;
    nehemiah_12_group_07(story)?;
    nehemiah_12_group_08(story)?;
    nehemiah_12_group_09(story)?;
    nehemiah_12_group_10(story)?;
    nehemiah_12_group_11(story)?;
    nehemiah_12_group_12(story)?;
    Ok(())
}

fn nehemiah_13(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Nehemiah", 13, "Nehemiah returns to find temple rooms misused, Levites unpaid, Sabbath commerce active, and mixed marriages reintroduced.", "He cleans the rooms, restores offerings, closes the gates, and rebukes the offenders, ending with a final plea to be remembered for faithful maintenance.");
    nehemiah_13_group_01(story)?;
    nehemiah_13_group_02(story)?;
    nehemiah_13_group_03(story)?;
    nehemiah_13_group_04(story)?;
    nehemiah_13_group_05(story)?;
    nehemiah_13_group_06(story)?;
    nehemiah_13_group_07(story)?;
    nehemiah_13_group_08(story)?;
    Ok(())
}

fn nehemiah_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        1,
        1,
        4,
        "words move_value -> call words nehemiah son",
        &["words", "nehemiah", "son", "hachaliah", "pass"],
    );
    segment.move_value(
        1,
        "words",
        &["nehemiah", "son", "hachaliah"],
        &["words", "nehemiah", "son", "hachaliah", "pass"],
    )?;
    segment.move_value(
        2,
        "men",
        &["hanani", "brethren", "certain"],
        &["men", "hanani", "brethren", "certain", "judah"],
    )?;
    segment.call(
        3,
        "unto",
        &["remnant", "left", "captivity"],
        &["unto", "remnant", "left", "captivity", "province"],
    )?;
    segment.move_value(
        4,
        "pass",
        &["heard", "words", "sat"],
        &["pass", "heard", "words", "sat", "down"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        1,
        5,
        8,
        "lord call -> result -> guard lord beseech thee",
        &["lord", "beseech", "thee", "god", "heaven"],
    );
    segment.call(
        5,
        "lord",
        &["beseech", "thee", "god"],
        &["lord", "beseech", "thee", "god", "heaven"],
    )?;
    segment.result(
        6,
        "thine",
        &["ear", "now", "attentive"],
        &["thine", "ear", "now", "attentive", "eyes"],
    )?;
    segment.guard(
        7,
        "dealt",
        &["very", "corruptly", "against"],
        &["dealt", "very", "corruptly", "against", "thee"],
    )?;
    segment.call(
        8,
        "moses",
        &["remember", "beseech", "thee"],
        &["moses", "remember", "beseech", "thee", "word"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        1,
        9,
        11,
        "turn move_value -> expression -> result turn unto keep",
        &["turn", "unto", "keep", "commandments", "though"],
    );
    segment.move_value(
        9,
        "turn",
        &["unto", "keep", "commandments"],
        &["turn", "unto", "keep", "commandments", "though"],
    )?;
    segment.expression(
        10,
        "people",
        &["now", "servants", "hast"],
        &["people", "now", "servants", "hast", "redeemed"],
    )?;
    segment.result(
        11,
        "lord",
        &["beseech", "thee", "now"],
        &["lord", "beseech", "thee", "now", "thine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        2,
        1,
        4,
        "king move_value -> guard -> construct king pass month",
        &["king", "pass", "month", "nisan", "twentieth"],
    );
    segment.move_value(
        1,
        "king",
        &["pass", "month", "nisan"],
        &["king", "pass", "month", "nisan", "twentieth"],
    )?;
    segment.guard(
        2,
        "king",
        &["wherefore", "unto", "why"],
        &["king", "wherefore", "unto", "why", "countenance"],
    )?;
    segment.guard(
        3,
        "king",
        &["unto", "live", "ever"],
        &["king", "unto", "live", "ever", "why"],
    )?;
    segment.construct(
        4,
        "king",
        &["unto", "dost", "make"],
        &["king", "unto", "dost", "make", "request"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        2,
        5,
        8,
        "king construct -> move_value king unto please",
        &["king", "unto", "please", "servant", "found"],
    );
    segment.construct(
        5,
        "king",
        &["unto", "please", "servant"],
        &["king", "unto", "please", "servant", "found"],
    )?;
    segment.move_value(
        6,
        "king",
        &["unto", "queen", "sitting"],
        &["king", "unto", "queen", "sitting", "long"],
    )?;
    segment.move_value(
        7,
        "king",
        &["moreover", "unto", "please"],
        &["king", "moreover", "unto", "please", "letters"],
    )?;
    segment.construct(
        8,
        "letter",
        &["unto", "asaph", "keeper"],
        &["letter", "unto", "asaph", "keeper", "king's"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        2,
        9,
        12,
        "king move_value -> reinitialize king governors beyond",
        &["king", "governors", "beyond", "river", "gave"],
    );
    segment.move_value(
        9,
        "king",
        &["governors", "beyond", "river"],
        &["king", "governors", "beyond", "river", "gave"],
    )?;
    segment.move_value(
        10,
        "sanballat",
        &["horonite", "tobiah", "servant"],
        &["sanballat", "horonite", "tobiah", "servant", "ammonite"],
    )?;
    segment.move_value(
        11,
        "jerusalem",
        &["three", "days"],
        &["jerusalem", "three", "days"],
    )?;
    segment.reinitialize(
        12,
        "men",
        &["arose", "night", "few"],
        &["men", "arose", "night", "few", "neither"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        2,
        13,
        16,
        "went move_value went night gate",
        &["went", "night", "gate", "valley", "even"],
    );
    segment.move_value(
        13,
        "went",
        &["night", "gate", "valley"],
        &["went", "night", "gate", "valley", "even"],
    )?;
    segment.move_value(
        14,
        "went",
        &["gate", "fountain", "king's"],
        &["went", "gate", "fountain", "king's", "pool"],
    )?;
    segment.move_value(
        15,
        "went",
        &["night", "brook", "viewed"],
        &["went", "night", "brook", "viewed", "wall"],
    )?;
    segment.move_value(
        16,
        "rulers",
        &["knew", "not", "whither"],
        &["rulers", "knew", "not", "whither", "went"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        2,
        17,
        20,
        "unto construct -> call unto see distress",
        &["unto", "see", "distress", "jerusalem", "lieth"],
    );
    segment.construct(
        17,
        "unto",
        &["see", "distress", "jerusalem"],
        &["unto", "see", "distress", "jerusalem", "lieth"],
    )?;
    segment.construct(
        18,
        "god",
        &["told", "hand", "good"],
        &["god", "told", "hand", "good", "king's"],
    )?;
    segment.call(
        19,
        "sanballat",
        &["horonite", "tobiah", "servant"],
        &["sanballat", "horonite", "tobiah", "servant", "ammonite"],
    )?;
    segment.construct(
        20,
        "god",
        &["answered", "unto", "heaven"],
        &["god", "answered", "unto", "heaven", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        1,
        4,
        "priest expression -> construct priest eliashib high",
        &["priest", "eliashib", "high", "rose", "brethren"],
    );
    segment.expression(
        1,
        "priest",
        &["eliashib", "high", "rose"],
        &["priest", "eliashib", "high", "rose", "brethren"],
    )?;
    segment.expression(
        2,
        "men",
        &["next", "unto", "builded"],
        &["men", "next", "unto", "builded", "jericho"],
    )?;
    segment.construct(
        3,
        "fish",
        &["gate", "sons", "hassenaah"],
        &["fish", "gate", "sons", "hassenaah", "build"],
    )?;
    segment.expression(
        4,
        "next",
        &["unto", "repaired", "meremoth"],
        &["next", "unto", "repaired", "meremoth", "son"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        5,
        8,
        "next guard -> expression next unto tekoites",
        &["next", "unto", "tekoites", "repaired", "nobles"],
    );
    segment.guard(
        5,
        "next",
        &["unto", "tekoites", "repaired"],
        &["next", "unto", "tekoites", "repaired", "nobles"],
    )?;
    segment.expression(
        6,
        "moreover",
        &["old", "gate", "repaired"],
        &["moreover", "old", "gate", "repaired", "jehoiada"],
    )?;
    segment.expression(
        7,
        "men",
        &["next", "unto", "repaired"],
        &["men", "next", "unto", "repaired", "melatiah"],
    )?;
    segment.expression(
        8,
        "next",
        &["unto", "repaired", "uzziel"],
        &["next", "unto", "repaired", "uzziel", "son"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        9,
        12,
        "next expression next unto repaired",
        &["next", "unto", "repaired", "rephaiah", "son"],
    );
    segment.expression(
        9,
        "next",
        &["unto", "repaired", "rephaiah"],
        &["next", "unto", "repaired", "rephaiah", "son"],
    )?;
    segment.expression(
        10,
        "next",
        &["unto", "repaired", "jedaiah"],
        &["next", "unto", "repaired", "jedaiah", "son"],
    )?;
    segment.expression(
        11,
        "malchijah",
        &["son", "harim", "hashub"],
        &["malchijah", "son", "harim", "hashub", "pahathmoab"],
    )?;
    segment.expression(
        12,
        "next",
        &["unto", "repaired", "shallum"],
        &["next", "unto", "repaired", "shallum", "son"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        13,
        16,
        "valley construct valley gate repaired",
        &["valley", "gate", "repaired", "hanun", "inhabitants"],
    );
    segment.construct(
        13,
        "valley",
        &["gate", "repaired", "hanun"],
        &["valley", "gate", "repaired", "hanun", "inhabitants"],
    )?;
    segment.construct(
        14,
        "dung",
        &["gate", "repaired", "malchiah"],
        &["dung", "gate", "repaired", "malchiah", "son"],
    )?;
    segment.construct(
        15,
        "gate",
        &["fountain", "repaired", "shallun"],
        &["gate", "fountain", "repaired", "shallun", "son"],
    )?;
    segment.construct(
        16,
        "repaired",
        &["nehemiah", "son", "azbuk"],
        &["repaired", "nehemiah", "son", "azbuk", "ruler"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        17,
        20,
        "repaired expression repaired levites rehum",
        &["repaired", "levites", "rehum", "son", "bani"],
    );
    segment.expression(
        17,
        "repaired",
        &["levites", "rehum", "son"],
        &["repaired", "levites", "rehum", "son", "bani"],
    )?;
    segment.expression(
        18,
        "repaired",
        &["brethren", "bavai", "son"],
        &["repaired", "brethren", "bavai", "son", "henadad"],
    )?;
    segment.expression(
        19,
        "next",
        &["repaired", "ezer", "son"],
        &["next", "repaired", "ezer", "son", "jeshua"],
    )?;
    segment.expression(
        20,
        "baruch",
        &["son", "zabbai", "earnestly"],
        &["baruch", "son", "zabbai", "earnestly", "repaired"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        21,
        24,
        "repaired expression repaired meremoth son",
        &["repaired", "meremoth", "son", "urijah", "koz"],
    );
    segment.expression(
        21,
        "repaired",
        &["meremoth", "son", "urijah"],
        &["repaired", "meremoth", "son", "urijah", "koz"],
    )?;
    segment.expression(
        22,
        "men",
        &["repaired", "priests", "plain"],
        &["men", "repaired", "priests", "plain"],
    )?;
    segment.expression(
        23,
        "repaired",
        &["benjamin", "hashub", "over"],
        &["repaired", "benjamin", "hashub", "over", "against"],
    )?;
    segment.expression(
        24,
        "repaired",
        &["binnui", "son", "henadad"],
        &["repaired", "binnui", "son", "henadad", "another"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        25,
        28,
        "palal expression -> iterate palal son uzai",
        &["palal", "son", "uzai", "over", "against"],
    );
    segment.expression(
        25,
        "palal",
        &["son", "uzai", "over"],
        &["palal", "son", "uzai", "over", "against"],
    )?;
    segment.expression(
        26,
        "moreover",
        &["nethinims", "dwelt", "ophel"],
        &["moreover", "nethinims", "dwelt", "ophel", "unto"],
    )?;
    segment.expression(
        27,
        "tekoites",
        &["repaired", "another", "piece"],
        &["tekoites", "repaired", "another", "piece", "over"],
    )?;
    segment.iterate(
        28,
        "above",
        &["horse", "gate", "repaired"],
        &["above", "horse", "gate", "repaired", "priests"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_3_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        3,
        29,
        32,
        "repaired expression repaired zadok son",
        &["repaired", "zadok", "son", "immer", "over"],
    );
    segment.expression(
        29,
        "repaired",
        &["zadok", "son", "immer"],
        &["repaired", "zadok", "son", "immer", "over"],
    )?;
    segment.expression(
        30,
        "repaired",
        &["hananiah", "son", "shelemiah"],
        &["repaired", "hananiah", "son", "shelemiah", "hanun"],
    )?;
    segment.expression(
        31,
        "repaired",
        &["malchiah", "goldsmith's", "son"],
        &["repaired", "malchiah", "goldsmith's", "son", "unto"],
    )?;
    segment.expression(
        32,
        "between",
        &["going", "corner", "unto"],
        &["between", "going", "corner", "unto", "sheep"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        4,
        1,
        4,
        "pass move_value -> construct -> expression pass sanballat heard",
        &["pass", "sanballat", "heard", "builded", "wall"],
    );
    segment.move_value(
        1,
        "pass",
        &["sanballat", "heard", "builded"],
        &["pass", "sanballat", "heard", "builded", "wall"],
    )?;
    segment.construct(
        2,
        "spake",
        &["brethren", "army", "samaria"],
        &["spake", "brethren", "army", "samaria", "feeble"],
    )?;
    segment.construct(
        3,
        "now",
        &["tobiah", "ammonite", "even"],
        &["now", "tobiah", "ammonite", "even", "build"],
    )?;
    segment.expression(
        4,
        "god",
        &["hear", "despised", "turn"],
        &["god", "hear", "despised", "turn", "reproach"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        4,
        5,
        8,
        "cover guard -> construct -> move_value cover not iniquity",
        &["cover", "not", "iniquity", "sin", "blotted"],
    );
    segment.guard(
        5,
        "cover",
        &["not", "iniquity", "sin"],
        &["cover", "not", "iniquity", "sin", "blotted"],
    )?;
    segment.construct(
        6,
        "people",
        &["built", "wall", "joined"],
        &["people", "built", "wall", "joined", "together"],
    )?;
    segment.construct(
        7,
        "pass",
        &["sanballat", "tobiah", "arabians"],
        &["pass", "sanballat", "tobiah", "arabians", "ammonites"],
    )?;
    segment.move_value(
        8,
        "conspired",
        &["together", "fight", "against"],
        &["conspired", "together", "fight", "against", "jerusalem"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        4,
        9,
        12,
        "god construct -> move_value god nevertheless made",
        &["god", "nevertheless", "made", "prayer", "unto"],
    );
    segment.construct(
        9,
        "god",
        &["nevertheless", "made", "prayer"],
        &["god", "nevertheless", "made", "prayer", "unto"],
    )?;
    segment.construct(
        10,
        "judah",
        &["strength", "bearers", "burdens"],
        &["judah", "strength", "bearers", "burdens", "decayed"],
    )?;
    segment.move_value(
        11,
        "adversaries",
        &["shall", "not", "know"],
        &["adversaries", "shall", "not", "know", "neither"],
    )?;
    segment.move_value(
        12,
        "pass",
        &["jews", "dwelt", "unto"],
        &["pass", "jews", "dwelt", "unto", "ten"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        4,
        13,
        16,
        "therefore expression -> guard -> move_value therefore set lower",
        &["therefore", "set", "lower", "places", "behind"],
    );
    segment.expression(
        13,
        "therefore",
        &["set", "lower", "places"],
        &["therefore", "set", "lower", "places", "behind"],
    )?;
    segment.guard(
        14,
        "people",
        &["looked", "rose", "unto"],
        &["people", "looked", "rose", "unto", "nobles"],
    )?;
    segment.move_value(
        15,
        "god",
        &["pass", "enemies", "heard"],
        &["god", "pass", "enemies", "heard", "known"],
    )?;
    segment.move_value(
        16,
        "pass",
        &["time", "forth", "half"],
        &["pass", "time", "forth", "half", "servants"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        4,
        17,
        20,
        "builded iterate -> call -> expression builded wall bare",
        &["builded", "wall", "bare", "burdens", "laded"],
    );
    segment.iterate(
        17,
        "builded",
        &["wall", "bare", "burdens"],
        &["builded", "wall", "bare", "burdens", "laded"],
    )?;
    segment.iterate(
        18,
        "builders",
        &["every", "sword", "girded"],
        &["builders", "every", "sword", "girded", "side"],
    )?;
    segment.call(
        19,
        "people",
        &["unto", "nobles", "rulers"],
        &["people", "unto", "nobles", "rulers", "rest"],
    )?;
    segment.expression(
        20,
        "place",
        &["therefore", "hear", "sound"],
        &["place", "therefore", "hear", "sound", "trumpet"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        4,
        21,
        23,
        "laboured expression -> call -> guard laboured work half",
        &["laboured", "work", "half", "held", "spears"],
    );
    segment.expression(
        21,
        "laboured",
        &["work", "half", "held"],
        &["laboured", "work", "half", "held", "spears"],
    )?;
    segment.call(
        22,
        "people",
        &["likewise", "same", "time"],
        &["people", "likewise", "same", "time", "unto"],
    )?;
    segment.guard(
        23,
        "men",
        &["neither", "nor", "brethren"],
        &["men", "neither", "nor", "brethren", "servants"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        5,
        1,
        4,
        "people result -> call people great cry",
        &["people", "great", "cry", "wives", "against"],
    );
    segment.result(
        1,
        "people",
        &["great", "cry", "wives"],
        &["people", "great", "cry", "wives", "against"],
    )?;
    segment.call(
        2,
        "sons",
        &["daughters", "many", "therefore"],
        &["sons", "daughters", "many", "therefore", "take"],
    )?;
    segment.call(
        3,
        "mortgaged",
        &["lands", "vineyards", "houses"],
        &["mortgaged", "lands", "vineyards", "houses", "might"],
    )?;
    segment.call(
        4,
        "borrowed",
        &["money", "king's", "tribute"],
        &["borrowed", "money", "king's", "tribute", "lands"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        5,
        5,
        8,
        "children move_value -> result -> call children yet now",
        &["children", "yet", "now", "flesh", "brethren"],
    );
    segment.move_value(
        5,
        "children",
        &["yet", "now", "flesh"],
        &["children", "yet", "now", "flesh", "brethren"],
    )?;
    segment.result(
        6,
        "very",
        &["angry", "heard", "cry"],
        &["very", "angry", "heard", "cry", "words"],
    )?;
    segment.call(
        7,
        "consulted",
        &["myself", "rebuked", "nobles"],
        &["consulted", "myself", "rebuked", "nobles", "rulers"],
    )?;
    segment.call(
        8,
        "unto",
        &["ability", "redeemed", "brethren"],
        &["unto", "ability", "redeemed", "brethren", "jews"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        5,
        9,
        12,
        "god guard -> result -> reinitialize god not good",
        &["god", "not", "good", "ought", "walk"],
    );
    segment.guard(
        9,
        "god",
        &["not", "good", "ought"],
        &["god", "not", "good", "ought", "walk"],
    )?;
    segment.result(
        10,
        "likewise",
        &["brethren", "servants", "might"],
        &["likewise", "brethren", "servants", "might", "exact"],
    )?;
    segment.reinitialize(
        11,
        "restore",
        &["pray", "even", "day"],
        &["restore", "pray", "even", "day", "lands"],
    )?;
    segment.reinitialize(
        12,
        "will",
        &["restore", "require", "nothing"],
        &["will", "restore", "require", "nothing", "sayest"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        5,
        13,
        16,
        "god guard god shook lap",
        &["god", "shook", "lap", "shake", "every"],
    );
    segment.guard(
        13,
        "god",
        &["shook", "lap", "shake"],
        &["god", "shook", "lap", "shake", "every"],
    )?;
    segment.guard(
        14,
        "judah",
        &["moreover", "time", "appointed"],
        &["judah", "moreover", "time", "appointed", "governor"],
    )?;
    segment.guard(
        15,
        "people",
        &["former", "governors", "chargeable"],
        &["people", "former", "governors", "chargeable", "unto"],
    )?;
    segment.guard(
        16,
        "yea",
        &["continued", "work", "wall"],
        &["yea", "continued", "work", "wall", "neither"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        5,
        17,
        19,
        "moreover move_value -> guard -> expression moreover table hundred",
        &["moreover", "table", "hundred", "fifty", "jews"],
    );
    segment.move_value(
        17,
        "moreover",
        &["table", "hundred", "fifty"],
        &["moreover", "table", "hundred", "fifty", "jews"],
    )?;
    segment.guard(
        18,
        "now",
        &["prepared", "daily", "six"],
        &["now", "prepared", "daily", "six", "choice"],
    )?;
    segment.expression(
        19,
        "god",
        &["think", "good", "according"],
        &["god", "think", "good", "according", "done"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        6,
        1,
        4,
        "now move_value now pass sanballat",
        &["now", "pass", "sanballat", "tobiah", "geshem"],
    );
    segment.move_value(
        1,
        "now",
        &["pass", "sanballat", "tobiah"],
        &["now", "pass", "sanballat", "tobiah", "geshem"],
    )?;
    segment.move_value(
        2,
        "sanballat",
        &["geshem", "sent", "unto"],
        &["sanballat", "geshem", "sent", "unto", "saying"],
    )?;
    segment.move_value(
        3,
        "sent",
        &["messengers", "unto", "saying"],
        &["sent", "messengers", "unto", "saying", "doing"],
    )?;
    segment.move_value(
        4,
        "yet",
        &["sent", "unto", "four"],
        &["yet", "sent", "unto", "four", "times"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        6,
        5,
        8,
        "sent move_value -> expression sent sanballat servant",
        &["sent", "sanballat", "servant", "unto", "like"],
    );
    segment.move_value(
        5,
        "sent",
        &["sanballat", "servant", "unto"],
        &["sent", "sanballat", "servant", "unto", "like"],
    )?;
    segment.expression(
        6,
        "wherein",
        &["written", "reported", "among"],
        &["wherein", "written", "reported", "among", "heathen"],
    )?;
    segment.move_value(
        7,
        "king",
        &["hast", "appointed", "prophets"],
        &["king", "hast", "appointed", "prophets", "preach"],
    )?;
    segment.move_value(
        8,
        "sent",
        &["unto", "saying", "such"],
        &["sent", "unto", "saying", "such", "things"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        6,
        9,
        12,
        "made construct -> move_value made afraid saying",
        &["made", "afraid", "saying", "hands", "shall"],
    );
    segment.construct(
        9,
        "made",
        &["afraid", "saying", "hands"],
        &["made", "afraid", "saying", "hands", "shall"],
    )?;
    segment.move_value(
        10,
        "afterward",
        &["unto", "house", "shemaiah"],
        &["afterward", "unto", "house", "shemaiah", "son"],
    )?;
    segment.move_value(
        11,
        "man",
        &["such", "flee", "would"],
        &["man", "such", "flee", "would", "temple"],
    )?;
    segment.move_value(
        12,
        "god",
        &["perceived", "not", "sent"],
        &["god", "perceived", "not", "sent", "pronounced"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        6,
        13,
        16,
        "therefore panic -> expression -> move_value therefore hired afraid",
        &["therefore", "hired", "afraid", "sin", "might"],
    );
    segment.panic(
        13,
        "therefore",
        &["hired", "afraid", "sin"],
        &["therefore", "hired", "afraid", "sin", "might"],
    )?;
    segment.panic(
        14,
        "god",
        &["think", "tobiah", "sanballat"],
        &["god", "think", "tobiah", "sanballat", "according"],
    )?;
    segment.expression(
        15,
        "wall",
        &["finished", "twenty", "fifth"],
        &["wall", "finished", "twenty", "fifth", "day"],
    )?;
    segment.move_value(
        16,
        "pass",
        &["enemies", "heard", "thereof"],
        &["pass", "enemies", "heard", "thereof", "heathen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_6_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        6,
        17,
        19,
        "judah move_value -> guard judah moreover days",
        &["judah", "moreover", "days", "nobles", "sent"],
    );
    segment.move_value(
        17,
        "judah",
        &["moreover", "days", "nobles"],
        &["judah", "moreover", "days", "nobles", "sent"],
    )?;
    segment.guard(
        18,
        "judah",
        &["many", "sworn", "unto"],
        &["judah", "many", "sworn", "unto", "son"],
    )?;
    segment.move_value(
        19,
        "reported",
        &["good", "deeds", "uttered"],
        &["reported", "good", "deeds", "uttered", "words"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        1,
        4,
        "now construct -> expression -> guard now pass wall",
        &["now", "pass", "wall", "built", "set"],
    );
    segment.construct(
        1,
        "now",
        &["pass", "wall", "built"],
        &["now", "pass", "wall", "built", "set"],
    )?;
    segment.expression(
        2,
        "gave",
        &["brother", "hanani", "hananiah"],
        &["gave", "brother", "hanani", "hananiah", "ruler"],
    )?;
    segment.guard(
        3,
        "unto",
        &["not", "gates", "jerusalem"],
        &["unto", "not", "gates", "jerusalem", "opened"],
    )?;
    segment.guard(
        4,
        "people",
        &["now", "city", "large"],
        &["people", "now", "city", "large", "great"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        5,
        8,
        "god move_value -> expression god put mine",
        &["god", "put", "mine", "heart", "gather"],
    );
    segment.move_value(
        5,
        "god",
        &["put", "mine", "heart"],
        &["god", "put", "mine", "heart", "gather"],
    )?;
    segment.move_value(
        6,
        "children",
        &["province", "went", "captivity"],
        &["children", "province", "went", "captivity", "carried"],
    )?;
    segment.move_value(
        7,
        "zerubbabel",
        &["jeshua", "nehemiah", "azariah"],
        &["zerubbabel", "jeshua", "nehemiah", "azariah", "raamiah"],
    )?;
    segment.expression(
        8,
        "children",
        &["parosh", "two", "thousand"],
        &["children", "parosh", "two", "thousand", "hundred"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        9,
        12,
        "children expression children shephatiah three",
        &["children", "shephatiah", "three", "hundred", "seventy"],
    );
    segment.expression(
        9,
        "children",
        &["shephatiah", "three", "hundred"],
        &["children", "shephatiah", "three", "hundred", "seventy"],
    )?;
    segment.expression(
        10,
        "children",
        &["arah", "six", "hundred"],
        &["children", "arah", "six", "hundred", "fifty"],
    )?;
    segment.expression(
        11,
        "children",
        &["pahathmoab", "jeshua", "joab"],
        &["children", "pahathmoab", "jeshua", "joab", "two"],
    )?;
    segment.expression(
        12,
        "children",
        &["elam", "thousand", "two"],
        &["children", "elam", "thousand", "two", "hundred"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        13,
        16,
        "children expression children zattu eight",
        &["children", "zattu", "eight", "hundred", "forty"],
    );
    segment.expression(
        13,
        "children",
        &["zattu", "eight", "hundred"],
        &["children", "zattu", "eight", "hundred", "forty"],
    )?;
    segment.expression(
        14,
        "children",
        &["zaccai", "seven", "hundred"],
        &["children", "zaccai", "seven", "hundred", "threescore"],
    )?;
    segment.expression(
        15,
        "children",
        &["binnui", "six", "hundred"],
        &["children", "binnui", "six", "hundred", "forty"],
    )?;
    segment.expression(
        16,
        "children",
        &["bebai", "six", "hundred"],
        &["children", "bebai", "six", "hundred", "twenty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        17,
        20,
        "children expression children azgad two",
        &["children", "azgad", "two", "thousand", "three"],
    );
    segment.expression(
        17,
        "children",
        &["azgad", "two", "thousand"],
        &["children", "azgad", "two", "thousand", "three"],
    )?;
    segment.expression(
        18,
        "children",
        &["adonikam", "six", "hundred"],
        &["children", "adonikam", "six", "hundred", "threescore"],
    )?;
    segment.expression(
        19,
        "children",
        &["bigvai", "two", "thousand"],
        &["children", "bigvai", "two", "thousand", "threescore"],
    )?;
    segment.expression(
        20,
        "children",
        &["adin", "six", "hundred"],
        &["children", "adin", "six", "hundred", "fifty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        21,
        24,
        "children expression children ater hezekiah",
        &["children", "ater", "hezekiah", "ninety", "eight"],
    );
    segment.expression(
        21,
        "children",
        &["ater", "hezekiah", "ninety"],
        &["children", "ater", "hezekiah", "ninety", "eight"],
    )?;
    segment.expression(
        22,
        "children",
        &["hashum", "three", "hundred"],
        &["children", "hashum", "three", "hundred", "twenty"],
    )?;
    segment.expression(
        23,
        "children",
        &["bezai", "three", "hundred"],
        &["children", "bezai", "three", "hundred", "twenty"],
    )?;
    segment.expression(
        24,
        "children",
        &["hariph", "hundred", "twelve"],
        &["children", "hariph", "hundred", "twelve"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        25,
        28,
        "children expression children gibeon ninety",
        &["children", "gibeon", "ninety", "five", "men"],
    );
    segment.expression(
        25,
        "children",
        &["gibeon", "ninety", "five"],
        &["children", "gibeon", "ninety", "five"],
    )?;
    segment.expression(
        26,
        "men",
        &["bethlehem", "netophah", "hundred"],
        &["men", "bethlehem", "netophah", "hundred", "fourscore"],
    )?;
    segment.expression(
        27,
        "men",
        &["anathoth", "hundred", "twenty"],
        &["men", "anathoth", "hundred", "twenty", "eight"],
    )?;
    segment.expression(
        28,
        "men",
        &["bethazmaveth", "forty", "two"],
        &["men", "bethazmaveth", "forty", "two"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        29,
        32,
        "men expression men kirjathjearim chephirah",
        &["men", "kirjathjearim", "chephirah", "beeroth", "seven"],
    );
    segment.expression(
        29,
        "men",
        &["kirjathjearim", "chephirah", "beeroth"],
        &["men", "kirjathjearim", "chephirah", "beeroth", "seven"],
    )?;
    segment.expression(
        30,
        "men",
        &["ramah", "gaba", "six"],
        &["men", "ramah", "gaba", "six", "hundred"],
    )?;
    segment.expression(
        31,
        "men",
        &["michmas", "hundred", "twenty"],
        &["men", "michmas", "hundred", "twenty", "two"],
    )?;
    segment.expression(
        32,
        "men",
        &["bethel", "hundred", "twenty"],
        &["men", "bethel", "hundred", "twenty", "three"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        33,
        36,
        "men expression men other nebo",
        &["men", "other", "nebo", "fifty", "two"],
    );
    segment.expression(
        33,
        "men",
        &["other", "nebo", "fifty"],
        &["men", "other", "nebo", "fifty", "two"],
    )?;
    segment.expression(
        34,
        "children",
        &["other", "elam", "thousand"],
        &["children", "other", "elam", "thousand", "two"],
    )?;
    segment.expression(
        35,
        "children",
        &["harim", "three", "hundred"],
        &["children", "harim", "three", "hundred", "twenty"],
    )?;
    segment.expression(
        36,
        "children",
        &["jericho", "three", "hundred"],
        &["children", "jericho", "three", "hundred", "forty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        37,
        40,
        "children expression children lod hadid",
        &["children", "lod", "hadid", "ono", "seven"],
    );
    segment.expression(
        37,
        "children",
        &["lod", "hadid", "ono"],
        &["children", "lod", "hadid", "ono", "seven"],
    )?;
    segment.expression(
        38,
        "children",
        &["senaah", "three", "thousand"],
        &["children", "senaah", "three", "thousand", "nine"],
    )?;
    segment.expression(
        39,
        "children",
        &["priests", "jedaiah", "house"],
        &["children", "priests", "jedaiah", "house", "jeshua"],
    )?;
    segment.expression(
        40,
        "children",
        &["immer", "thousand", "fifty"],
        &["children", "immer", "thousand", "fifty", "two"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        41,
        44,
        "children expression children pashur thousand",
        &["children", "pashur", "thousand", "two", "hundred"],
    );
    segment.expression(
        41,
        "children",
        &["pashur", "thousand", "two"],
        &["children", "pashur", "thousand", "two", "hundred"],
    )?;
    segment.expression(
        42,
        "children",
        &["harim", "thousand", "seventeen"],
        &["children", "harim", "thousand", "seventeen"],
    )?;
    segment.expression(
        43,
        "children",
        &["levites", "jeshua", "kadmiel"],
        &["children", "levites", "jeshua", "kadmiel", "hodevah"],
    )?;
    segment.expression(
        44,
        "children",
        &["singers", "asaph", "hundred"],
        &["children", "singers", "asaph", "hundred", "forty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        45,
        48,
        "children expression children porters shallum",
        &["children", "porters", "shallum", "ater", "talmon"],
    );
    segment.expression(
        45,
        "children",
        &["porters", "shallum", "ater"],
        &["children", "porters", "shallum", "ater", "talmon"],
    )?;
    segment.expression(
        46,
        "children",
        &["nethinims", "ziha", "hashupha"],
        &["children", "nethinims", "ziha", "hashupha", "tabbaoth"],
    )?;
    segment.expression(
        47,
        "children",
        &["keros", "sia", "padon"],
        &["children", "keros", "sia", "padon"],
    )?;
    segment.expression(
        48,
        "children",
        &["lebana", "hagaba", "shalmai"],
        &["children", "lebana", "hagaba", "shalmai"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_13(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        49,
        52,
        "children expression children hanan giddel",
        &["children", "hanan", "giddel", "gahar", "reaiah"],
    );
    segment.expression(
        49,
        "children",
        &["hanan", "giddel", "gahar"],
        &["children", "hanan", "giddel", "gahar"],
    )?;
    segment.expression(
        50,
        "children",
        &["reaiah", "rezin", "nekoda"],
        &["children", "reaiah", "rezin", "nekoda"],
    )?;
    segment.expression(
        51,
        "children",
        &["gazzam", "uzza", "phaseah"],
        &["children", "gazzam", "uzza", "phaseah"],
    )?;
    segment.expression(
        52,
        "children",
        &["besai", "meunim", "nephishesim"],
        &["children", "besai", "meunim", "nephishesim"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_14(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        53,
        56,
        "children expression children bakbuk hakupha",
        &["children", "bakbuk", "hakupha", "harhur", "bazlith"],
    );
    segment.expression(
        53,
        "children",
        &["bakbuk", "hakupha", "harhur"],
        &["children", "bakbuk", "hakupha", "harhur"],
    )?;
    segment.expression(
        54,
        "children",
        &["bazlith", "mehida", "harsha"],
        &["children", "bazlith", "mehida", "harsha"],
    )?;
    segment.expression(
        55,
        "children",
        &["barkos", "sisera", "tamah"],
        &["children", "barkos", "sisera", "tamah"],
    )?;
    segment.expression(
        56,
        "children",
        &["neziah", "hatipha"],
        &["children", "neziah", "hatipha"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_15(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        57,
        60,
        "children expression children solomon's servants",
        &["children", "solomon's", "servants", "sotai", "sophereth"],
    );
    segment.expression(
        57,
        "children",
        &["solomon's", "servants", "sotai"],
        &["children", "solomon's", "servants", "sotai", "sophereth"],
    )?;
    segment.expression(
        58,
        "children",
        &["jaala", "darkon", "giddel"],
        &["children", "jaala", "darkon", "giddel"],
    )?;
    segment.expression(
        59,
        "children",
        &["shephatiah", "hattil", "pochereth"],
        &["children", "shephatiah", "hattil", "pochereth", "zebaim"],
    )?;
    segment.expression(
        60,
        "children",
        &["nethinims", "solomon's", "servants"],
        &["children", "nethinims", "solomon's", "servants", "three"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_16(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        61,
        64,
        "cherub move_value -> expression -> guard cherub went telmelah",
        &["cherub", "went", "telmelah", "telharesha", "addon"],
    );
    segment.move_value(
        61,
        "cherub",
        &["went", "telmelah", "telharesha"],
        &["cherub", "went", "telmelah", "telharesha", "addon"],
    )?;
    segment.expression(
        62,
        "children",
        &["delaiah", "tobiah", "nekoda"],
        &["children", "delaiah", "tobiah", "nekoda", "six"],
    )?;
    segment.expression(
        63,
        "children",
        &["priests", "habaiah", "koz"],
        &["children", "priests", "habaiah", "koz", "barzillai"],
    )?;
    segment.guard(
        64,
        "sought",
        &["register", "among", "reckoned"],
        &["sought", "register", "among", "reckoned", "genealogy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_17(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        65,
        68,
        "tirshatha guard -> expression tirshatha unto not",
        &["tirshatha", "unto", "not", "eat", "most"],
    );
    segment.guard(
        65,
        "tirshatha",
        &["unto", "not", "eat"],
        &["tirshatha", "unto", "not", "eat", "most"],
    )?;
    segment.expression(
        66,
        "whole",
        &["congregation", "together", "forty"],
        &["whole", "congregation", "together", "forty", "two"],
    )?;
    segment.expression(
        67,
        "beside",
        &["manservants", "maidservants", "seven"],
        &["beside", "manservants", "maidservants", "seven", "thousand"],
    )?;
    segment.expression(
        68,
        "horses",
        &["seven", "hundred", "thirty"],
        &["horses", "seven", "hundred", "thirty", "six"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_7_group_18(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        7,
        69,
        73,
        "camels expression -> move_value camels four hundred",
        &["camels", "four", "hundred", "thirty", "five"],
    );
    segment.expression(
        69,
        "camels",
        &["four", "hundred", "thirty"],
        &["camels", "four", "hundred", "thirty", "five"],
    )?;
    segment.expression(
        70,
        "chief",
        &["fathers", "gave", "unto"],
        &["chief", "fathers", "gave", "unto", "work"],
    )?;
    segment.expression(
        71,
        "chief",
        &["fathers", "gave", "treasure"],
        &["chief", "fathers", "gave", "treasure", "work"],
    )?;
    segment.expression(
        72,
        "people",
        &["rest", "gave", "twenty"],
        &["people", "rest", "gave", "twenty", "thousand"],
    )?;
    segment.move_value(
        73,
        "people",
        &["priests", "levites", "porters"],
        &["people", "priests", "levites", "porters", "singers"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_8_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        8,
        1,
        4,
        "people move_value -> guard -> construct people gathered themselves",
        &["people", "gathered", "themselves", "together", "man"],
    );
    segment.move_value(
        1,
        "people",
        &["gathered", "themselves", "together"],
        &["people", "gathered", "themselves", "together", "man"],
    )?;
    segment.move_value(
        2,
        "priest",
        &["ezra", "brought", "law"],
        &["priest", "ezra", "brought", "law", "congregation"],
    )?;
    segment.guard(
        3,
        "read",
        &["therein", "street", "water"],
        &["read", "therein", "street", "water", "gate"],
    )?;
    segment.construct(
        4,
        "ezra",
        &["scribe", "stood", "pulpit"],
        &["ezra", "scribe", "stood", "pulpit", "wood"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_8_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        8,
        5,
        8,
        "people expression -> call -> guard people ezra opened",
        &["people", "ezra", "opened", "book", "sight"],
    );
    segment.expression(
        5,
        "people",
        &["ezra", "opened", "book"],
        &["people", "ezra", "opened", "book", "sight"],
    )?;
    segment.call(
        6,
        "lord",
        &["ezra", "blessed", "great"],
        &["lord", "ezra", "blessed", "great", "god"],
    )?;
    segment.guard(
        7,
        "jeshua",
        &["bani", "sherebiah", "jamin"],
        &["jeshua", "bani", "sherebiah", "jamin", "akkub"],
    )?;
    segment.guard(
        8,
        "god",
        &["read", "book", "law"],
        &["god", "read", "book", "law", "distinctly"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_8_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        8,
        9,
        12,
        "priest guard -> move_value -> construct priest nehemiah tirshatha",
        &["priest", "nehemiah", "tirshatha", "ezra", "scribe"],
    );
    segment.guard(
        9,
        "priest",
        &["nehemiah", "tirshatha", "ezra"],
        &["priest", "nehemiah", "tirshatha", "ezra", "scribe"],
    )?;
    segment.move_value(
        10,
        "unto",
        &["way", "eat", "fat"],
        &["unto", "way", "eat", "fat", "drink"],
    )?;
    segment.guard(
        11,
        "people",
        &["levites", "stilled", "saying"],
        &["people", "levites", "stilled", "saying", "hold"],
    )?;
    segment.construct(
        12,
        "people",
        &["went", "way", "eat"],
        &["people", "went", "way", "eat", "drink"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_8_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        8,
        13,
        15,
        "people guard -> construct people second day",
        &["people", "second", "day", "gathered", "together"],
    );
    segment.guard(
        13,
        "people",
        &["second", "day", "gathered"],
        &["people", "second", "day", "gathered", "together"],
    )?;
    segment.guard(
        14,
        "lord",
        &["found", "written", "law"],
        &["lord", "found", "written", "law", "commanded"],
    )?;
    segment.construct(
        15,
        "publish",
        &["proclaim", "cities", "jerusalem"],
        &["publish", "proclaim", "cities", "jerusalem", "saying"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_8_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        8,
        16,
        18,
        "people construct -> guard people went forth",
        &["people", "went", "forth", "brought", "made"],
    );
    segment.construct(
        16,
        "people",
        &["went", "forth", "brought"],
        &["people", "went", "forth", "brought", "made"],
    )?;
    segment.construct(
        17,
        "congregation",
        &["again", "captivity", "made"],
        &["congregation", "again", "captivity", "made", "booths"],
    )?;
    segment.guard(
        18,
        "god",
        &["day", "first", "unto"],
        &["god", "day", "first", "unto", "last"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        1,
        4,
        "children expression -> guard -> result children now twenty",
        &["children", "now", "twenty", "fourth", "day"],
    );
    segment.expression(
        1,
        "children",
        &["now", "twenty", "fourth"],
        &["children", "now", "twenty", "fourth", "day"],
    )?;
    segment.expression(
        2,
        "israel",
        &["seed", "separated", "themselves"],
        &["israel", "seed", "separated", "themselves", "strangers"],
    )?;
    segment.guard(
        3,
        "lord",
        &["stood", "place", "read"],
        &["lord", "stood", "place", "read", "book"],
    )?;
    segment.result(
        4,
        "stood",
        &["stairs", "levites", "jeshua"],
        &["stood", "stairs", "levites", "jeshua", "bani"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        5,
        8,
        "levites call -> construct -> expression levites jeshua kadmiel",
        &["levites", "jeshua", "kadmiel", "bani", "hashabniah"],
    );
    segment.call(
        5,
        "levites",
        &["jeshua", "kadmiel", "bani"],
        &["levites", "jeshua", "kadmiel", "bani", "hashabniah"],
    )?;
    segment.construct(
        6,
        "lord",
        &["even", "art", "alone"],
        &["lord", "even", "art", "alone", "hast"],
    )?;
    segment.expression(
        7,
        "lord",
        &["art", "god", "didst"],
        &["lord", "art", "god", "didst", "choose"],
    )?;
    segment.call(
        8,
        "foundest",
        &["heart", "faithful", "thee"],
        &["foundest", "heart", "faithful", "thee", "madest"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        9,
        12,
        "didst result -> expression -> move_value didst see affliction",
        &["didst", "see", "affliction", "fathers", "egypt"],
    );
    segment.result(
        9,
        "didst",
        &["see", "affliction", "fathers"],
        &["didst", "see", "affliction", "fathers", "egypt"],
    )?;
    segment.expression(
        10,
        "pharaoh",
        &["shewedst", "signs", "wonders"],
        &["pharaoh", "shewedst", "signs", "wonders", "servants"],
    )?;
    segment.move_value(
        11,
        "didst",
        &["divide", "sea", "went"],
        &["didst", "divide", "sea", "went", "through"],
    )?;
    segment.move_value(
        12,
        "moreover",
        &["leddest", "day", "cloudy"],
        &["moreover", "leddest", "day", "cloudy", "pillar"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        13,
        16,
        "camest expression -> move_value -> guard camest down mount",
        &["camest", "down", "mount", "sinai", "spakest"],
    );
    segment.expression(
        13,
        "camest",
        &["down", "mount", "sinai"],
        &["camest", "down", "mount", "sinai", "spakest"],
    )?;
    segment.expression(
        14,
        "madest",
        &["known", "unto", "holy"],
        &["madest", "known", "unto", "holy", "sabbath"],
    )?;
    segment.move_value(
        15,
        "gavest",
        &["bread", "heaven", "hunger"],
        &["gavest", "bread", "heaven", "hunger", "broughtest"],
    )?;
    segment.guard(
        16,
        "fathers",
        &["dealt", "proudly", "hardened"],
        &["fathers", "dealt", "proudly", "hardened", "necks"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        17,
        20,
        "refused guard -> construct -> move_value refused obey neither",
        &["refused", "obey", "neither", "mindful", "wonders"],
    );
    segment.guard(
        17,
        "refused",
        &["obey", "neither", "mindful"],
        &["refused", "obey", "neither", "mindful", "wonders"],
    )?;
    segment.construct(
        18,
        "god",
        &["yea", "made", "molten"],
        &["god", "yea", "made", "molten", "calf"],
    )?;
    segment.move_value(
        19,
        "yet",
        &["manifold", "mercies", "forsookest"],
        &["yet", "manifold", "mercies", "forsookest", "not"],
    )?;
    segment.guard(
        20,
        "spirit",
        &["gavest", "good", "instruct"],
        &["spirit", "gavest", "good", "instruct", "withheldest"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        21,
        24,
        "yea guard -> expression -> move_value yea forty years",
        &["yea", "forty", "years", "didst", "sustain"],
    );
    segment.guard(
        21,
        "yea",
        &["forty", "years", "didst"],
        &["yea", "forty", "years", "didst", "sustain"],
    )?;
    segment.expression(
        22,
        "nations",
        &["moreover", "gavest", "kingdoms"],
        &["nations", "moreover", "gavest", "kingdoms", "didst"],
    )?;
    segment.move_value(
        23,
        "children",
        &["multipliedst", "stars", "heaven"],
        &["children", "multipliedst", "stars", "heaven", "broughtest"],
    )?;
    segment.move_value(
        24,
        "children",
        &["went", "possessed", "land"],
        &["children", "went", "possessed", "land", "subduedst"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        25,
        28,
        "took expression -> guard -> result took strong cities",
        &["took", "strong", "cities", "fat", "land"],
    );
    segment.expression(
        25,
        "took",
        &["strong", "cities", "fat"],
        &["took", "strong", "cities", "fat", "land"],
    )?;
    segment.guard(
        26,
        "nevertheless",
        &["disobedient", "rebelled", "against"],
        &["nevertheless", "disobedient", "rebelled", "against", "thee"],
    )?;
    segment.result(
        27,
        "therefore",
        &["deliveredst", "hand", "enemies"],
        &["therefore", "deliveredst", "hand", "enemies", "vexed"],
    )?;
    segment.result(
        28,
        "rest",
        &["evil", "again", "thee"],
        &["rest", "evil", "again", "thee", "therefore"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        29,
        32,
        "testifiedst move_value -> guard testifiedst against mightest",
        &["testifiedst", "against", "mightest", "bring", "again"],
    );
    segment.move_value(
        29,
        "testifiedst",
        &["against", "mightest", "bring"],
        &["testifiedst", "against", "mightest", "bring", "again"],
    )?;
    segment.guard(
        30,
        "spirit",
        &["yet", "many", "years"],
        &["spirit", "yet", "many", "years", "didst"],
    )?;
    segment.guard(
        31,
        "nevertheless",
        &["great", "mercies", "sake"],
        &["nevertheless", "great", "mercies", "sake", "didst"],
    )?;
    segment.move_value(
        32,
        "god",
        &["now", "therefore", "great"],
        &["god", "now", "therefore", "great", "mighty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        33,
        35,
        "howbeit move_value -> guard howbeit art just",
        &["howbeit", "art", "just", "brought", "hast"],
    );
    segment.move_value(
        33,
        "howbeit",
        &["art", "just", "brought"],
        &["howbeit", "art", "just", "brought", "hast"],
    )?;
    segment.guard(
        34,
        "neither",
        &["kings", "princes", "priests"],
        &["neither", "kings", "princes", "priests", "nor"],
    )?;
    segment.guard(
        35,
        "not",
        &["served", "thee", "kingdom"],
        &["not", "served", "thee", "kingdom", "great"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_9_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        9,
        36,
        38,
        "behold match_value -> expression -> construct behold servants day",
        &["behold", "servants", "day", "land", "gavest"],
    );
    segment.match_value(
        36,
        "behold",
        &["servants", "day", "land"],
        &["behold", "servants", "day", "land", "gavest"],
    )?;
    segment.expression(
        37,
        "yieldeth",
        &["much", "increase", "unto"],
        &["yieldeth", "much", "increase", "unto", "kings"],
    )?;
    segment.construct(
        38,
        "make",
        &["sure", "covenant", "write"],
        &["make", "sure", "covenant", "write", "princes"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        1,
        4,
        "now expression now sealed nehemiah",
        &["now", "sealed", "nehemiah", "tirshatha", "son"],
    );
    segment.expression(
        1,
        "now",
        &["sealed", "nehemiah", "tirshatha"],
        &["now", "sealed", "nehemiah", "tirshatha", "son"],
    )?;
    segment.expression(
        2,
        "seraiah",
        &["azariah", "jeremiah"],
        &["seraiah", "azariah", "jeremiah"],
    )?;
    segment.expression(
        3,
        "pashur",
        &["amariah", "malchijah"],
        &["pashur", "amariah", "malchijah"],
    )?;
    segment.expression(
        4,
        "hattush",
        &["shebaniah", "malluch"],
        &["hattush", "shebaniah", "malluch"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        5,
        8,
        "harim expression harim meremoth obadiah",
        &["harim", "meremoth", "obadiah", "daniel", "ginnethon"],
    );
    segment.expression(
        5,
        "harim",
        &["meremoth", "obadiah"],
        &["harim", "meremoth", "obadiah"],
    )?;
    segment.expression(
        6,
        "daniel",
        &["ginnethon", "baruch"],
        &["daniel", "ginnethon", "baruch"],
    )?;
    segment.expression(
        7,
        "meshullam",
        &["abijah", "mijamin"],
        &["meshullam", "abijah", "mijamin"],
    )?;
    segment.expression(
        8,
        "maaziah",
        &["bilgai", "shemaiah", "priests"],
        &["maaziah", "bilgai", "shemaiah", "priests"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        9,
        12,
        "levites iterate -> expression levites both jeshua",
        &["levites", "both", "jeshua", "son", "azaniah"],
    );
    segment.iterate(
        9,
        "levites",
        &["both", "jeshua", "son"],
        &["levites", "both", "jeshua", "son", "azaniah"],
    )?;
    segment.expression(
        10,
        "brethren",
        &["shebaniah", "hodijah", "kelita"],
        &["brethren", "shebaniah", "hodijah", "kelita", "pelaiah"],
    )?;
    segment.expression(
        11,
        "micha",
        &["rehob", "hashabiah"],
        &["micha", "rehob", "hashabiah"],
    )?;
    segment.expression(
        12,
        "zaccur",
        &["sherebiah", "shebaniah"],
        &["zaccur", "sherebiah", "shebaniah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        13,
        16,
        "hodijah expression hodijah bani beninu",
        &["hodijah", "bani", "beninu", "people", "chief"],
    );
    segment.expression(
        13,
        "hodijah",
        &["bani", "beninu"],
        &["hodijah", "bani", "beninu"],
    )?;
    segment.expression(
        14,
        "people",
        &["chief", "parosh", "pahathmoab"],
        &["people", "chief", "parosh", "pahathmoab", "elam"],
    )?;
    segment.expression(
        15,
        "bunni",
        &["azgad", "bebai"],
        &["bunni", "azgad", "bebai"],
    )?;
    segment.expression(
        16,
        "adonijah",
        &["bigvai", "adin"],
        &["adonijah", "bigvai", "adin"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        17,
        20,
        "ater expression ater hizkijah azzur",
        &["ater", "hizkijah", "azzur", "hodijah", "hashum"],
    );
    segment.expression(
        17,
        "ater",
        &["hizkijah", "azzur"],
        &["ater", "hizkijah", "azzur"],
    )?;
    segment.expression(
        18,
        "hodijah",
        &["hashum", "bezai"],
        &["hodijah", "hashum", "bezai"],
    )?;
    segment.expression(
        19,
        "hariph",
        &["anathoth", "nebai"],
        &["hariph", "anathoth", "nebai"],
    )?;
    segment.expression(
        20,
        "magpiash",
        &["meshullam", "hezir"],
        &["magpiash", "meshullam", "hezir"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        21,
        24,
        "meshezabeel expression meshezabeel zadok jaddua",
        &["meshezabeel", "zadok", "jaddua", "pelatiah", "hanan"],
    );
    segment.expression(
        21,
        "meshezabeel",
        &["zadok", "jaddua"],
        &["meshezabeel", "zadok", "jaddua"],
    )?;
    segment.expression(
        22,
        "pelatiah",
        &["hanan", "anaiah"],
        &["pelatiah", "hanan", "anaiah"],
    )?;
    segment.expression(
        23,
        "hoshea",
        &["hananiah", "hashub"],
        &["hoshea", "hananiah", "hashub"],
    )?;
    segment.expression(
        24,
        "hallohesh",
        &["pileha", "shobek"],
        &["hallohesh", "pileha", "shobek"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        25,
        28,
        "rehum expression -> guard rehum hashabnah maaseiah",
        &["rehum", "hashabnah", "maaseiah", "ahijah", "hanan"],
    );
    segment.expression(
        25,
        "rehum",
        &["hashabnah", "maaseiah"],
        &["rehum", "hashabnah", "maaseiah"],
    )?;
    segment.expression(
        26,
        "ahijah",
        &["hanan", "anan"],
        &["ahijah", "hanan", "anan"],
    )?;
    segment.expression(
        27,
        "malluch",
        &["harim", "baanah"],
        &["malluch", "harim", "baanah"],
    )?;
    segment.guard(
        28,
        "people",
        &["rest", "priests", "levites"],
        &["people", "rest", "priests", "levites", "porters"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        29,
        32,
        "clave guard -> move_value -> construct clave brethren nobles",
        &["clave", "brethren", "nobles", "entered", "curse"],
    );
    segment.guard(
        29,
        "clave",
        &["brethren", "nobles", "entered"],
        &["clave", "brethren", "nobles", "entered", "curse"],
    )?;
    segment.guard(
        30,
        "people",
        &["would", "not", "give"],
        &["people", "would", "not", "give", "daughters"],
    )?;
    segment.move_value(
        31,
        "people",
        &["land", "bring", "ware"],
        &["people", "land", "bring", "ware", "any"],
    )?;
    segment.construct(
        32,
        "made",
        &["ordinances", "charge", "ourselves"],
        &["made", "ordinances", "charge", "ourselves", "yearly"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        33,
        36,
        "shewbread construct -> move_value shewbread continual meat",
        &["shewbread", "continual", "meat", "offering", "burnt"],
    );
    segment.construct(
        33,
        "shewbread",
        &["continual", "meat", "offering"],
        &["shewbread", "continual", "meat", "offering", "burnt"],
    )?;
    segment.move_value(
        34,
        "people",
        &["cast", "lots", "among"],
        &["people", "cast", "lots", "among", "priests"],
    )?;
    segment.move_value(
        35,
        "bring",
        &["firstfruits", "ground", "fruit"],
        &["bring", "firstfruits", "ground", "fruit", "trees"],
    )?;
    segment.move_value(
        36,
        "firstborn",
        &["sons", "cattle", "written"],
        &["firstborn", "sons", "cattle", "written", "law"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_10_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        10,
        37,
        39,
        "bring move_value bring firstfruits dough",
        &["bring", "firstfruits", "dough", "offerings", "fruit"],
    );
    segment.move_value(
        37,
        "bring",
        &["firstfruits", "dough", "offerings"],
        &["bring", "firstfruits", "dough", "offerings", "fruit"],
    )?;
    segment.move_value(
        38,
        "priest",
        &["son", "aaron", "shall"],
        &["priest", "son", "aaron", "shall", "levites"],
    )?;
    segment.move_value(
        39,
        "children",
        &["israel", "levi", "shall"],
        &["children", "israel", "levi", "shall", "bring"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        1,
        4,
        "people move_value -> trait_bound -> iterate -> expression people rulers dwelt",
        &["people", "rulers", "dwelt", "jerusalem", "rest"],
    );
    segment.move_value(
        1,
        "people",
        &["rulers", "dwelt", "jerusalem"],
        &["people", "rulers", "dwelt", "jerusalem", "rest"],
    )?;
    segment.trait_bound(
        2,
        "people",
        &["blessed", "men", "willingly"],
        &["people", "blessed", "men", "willingly", "offered"],
    )?;
    segment.iterate(
        3,
        "judah",
        &["now", "chief", "province"],
        &["judah", "now", "chief", "province", "dwelt"],
    )?;
    segment.expression(
        4,
        "children",
        &["jerusalem", "dwelt", "certain"],
        &["children", "jerusalem", "dwelt", "certain", "judah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        5,
        8,
        "maaseiah expression -> iterate maaseiah son baruch",
        &["maaseiah", "son", "baruch", "colhozeh", "hazaiah"],
    );
    segment.expression(
        5,
        "maaseiah",
        &["son", "baruch", "colhozeh"],
        &["maaseiah", "son", "baruch", "colhozeh", "hazaiah"],
    )?;
    segment.iterate(
        6,
        "sons",
        &["perez", "dwelt", "jerusalem"],
        &["sons", "perez", "dwelt", "jerusalem", "four"],
    )?;
    segment.iterate(
        7,
        "sons",
        &["benjamin", "sallu", "son"],
        &["sons", "benjamin", "sallu", "son", "meshullam"],
    )?;
    segment.expression(
        8,
        "gabbai",
        &["sallai", "nine", "hundred"],
        &["gabbai", "sallai", "nine", "hundred", "twenty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        9,
        12,
        "judah expression judah joel son",
        &["judah", "joel", "son", "zichri", "overseer"],
    );
    segment.expression(
        9,
        "judah",
        &["joel", "son", "zichri"],
        &["judah", "joel", "son", "zichri", "overseer"],
    )?;
    segment.expression(
        10,
        "priests",
        &["jedaiah", "son", "joiarib"],
        &["priests", "jedaiah", "son", "joiarib", "jachin"],
    )?;
    segment.expression(
        11,
        "seraiah",
        &["son", "hilkiah", "meshullam"],
        &["seraiah", "son", "hilkiah", "meshullam", "zadok"],
    )?;
    segment.expression(
        12,
        "brethren",
        &["work", "house", "eight"],
        &["brethren", "work", "house", "eight", "hundred"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        13,
        16,
        "brethren expression brethren chief fathers",
        &["brethren", "chief", "fathers", "two", "hundred"],
    );
    segment.expression(
        13,
        "brethren",
        &["chief", "fathers", "two"],
        &["brethren", "chief", "fathers", "two", "hundred"],
    )?;
    segment.expression(
        14,
        "men",
        &["brethren", "mighty", "valour"],
        &["men", "brethren", "mighty", "valour", "hundred"],
    )?;
    segment.expression(
        15,
        "levites",
        &["shemaiah", "son", "hashub"],
        &["levites", "shemaiah", "son", "hashub", "azrikam"],
    )?;
    segment.expression(
        16,
        "shabbethai",
        &["jozabad", "chief", "levites"],
        &["shabbethai", "jozabad", "chief", "levites", "oversight"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        17,
        20,
        "mattaniah result -> expression -> return_value mattaniah son micha",
        &["mattaniah", "son", "micha", "zabdi", "asaph"],
    );
    segment.result(
        17,
        "mattaniah",
        &["son", "micha", "zabdi"],
        &["mattaniah", "son", "micha", "zabdi", "asaph"],
    )?;
    segment.expression(
        18,
        "levites",
        &["holy", "city", "two"],
        &["levites", "holy", "city", "two", "hundred"],
    )?;
    segment.expression(
        19,
        "moreover",
        &["porters", "akkub", "talmon"],
        &["moreover", "porters", "akkub", "talmon", "brethren"],
    )?;
    segment.return_value(
        20,
        "israel",
        &["residue", "priests", "levites"],
        &["israel", "residue", "priests", "levites", "cities"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        21,
        24,
        "nethinims expression -> iterate -> guard nethinims dwelt ophel",
        &["nethinims", "dwelt", "ophel", "ziha", "gispa"],
    );
    segment.expression(
        21,
        "nethinims",
        &["dwelt", "ophel", "ziha"],
        &["nethinims", "dwelt", "ophel", "ziha", "gispa"],
    )?;
    segment.iterate(
        22,
        "overseer",
        &["levites", "jerusalem", "uzzi"],
        &["overseer", "levites", "jerusalem", "uzzi", "son"],
    )?;
    segment.guard(
        23,
        "king's",
        &["commandment", "concerning", "certain"],
        &["king's", "commandment", "concerning", "certain", "portion"],
    )?;
    segment.expression(
        24,
        "children",
        &["pethahiah", "son", "meshezabeel"],
        &["children", "pethahiah", "son", "meshezabeel", "zerah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        25,
        28,
        "children expression children villages fields",
        &["children", "villages", "fields", "judah", "dwelt"],
    );
    segment.expression(
        25,
        "children",
        &["villages", "fields", "judah"],
        &["children", "villages", "fields", "judah", "dwelt"],
    )?;
    segment.expression(
        26,
        "jeshua",
        &["moladah", "bethphelet"],
        &["jeshua", "moladah", "bethphelet"],
    )?;
    segment.expression(
        27,
        "hazarshual",
        &["beersheba", "villages", "thereof"],
        &["hazarshual", "beersheba", "villages", "thereof"],
    )?;
    segment.expression(
        28,
        "ziklag",
        &["mekonah", "villages", "thereof"],
        &["ziklag", "mekonah", "villages", "thereof"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        29,
        32,
        "enrimmon expression enrimmon zareah jarmuth",
        &["enrimmon", "zareah", "jarmuth", "zanoah", "adullam"],
    );
    segment.expression(
        29,
        "enrimmon",
        &["zareah", "jarmuth"],
        &["enrimmon", "zareah", "jarmuth"],
    )?;
    segment.expression(
        30,
        "zanoah",
        &["adullam", "villages", "lachish"],
        &["zanoah", "adullam", "villages", "lachish", "fields"],
    )?;
    segment.expression(
        31,
        "children",
        &["benjamin", "geba", "dwelt"],
        &["children", "benjamin", "geba", "dwelt", "michmash"],
    )?;
    segment.expression(
        32,
        "anathoth",
        &["nob", "ananiah"],
        &["anathoth", "nob", "ananiah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_11_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        11,
        33,
        36,
        "hazor expression hazor ramah gittaim",
        &["hazor", "ramah", "gittaim", "hadid", "zeboim"],
    );
    segment.expression(
        33,
        "hazor",
        &["ramah", "gittaim"],
        &["hazor", "ramah", "gittaim"],
    )?;
    segment.expression(
        34,
        "hadid",
        &["zeboim", "neballat"],
        &["hadid", "zeboim", "neballat"],
    )?;
    segment.expression(
        35,
        "lod",
        &["ono", "valley", "craftsmen"],
        &["lod", "ono", "valley", "craftsmen"],
    )?;
    segment.expression(
        36,
        "judah",
        &["levites", "divisions", "benjamin"],
        &["judah", "levites", "divisions", "benjamin"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        1,
        4,
        "now move_value -> expression now priests levites",
        &["now", "priests", "levites", "went", "zerubbabel"],
    );
    segment.move_value(
        1,
        "now",
        &["priests", "levites", "went"],
        &["now", "priests", "levites", "went", "zerubbabel"],
    )?;
    segment.expression(
        2,
        "amariah",
        &["malluch", "hattush"],
        &["amariah", "malluch", "hattush"],
    )?;
    segment.expression(
        3,
        "shechaniah",
        &["rehum", "meremoth"],
        &["shechaniah", "rehum", "meremoth"],
    )?;
    segment.expression(
        4,
        "iddo",
        &["ginnetho", "abijah"],
        &["iddo", "ginnetho", "abijah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        5,
        8,
        "miamin expression miamin maadiah bilgah",
        &["miamin", "maadiah", "bilgah", "shemaiah", "joiarib"],
    );
    segment.expression(
        5,
        "miamin",
        &["maadiah", "bilgah"],
        &["miamin", "maadiah", "bilgah"],
    )?;
    segment.expression(
        6,
        "shemaiah",
        &["joiarib", "jedaiah"],
        &["shemaiah", "joiarib", "jedaiah"],
    )?;
    segment.expression(
        7,
        "sallu",
        &["amok", "hilkiah", "jedaiah"],
        &["sallu", "amok", "hilkiah", "jedaiah", "chief"],
    )?;
    segment.expression(
        8,
        "judah",
        &["moreover", "levites", "jeshua"],
        &["judah", "moreover", "levites", "jeshua", "binnui"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        9,
        12,
        "bakbukiah expression -> construct bakbukiah unni brethren",
        &["bakbukiah", "unni", "brethren", "over", "against"],
    );
    segment.expression(
        9,
        "bakbukiah",
        &["unni", "brethren", "over"],
        &["bakbukiah", "unni", "brethren", "over", "against"],
    )?;
    segment.construct(
        10,
        "jeshua",
        &["begat", "joiakim", "eliashib"],
        &["jeshua", "begat", "joiakim", "eliashib", "joiada"],
    )?;
    segment.construct(
        11,
        "joiada",
        &["begat", "jonathan", "jaddua"],
        &["joiada", "begat", "jonathan", "jaddua"],
    )?;
    segment.expression(
        12,
        "days",
        &["joiakim", "priests", "chief"],
        &["days", "joiakim", "priests", "chief", "fathers"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        13,
        16,
        "ezra expression ezra meshullam amariah",
        &["ezra", "meshullam", "amariah", "jehohanan", "joseph"],
    );
    segment.expression(
        13,
        "ezra",
        &["meshullam", "amariah", "jehohanan"],
        &["ezra", "meshullam", "amariah", "jehohanan"],
    )?;
    segment.expression(
        14,
        "joseph",
        &["melicu", "jonathan", "shebaniah"],
        &["joseph", "melicu", "jonathan", "shebaniah"],
    )?;
    segment.expression(
        15,
        "harim",
        &["adna", "meraioth", "helkai"],
        &["harim", "adna", "meraioth", "helkai"],
    )?;
    segment.expression(
        16,
        "iddo",
        &["zechariah", "ginnethon", "meshullam"],
        &["iddo", "zechariah", "ginnethon", "meshullam"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        17,
        20,
        "abijah expression abijah zichri miniamin",
        &["abijah", "zichri", "miniamin", "moadiah", "piltai"],
    );
    segment.expression(
        17,
        "abijah",
        &["zichri", "miniamin", "moadiah"],
        &["abijah", "zichri", "miniamin", "moadiah", "piltai"],
    )?;
    segment.expression(
        18,
        "bilgah",
        &["shammua", "shemaiah", "jehonathan"],
        &["bilgah", "shammua", "shemaiah", "jehonathan"],
    )?;
    segment.expression(
        19,
        "joiarib",
        &["mattenai", "jedaiah", "uzzi"],
        &["joiarib", "mattenai", "jedaiah", "uzzi"],
    )?;
    segment.expression(
        20,
        "sallai",
        &["kallai", "amok", "eber"],
        &["sallai", "kallai", "amok", "eber"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        21,
        24,
        "hilkiah expression -> iterate -> guard hilkiah hashabiah jedaiah",
        &["hilkiah", "hashabiah", "jedaiah", "nethaneel", "levites"],
    );
    segment.expression(
        21,
        "hilkiah",
        &["hashabiah", "jedaiah", "nethaneel"],
        &["hilkiah", "hashabiah", "jedaiah", "nethaneel"],
    )?;
    segment.expression(
        22,
        "levites",
        &["days", "eliashib", "joiada"],
        &["levites", "days", "eliashib", "joiada", "johanan"],
    )?;
    segment.iterate(
        23,
        "sons",
        &["levi", "chief", "fathers"],
        &["sons", "levi", "chief", "fathers", "written"],
    )?;
    segment.guard(
        24,
        "chief",
        &["levites", "hashabiah", "sherebiah"],
        &["chief", "levites", "hashabiah", "sherebiah", "jeshua"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        25,
        28,
        "mattaniah expression -> move_value -> iterate mattaniah bakbukiah obadiah",
        &["mattaniah", "bakbukiah", "obadiah", "meshullam", "talmon"],
    );
    segment.expression(
        25,
        "mattaniah",
        &["bakbukiah", "obadiah", "meshullam"],
        &["mattaniah", "bakbukiah", "obadiah", "meshullam", "talmon"],
    )?;
    segment.expression(
        26,
        "days",
        &["joiakim", "son", "jeshua"],
        &["days", "joiakim", "son", "jeshua", "jozadak"],
    )?;
    segment.move_value(
        27,
        "dedication",
        &["wall", "jerusalem", "sought"],
        &["dedication", "wall", "jerusalem", "sought", "levites"],
    )?;
    segment.iterate(
        28,
        "sons",
        &["singers", "gathered", "themselves"],
        &["sons", "singers", "gathered", "themselves", "together"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        29,
        32,
        "house expression -> move_value house gilgal fields",
        &["house", "gilgal", "fields", "geba", "azmaveth"],
    );
    segment.expression(
        29,
        "house",
        &["gilgal", "fields", "geba"],
        &["house", "gilgal", "fields", "geba", "azmaveth"],
    )?;
    segment.expression(
        30,
        "people",
        &["priests", "levites", "purified"],
        &["people", "priests", "levites", "purified", "themselves"],
    )?;
    segment.move_value(
        31,
        "judah",
        &["brought", "princes", "wall"],
        &["judah", "brought", "princes", "wall", "appointed"],
    )?;
    segment.move_value(
        32,
        "judah",
        &["went", "hoshaiah", "half"],
        &["judah", "went", "hoshaiah", "half", "princes"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        33,
        36,
        "azariah expression -> iterate azariah ezra meshullam",
        &["azariah", "ezra", "meshullam", "judah", "benjamin"],
    );
    segment.expression(
        33,
        "azariah",
        &["ezra", "meshullam"],
        &["azariah", "ezra", "meshullam"],
    )?;
    segment.expression(
        34,
        "judah",
        &["benjamin", "shemaiah", "jeremiah"],
        &["judah", "benjamin", "shemaiah", "jeremiah"],
    )?;
    segment.iterate(
        35,
        "certain",
        &["priests", "sons", "trumpets"],
        &["certain", "priests", "sons", "trumpets", "namely"],
    )?;
    segment.expression(
        36,
        "judah",
        &["brethren", "shemaiah", "azarael"],
        &["judah", "brethren", "shemaiah", "azarael", "milalai"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        37,
        40,
        "david move_value -> expression david fountain gate",
        &["david", "fountain", "gate", "over", "against"],
    );
    segment.move_value(
        37,
        "david",
        &["fountain", "gate", "over"],
        &["david", "fountain", "gate", "over", "against"],
    )?;
    segment.move_value(
        38,
        "other",
        &["company", "gave", "thanks"],
        &["other", "company", "gave", "thanks", "went"],
    )?;
    segment.expression(
        39,
        "above",
        &["gate", "ephraim", "old"],
        &["above", "gate", "ephraim", "old", "fish"],
    )?;
    segment.expression(
        40,
        "god",
        &["stood", "two", "companies"],
        &["god", "stood", "two", "companies", "gave"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        41,
        44,
        "priests expression -> construct -> guard priests eliakim maaseiah",
        &["priests", "eliakim", "maaseiah", "miniamin", "michaiah"],
    );
    segment.expression(
        41,
        "priests",
        &["eliakim", "maaseiah", "miniamin"],
        &["priests", "eliakim", "maaseiah", "miniamin", "michaiah"],
    )?;
    segment.expression(
        42,
        "maaseiah",
        &["shemaiah", "eleazar", "uzzi"],
        &["maaseiah", "shemaiah", "eleazar", "uzzi", "jehohanan"],
    )?;
    segment.construct(
        43,
        "god",
        &["day", "offered", "great"],
        &["god", "day", "offered", "great", "sacrifices"],
    )?;
    segment.guard(
        44,
        "time",
        &["appointed", "over", "chambers"],
        &["time", "appointed", "over", "chambers", "treasures"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_12_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        12,
        45,
        47,
        "god guard -> trait_bound -> iterate god both singers",
        &["god", "both", "singers", "porters", "kept"],
    );
    segment.guard(
        45,
        "god",
        &["both", "singers", "porters"],
        &["god", "both", "singers", "porters", "kept"],
    )?;
    segment.trait_bound(
        46,
        "david",
        &["days", "asaph", "old"],
        &["david", "days", "asaph", "old", "chief"],
    )?;
    segment.iterate(
        47,
        "israel",
        &["days", "zerubbabel", "nehemiah"],
        &["israel", "days", "zerubbabel", "nehemiah", "gave"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        1,
        4,
        "moses move_value -> guard -> expression moses day read",
        &["moses", "day", "read", "book", "audience"],
    );
    segment.move_value(
        1,
        "moses",
        &["day", "read", "book"],
        &["moses", "day", "read", "book", "audience"],
    )?;
    segment.guard(
        2,
        "children",
        &["met", "not", "israel"],
        &["children", "met", "not", "israel", "bread"],
    )?;
    segment.move_value(
        3,
        "israel",
        &["now", "pass", "heard"],
        &["israel", "now", "pass", "heard", "law"],
    )?;
    segment.expression(
        4,
        "priest",
        &["eliashib", "having", "oversight"],
        &["priest", "eliashib", "having", "oversight", "chamber"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        5,
        8,
        "prepared reinitialize -> move_value -> expression prepared great chamber",
        &["prepared", "great", "chamber", "aforetime", "laid"],
    );
    segment.reinitialize(
        5,
        "prepared",
        &["great", "chamber", "aforetime"],
        &["prepared", "great", "chamber", "aforetime", "laid"],
    )?;
    segment.move_value(
        6,
        "king",
        &["time", "not", "jerusalem"],
        &["king", "time", "not", "jerusalem", "two"],
    )?;
    segment.move_value(
        7,
        "jerusalem",
        &["understood", "evil", "eliashib"],
        &["jerusalem", "understood", "evil", "eliashib", "tobiah"],
    )?;
    segment.expression(
        8,
        "grieved",
        &["sore", "therefore", "cast"],
        &["grieved", "sore", "therefore", "cast", "forth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        9,
        12,
        "commanded move_value -> call commanded cleansed chambers",
        &["commanded", "cleansed", "chambers", "thither", "brought"],
    );
    segment.move_value(
        9,
        "commanded",
        &["cleansed", "chambers", "thither"],
        &["commanded", "cleansed", "chambers", "thither", "brought"],
    )?;
    segment.move_value(
        10,
        "perceived",
        &["portions", "levites", "not"],
        &["perceived", "portions", "levites", "not", "given"],
    )?;
    segment.call(
        11,
        "god",
        &["contended", "rulers", "why"],
        &["god", "contended", "rulers", "why", "house"],
    )?;
    segment.move_value(
        12,
        "judah",
        &["brought", "tithe", "corn"],
        &["judah", "brought", "tithe", "corn", "new"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        13,
        16,
        "priest construct -> guard -> move_value priest made treasurers",
        &["priest", "made", "treasurers", "over", "treasuries"],
    );
    segment.construct(
        13,
        "priest",
        &["made", "treasurers", "over"],
        &["priest", "made", "treasurers", "over", "treasuries"],
    )?;
    segment.guard(
        14,
        "god",
        &["remember", "concerning", "wipe"],
        &["god", "remember", "concerning", "wipe", "not"],
    )?;
    segment.move_value(
        15,
        "judah",
        &["days", "saw", "treading"],
        &["judah", "days", "saw", "treading", "wine"],
    )?;
    segment.move_value(
        16,
        "men",
        &["dwelt", "tyre", "therein"],
        &["men", "dwelt", "tyre", "therein", "brought"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        17,
        20,
        "judah call -> move_value -> expression judah contended nobles",
        &["judah", "contended", "nobles", "unto", "evil"],
    );
    segment.call(
        17,
        "judah",
        &["contended", "nobles", "unto"],
        &["judah", "contended", "nobles", "unto", "evil"],
    )?;
    segment.move_value(
        18,
        "god",
        &["not", "fathers", "thus"],
        &["god", "not", "fathers", "thus", "bring"],
    )?;
    segment.move_value(
        19,
        "pass",
        &["gates", "jerusalem", "began"],
        &["pass", "gates", "jerusalem", "began", "dark"],
    )?;
    segment.expression(
        20,
        "merchants",
        &["sellers", "kind", "ware"],
        &["merchants", "sellers", "kind", "ware", "lodged"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        21,
        24,
        "testified move_value -> match_value -> guard testified against unto",
        &["testified", "against", "unto", "why", "lodge"],
    );
    segment.move_value(
        21,
        "testified",
        &["against", "unto", "why"],
        &["testified", "against", "unto", "why", "lodge"],
    )?;
    segment.move_value(
        22,
        "commanded",
        &["levites", "cleanse", "themselves"],
        &["commanded", "levites", "cleanse", "themselves", "keep"],
    )?;
    segment.match_value(
        23,
        "days",
        &["saw", "jews", "married"],
        &["days", "saw", "jews", "married", "wives"],
    )?;
    segment.guard(
        24,
        "children",
        &["spake", "half", "speech"],
        &["children", "spake", "half", "speech", "ashdod"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        25,
        28,
        "contended construct -> expression -> guard contended cursed smote",
        &["contended", "cursed", "smote", "certain", "plucked"],
    );
    segment.construct(
        25,
        "contended",
        &["cursed", "smote", "certain"],
        &["contended", "cursed", "smote", "certain", "plucked"],
    )?;
    segment.construct(
        26,
        "king",
        &["not", "solomon", "israel"],
        &["king", "not", "solomon", "israel", "sin"],
    )?;
    segment.expression(
        27,
        "god",
        &["shall", "hearken", "unto"],
        &["god", "shall", "hearken", "unto", "great"],
    )?;
    segment.guard(
        28,
        "priest",
        &["sons", "joiada", "son"],
        &["priest", "sons", "joiada", "son", "eliashib"],
    )?;
    segment.commit()?;
    Ok(())
}

fn nehemiah_13_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Nehemiah,
        13,
        29,
        31,
        "god return_value -> iterate -> trait_bound god remember defiled",
        &["god", "remember", "defiled", "priesthood", "covenant"],
    );
    segment.return_value(
        29,
        "god",
        &["remember", "defiled", "priesthood"],
        &["god", "remember", "defiled", "priesthood", "covenant"],
    )?;
    segment.iterate(
        30,
        "thus",
        &["cleansed", "strangers", "appointed"],
        &["thus", "cleansed", "strangers", "appointed", "wards"],
    )?;
    segment.trait_bound(
        31,
        "god",
        &["wood", "offering", "times"],
        &["god", "wood", "offering", "times", "appointed"],
    )?;
    segment.commit()?;
    Ok(())
}
