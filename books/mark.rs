use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    mark(story)?;
    Ok(())
}

fn mark(story: &mut Story) -> Result<(), StoryError> {
    mark_1(story)?;
    mark_2(story)?;
    mark_3(story)?;
    mark_4(story)?;
    mark_5(story)?;
    mark_6(story)?;
    mark_7(story)?;
    mark_8(story)?;
    mark_9(story)?;
    mark_10(story)?;
    mark_11(story)?;
    mark_12(story)?;
    mark_13(story)?;
    mark_14(story)?;
    mark_15(story)?;
    mark_16(story)?;
    Ok(())
}

fn mark_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 1, "Mark begins at full speed with John's wilderness call, Jesus' baptism, and a forty-day test before public work launches.", "Fishermen are called, a synagogue spirit is silenced, and healings spread through Capernaum until even solitude requires deliberate scheduling.");
    mark_1_group_01(story)?;
    mark_1_group_02(story)?;
    mark_1_group_03(story)?;
    mark_1_group_04(story)?;
    mark_1_group_05(story)?;
    mark_1_group_06(story)?;
    mark_1_group_07(story)?;
    mark_1_group_08(story)?;
    mark_1_group_09(story)?;
    mark_1_group_10(story)?;
    mark_1_group_11(story)?;
    Ok(())
}

fn mark_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 2, "Friends lower a paralyzed man through a roof, and Jesus heals him while also asserting authority to forgive sins.", "He calls Levi, eats with outsiders, and explains that Sabbath and fasting belong to a living mission rather than dead maintenance.");
    mark_2_group_01(story)?;
    mark_2_group_02(story)?;
    mark_2_group_03(story)?;
    mark_2_group_04(story)?;
    mark_2_group_05(story)?;
    mark_2_group_06(story)?;
    mark_2_group_07(story)?;
    Ok(())
}

fn mark_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 3, "Jesus restores a man's hand on the Sabbath, drawing both admiration and a plot from systems threatened by mercy.", "He appoints twelve companions, answers the Beelzebul accusation, and names those who do God's will as his real household.");
    mark_3_group_01(story)?;
    mark_3_group_02(story)?;
    mark_3_group_03(story)?;
    mark_3_group_04(story)?;
    mark_3_group_05(story)?;
    mark_3_group_06(story)?;
    mark_3_group_07(story)?;
    mark_3_group_08(story)?;
    mark_3_group_09(story)?;
    Ok(())
}

fn mark_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 4, "Jesus teaches from a boat with stories of seed, lamp-light, growing crops, and a mustard seed that starts tiny but becomes shelter.", "He calms a storm, leaving the disciples to ask what kind of person can command the sea and receive obedience.");
    mark_4_group_01(story)?;
    mark_4_group_02(story)?;
    mark_4_group_03(story)?;
    mark_4_group_04(story)?;
    mark_4_group_05(story)?;
    mark_4_group_06(story)?;
    mark_4_group_07(story)?;
    mark_4_group_08(story)?;
    mark_4_group_09(story)?;
    mark_4_group_10(story)?;
    Ok(())
}

fn mark_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 5, "A man possessed by a legion of destructive voices is restored to his right mind, though the local economy prefers the old arrangement.", "Jesus heals a long-suffering woman and raises Jairus's daughter, showing life returning in both public and intimate spaces.");
    mark_5_group_01(story)?;
    mark_5_group_02(story)?;
    mark_5_group_03(story)?;
    mark_5_group_04(story)?;
    mark_5_group_05(story)?;
    mark_5_group_06(story)?;
    mark_5_group_07(story)?;
    mark_5_group_08(story)?;
    mark_5_group_09(story)?;
    mark_5_group_10(story)?;
    mark_5_group_11(story)?;
    Ok(())
}

fn mark_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 6, "Nazareth takes offense at Jesus, and he sends the twelve in pairs with minimal luggage and maximum dependence on hospitality.", "John is executed at Herod's banquet; Jesus feeds five thousand, walks on water, and heals wherever people reach for him.");
    mark_6_group_01(story)?;
    mark_6_group_02(story)?;
    mark_6_group_03(story)?;
    mark_6_group_04(story)?;
    mark_6_group_05(story)?;
    mark_6_group_06(story)?;
    mark_6_group_07(story)?;
    mark_6_group_08(story)?;
    mark_6_group_09(story)?;
    mark_6_group_10(story)?;
    mark_6_group_11(story)?;
    mark_6_group_12(story)?;
    mark_6_group_13(story)?;
    mark_6_group_14(story)?;
    Ok(())
}

fn mark_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 7, "Jesus challenges hand-washing traditions that conceal corrupt hearts and teaches that inner output, not ritual input, makes a person unclean.", "He heals a Gentile woman's daughter and a deaf man with a speech impediment, opening ears and loosening tongues.");
    mark_7_group_01(story)?;
    mark_7_group_02(story)?;
    mark_7_group_03(story)?;
    mark_7_group_04(story)?;
    mark_7_group_05(story)?;
    mark_7_group_06(story)?;
    mark_7_group_07(story)?;
    mark_7_group_08(story)?;
    mark_7_group_09(story)?;
    Ok(())
}

fn mark_8(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 8, "A second crowd is fed, but the disciples still confuse bread logistics with the deeper warning about hardened understanding.", "Peter identifies Jesus as the Messiah; Jesus predicts suffering, rebukes shortcut thinking, and calls the crowd to carry a cross-shaped life.");
    mark_8_group_01(story)?;
    mark_8_group_02(story)?;
    mark_8_group_03(story)?;
    mark_8_group_04(story)?;
    mark_8_group_05(story)?;
    mark_8_group_06(story)?;
    mark_8_group_07(story)?;
    mark_8_group_08(story)?;
    mark_8_group_09(story)?;
    mark_8_group_10(story)?;
    Ok(())
}

fn mark_9(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 9, "Jesus shines on the mountain while Moses and Elijah appear, then descends to a failed exorcism and heals a boy everyone else could not free.", "He teaches about servant greatness, welcoming the little ones, and removing whatever repeatedly makes a life stumble.");
    mark_9_group_01(story)?;
    mark_9_group_02(story)?;
    mark_9_group_03(story)?;
    mark_9_group_04(story)?;
    mark_9_group_05(story)?;
    mark_9_group_06(story)?;
    mark_9_group_07(story)?;
    mark_9_group_08(story)?;
    mark_9_group_09(story)?;
    mark_9_group_10(story)?;
    mark_9_group_11(story)?;
    mark_9_group_12(story)?;
    mark_9_group_13(story)?;
    Ok(())
}

fn mark_10(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 10, "Jesus teaches covenant faithfulness, welcomes children, and tells a wealthy man that possessions can become a locked gate.", "He predicts the cross, corrects the disciples' ambition, and gives sight to Bartimaeus, whose persistent call becomes a model of trust.");
    mark_10_group_01(story)?;
    mark_10_group_02(story)?;
    mark_10_group_03(story)?;
    mark_10_group_04(story)?;
    mark_10_group_05(story)?;
    mark_10_group_06(story)?;
    mark_10_group_07(story)?;
    mark_10_group_08(story)?;
    mark_10_group_09(story)?;
    mark_10_group_10(story)?;
    mark_10_group_11(story)?;
    mark_10_group_12(story)?;
    mark_10_group_13(story)?;
    Ok(())
}

fn mark_11(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 11, "Jesus enters Jerusalem, curses a leafy but fruitless fig tree, and clears the temple of commerce that has crowded out prayer.", "He teaches about faith, forgiveness, and authority while the religious officials circle him with questions designed as traps.");
    mark_11_group_01(story)?;
    mark_11_group_02(story)?;
    mark_11_group_03(story)?;
    mark_11_group_04(story)?;
    mark_11_group_05(story)?;
    mark_11_group_06(story)?;
    mark_11_group_07(story)?;
    mark_11_group_08(story)?;
    Ok(())
}

fn mark_12(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 12, "The vineyard tenants reject the owner's son, and Jesus uses the story to expose leaders who mistake stewardship for ownership.", "He answers questions about taxes and resurrection, names love of God and neighbor as central, and praises a widow's two small coins.");
    mark_12_group_01(story)?;
    mark_12_group_02(story)?;
    mark_12_group_03(story)?;
    mark_12_group_04(story)?;
    mark_12_group_05(story)?;
    mark_12_group_06(story)?;
    mark_12_group_07(story)?;
    mark_12_group_08(story)?;
    mark_12_group_09(story)?;
    mark_12_group_10(story)?;
    mark_12_group_11(story)?;
    Ok(())
}

fn mark_13(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 13, "Jesus foretells the temple's ruin and describes turmoil, persecution, false messiahs, and cosmic signs without giving a timetable.", "The repeated command is watchfulness: servants should keep doing their assigned work because the returning master may arrive unseen.");
    mark_13_group_01(story)?;
    mark_13_group_02(story)?;
    mark_13_group_03(story)?;
    mark_13_group_04(story)?;
    mark_13_group_05(story)?;
    mark_13_group_06(story)?;
    mark_13_group_07(story)?;
    mark_13_group_08(story)?;
    mark_13_group_09(story)?;
    Ok(())
}

fn mark_14(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 14, "A woman anoints Jesus, Judas agrees to betray him, and the disciples share a final meal marked by bread, cup, and a warning.", "Prayer in Gethsemane gives way to arrest, desertion, a torn high-priestly courtroom, and Peter's three denials before the rooster log.");
    mark_14_group_01(story)?;
    mark_14_group_02(story)?;
    mark_14_group_03(story)?;
    mark_14_group_04(story)?;
    mark_14_group_05(story)?;
    mark_14_group_06(story)?;
    mark_14_group_07(story)?;
    mark_14_group_08(story)?;
    mark_14_group_09(story)?;
    mark_14_group_10(story)?;
    mark_14_group_11(story)?;
    mark_14_group_12(story)?;
    mark_14_group_13(story)?;
    mark_14_group_14(story)?;
    mark_14_group_15(story)?;
    mark_14_group_16(story)?;
    mark_14_group_17(story)?;
    mark_14_group_18(story)?;
    Ok(())
}

fn mark_15(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 15, "Pilate yields Jesus to the crowd, soldiers mock him, and Simon of Cyrene is pulled into the procession toward Golgotha.", "Jesus dies on the cross, the temple curtain tears, and Joseph of Arimathea places him in a rock-cut tomb before the Sabbath begins.");
    mark_15_group_01(story)?;
    mark_15_group_02(story)?;
    mark_15_group_03(story)?;
    mark_15_group_04(story)?;
    mark_15_group_05(story)?;
    mark_15_group_06(story)?;
    mark_15_group_07(story)?;
    mark_15_group_08(story)?;
    mark_15_group_09(story)?;
    mark_15_group_10(story)?;
    mark_15_group_11(story)?;
    mark_15_group_12(story)?;
    Ok(())
}

fn mark_16(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Mark", 16, "Women arrive with burial spices and find the stone rolled away, receiving the startling news that Jesus has risen and gone ahead.", "The witnesses leave trembling wonder, and the risen Lord's mission spreads through signs, proclamation, and the disciples' continued work.");
    mark_16_group_01(story)?;
    mark_16_group_02(story)?;
    mark_16_group_03(story)?;
    mark_16_group_04(story)?;
    mark_16_group_05(story)?;
    Ok(())
}

fn mark_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        1,
        4,
        "jesus expression -> move_value -> construct -> call jesus beginning gospel",
        &["jesus", "beginning", "gospel", "christ", "son"],
    );
    segment.expression(
        1,
        "jesus",
        &["beginning", "gospel", "christ"],
        &["jesus", "beginning", "gospel", "christ", "son"],
    )?;
    segment.move_value(
        2,
        "written",
        &["prophets", "behold", "send"],
        &["written", "prophets", "behold", "send", "messenger"],
    )?;
    segment.construct(
        3,
        "lord",
        &["voice", "crying", "wilderness"],
        &["lord", "voice", "crying", "wilderness", "prepare"],
    )?;
    segment.call(
        4,
        "john",
        &["baptize", "wilderness", "preach"],
        &["john", "baptize", "wilderness", "preach", "baptism"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        5,
        8,
        "went move_value -> expression -> guard went unto land",
        &["went", "unto", "land", "judaea", "jerusalem"],
    );
    segment.move_value(
        5,
        "went",
        &["unto", "land", "judaea"],
        &["went", "unto", "land", "judaea", "jerusalem"],
    )?;
    segment.expression(
        6,
        "john",
        &["clothed", "camel's", "hair"],
        &["john", "clothed", "camel's", "hair", "girdle"],
    )?;
    segment.guard(
        7,
        "preached",
        &["saying", "cometh", "mightier"],
        &["preached", "saying", "cometh", "mightier", "latchet"],
    )?;
    segment.expression(
        8,
        "indeed",
        &["baptized", "water", "shall"],
        &["indeed", "baptized", "water", "shall", "baptize"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        9,
        12,
        "jesus move_value -> match_value -> expression jesus pass days",
        &["jesus", "pass", "days", "nazareth", "galilee"],
    );
    segment.move_value(
        9,
        "jesus",
        &["pass", "days", "nazareth"],
        &["jesus", "pass", "days", "nazareth", "galilee"],
    )?;
    segment.match_value(
        10,
        "spirit",
        &["straightway", "coming", "water"],
        &["spirit", "straightway", "coming", "water", "saw"],
    )?;
    segment.move_value(
        11,
        "voice",
        &["heaven", "saying", "art"],
        &["voice", "heaven", "saying", "art", "beloved"],
    )?;
    segment.expression(
        12,
        "spirit",
        &["immediately", "driveth", "wilderness"],
        &["spirit", "immediately", "driveth", "wilderness"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        13,
        16,
        "wilderness expression -> move_value -> await_value -> match_value wilderness forty days",
        &["wilderness", "forty", "days", "tempted", "satan"],
    );
    segment.expression(
        13,
        "wilderness",
        &["forty", "days", "tempted"],
        &["wilderness", "forty", "days", "tempted", "satan"],
    )?;
    segment.move_value(
        14,
        "john",
        &["now", "put", "prison"],
        &["john", "now", "put", "prison", "jesus"],
    )?;
    segment.await_value(
        15,
        "god",
        &["saying", "time", "fulfilled"],
        &["god", "saying", "time", "fulfilled", "kingdom"],
    )?;
    segment.match_value(
        16,
        "now",
        &["walked", "sea", "galilee"],
        &["now", "walked", "sea", "galilee", "saw"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        17,
        20,
        "jesus construct -> expression -> move_value jesus unto will",
        &["jesus", "unto", "will", "make", "become"],
    );
    segment.construct(
        17,
        "jesus",
        &["unto", "will", "make"],
        &["jesus", "unto", "will", "make", "become"],
    )?;
    segment.expression(
        18,
        "straightway",
        &["forsook", "nets", "followed"],
        &["straightway", "forsook", "nets", "followed"],
    )?;
    segment.move_value(
        19,
        "gone",
        &["little", "farther", "thence"],
        &["gone", "little", "farther", "thence", "saw"],
    )?;
    segment.move_value(
        20,
        "straightway",
        &["called", "left", "father"],
        &["straightway", "called", "left", "father", "zebedee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        21,
        24,
        "went move_value -> guard -> result went capernaum straightway",
        &["went", "capernaum", "straightway", "sabbath", "day"],
    );
    segment.move_value(
        21,
        "went",
        &["capernaum", "straightway", "sabbath"],
        &["went", "capernaum", "straightway", "sabbath", "day"],
    )?;
    segment.guard(
        22,
        "astonished",
        &["doctrine", "taught", "authority"],
        &["astonished", "doctrine", "taught", "authority", "not"],
    )?;
    segment.result(
        23,
        "man",
        &["synagogue", "unclean", "spirit"],
        &["man", "synagogue", "unclean", "spirit", "cried"],
    )?;
    segment.move_value(
        24,
        "jesus",
        &["saying", "alone", "thee"],
        &["jesus", "saying", "alone", "thee", "nazareth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        25,
        28,
        "jesus move_value -> reinitialize -> expression jesus rebuked saying",
        &["jesus", "rebuked", "saying", "hold", "peace"],
    );
    segment.move_value(
        25,
        "jesus",
        &["rebuked", "saying", "hold"],
        &["jesus", "rebuked", "saying", "hold", "peace"],
    )?;
    segment.move_value(
        26,
        "spirit",
        &["unclean", "torn", "cried"],
        &["spirit", "unclean", "torn", "cried", "loud"],
    )?;
    segment.reinitialize(
        27,
        "amazed",
        &["insomuch", "questioned", "among"],
        &["amazed", "insomuch", "questioned", "among", "themselves"],
    )?;
    segment.expression(
        28,
        "immediately",
        &["fame", "spread", "abroad"],
        &["immediately", "fame", "spread", "abroad", "throughout"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        29,
        32,
        "john move_value -> expression john forthwith synagogue",
        &["john", "forthwith", "synagogue", "entered", "house"],
    );
    segment.move_value(
        29,
        "john",
        &["forthwith", "synagogue", "entered"],
        &["john", "forthwith", "synagogue", "entered", "house"],
    )?;
    segment.expression(
        30,
        "simon's",
        &["wife's", "mother", "lay"],
        &["simon's", "wife's", "mother", "lay", "sick"],
    )?;
    segment.move_value(
        31,
        "took",
        &["hand", "lifted", "immediately"],
        &["took", "hand", "lifted", "immediately", "fever"],
    )?;
    segment.move_value(
        32,
        "even",
        &["sun", "set", "brought"],
        &["even", "sun", "set", "brought", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        33,
        36,
        "city expression -> guard -> move_value city gathered together",
        &["city", "gathered", "together", "door", "healed"],
    );
    segment.expression(
        33,
        "city",
        &["gathered", "together", "door"],
        &["city", "gathered", "together", "door"],
    )?;
    segment.guard(
        34,
        "healed",
        &["many", "sick", "divers"],
        &["healed", "many", "sick", "divers", "diseases"],
    )?;
    segment.move_value(
        35,
        "morning",
        &["rising", "great", "while"],
        &["morning", "rising", "great", "while", "day"],
    )?;
    segment.expression(36, "simon", &["followed"], &["simon", "followed"])?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        37,
        40,
        "men call -> move_value -> expression -> construct men found unto",
        &["men", "found", "unto", "seek", "thee"],
    );
    segment.call(
        37,
        "men",
        &["found", "unto", "seek"],
        &["men", "found", "unto", "seek", "thee"],
    )?;
    segment.move_value(
        38,
        "unto",
        &["next", "towns", "preach"],
        &["unto", "next", "towns", "preach", "therefore"],
    )?;
    segment.expression(
        39,
        "preached",
        &["synagogues", "throughout", "galilee"],
        &["preached", "synagogues", "throughout", "galilee", "cast"],
    )?;
    segment.construct(
        40,
        "leper",
        &["beseeching", "kneeling", "down"],
        &["leper", "beseeching", "kneeling", "down", "saying"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_1_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        1,
        41,
        45,
        "jesus borrow -> expression -> move_value jesus moved compassion",
        &["jesus", "moved", "compassion", "put", "forth"],
    );
    segment.borrow(
        41,
        "jesus",
        &["moved", "compassion", "put"],
        &["jesus", "moved", "compassion", "put", "forth"],
    )?;
    segment.expression(
        42,
        "soon",
        &["spoken", "immediately", "leprosy"],
        &["soon", "spoken", "immediately", "leprosy", "departed"],
    )?;
    segment.move_value(
        43,
        "straitly",
        &["charged", "forthwith", "sent"],
        &["straitly", "charged", "forthwith", "sent", "away"],
    )?;
    segment.move_value(
        44,
        "man",
        &["saith", "unto", "see"],
        &["man", "saith", "unto", "see", "say"],
    )?;
    segment.move_value(
        45,
        "went",
        &["began", "publish", "much"],
        &["went", "began", "publish", "much", "blaze"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        2,
        1,
        4,
        "again expression -> guard -> move_value again entered capernaum",
        &["again", "entered", "capernaum", "days", "noised"],
    );
    segment.expression(
        1,
        "again",
        &["entered", "capernaum", "days"],
        &["again", "entered", "capernaum", "days", "noised"],
    )?;
    segment.guard(
        2,
        "straightway",
        &["many", "gathered", "together"],
        &["straightway", "many", "gathered", "together", "insomuch"],
    )?;
    segment.move_value(
        3,
        "unto",
        &["bringing", "sick", "palsy"],
        &["unto", "bringing", "sick", "palsy", "borne"],
    )?;
    segment.move_value(
        4,
        "not",
        &["nigh", "unto", "press"],
        &["not", "nigh", "unto", "press", "uncovered"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        2,
        5,
        8,
        "jesus match_value -> expression -> call jesus saw faith",
        &["jesus", "saw", "faith", "unto", "sick"],
    );
    segment.match_value(
        5,
        "jesus",
        &["saw", "faith", "unto"],
        &["jesus", "saw", "faith", "unto", "sick"],
    )?;
    segment.expression(
        6,
        "certain",
        &["scribes", "sitting", "reasoning"],
        &["certain", "scribes", "sitting", "reasoning", "hearts"],
    )?;
    segment.call(
        7,
        "man",
        &["why", "doth", "thus"],
        &["man", "why", "doth", "thus", "speak"],
    )?;
    segment.call(
        8,
        "jesus",
        &["immediately", "perceived", "spirit"],
        &["jesus", "immediately", "perceived", "spirit", "reasoned"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        2,
        9,
        12,
        "whether match_value -> expression -> move_value whether easier say",
        &["whether", "easier", "say", "sick", "palsy"],
    );
    segment.match_value(
        9,
        "whether",
        &["easier", "say", "sick"],
        &["whether", "easier", "say", "sick", "palsy"],
    )?;
    segment.expression(
        10,
        "man",
        &["know", "son", "hath"],
        &["man", "know", "son", "hath", "power"],
    )?;
    segment.move_value(
        11,
        "say",
        &["unto", "thee", "arise"],
        &["say", "unto", "thee", "arise", "take"],
    )?;
    segment.move_value(
        12,
        "immediately",
        &["arose", "took", "bed"],
        &["immediately", "arose", "took", "bed", "went"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        2,
        13,
        16,
        "went move_value -> reinitialize -> match_value went forth again",
        &["went", "forth", "again", "sea", "side"],
    );
    segment.move_value(
        13,
        "went",
        &["forth", "again", "sea"],
        &["went", "forth", "again", "sea", "side"],
    )?;
    segment.reinitialize(
        14,
        "passed",
        &["saw", "levi", "son"],
        &["passed", "saw", "levi", "son", "alphaeus"],
    )?;
    segment.move_value(
        15,
        "jesus",
        &["pass", "sat", "meat"],
        &["jesus", "pass", "sat", "meat", "house"],
    )?;
    segment.match_value(
        16,
        "scribes",
        &["pharisees", "saw", "eat"],
        &["scribes", "pharisees", "saw", "eat", "publicans"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        2,
        17,
        20,
        "jesus move_value -> call jesus heard saith",
        &["jesus", "heard", "saith", "unto", "whole"],
    );
    segment.move_value(
        17,
        "jesus",
        &["heard", "saith", "unto"],
        &["jesus", "heard", "saith", "unto", "whole"],
    )?;
    segment.move_value(
        18,
        "john",
        &["disciples", "pharisees", "used"],
        &["john", "disciples", "pharisees", "used", "fast"],
    )?;
    segment.call(
        19,
        "jesus",
        &["unto", "children", "bridechamber"],
        &["jesus", "unto", "children", "bridechamber", "fast"],
    )?;
    segment.move_value(
        20,
        "days",
        &["will", "bridegroom", "shall"],
        &["days", "will", "bridegroom", "shall", "taken"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        2,
        21,
        24,
        "man construct -> reinitialize -> move_value -> guard man seweth piece",
        &["man", "seweth", "piece", "new", "cloth"],
    );
    segment.construct(
        21,
        "man",
        &["seweth", "piece", "new"],
        &["man", "seweth", "piece", "new", "cloth"],
    )?;
    segment.reinitialize(
        22,
        "man",
        &["putteth", "new", "wine"],
        &["man", "putteth", "new", "wine", "old"],
    )?;
    segment.move_value(
        23,
        "pass",
        &["went", "through", "corn"],
        &["pass", "went", "through", "corn", "fields"],
    )?;
    segment.guard(
        24,
        "pharisees",
        &["unto", "behold", "why"],
        &["pharisees", "unto", "behold", "why", "sabbath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        2,
        25,
        28,
        "david call -> move_value -> construct -> expression david unto never",
        &["david", "unto", "never", "read", "need"],
    );
    segment.call(
        25,
        "david",
        &["unto", "never", "read"],
        &["david", "unto", "never", "read", "need"],
    )?;
    segment.move_value(
        26,
        "god",
        &["went", "house", "days"],
        &["god", "went", "house", "days", "abiathar"],
    )?;
    segment.construct(
        27,
        "man",
        &["unto", "sabbath", "made"],
        &["man", "unto", "sabbath", "made", "not"],
    )?;
    segment.expression(
        28,
        "man",
        &["therefore", "son", "lord"],
        &["man", "therefore", "son", "lord", "sabbath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        1,
        4,
        "man expression -> match_value -> drop_value man entered again",
        &["man", "entered", "again", "synagogue", "withered"],
    );
    segment.expression(
        1,
        "man",
        &["entered", "again", "synagogue"],
        &["man", "entered", "again", "synagogue", "withered"],
    )?;
    segment.match_value(
        2,
        "watched",
        &["whether", "would", "heal"],
        &["watched", "whether", "would", "heal", "sabbath"],
    )?;
    segment.expression(
        3,
        "man",
        &["saith", "unto", "withered"],
        &["man", "saith", "unto", "withered", "hand"],
    )?;
    segment.drop_value(
        4,
        "saith",
        &["unto", "lawful", "good"],
        &["saith", "unto", "lawful", "good", "sabbath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        5,
        8,
        "looked reinitialize -> move_value -> expression looked round anger",
        &["looked", "round", "anger", "grieved", "hardness"],
    );
    segment.reinitialize(
        5,
        "looked",
        &["round", "anger", "grieved"],
        &["looked", "round", "anger", "grieved", "hardness"],
    )?;
    segment.move_value(
        6,
        "pharisees",
        &["went", "forth", "straightway"],
        &["pharisees", "went", "forth", "straightway", "took"],
    )?;
    segment.expression(
        7,
        "jesus",
        &["withdrew", "himself", "disciples"],
        &["jesus", "withdrew", "himself", "disciples", "sea"],
    )?;
    segment.move_value(
        8,
        "jerusalem",
        &["idumaea", "beyond", "jordan"],
        &["jerusalem", "idumaea", "beyond", "jordan", "tyre"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        9,
        12,
        "spake call -> expression -> result -> construct spake disciples small",
        &["spake", "disciples", "small", "ship", "wait"],
    );
    segment.call(
        9,
        "spake",
        &["disciples", "small", "ship"],
        &["spake", "disciples", "small", "ship", "wait"],
    )?;
    segment.expression(
        10,
        "healed",
        &["many", "insomuch", "pressed"],
        &["healed", "many", "insomuch", "pressed", "touch"],
    )?;
    segment.result(
        11,
        "unclean",
        &["spirits", "saw", "fell"],
        &["unclean", "spirits", "saw", "fell", "down"],
    )?;
    segment.construct(
        12,
        "straitly",
        &["charged", "not", "make"],
        &["straitly", "charged", "not", "make", "known"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        13,
        16,
        "goeth move_value -> expression goeth mountain calleth",
        &["goeth", "mountain", "calleth", "unto", "would"],
    );
    segment.move_value(
        13,
        "goeth",
        &["mountain", "calleth", "unto"],
        &["goeth", "mountain", "calleth", "unto", "would"],
    )?;
    segment.move_value(
        14,
        "ordained",
        &["twelve", "might", "send"],
        &["ordained", "twelve", "might", "send", "forth"],
    )?;
    segment.expression(
        15,
        "power",
        &["heal", "sicknesses", "cast"],
        &["power", "heal", "sicknesses", "cast", "devils"],
    )?;
    segment.expression(
        16,
        "peter",
        &["simon", "surnamed"],
        &["peter", "simon", "surnamed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        17,
        20,
        "john iterate -> expression -> move_value -> guard john james son",
        &["john", "james", "son", "zebedee", "brother"],
    );
    segment.iterate(
        17,
        "john",
        &["james", "son", "zebedee"],
        &["john", "james", "son", "zebedee", "brother"],
    )?;
    segment.expression(
        18,
        "andrew",
        &["philip", "bartholomew", "matthew"],
        &["andrew", "philip", "bartholomew", "matthew", "thomas"],
    )?;
    segment.move_value(
        19,
        "judas",
        &["iscariot", "betrayed", "went"],
        &["judas", "iscariot", "betrayed", "went", "house"],
    )?;
    segment.guard(
        20,
        "multitude",
        &["cometh", "together", "again"],
        &["multitude", "cometh", "together", "again", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        21,
        24,
        "friends move_value -> call -> expression friends heard went",
        &["friends", "heard", "went", "lay", "hold"],
    );
    segment.move_value(
        21,
        "friends",
        &["heard", "went", "lay"],
        &["friends", "heard", "went", "lay", "hold"],
    )?;
    segment.move_value(
        22,
        "scribes",
        &["down", "jerusalem", "hath"],
        &["scribes", "down", "jerusalem", "hath", "beelzebub"],
    )?;
    segment.call(
        23,
        "called",
        &["unto", "parables", "satan"],
        &["called", "unto", "parables", "satan", "cast"],
    )?;
    segment.expression(
        24,
        "kingdom",
        &["divided", "against", "itself"],
        &["kingdom", "divided", "against", "itself", "cannot"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        25,
        28,
        "house expression -> guard -> call house divided against",
        &["house", "divided", "against", "itself", "cannot"],
    );
    segment.expression(
        25,
        "house",
        &["divided", "against", "itself"],
        &["house", "divided", "against", "itself", "cannot"],
    )?;
    segment.expression(
        26,
        "satan",
        &["rise", "against", "himself"],
        &["satan", "rise", "against", "himself", "divided"],
    )?;
    segment.guard(
        27,
        "man",
        &["enter", "strong", "man's"],
        &["man", "enter", "strong", "man's", "house"],
    )?;
    segment.call(
        28,
        "men",
        &["verily", "say", "unto"],
        &["men", "verily", "say", "unto", "sins"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        29,
        32,
        "shall expression -> call -> move_value -> match_value shall blaspheme against",
        &["shall", "blaspheme", "against", "holy", "ghost"],
    );
    segment.expression(
        29,
        "shall",
        &["blaspheme", "against", "holy"],
        &["shall", "blaspheme", "against", "holy", "ghost"],
    )?;
    segment.call(
        30,
        "spirit",
        &["hath", "unclean"],
        &["spirit", "hath", "unclean"],
    )?;
    segment.move_value(
        31,
        "brethren",
        &["mother", "standing", "without"],
        &["brethren", "mother", "standing", "without", "sent"],
    )?;
    segment.match_value(
        32,
        "multitude",
        &["sat", "unto", "behold"],
        &["multitude", "sat", "unto", "behold", "mother"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_3_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        3,
        33,
        35,
        "answered call -> match_value -> expression answered saying mother",
        &["answered", "saying", "mother", "brethren", "looked"],
    );
    segment.call(
        33,
        "answered",
        &["saying", "mother", "brethren"],
        &["answered", "saying", "mother", "brethren"],
    )?;
    segment.match_value(
        34,
        "looked",
        &["round", "sat", "behold"],
        &["looked", "round", "sat", "behold", "mother"],
    )?;
    segment.expression(
        35,
        "god",
        &["whosoever", "shall", "will"],
        &["god", "whosoever", "shall", "will", "same"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        1,
        4,
        "began call -> move_value began again teach",
        &["began", "again", "teach", "sea", "side"],
    );
    segment.call(
        1,
        "began",
        &["again", "teach", "sea"],
        &["began", "again", "teach", "sea", "side"],
    )?;
    segment.call(
        2,
        "taught",
        &["many", "things", "parables"],
        &["taught", "many", "things", "parables", "unto"],
    )?;
    segment.move_value(
        3,
        "hearken",
        &["behold", "went", "sower"],
        &["hearken", "behold", "went", "sower", "sow"],
    )?;
    segment.move_value(
        4,
        "pass",
        &["sowed", "fell", "way"],
        &["pass", "sowed", "fell", "way", "side"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        5,
        8,
        "fell guard -> move_value fell stony ground",
        &["fell", "stony", "ground", "not", "much"],
    );
    segment.guard(
        5,
        "fell",
        &["stony", "ground", "not"],
        &["fell", "stony", "ground", "not", "much"],
    )?;
    segment.guard(
        6,
        "sun",
        &["scorched", "root", "withered"],
        &["sun", "scorched", "root", "withered", "away"],
    )?;
    segment.guard(
        7,
        "fell",
        &["among", "thorns", "grew"],
        &["fell", "among", "thorns", "grew", "choked"],
    )?;
    segment.move_value(
        8,
        "other",
        &["fell", "good", "ground"],
        &["other", "fell", "good", "ground", "yield"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        9,
        12,
        "unto call -> result -> guard unto hath ears",
        &["unto", "hath", "ears", "hear", "alone"],
    );
    segment.call(
        9,
        "unto",
        &["hath", "ears", "hear"],
        &["unto", "hath", "ears", "hear"],
    )?;
    segment.result(
        10,
        "alone",
        &["twelve", "asked", "parable"],
        &["alone", "twelve", "asked", "parable"],
    )?;
    segment.call(
        11,
        "god",
        &["unto", "given", "know"],
        &["god", "unto", "given", "know", "mystery"],
    )?;
    segment.guard(
        12,
        "seeing",
        &["see", "not", "perceive"],
        &["seeing", "see", "not", "perceive", "hearing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        13,
        16,
        "unto guard -> call unto know not",
        &["unto", "know", "not", "parable", "will"],
    );
    segment.guard(
        13,
        "unto",
        &["know", "not", "parable"],
        &["unto", "know", "not", "parable", "will"],
    )?;
    segment.call(
        14,
        "sower",
        &["soweth", "word"],
        &["sower", "soweth", "word"],
    )?;
    segment.call(
        15,
        "way",
        &["side", "word", "sown"],
        &["way", "side", "word", "sown", "heard"],
    )?;
    segment.call(
        16,
        "likewise",
        &["sown", "stony", "ground"],
        &["likewise", "sown", "stony", "ground", "heard"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        17,
        20,
        "root guard -> call -> move_value root themselves endure",
        &["root", "themselves", "endure", "time", "afterward"],
    );
    segment.guard(
        17,
        "root",
        &["themselves", "endure", "time"],
        &["root", "themselves", "endure", "time", "afterward"],
    )?;
    segment.call(
        18,
        "sown",
        &["among", "thorns", "such"],
        &["sown", "among", "thorns", "such", "hear"],
    )?;
    segment.call(
        19,
        "cares",
        &["world", "deceitfulness", "riches"],
        &["cares", "world", "deceitfulness", "riches", "lusts"],
    )?;
    segment.move_value(
        20,
        "sown",
        &["good", "ground", "such"],
        &["sown", "good", "ground", "such", "hear"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        21,
        24,
        "unto move_value -> expression -> call unto candle brought",
        &["unto", "candle", "brought", "put", "under"],
    );
    segment.move_value(
        21,
        "unto",
        &["candle", "brought", "put"],
        &["unto", "candle", "brought", "put", "under"],
    )?;
    segment.move_value(
        22,
        "nothing",
        &["hid", "shall", "not"],
        &["nothing", "hid", "shall", "not", "manifested"],
    )?;
    segment.expression(
        23,
        "man",
        &["any", "ears", "hear"],
        &["man", "any", "ears", "hear"],
    )?;
    segment.call(
        24,
        "unto",
        &["take", "heed", "hear"],
        &["unto", "take", "heed", "hear", "measure"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        25,
        28,
        "hath guard -> call -> expression hath shall given",
        &["hath", "shall", "given", "not", "taken"],
    );
    segment.guard(
        25,
        "hath",
        &["shall", "given", "not"],
        &["hath", "shall", "given", "not", "taken"],
    )?;
    segment.call(
        26,
        "god",
        &["kingdom", "man", "cast"],
        &["god", "kingdom", "man", "cast", "seed"],
    )?;
    segment.guard(
        27,
        "sleep",
        &["rise", "night", "day"],
        &["sleep", "rise", "night", "day", "seed"],
    )?;
    segment.expression(
        28,
        "earth",
        &["bringeth", "forth", "fruit"],
        &["earth", "bringeth", "forth", "fruit", "herself"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        29,
        32,
        "fruit move_value -> call -> expression fruit brought forth",
        &["fruit", "brought", "forth", "immediately", "putteth"],
    );
    segment.move_value(
        29,
        "fruit",
        &["brought", "forth", "immediately"],
        &["fruit", "brought", "forth", "immediately", "putteth"],
    )?;
    segment.call(
        30,
        "god",
        &["whereunto", "shall", "liken"],
        &["god", "whereunto", "shall", "liken", "kingdom"],
    )?;
    segment.expression(
        31,
        "like",
        &["grain", "mustard", "seed"],
        &["like", "grain", "mustard", "seed", "sown"],
    )?;
    segment.expression(
        32,
        "sown",
        &["groweth", "becometh", "greater"],
        &["sown", "groweth", "becometh", "greater", "herbs"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        33,
        36,
        "many call -> guard -> move_value many such parables",
        &["many", "such", "parables", "spake", "word"],
    );
    segment.call(
        33,
        "many",
        &["such", "parables", "spake"],
        &["many", "such", "parables", "spake", "word"],
    )?;
    segment.guard(
        34,
        "without",
        &["parable", "spake", "not"],
        &["without", "parable", "spake", "not", "unto"],
    )?;
    segment.move_value(
        35,
        "same",
        &["day", "even", "saith"],
        &["same", "day", "even", "saith", "unto"],
    )?;
    segment.move_value(
        36,
        "sent",
        &["away", "multitude", "took"],
        &["sent", "away", "multitude", "took", "even"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_4_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        4,
        37,
        41,
        "arose reinitialize -> guard -> call arose great storm",
        &["arose", "great", "storm", "wind", "waves"],
    );
    segment.reinitialize(
        37,
        "arose",
        &["great", "storm", "wind"],
        &["arose", "great", "storm", "wind", "waves"],
    )?;
    segment.guard(
        38,
        "hinder",
        &["part", "ship", "asleep"],
        &["hinder", "part", "ship", "asleep", "pillow"],
    )?;
    segment.reinitialize(
        39,
        "arose",
        &["rebuked", "wind", "unto"],
        &["arose", "rebuked", "wind", "unto", "sea"],
    )?;
    segment.guard(
        40,
        "unto",
        &["why", "fearful", "faith"],
        &["unto", "why", "fearful", "faith"],
    )?;
    segment.call(
        41,
        "man",
        &["feared", "exceedingly", "another"],
        &["man", "feared", "exceedingly", "another", "manner"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        1,
        4,
        "over move_value -> guard over unto other",
        &["over", "unto", "other", "side", "sea"],
    );
    segment.move_value(
        1,
        "over",
        &["unto", "other", "side"],
        &["over", "unto", "other", "side", "sea"],
    )?;
    segment.move_value(
        2,
        "man",
        &["ship", "immediately", "met"],
        &["man", "ship", "immediately", "met", "tombs"],
    )?;
    segment.guard(
        3,
        "man",
        &["dwelling", "among", "tombs"],
        &["man", "dwelling", "among", "tombs", "bind"],
    )?;
    segment.guard(
        4,
        "often",
        &["bound", "fetters", "chains"],
        &["often", "bound", "fetters", "chains", "plucked"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        5,
        8,
        "always expression -> match_value -> guard -> move_value always night day",
        &["always", "night", "day", "mountains", "tombs"],
    );
    segment.expression(
        5,
        "always",
        &["night", "day", "mountains"],
        &["always", "night", "day", "mountains", "tombs"],
    )?;
    segment.match_value(
        6,
        "jesus",
        &["saw", "afar", "off"],
        &["jesus", "saw", "afar", "off", "ran"],
    )?;
    segment.guard(
        7,
        "jesus",
        &["cried", "loud", "voice"],
        &["jesus", "cried", "loud", "voice", "thee"],
    )?;
    segment.move_value(
        8,
        "man",
        &["unto", "unclean", "spirit"],
        &["man", "unto", "unclean", "spirit"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        9,
        12,
        "asked result -> move_value -> expression asked name answered",
        &["asked", "name", "answered", "saying", "legion"],
    );
    segment.result(
        9,
        "asked",
        &["name", "answered", "saying"],
        &["asked", "name", "answered", "saying", "legion"],
    )?;
    segment.move_value(
        10,
        "besought",
        &["much", "would", "not"],
        &["besought", "much", "would", "not", "send"],
    )?;
    segment.expression(
        11,
        "now",
        &["nigh", "unto", "mountains"],
        &["now", "nigh", "unto", "mountains", "great"],
    )?;
    segment.move_value(
        12,
        "devils",
        &["besought", "saying", "send"],
        &["devils", "besought", "saying", "send", "swine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        13,
        16,
        "jesus move_value -> match_value jesus forthwith gave",
        &["jesus", "forthwith", "gave", "leave", "unclean"],
    );
    segment.move_value(
        13,
        "jesus",
        &["forthwith", "gave", "leave"],
        &["jesus", "forthwith", "gave", "leave", "unclean"],
    )?;
    segment.move_value(
        14,
        "fed",
        &["swine", "fled", "told"],
        &["fed", "swine", "fled", "told", "city"],
    )?;
    segment.move_value(
        15,
        "jesus",
        &["see", "possessed", "devil"],
        &["jesus", "see", "possessed", "devil", "legion"],
    )?;
    segment.match_value(
        16,
        "saw",
        &["told", "befell", "possessed"],
        &["saw", "told", "befell", "possessed", "devil"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        17,
        20,
        "began move_value -> expression began pray depart",
        &["began", "pray", "depart", "coasts", "ship"],
    );
    segment.move_value(
        17,
        "began",
        &["pray", "depart", "coasts"],
        &["began", "pray", "depart", "coasts"],
    )?;
    segment.move_value(
        18,
        "ship",
        &["possessed", "devil", "prayed"],
        &["ship", "possessed", "devil", "prayed", "might"],
    )?;
    segment.move_value(
        19,
        "jesus",
        &["howbeit", "suffered", "not"],
        &["jesus", "howbeit", "suffered", "not", "saith"],
    )?;
    segment.expression(
        20,
        "jesus",
        &["departed", "began", "publish"],
        &["jesus", "departed", "began", "publish", "decapolis"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        21,
        24,
        "jesus expression -> match_value -> move_value jesus passed over",
        &["jesus", "passed", "over", "again", "ship"],
    );
    segment.expression(
        21,
        "jesus",
        &["passed", "over", "again"],
        &["jesus", "passed", "over", "again", "ship"],
    )?;
    segment.match_value(
        22,
        "behold",
        &["cometh", "rulers", "synagogue"],
        &["behold", "cometh", "rulers", "synagogue", "jairus"],
    )?;
    segment.move_value(
        23,
        "besought",
        &["greatly", "saying", "little"],
        &["besought", "greatly", "saying", "little", "daughter"],
    )?;
    segment.move_value(
        24,
        "jesus",
        &["went", "much", "people"],
        &["jesus", "went", "much", "people", "followed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        25,
        28,
        "woman expression -> move_value -> call woman certain issue",
        &["woman", "certain", "issue", "blood", "twelve"],
    );
    segment.expression(
        25,
        "woman",
        &["certain", "issue", "blood"],
        &["woman", "certain", "issue", "blood", "twelve"],
    )?;
    segment.expression(
        26,
        "suffered",
        &["many", "things", "physicians"],
        &["suffered", "many", "things", "physicians", "spent"],
    )?;
    segment.move_value(
        27,
        "jesus",
        &["heard", "press", "behind"],
        &["jesus", "heard", "press", "behind", "touched"],
    )?;
    segment.call(
        28,
        "touch",
        &["clothes", "shall", "whole"],
        &["touch", "clothes", "shall", "whole"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        29,
        32,
        "straightway expression -> move_value -> call straightway fountain blood",
        &["straightway", "fountain", "blood", "dried", "felt"],
    );
    segment.expression(
        29,
        "straightway",
        &["fountain", "blood", "dried"],
        &["straightway", "fountain", "blood", "dried", "felt"],
    )?;
    segment.move_value(
        30,
        "jesus",
        &["immediately", "knowing", "himself"],
        &["jesus", "immediately", "knowing", "himself", "virtue"],
    )?;
    segment.call(
        31,
        "disciples",
        &["unto", "seest", "multitude"],
        &["disciples", "unto", "seest", "multitude", "thronging"],
    )?;
    segment.expression(
        32,
        "looked",
        &["round", "see", "done"],
        &["looked", "round", "see", "done", "thing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        33,
        36,
        "woman move_value -> construct -> guard woman fearing trembling",
        &["woman", "fearing", "trembling", "knowing", "done"],
    );
    segment.move_value(
        33,
        "woman",
        &["fearing", "trembling", "knowing"],
        &["woman", "fearing", "trembling", "knowing", "done"],
    )?;
    segment.construct(
        34,
        "unto",
        &["daughter", "faith", "hath"],
        &["unto", "daughter", "faith", "hath", "made"],
    )?;
    segment.move_value(
        35,
        "while",
        &["yet", "spake", "ruler"],
        &["while", "yet", "spake", "ruler", "synagogue's"],
    )?;
    segment.guard(
        36,
        "jesus",
        &["soon", "heard", "word"],
        &["jesus", "soon", "heard", "word", "spoken"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        37,
        40,
        "man guard -> result -> construct -> expression man suffered follow",
        &["man", "suffered", "follow", "save", "peter"],
    );
    segment.guard(
        37,
        "man",
        &["suffered", "follow", "save"],
        &["man", "suffered", "follow", "save", "peter"],
    )?;
    segment.result(
        38,
        "cometh",
        &["house", "ruler", "synagogue"],
        &["cometh", "house", "ruler", "synagogue", "seeth"],
    )?;
    segment.construct(
        39,
        "saith",
        &["unto", "why", "make"],
        &["saith", "unto", "why", "make", "ado"],
    )?;
    segment.expression(
        40,
        "laughed",
        &["scorn", "put", "taketh"],
        &["laughed", "scorn", "put", "taketh", "father"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_5_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        5,
        41,
        43,
        "took call -> reinitialize -> guard took damsel hand",
        &["took", "damsel", "hand", "unto", "talitha"],
    );
    segment.call(
        41,
        "took",
        &["damsel", "hand", "unto"],
        &["took", "damsel", "hand", "unto", "talitha"],
    )?;
    segment.reinitialize(
        42,
        "straightway",
        &["damsel", "arose", "walked"],
        &["straightway", "damsel", "arose", "walked", "age"],
    )?;
    segment.guard(
        43,
        "man",
        &["charged", "straitly", "know"],
        &["man", "charged", "straitly", "know", "commanded"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        1,
        4,
        "went move_value -> guard went thence own",
        &["went", "thence", "own", "country", "disciples"],
    );
    segment.move_value(
        1,
        "went",
        &["thence", "own", "country"],
        &["went", "thence", "own", "country", "disciples"],
    )?;
    segment.move_value(
        2,
        "sabbath",
        &["day", "began", "teach"],
        &["sabbath", "day", "began", "teach", "synagogue"],
    )?;
    segment.guard(
        3,
        "not",
        &["carpenter", "son", "mary"],
        &["not", "carpenter", "son", "mary", "brother"],
    )?;
    segment.guard(
        4,
        "jesus",
        &["unto", "prophet", "not"],
        &["jesus", "unto", "prophet", "not", "without"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        5,
        8,
        "mighty guard -> move_value mighty work save",
        &["mighty", "work", "save", "laid", "hands"],
    );
    segment.guard(
        5,
        "mighty",
        &["work", "save", "laid"],
        &["mighty", "work", "save", "laid", "hands"],
    )?;
    segment.move_value(
        6,
        "marvelled",
        &["unbelief", "went", "round"],
        &["marvelled", "unbelief", "went", "round", "villages"],
    )?;
    segment.move_value(
        7,
        "called",
        &["unto", "twelve", "began"],
        &["called", "unto", "twelve", "began", "send"],
    )?;
    segment.move_value(
        8,
        "commanded",
        &["take", "nothing", "journey"],
        &["commanded", "take", "nothing", "journey", "save"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        9,
        12,
        "shod guard -> move_value shod sandals not",
        &["shod", "sandals", "not", "put", "two"],
    );
    segment.guard(
        9,
        "shod",
        &["sandals", "not", "put"],
        &["shod", "sandals", "not", "put", "two"],
    )?;
    segment.move_value(
        10,
        "unto",
        &["place", "soever", "enter"],
        &["unto", "place", "soever", "enter", "house"],
    )?;
    segment.move_value(
        11,
        "whosoever",
        &["shall", "not", "receive"],
        &["whosoever", "shall", "not", "receive", "nor"],
    )?;
    segment.move_value(
        12,
        "men",
        &["went", "preached", "repent"],
        &["men", "went", "preached", "repent"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        13,
        16,
        "cast expression -> drop_value -> call cast many devils",
        &["cast", "many", "devils", "anointed", "oil"],
    );
    segment.expression(
        13,
        "cast",
        &["many", "devils", "anointed"],
        &["cast", "many", "devils", "anointed", "oil"],
    )?;
    segment.drop_value(
        14,
        "king",
        &["herod", "heard", "name"],
        &["king", "herod", "heard", "name", "spread"],
    )?;
    segment.call(
        15,
        "prophet",
        &["others", "elias", "prophets"],
        &["prophet", "others", "elias", "prophets"],
    )?;
    segment.drop_value(
        16,
        "john",
        &["herod", "heard", "thereof"],
        &["john", "herod", "heard", "thereof", "beheaded"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        17,
        20,
        "john move_value -> guard -> drop_value -> expression john herod himself",
        &["john", "herod", "himself", "sent", "forth"],
    );
    segment.move_value(
        17,
        "john",
        &["herod", "himself", "sent"],
        &["john", "herod", "himself", "sent", "forth"],
    )?;
    segment.guard(
        18,
        "john",
        &["unto", "herod", "not"],
        &["john", "unto", "herod", "not", "lawful"],
    )?;
    segment.drop_value(
        19,
        "therefore",
        &["herodias", "quarrel", "against"],
        &["therefore", "herodias", "quarrel", "against", "would"],
    )?;
    segment.expression(
        20,
        "john",
        &["herod", "feared", "knowing"],
        &["john", "herod", "feared", "knowing", "just"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        21,
        24,
        "convenient construct -> move_value -> guard convenient day herod",
        &["convenient", "day", "herod", "birthday", "made"],
    );
    segment.construct(
        21,
        "convenient",
        &["day", "herod", "birthday"],
        &["convenient", "day", "herod", "birthday", "made"],
    )?;
    segment.move_value(
        22,
        "king",
        &["daughter", "herodias", "danced"],
        &["king", "daughter", "herodias", "danced", "pleased"],
    )?;
    segment.guard(
        23,
        "sware",
        &["unto", "whatsoever", "shalt"],
        &["sware", "unto", "whatsoever", "shalt", "ask"],
    )?;
    segment.move_value(
        24,
        "john",
        &["went", "forth", "unto"],
        &["john", "went", "forth", "unto", "mother"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        25,
        28,
        "king move_value -> guard king straightway haste",
        &["king", "straightway", "haste", "unto", "asked"],
    );
    segment.move_value(
        25,
        "king",
        &["straightway", "haste", "unto"],
        &["king", "straightway", "haste", "unto", "asked"],
    )?;
    segment.guard(
        26,
        "king",
        &["exceeding", "sorry", "yet"],
        &["king", "exceeding", "sorry", "yet", "oath's"],
    )?;
    segment.move_value(
        27,
        "king",
        &["immediately", "sent", "executioner"],
        &["king", "immediately", "sent", "executioner", "commanded"],
    )?;
    segment.move_value(
        28,
        "brought",
        &["head", "charger", "gave"],
        &["brought", "head", "charger", "gave", "damsel"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        29,
        32,
        "disciples move_value -> call -> expression disciples heard took",
        &["disciples", "heard", "took", "corpse", "laid"],
    );
    segment.move_value(
        29,
        "disciples",
        &["heard", "took", "corpse"],
        &["disciples", "heard", "took", "corpse", "laid"],
    )?;
    segment.call(
        30,
        "jesus",
        &["apostles", "gathered", "themselves"],
        &["jesus", "apostles", "gathered", "themselves", "together"],
    )?;
    segment.move_value(
        31,
        "unto",
        &["yourselves", "apart", "desert"],
        &["unto", "yourselves", "apart", "desert", "place"],
    )?;
    segment.expression(
        32,
        "departed",
        &["desert", "place", "ship"],
        &["departed", "desert", "place", "ship", "privately"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        33,
        36,
        "people move_value people saw departing",
        &["people", "saw", "departing", "many", "knew"],
    );
    segment.move_value(
        33,
        "people",
        &["saw", "departing", "many"],
        &["people", "saw", "departing", "many", "knew"],
    )?;
    segment.move_value(
        34,
        "jesus",
        &["saw", "much", "people"],
        &["jesus", "saw", "much", "people", "moved"],
    )?;
    segment.move_value(
        35,
        "day",
        &["now", "far", "spent"],
        &["day", "now", "far", "spent", "disciples"],
    )?;
    segment.move_value(
        36,
        "send",
        &["away", "country", "round"],
        &["send", "away", "country", "round", "villages"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        37,
        40,
        "answered move_value -> construct -> expression answered unto give",
        &["answered", "unto", "give", "eat", "say"],
    );
    segment.move_value(
        37,
        "answered",
        &["unto", "give", "eat"],
        &["answered", "unto", "give", "eat", "say"],
    )?;
    segment.move_value(
        38,
        "saith",
        &["unto", "many", "loaves"],
        &["saith", "unto", "many", "loaves", "see"],
    )?;
    segment.construct(
        39,
        "commanded",
        &["make", "sit", "down"],
        &["commanded", "make", "sit", "down", "companies"],
    )?;
    segment.expression(
        40,
        "sat",
        &["down", "ranks", "hundreds"],
        &["sat", "down", "ranks", "hundreds", "fifties"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        41,
        44,
        "taken return_value -> expression taken five loaves",
        &["taken", "five", "loaves", "two", "fishes"],
    );
    segment.return_value(
        41,
        "taken",
        &["five", "loaves", "two"],
        &["taken", "five", "loaves", "two", "fishes"],
    )?;
    segment.expression(42, "eat", &["filled"], &["eat", "filled"])?;
    segment.expression(
        43,
        "took",
        &["twelve", "baskets", "full"],
        &["took", "twelve", "baskets", "full", "fragments"],
    )?;
    segment.expression(
        44,
        "men",
        &["eat", "loaves", "five"],
        &["men", "eat", "loaves", "five", "thousand"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        45,
        48,
        "straightway move_value -> match_value straightway constrained disciples",
        &["straightway", "constrained", "disciples", "get", "ship"],
    );
    segment.move_value(
        45,
        "straightway",
        &["constrained", "disciples", "get"],
        &["straightway", "constrained", "disciples", "get", "ship"],
    )?;
    segment.move_value(
        46,
        "sent",
        &["away", "departed", "mountain"],
        &["sent", "away", "departed", "mountain", "pray"],
    )?;
    segment.move_value(
        47,
        "even",
        &["ship", "midst", "sea"],
        &["even", "ship", "midst", "sea", "alone"],
    )?;
    segment.match_value(
        48,
        "saw",
        &["toiling", "rowing", "wind"],
        &["saw", "toiling", "rowing", "wind", "contrary"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_13(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        49,
        52,
        "spirit result -> guard -> move_value spirit saw walking",
        &["spirit", "saw", "walking", "sea", "supposed"],
    );
    segment.result(
        49,
        "spirit",
        &["saw", "walking", "sea"],
        &["spirit", "saw", "walking", "sea", "supposed"],
    )?;
    segment.guard(
        50,
        "saw",
        &["troubled", "immediately", "talked"],
        &["saw", "troubled", "immediately", "talked", "saith"],
    )?;
    segment.move_value(
        51,
        "went",
        &["unto", "ship", "wind"],
        &["went", "unto", "ship", "wind", "ceased"],
    )?;
    segment.guard(
        52,
        "considered",
        &["not", "miracle", "loaves"],
        &["considered", "not", "miracle", "loaves", "heart"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_6_group_14(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        6,
        53,
        56,
        "passed move_value -> expression -> construct passed over land",
        &["passed", "over", "land", "gennesaret", "drew"],
    );
    segment.move_value(
        53,
        "passed",
        &["over", "land", "gennesaret"],
        &["passed", "over", "land", "gennesaret", "drew"],
    )?;
    segment.move_value(
        54,
        "ship",
        &["straightway", "knew"],
        &["ship", "straightway", "knew"],
    )?;
    segment.expression(
        55,
        "ran",
        &["through", "whole", "region"],
        &["ran", "through", "whole", "region", "round"],
    )?;
    segment.construct(
        56,
        "whithersoever",
        &["entered", "villages", "cities"],
        &["whithersoever", "entered", "villages", "cities", "country"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        1,
        4,
        "together move_value -> match_value -> guard together unto pharisees",
        &["together", "unto", "pharisees", "certain", "scribes"],
    );
    segment.move_value(
        1,
        "together",
        &["unto", "pharisees", "certain"],
        &["together", "unto", "pharisees", "certain", "scribes"],
    )?;
    segment.match_value(
        2,
        "saw",
        &["disciples", "eat", "bread"],
        &["saw", "disciples", "eat", "bread", "defiled"],
    )?;
    segment.guard(
        3,
        "pharisees",
        &["jews", "except", "wash"],
        &["pharisees", "jews", "except", "wash", "hands"],
    )?;
    segment.move_value(
        4,
        "market",
        &["except", "wash", "eat"],
        &["market", "except", "wash", "eat", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        5,
        8,
        "pharisees guard -> call -> trait_bound pharisees scribes asked",
        &["pharisees", "scribes", "asked", "why", "walk"],
    );
    segment.guard(
        5,
        "pharisees",
        &["scribes", "asked", "why"],
        &["pharisees", "scribes", "asked", "why", "walk"],
    )?;
    segment.call(
        6,
        "answered",
        &["unto", "well", "hath"],
        &["answered", "unto", "well", "hath", "esaias"],
    )?;
    segment.trait_bound(
        7,
        "men",
        &["howbeit", "vain", "worship"],
        &["men", "howbeit", "vain", "worship", "teaching"],
    )?;
    segment.guard(
        8,
        "god",
        &["laying", "aside", "commandment"],
        &["god", "laying", "aside", "commandment", "hold"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        9,
        12,
        "god guard -> drop_value -> call god unto full",
        &["god", "unto", "full", "well", "reject"],
    );
    segment.guard(
        9,
        "god",
        &["unto", "full", "well"],
        &["god", "unto", "full", "well", "reject"],
    )?;
    segment.drop_value(
        10,
        "moses",
        &["honour", "father", "mother"],
        &["moses", "honour", "father", "mother", "whoso"],
    )?;
    segment.call(
        11,
        "man",
        &["say", "shall", "father"],
        &["man", "say", "shall", "father", "mother"],
    )?;
    segment.guard(
        12,
        "suffer",
        &["ought", "father", "mother"],
        &["suffer", "ought", "father", "mother"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        13,
        16,
        "god call -> move_value -> expression god making word",
        &["god", "making", "word", "none", "effect"],
    );
    segment.call(
        13,
        "god",
        &["making", "word", "none"],
        &["god", "making", "word", "none", "effect"],
    )?;
    segment.call(
        14,
        "people",
        &["called", "unto", "hearken"],
        &["people", "called", "unto", "hearken", "every"],
    )?;
    segment.move_value(
        15,
        "man",
        &["nothing", "without", "entering"],
        &["man", "nothing", "without", "entering", "defile"],
    )?;
    segment.expression(
        16,
        "man",
        &["any", "ears", "hear"],
        &["man", "any", "ears", "hear"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        17,
        20,
        "people result -> guard -> call people entered house",
        &["people", "entered", "house", "disciples", "asked"],
    );
    segment.result(
        17,
        "people",
        &["entered", "house", "disciples"],
        &["people", "entered", "house", "disciples", "asked"],
    )?;
    segment.guard(
        18,
        "saith",
        &["unto", "without", "understanding"],
        &["saith", "unto", "without", "understanding", "not"],
    )?;
    segment.guard(
        19,
        "entereth",
        &["not", "heart", "belly"],
        &["entereth", "not", "heart", "belly", "goeth"],
    )?;
    segment.call(
        20,
        "man",
        &["cometh", "defileth"],
        &["man", "cometh", "defileth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        21,
        24,
        "men expression -> move_value men within heart",
        &["men", "within", "heart", "proceed", "evil"],
    );
    segment.expression(
        21,
        "men",
        &["within", "heart", "proceed"],
        &["men", "within", "heart", "proceed", "evil"],
    )?;
    segment.expression(
        22,
        "thefts",
        &["covetousness", "wickedness", "deceit"],
        &[
            "thefts",
            "covetousness",
            "wickedness",
            "deceit",
            "lasciviousness",
        ],
    )?;
    segment.move_value(
        23,
        "man",
        &["evil", "things", "within"],
        &["man", "evil", "things", "within", "defile"],
    )?;
    segment.move_value(
        24,
        "thence",
        &["arose", "went", "borders"],
        &["thence", "arose", "went", "borders", "tyre"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        25,
        28,
        "woman move_value -> expression -> guard -> call woman certain whose",
        &["woman", "certain", "whose", "young", "daughter"],
    );
    segment.move_value(
        25,
        "woman",
        &["certain", "whose", "young"],
        &["woman", "certain", "whose", "young", "daughter"],
    )?;
    segment.expression(
        26,
        "woman",
        &["greek", "syrophenician", "nation"],
        &["woman", "greek", "syrophenician", "nation", "besought"],
    )?;
    segment.guard(
        27,
        "jesus",
        &["unto", "children", "first"],
        &["jesus", "unto", "children", "first", "filled"],
    )?;
    segment.call(
        28,
        "lord",
        &["answered", "unto", "yes"],
        &["lord", "answered", "unto", "yes", "yet"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        29,
        32,
        "unto move_value unto saying way",
        &["unto", "saying", "way", "devil", "gone"],
    );
    segment.move_value(
        29,
        "unto",
        &["saying", "way", "devil"],
        &["unto", "saying", "way", "devil", "gone"],
    )?;
    segment.move_value(
        30,
        "house",
        &["found", "devil", "gone"],
        &["house", "found", "devil", "gone", "daughter"],
    )?;
    segment.move_value(
        31,
        "again",
        &["departing", "coasts", "tyre"],
        &["again", "departing", "coasts", "tyre", "sidon"],
    )?;
    segment.move_value(
        32,
        "bring",
        &["unto", "deaf", "impediment"],
        &["bring", "unto", "deaf", "impediment", "speech"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_7_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        7,
        33,
        37,
        "took expression -> call -> guard took aside multitude",
        &["took", "aside", "multitude", "put", "fingers"],
    );
    segment.expression(
        33,
        "took",
        &["aside", "multitude", "put"],
        &["took", "aside", "multitude", "put", "fingers"],
    )?;
    segment.expression(
        34,
        "looking",
        &["heaven", "sighed", "saith"],
        &["looking", "heaven", "sighed", "saith", "unto"],
    )?;
    segment.call(
        35,
        "straightway",
        &["ears", "opened", "string"],
        &["straightway", "ears", "opened", "string", "tongue"],
    )?;
    segment.guard(
        36,
        "man",
        &["charged", "tell", "much"],
        &["man", "charged", "tell", "much", "great"],
    )?;
    segment.call(
        37,
        "beyond",
        &["measure", "astonished", "saying"],
        &["beyond", "measure", "astonished", "saying", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        1,
        4,
        "jesus expression -> borrow -> move_value -> call jesus days multitude",
        &["jesus", "days", "multitude", "very", "great"],
    );
    segment.expression(
        1,
        "jesus",
        &["days", "multitude", "very"],
        &["jesus", "days", "multitude", "very", "great"],
    )?;
    segment.borrow(
        2,
        "compassion",
        &["multitude", "now", "three"],
        &["compassion", "multitude", "now", "three", "days"],
    )?;
    segment.move_value(
        3,
        "send",
        &["away", "fasting", "own"],
        &["send", "away", "fasting", "own", "houses"],
    )?;
    segment.call(
        4,
        "man",
        &["disciples", "answered", "whence"],
        &["man", "disciples", "answered", "whence", "satisfy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        5,
        8,
        "asked result -> expression -> return_value asked many loaves",
        &["asked", "many", "loaves", "seven", "people"],
    );
    segment.result(
        5,
        "asked",
        &["many", "loaves", "seven"],
        &["asked", "many", "loaves", "seven"],
    )?;
    segment.expression(
        6,
        "people",
        &["commanded", "sit", "down"],
        &["people", "commanded", "sit", "down", "ground"],
    )?;
    segment.return_value(
        7,
        "few",
        &["small", "fishes", "blessed"],
        &["few", "small", "fishes", "blessed", "commanded"],
    )?;
    segment.expression(
        8,
        "eat",
        &["filled", "took", "broken"],
        &["eat", "filled", "took", "broken", "meat"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        9,
        12,
        "eaten move_value -> guard eaten four thousand",
        &["eaten", "four", "thousand", "sent", "away"],
    );
    segment.move_value(
        9,
        "eaten",
        &["four", "thousand", "sent"],
        &["eaten", "four", "thousand", "sent", "away"],
    )?;
    segment.move_value(
        10,
        "straightway",
        &["entered", "ship", "disciples"],
        &["straightway", "entered", "ship", "disciples", "parts"],
    )?;
    segment.move_value(
        11,
        "pharisees",
        &["forth", "began", "question"],
        &["pharisees", "forth", "began", "question", "seeking"],
    )?;
    segment.guard(
        12,
        "spirit",
        &["sighed", "deeply", "saith"],
        &["spirit", "sighed", "deeply", "saith", "why"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        13,
        16,
        "left expression -> guard left entering ship",
        &["left", "entering", "ship", "again", "departed"],
    );
    segment.expression(
        13,
        "left",
        &["entering", "ship", "again"],
        &["left", "entering", "ship", "again", "departed"],
    )?;
    segment.guard(
        14,
        "now",
        &["disciples", "forgotten", "take"],
        &["now", "disciples", "forgotten", "take", "bread"],
    )?;
    segment.expression(
        15,
        "charged",
        &["saying", "take", "heed"],
        &["charged", "saying", "take", "heed", "beware"],
    )?;
    segment.guard(
        16,
        "reasoned",
        &["among", "themselves", "saying"],
        &["reasoned", "among", "themselves", "saying", "bread"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        17,
        20,
        "jesus guard -> call jesus knew saith",
        &["jesus", "knew", "saith", "unto", "why"],
    );
    segment.guard(
        17,
        "jesus",
        &["knew", "saith", "unto"],
        &["jesus", "knew", "saith", "unto", "why"],
    )?;
    segment.guard(
        18,
        "having",
        &["eyes", "see", "not"],
        &["having", "eyes", "see", "not", "ears"],
    )?;
    segment.call(
        19,
        "brake",
        &["five", "loaves", "among"],
        &["brake", "five", "loaves", "among", "thousand"],
    )?;
    segment.call(
        20,
        "seven",
        &["among", "four", "thousand"],
        &["seven", "among", "four", "thousand", "many"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        21,
        24,
        "unto guard -> move_value -> result -> call unto not understand",
        &["unto", "not", "understand", "man", "cometh"],
    );
    segment.guard(
        21,
        "unto",
        &["not", "understand"],
        &["unto", "not", "understand"],
    )?;
    segment.move_value(
        22,
        "man",
        &["cometh", "bethsaida", "bring"],
        &["man", "cometh", "bethsaida", "bring", "blind"],
    )?;
    segment.result(
        23,
        "man",
        &["took", "blind", "hand"],
        &["man", "took", "blind", "hand", "led"],
    )?;
    segment.call(
        24,
        "men",
        &["looked", "see", "trees"],
        &["men", "looked", "see", "trees", "walking"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        25,
        28,
        "put construct -> move_value -> call put hands again",
        &["put", "hands", "again", "eyes", "made"],
    );
    segment.construct(
        25,
        "put",
        &["hands", "again", "eyes"],
        &["put", "hands", "again", "eyes", "made"],
    )?;
    segment.move_value(
        26,
        "sent",
        &["away", "house", "saying"],
        &["sent", "away", "house", "saying", "neither"],
    )?;
    segment.move_value(
        27,
        "jesus",
        &["went", "disciples", "towns"],
        &["jesus", "went", "disciples", "towns", "caesarea"],
    )?;
    segment.call(
        28,
        "john",
        &["answered", "baptist", "say"],
        &["john", "answered", "baptist", "say", "elias"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        29,
        32,
        "peter call -> guard -> drop_value peter saith unto",
        &["peter", "saith", "unto", "say", "answereth"],
    );
    segment.call(
        29,
        "peter",
        &["saith", "unto", "say"],
        &["peter", "saith", "unto", "say", "answereth"],
    )?;
    segment.guard(30, "man", &["charged", "tell"], &["man", "charged", "tell"])?;
    segment.drop_value(
        31,
        "man",
        &["began", "teach", "son"],
        &["man", "began", "teach", "son", "must"],
    )?;
    segment.call(
        32,
        "peter",
        &["spake", "saying", "openly"],
        &["peter", "spake", "saying", "openly", "took"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        33,
        35,
        "peter guard -> move_value -> expression peter turned looked",
        &["peter", "turned", "looked", "disciples", "rebuked"],
    );
    segment.guard(
        33,
        "peter",
        &["turned", "looked", "disciples"],
        &["peter", "turned", "looked", "disciples", "rebuked"],
    )?;
    segment.move_value(
        34,
        "people",
        &["called", "unto", "disciples"],
        &["people", "called", "unto", "disciples", "whosoever"],
    )?;
    segment.expression(
        35,
        "whosoever",
        &["will", "save", "life"],
        &["whosoever", "will", "save", "life", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_8_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        8,
        36,
        38,
        "man expression -> trait_bound man shall profit",
        &["man", "shall", "profit", "gain", "whole"],
    );
    segment.expression(
        36,
        "man",
        &["shall", "profit", "gain"],
        &["man", "shall", "profit", "gain", "whole"],
    )?;
    segment.expression(
        37,
        "man",
        &["shall", "give", "exchange"],
        &["man", "shall", "give", "exchange", "soul"],
    )?;
    segment.trait_bound(
        38,
        "whosoever",
        &["therefore", "shall", "ashamed"],
        &["whosoever", "therefore", "shall", "ashamed", "words"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        1,
        4,
        "unto move_value -> expression -> guard unto verily say",
        &["unto", "verily", "say", "stand", "shall"],
    );
    segment.move_value(
        1,
        "unto",
        &["verily", "say", "stand"],
        &["unto", "verily", "say", "stand", "shall"],
    )?;
    segment.expression(
        2,
        "jesus",
        &["six", "days", "taketh"],
        &["jesus", "six", "days", "taketh", "peter"],
    )?;
    segment.guard(
        3,
        "raiment",
        &["became", "shining", "exceeding"],
        &["raiment", "became", "shining", "exceeding", "white"],
    )?;
    segment.expression(
        4,
        "moses",
        &["appeared", "unto", "elias"],
        &["moses", "appeared", "unto", "elias", "talking"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        5,
        8,
        "peter construct -> guard -> move_value peter answered jesus",
        &["peter", "answered", "jesus", "master", "good"],
    );
    segment.construct(
        5,
        "peter",
        &["answered", "jesus", "master"],
        &["peter", "answered", "jesus", "master", "good"],
    )?;
    segment.guard(
        6,
        "wist",
        &["not", "say", "sore"],
        &["wist", "not", "say", "sore", "afraid"],
    )?;
    segment.move_value(
        7,
        "cloud",
        &["overshadowed", "voice", "saying"],
        &["cloud", "overshadowed", "voice", "saying", "beloved"],
    )?;
    segment.guard(
        8,
        "man",
        &["suddenly", "looked", "round"],
        &["man", "suddenly", "looked", "round", "saw"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        9,
        12,
        "man move_value -> drop_value -> guard man down mountain",
        &["man", "down", "mountain", "charged", "tell"],
    );
    segment.move_value(
        9,
        "man",
        &["down", "mountain", "charged"],
        &["man", "down", "mountain", "charged", "tell"],
    )?;
    segment.drop_value(
        10,
        "kept",
        &["saying", "themselves", "questioning"],
        &["kept", "saying", "themselves", "questioning", "another"],
    )?;
    segment.move_value(
        11,
        "asked",
        &["saying", "why", "say"],
        &["asked", "saying", "why", "say", "scribes"],
    )?;
    segment.guard(
        12,
        "answered",
        &["told", "elias", "verily"],
        &["answered", "told", "elias", "verily", "cometh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        13,
        16,
        "say move_value -> expression -> result say unto elias",
        &["say", "unto", "elias", "indeed", "done"],
    );
    segment.move_value(
        13,
        "say",
        &["unto", "elias", "indeed"],
        &["say", "unto", "elias", "indeed", "done"],
    )?;
    segment.move_value(
        14,
        "disciples",
        &["saw", "great", "multitude"],
        &["disciples", "saw", "great", "multitude", "scribes"],
    )?;
    segment.expression(
        15,
        "people",
        &["straightway", "beheld", "greatly"],
        &["people", "straightway", "beheld", "greatly", "amazed"],
    )?;
    segment.result(
        16,
        "asked",
        &["scribes", "question"],
        &["asked", "scribes", "question"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        17,
        20,
        "multitude move_value -> guard multitude answered master",
        &["multitude", "answered", "master", "brought", "unto"],
    );
    segment.move_value(
        17,
        "multitude",
        &["answered", "master", "brought"],
        &["multitude", "answered", "master", "brought", "unto"],
    )?;
    segment.guard(
        18,
        "wheresoever",
        &["taketh", "teareth", "foameth"],
        &["wheresoever", "taketh", "teareth", "foameth", "gnasheth"],
    )?;
    segment.move_value(
        19,
        "answereth",
        &["saith", "faithless", "generation"],
        &["answereth", "saith", "faithless", "generation", "long"],
    )?;
    segment.move_value(
        20,
        "spirit",
        &["brought", "unto", "saw"],
        &["spirit", "brought", "unto", "saw", "straightway"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        21,
        24,
        "child move_value -> drop_value -> call -> result child asked father",
        &["child", "asked", "father", "long", "ago"],
    );
    segment.move_value(
        21,
        "child",
        &["asked", "father", "long"],
        &["child", "asked", "father", "long", "ago"],
    )?;
    segment.drop_value(
        22,
        "ofttimes",
        &["hath", "cast", "fire"],
        &["ofttimes", "hath", "cast", "fire", "waters"],
    )?;
    segment.call(
        23,
        "jesus",
        &["unto", "canst", "believe"],
        &["jesus", "unto", "canst", "believe", "things"],
    )?;
    segment.result(
        24,
        "child",
        &["straightway", "father", "cried"],
        &["child", "straightway", "father", "cried", "tears"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        25,
        28,
        "jesus move_value -> reinitialize jesus saw people",
        &["jesus", "saw", "people", "running", "together"],
    );
    segment.move_value(
        25,
        "jesus",
        &["saw", "people", "running"],
        &["jesus", "saw", "people", "running", "together"],
    )?;
    segment.move_value(
        26,
        "spirit",
        &["cried", "rent", "sore"],
        &["spirit", "cried", "rent", "sore", "dead"],
    )?;
    segment.reinitialize(
        27,
        "jesus",
        &["took", "hand", "lifted"],
        &["jesus", "took", "hand", "lifted", "arose"],
    )?;
    segment.move_value(
        28,
        "house",
        &["disciples", "asked", "privately"],
        &["house", "disciples", "asked", "privately", "why"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        29,
        32,
        "unto move_value -> guard -> drop_value unto kind forth",
        &["unto", "kind", "forth", "nothing", "prayer"],
    );
    segment.move_value(
        29,
        "unto",
        &["kind", "forth", "nothing"],
        &["unto", "kind", "forth", "nothing", "prayer"],
    )?;
    segment.guard(
        30,
        "departed",
        &["thence", "passed", "through"],
        &["departed", "thence", "passed", "through", "galilee"],
    )?;
    segment.drop_value(
        31,
        "man",
        &["taught", "disciples", "unto"],
        &["man", "taught", "disciples", "unto", "son"],
    )?;
    segment.guard(
        32,
        "understood",
        &["not", "saying", "afraid"],
        &["understood", "not", "saying", "afraid", "ask"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        33,
        36,
        "capernaum move_value -> expression -> call capernaum house asked",
        &["capernaum", "house", "asked", "disputed", "among"],
    );
    segment.move_value(
        33,
        "capernaum",
        &["house", "asked", "disputed"],
        &["capernaum", "house", "asked", "disputed", "among"],
    )?;
    segment.expression(
        34,
        "held",
        &["peace", "way", "disputed"],
        &["held", "peace", "way", "disputed", "among"],
    )?;
    segment.expression(
        35,
        "man",
        &["sat", "down", "called"],
        &["man", "sat", "down", "called", "twelve"],
    )?;
    segment.call(
        36,
        "child",
        &["took", "set", "midst"],
        &["child", "took", "set", "midst", "taken"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        37,
        40,
        "children move_value -> guard children whosoever shall",
        &["children", "whosoever", "shall", "receive", "such"],
    );
    segment.move_value(
        37,
        "children",
        &["whosoever", "shall", "receive"],
        &["children", "whosoever", "shall", "receive", "such"],
    )?;
    segment.guard(
        38,
        "john",
        &["answered", "saying", "master"],
        &["john", "answered", "saying", "master", "saw"],
    )?;
    segment.guard(
        39,
        "jesus",
        &["forbid", "not", "man"],
        &["jesus", "forbid", "not", "man", "shall"],
    )?;
    segment.guard(40, "not", &["against", "part"], &["not", "against", "part"])?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        41,
        44,
        "whosoever guard -> await_value -> move_value whosoever shall give",
        &["whosoever", "shall", "give", "cup", "water"],
    );
    segment.guard(
        41,
        "whosoever",
        &["shall", "give", "cup"],
        &["whosoever", "shall", "give", "cup", "water"],
    )?;
    segment.await_value(
        42,
        "whosoever",
        &["shall", "offend", "little"],
        &["whosoever", "shall", "offend", "little", "ones"],
    )?;
    segment.move_value(
        43,
        "hand",
        &["offend", "thee", "cut"],
        &["hand", "offend", "thee", "cut", "off"],
    )?;
    segment.guard(
        44,
        "worm",
        &["dieth", "not", "fire"],
        &["worm", "dieth", "not", "fire", "quenched"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        45,
        47,
        "foot expression -> guard foot offend thee",
        &["foot", "offend", "thee", "cut", "off"],
    );
    segment.expression(
        45,
        "foot",
        &["offend", "thee", "cut"],
        &["foot", "offend", "thee", "cut", "off"],
    )?;
    segment.guard(
        46,
        "worm",
        &["dieth", "not", "fire"],
        &["worm", "dieth", "not", "fire", "quenched"],
    )?;
    segment.expression(
        47,
        "thine",
        &["eye", "offend", "thee"],
        &["thine", "eye", "offend", "thee", "pluck"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_9_group_13(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        9,
        48,
        50,
        "worm guard -> trait_bound -> expression worm dieth not",
        &["worm", "dieth", "not", "fire", "quenched"],
    );
    segment.guard(
        48,
        "worm",
        &["dieth", "not", "fire"],
        &["worm", "dieth", "not", "fire", "quenched"],
    )?;
    segment.trait_bound(
        49,
        "every",
        &["shall", "salted", "fire"],
        &["every", "shall", "salted", "fire", "sacrifice"],
    )?;
    segment.expression(
        50,
        "salt",
        &["good", "lost", "saltness"],
        &["salt", "good", "lost", "saltness", "wherewith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        1,
        4,
        "arose reinitialize -> move_value -> guard -> call arose thence cometh",
        &["arose", "thence", "cometh", "coasts", "judaea"],
    );
    segment.reinitialize(
        1,
        "arose",
        &["thence", "cometh", "coasts"],
        &["arose", "thence", "cometh", "coasts", "judaea"],
    )?;
    segment.move_value(
        2,
        "man",
        &["pharisees", "asked", "lawful"],
        &["man", "pharisees", "asked", "lawful", "put"],
    )?;
    segment.guard(
        3,
        "moses",
        &["answered", "unto", "command"],
        &["moses", "answered", "unto", "command"],
    )?;
    segment.call(
        4,
        "moses",
        &["suffered", "write", "bill"],
        &["moses", "suffered", "write", "bill", "divorcement"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        5,
        8,
        "jesus guard -> construct -> expression jesus answered unto",
        &["jesus", "answered", "unto", "hardness", "heart"],
    );
    segment.guard(
        5,
        "jesus",
        &["answered", "unto", "hardness"],
        &["jesus", "answered", "unto", "hardness", "heart"],
    )?;
    segment.construct(
        6,
        "god",
        &["beginning", "creation", "made"],
        &["god", "beginning", "creation", "made", "male"],
    )?;
    segment.expression(
        7,
        "man",
        &["cause", "shall", "leave"],
        &["man", "cause", "shall", "leave", "father"],
    )?;
    segment.guard(
        8,
        "twain",
        &["shall", "flesh"],
        &["twain", "shall", "flesh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        9,
        12,
        "god guard -> result -> expression god therefore hath",
        &["god", "therefore", "hath", "joined", "together"],
    );
    segment.guard(
        9,
        "god",
        &["therefore", "hath", "joined"],
        &["god", "therefore", "hath", "joined", "together"],
    )?;
    segment.result(
        10,
        "house",
        &["disciples", "asked", "again"],
        &["house", "disciples", "asked", "again", "same"],
    )?;
    segment.expression(
        11,
        "saith",
        &["unto", "whosoever", "shall"],
        &["saith", "unto", "whosoever", "shall", "put"],
    )?;
    segment.expression(
        12,
        "woman",
        &["shall", "put", "away"],
        &["woman", "shall", "put", "away", "husband"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        13,
        16,
        "children move_value -> guard -> return_value children brought young",
        &["children", "brought", "young", "touch", "disciples"],
    );
    segment.move_value(
        13,
        "children",
        &["brought", "young", "touch"],
        &["children", "brought", "young", "touch", "disciples"],
    )?;
    segment.move_value(
        14,
        "jesus",
        &["saw", "much", "displeased"],
        &["jesus", "saw", "much", "displeased", "unto"],
    )?;
    segment.guard(
        15,
        "verily",
        &["say", "unto", "whosoever"],
        &["verily", "say", "unto", "whosoever", "shall"],
    )?;
    segment.return_value(
        16,
        "took",
        &["arms", "put", "hands"],
        &["took", "arms", "put", "hands", "blessed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        17,
        20,
        "gone move_value -> call -> drop_value gone forth way",
        &["gone", "forth", "way", "running", "kneeled"],
    );
    segment.move_value(
        17,
        "gone",
        &["forth", "way", "running"],
        &["gone", "forth", "way", "running", "kneeled"],
    )?;
    segment.call(
        18,
        "jesus",
        &["unto", "why", "callest"],
        &["jesus", "unto", "why", "callest", "good"],
    )?;
    segment.drop_value(
        19,
        "knowest",
        &["commandments", "not", "commit"],
        &["knowest", "commandments", "not", "commit", "adultery"],
    )?;
    segment.call(
        20,
        "answered",
        &["unto", "master", "observed"],
        &["answered", "unto", "master", "observed", "youth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        21,
        24,
        "jesus move_value -> expression -> await_value jesus beholding loved",
        &["jesus", "beholding", "loved", "unto", "thing"],
    );
    segment.move_value(
        21,
        "jesus",
        &["beholding", "loved", "unto"],
        &["jesus", "beholding", "loved", "unto", "thing"],
    )?;
    segment.move_value(
        22,
        "sad",
        &["saying", "went", "away"],
        &["sad", "saying", "went", "away", "grieved"],
    )?;
    segment.expression(
        23,
        "jesus",
        &["looked", "round", "saith"],
        &["jesus", "looked", "round", "saith", "unto"],
    )?;
    segment.await_value(
        24,
        "jesus",
        &["disciples", "astonished", "words"],
        &["jesus", "disciples", "astonished", "words", "answereth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        25,
        28,
        "man move_value -> expression -> guard -> call man easier camel",
        &["man", "easier", "camel", "through", "eye"],
    );
    segment.move_value(
        25,
        "man",
        &["easier", "camel", "through"],
        &["man", "easier", "camel", "through", "eye"],
    )?;
    segment.expression(
        26,
        "astonished",
        &["measure", "saying", "among"],
        &["astonished", "measure", "saying", "among", "themselves"],
    )?;
    segment.guard(
        27,
        "jesus",
        &["looking", "saith", "men"],
        &["jesus", "looking", "saith", "men", "impossible"],
    )?;
    segment.call(
        28,
        "peter",
        &["began", "say", "unto"],
        &["peter", "began", "say", "unto", "left"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        29,
        32,
        "jesus guard -> move_value -> expression jesus answered verily",
        &["jesus", "answered", "verily", "say", "unto"],
    );
    segment.guard(
        29,
        "jesus",
        &["answered", "verily", "say"],
        &["jesus", "answered", "verily", "say", "unto"],
    )?;
    segment.move_value(
        30,
        "shall",
        &["receive", "hundredfold", "now"],
        &["shall", "receive", "hundredfold", "now", "time"],
    )?;
    segment.expression(
        31,
        "many",
        &["first", "shall", "last"],
        &["many", "first", "shall", "last"],
    )?;
    segment.move_value(
        32,
        "jesus",
        &["way", "going", "jerusalem"],
        &["jesus", "way", "going", "jerusalem", "went"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        33,
        36,
        "man move_value -> drop_value -> call man saying behold",
        &["man", "saying", "behold", "jerusalem", "son"],
    );
    segment.move_value(
        33,
        "man",
        &["saying", "behold", "jerusalem"],
        &["man", "saying", "behold", "jerusalem", "son"],
    )?;
    segment.drop_value(
        34,
        "shall",
        &["mock", "scourge", "spit"],
        &["shall", "mock", "scourge", "spit", "kill"],
    )?;
    segment.move_value(
        35,
        "john",
        &["james", "sons", "zebedee"],
        &["john", "james", "sons", "zebedee", "unto"],
    )?;
    segment.call(36, "unto", &["would"], &["unto", "would"])?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        37,
        40,
        "unto call -> guard unto grant sit",
        &["unto", "grant", "sit", "right", "hand"],
    );
    segment.call(
        37,
        "unto",
        &["grant", "sit", "right"],
        &["unto", "grant", "sit", "right", "hand"],
    )?;
    segment.guard(
        38,
        "jesus",
        &["unto", "know", "not"],
        &["jesus", "unto", "know", "not", "ask"],
    )?;
    segment.call(
        39,
        "jesus",
        &["unto", "shall", "indeed"],
        &["jesus", "unto", "shall", "indeed", "drink"],
    )?;
    segment.guard(
        40,
        "sit",
        &["right", "hand", "left"],
        &["sit", "right", "hand", "left", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        41,
        44,
        "john expression -> guard john ten heard",
        &["john", "ten", "heard", "began", "much"],
    );
    segment.expression(
        41,
        "john",
        &["ten", "heard", "began"],
        &["john", "ten", "heard", "began", "much"],
    )?;
    segment.expression(
        42,
        "jesus",
        &["called", "saith", "unto"],
        &["jesus", "called", "saith", "unto", "know"],
    )?;
    segment.guard(
        43,
        "shall",
        &["not", "among", "whosoever"],
        &["shall", "not", "among", "whosoever", "will"],
    )?;
    segment.expression(
        44,
        "whosoever",
        &["will", "chiefest", "shall"],
        &["whosoever", "will", "chiefest", "shall", "servant"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        45,
        48,
        "man move_value -> result man even son",
        &["man", "even", "son", "not", "ministered"],
    );
    segment.move_value(
        45,
        "man",
        &["even", "son", "not"],
        &["man", "even", "son", "not", "ministered"],
    )?;
    segment.move_value(
        46,
        "people",
        &["jericho", "went", "disciples"],
        &["people", "jericho", "went", "disciples", "great"],
    )?;
    segment.result(
        47,
        "jesus",
        &["heard", "nazareth", "began"],
        &["jesus", "heard", "nazareth", "began", "cry"],
    )?;
    segment.result(
        48,
        "many",
        &["charged", "hold", "peace"],
        &["many", "charged", "hold", "peace", "cried"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_10_group_13(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        10,
        49,
        52,
        "jesus expression -> move_value -> call -> construct jesus stood still",
        &["jesus", "stood", "still", "commanded", "called"],
    );
    segment.expression(
        49,
        "jesus",
        &["stood", "still", "commanded"],
        &["jesus", "stood", "still", "commanded", "called"],
    )?;
    segment.move_value(
        50,
        "jesus",
        &["casting", "away", "garment"],
        &["jesus", "casting", "away", "garment", "rose"],
    )?;
    segment.call(
        51,
        "jesus",
        &["answered", "unto", "wilt"],
        &["jesus", "answered", "unto", "wilt", "thee"],
    )?;
    segment.construct(
        52,
        "jesus",
        &["unto", "way", "faith"],
        &["jesus", "unto", "way", "faith", "hath"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        1,
        4,
        "nigh move_value nigh jerusalem unto",
        &["nigh", "jerusalem", "unto", "bethphage", "bethany"],
    );
    segment.move_value(
        1,
        "nigh",
        &["jerusalem", "unto", "bethphage"],
        &["nigh", "jerusalem", "unto", "bethphage", "bethany"],
    )?;
    segment.move_value(
        2,
        "saith",
        &["unto", "way", "village"],
        &["saith", "unto", "way", "village", "over"],
    )?;
    segment.move_value(
        3,
        "man",
        &["any", "say", "unto"],
        &["man", "any", "say", "unto", "why"],
    )?;
    segment.move_value(
        4,
        "went",
        &["way", "found", "colt"],
        &["went", "way", "found", "colt", "tied"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        5,
        8,
        "certain call -> move_value -> expression certain stood unto",
        &["certain", "stood", "unto", "loosing", "colt"],
    );
    segment.call(
        5,
        "certain",
        &["stood", "unto", "loosing"],
        &["certain", "stood", "unto", "loosing", "colt"],
    )?;
    segment.move_value(
        6,
        "jesus",
        &["unto", "even", "commanded"],
        &["jesus", "unto", "even", "commanded"],
    )?;
    segment.move_value(
        7,
        "jesus",
        &["brought", "colt", "cast"],
        &["jesus", "brought", "colt", "cast", "garments"],
    )?;
    segment.expression(
        8,
        "many",
        &["spread", "garments", "way"],
        &["many", "spread", "garments", "way", "others"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        9,
        12,
        "went move_value -> return_value went followed cried",
        &["went", "followed", "cried", "saying", "hosanna"],
    );
    segment.move_value(
        9,
        "went",
        &["followed", "cried", "saying"],
        &["went", "followed", "cried", "saying", "hosanna"],
    )?;
    segment.return_value(
        10,
        "david",
        &["blessed", "kingdom", "father"],
        &["david", "blessed", "kingdom", "father", "cometh"],
    )?;
    segment.move_value(
        11,
        "jesus",
        &["entered", "jerusalem", "temple"],
        &["jesus", "entered", "jerusalem", "temple", "looked"],
    )?;
    segment.move_value(
        12,
        "morrow",
        &["bethany", "hungry"],
        &["morrow", "bethany", "hungry"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        13,
        16,
        "seeing move_value -> guard seeing fig tree",
        &["seeing", "fig", "tree", "afar", "off"],
    );
    segment.move_value(
        13,
        "seeing",
        &["fig", "tree", "afar"],
        &["seeing", "fig", "tree", "afar", "off"],
    )?;
    segment.guard(
        14,
        "jesus",
        &["answered", "unto", "man"],
        &["jesus", "answered", "unto", "man", "eat"],
    )?;
    segment.move_value(
        15,
        "jesus",
        &["jerusalem", "went", "temple"],
        &["jesus", "jerusalem", "went", "temple", "began"],
    )?;
    segment.guard(
        16,
        "man",
        &["would", "not", "suffer"],
        &["man", "would", "not", "suffer", "any"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        17,
        20,
        "taught construct -> drop_value -> move_value -> match_value taught saying unto",
        &["taught", "saying", "unto", "not", "written"],
    );
    segment.construct(
        17,
        "taught",
        &["saying", "unto", "not"],
        &["taught", "saying", "unto", "not", "written"],
    )?;
    segment.drop_value(
        18,
        "scribes",
        &["chief", "priests", "heard"],
        &["scribes", "chief", "priests", "heard", "sought"],
    )?;
    segment.move_value(19, "even", &["went", "city"], &["even", "went", "city"])?;
    segment.match_value(
        20,
        "morning",
        &["passed", "saw", "fig"],
        &["morning", "passed", "saw", "fig", "tree"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        21,
        24,
        "peter match_value -> await_value -> move_value -> result peter calling remembrance",
        &["peter", "calling", "remembrance", "saith", "unto"],
    );
    segment.match_value(
        21,
        "peter",
        &["calling", "remembrance", "saith"],
        &["peter", "calling", "remembrance", "saith", "unto"],
    )?;
    segment.await_value(
        22,
        "jesus",
        &["answering", "saith", "unto"],
        &["jesus", "answering", "saith", "unto", "faith"],
    )?;
    segment.move_value(
        23,
        "verily",
        &["say", "unto", "whosoever"],
        &["verily", "say", "unto", "whosoever", "shall"],
    )?;
    segment.result(
        24,
        "therefore",
        &["say", "unto", "things"],
        &["therefore", "say", "unto", "things", "soever"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        25,
        28,
        "stand expression -> guard -> move_value -> call stand praying forgive",
        &["stand", "praying", "forgive", "ought", "against"],
    );
    segment.expression(
        25,
        "stand",
        &["praying", "forgive", "ought"],
        &["stand", "praying", "forgive", "ought", "against"],
    )?;
    segment.guard(
        26,
        "not",
        &["forgive", "neither", "will"],
        &["not", "forgive", "neither", "will", "father"],
    )?;
    segment.move_value(
        27,
        "again",
        &["jerusalem", "walking", "temple"],
        &["again", "jerusalem", "walking", "temple", "chief"],
    )?;
    segment.call(
        28,
        "say",
        &["unto", "authority", "doest"],
        &["say", "unto", "authority", "doest", "things"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_11_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        11,
        29,
        33,
        "jesus result -> expression -> guard -> call jesus answered unto",
        &["jesus", "answered", "unto", "will", "ask"],
    );
    segment.result(
        29,
        "jesus",
        &["answered", "unto", "will"],
        &["jesus", "answered", "unto", "will", "ask"],
    )?;
    segment.expression(
        30,
        "john",
        &["baptism", "heaven", "men"],
        &["john", "baptism", "heaven", "men", "answer"],
    )?;
    segment.guard(
        31,
        "reasoned",
        &["themselves", "saying", "shall"],
        &["reasoned", "themselves", "saying", "shall", "say"],
    )?;
    segment.call(
        32,
        "men",
        &["shall", "say", "feared"],
        &["men", "shall", "say", "feared", "people"],
    )?;
    segment.guard(
        33,
        "jesus",
        &["answered", "unto", "cannot"],
        &["jesus", "answered", "unto", "cannot", "tell"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        1,
        4,
        "man construct -> move_value man began speak",
        &["man", "began", "speak", "unto", "parables"],
    );
    segment.construct(
        1,
        "man",
        &["began", "speak", "unto"],
        &["man", "began", "speak", "unto", "parables"],
    )?;
    segment.move_value(
        2,
        "season",
        &["sent", "husbandmen", "servant"],
        &["season", "sent", "husbandmen", "servant", "might"],
    )?;
    segment.move_value(
        3,
        "caught",
        &["beat", "sent", "away"],
        &["caught", "beat", "sent", "away", "empty"],
    )?;
    segment.move_value(
        4,
        "again",
        &["sent", "unto", "another"],
        &["again", "sent", "unto", "another", "servant"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        5,
        8,
        "again move_value -> drop_value again sent another",
        &["again", "sent", "another", "killed", "many"],
    );
    segment.move_value(
        5,
        "again",
        &["sent", "another", "killed"],
        &["again", "sent", "another", "killed", "many"],
    )?;
    segment.move_value(
        6,
        "having",
        &["yet", "therefore", "son"],
        &["having", "yet", "therefore", "son", "wellbeloved"],
    )?;
    segment.move_value(
        7,
        "husbandmen",
        &["among", "themselves", "heir"],
        &["husbandmen", "among", "themselves", "heir", "kill"],
    )?;
    segment.drop_value(
        8,
        "took",
        &["killed", "cast", "vineyard"],
        &["took", "killed", "cast", "vineyard"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        9,
        12,
        "lord move_value -> guard -> expression lord shall therefore",
        &["lord", "shall", "therefore", "vineyard", "will"],
    );
    segment.move_value(
        9,
        "lord",
        &["shall", "therefore", "vineyard"],
        &["lord", "shall", "therefore", "vineyard", "will"],
    )?;
    segment.guard(
        10,
        "not",
        &["read", "scripture", "stone"],
        &["not", "read", "scripture", "stone", "builders"],
    )?;
    segment.expression(
        11,
        "lord's",
        &["doing", "marvellous", "eyes"],
        &["lord's", "doing", "marvellous", "eyes"],
    )?;
    segment.move_value(
        12,
        "people",
        &["sought", "lay", "hold"],
        &["people", "sought", "lay", "hold", "feared"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        13,
        16,
        "send move_value send unto certain",
        &["send", "unto", "certain", "pharisees", "herodians"],
    );
    segment.move_value(
        13,
        "send",
        &["unto", "certain", "pharisees"],
        &["send", "unto", "certain", "pharisees", "herodians"],
    )?;
    segment.move_value(
        14,
        "man",
        &["say", "unto", "master"],
        &["man", "say", "unto", "master", "know"],
    )?;
    segment.move_value(
        15,
        "shall",
        &["give", "not", "knowing"],
        &["shall", "give", "not", "knowing", "hypocrisy"],
    )?;
    segment.move_value(
        16,
        "brought",
        &["saith", "unto", "whose"],
        &["brought", "saith", "unto", "whose", "image"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        17,
        20,
        "jesus call -> move_value -> guard jesus answering unto",
        &["jesus", "answering", "unto", "render", "caesar"],
    );
    segment.call(
        17,
        "jesus",
        &["answering", "unto", "render"],
        &["jesus", "answering", "unto", "render", "caesar"],
    )?;
    segment.move_value(
        18,
        "unto",
        &["sadducees", "say", "resurrection"],
        &["unto", "sadducees", "say", "resurrection", "asked"],
    )?;
    segment.guard(
        19,
        "moses",
        &["master", "wrote", "unto"],
        &["moses", "master", "wrote", "unto", "man's"],
    )?;
    segment.guard(
        20,
        "now",
        &["seven", "brethren", "first"],
        &["now", "seven", "brethren", "first", "took"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        21,
        24,
        "second drop_value -> reinitialize -> guard second took died",
        &["second", "took", "died", "neither", "left"],
    );
    segment.drop_value(
        21,
        "second",
        &["took", "died", "neither"],
        &["second", "took", "died", "neither", "left"],
    )?;
    segment.drop_value(
        22,
        "woman",
        &["seven", "left", "seed"],
        &["woman", "seven", "left", "seed", "last"],
    )?;
    segment.reinitialize(
        23,
        "resurrection",
        &["therefore", "shall", "rise"],
        &["resurrection", "therefore", "shall", "rise", "whose"],
    )?;
    segment.guard(
        24,
        "jesus",
        &["answering", "unto", "not"],
        &["jesus", "answering", "unto", "not", "therefore"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        25,
        28,
        "shall drop_value -> move_value shall rise dead",
        &["shall", "rise", "dead", "neither", "marry"],
    );
    segment.drop_value(
        25,
        "shall",
        &["rise", "dead", "neither"],
        &["shall", "rise", "dead", "neither", "marry"],
    )?;
    segment.drop_value(
        26,
        "moses",
        &["touching", "dead", "rise"],
        &["moses", "touching", "dead", "rise", "not"],
    )?;
    segment.drop_value(
        27,
        "god",
        &["not", "dead", "living"],
        &["god", "not", "dead", "living", "therefore"],
    )?;
    segment.move_value(
        28,
        "scribes",
        &["having", "heard", "reasoning"],
        &["scribes", "having", "heard", "reasoning", "together"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        29,
        32,
        "jesus call -> guard jesus answered first",
        &["jesus", "answered", "first", "commandments", "hear"],
    );
    segment.call(
        29,
        "jesus",
        &["answered", "first", "commandments"],
        &["jesus", "answered", "first", "commandments", "hear"],
    )?;
    segment.guard(
        30,
        "lord",
        &["shalt", "love", "god"],
        &["lord", "shalt", "love", "god", "heart"],
    )?;
    segment.guard(
        31,
        "second",
        &["like", "namely", "shalt"],
        &["second", "like", "namely", "shalt", "love"],
    )?;
    segment.call(
        32,
        "god",
        &["scribe", "unto", "well"],
        &["god", "scribe", "unto", "well", "master"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        33,
        36,
        "love borrow -> guard -> call -> construct love heart understanding",
        &["love", "heart", "understanding", "soul", "strength"],
    );
    segment.borrow(
        33,
        "love",
        &["heart", "understanding", "soul"],
        &["love", "heart", "understanding", "soul", "strength"],
    )?;
    segment.guard(
        34,
        "jesus",
        &["saw", "answered", "discreetly"],
        &["jesus", "saw", "answered", "discreetly", "unto"],
    )?;
    segment.call(
        35,
        "jesus",
        &["answered", "while", "taught"],
        &["jesus", "answered", "while", "taught", "temple"],
    )?;
    segment.construct(
        36,
        "david",
        &["himself", "holy", "ghost"],
        &["david", "himself", "holy", "ghost", "lord"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        37,
        40,
        "david expression -> move_value -> construct david therefore himself",
        &["david", "therefore", "himself", "calleth", "lord"],
    );
    segment.expression(
        37,
        "david",
        &["therefore", "himself", "calleth"],
        &["david", "therefore", "himself", "calleth", "lord"],
    )?;
    segment.move_value(
        38,
        "unto",
        &["doctrine", "beware", "scribes"],
        &["unto", "doctrine", "beware", "scribes", "love"],
    )?;
    segment.expression(
        39,
        "chief",
        &["seats", "synagogues", "uppermost"],
        &["chief", "seats", "synagogues", "uppermost", "rooms"],
    )?;
    segment.construct(
        40,
        "devour",
        &["widows", "houses", "pretence"],
        &["devour", "widows", "houses", "pretence", "make"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_12_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        12,
        41,
        44,
        "jesus expression -> construct -> call jesus sat over",
        &["jesus", "sat", "over", "against", "treasury"],
    );
    segment.expression(
        41,
        "jesus",
        &["sat", "over", "against"],
        &["jesus", "sat", "over", "against", "treasury"],
    )?;
    segment.construct(
        42,
        "certain",
        &["poor", "widow", "threw"],
        &["certain", "poor", "widow", "threw", "two"],
    )?;
    segment.call(
        43,
        "called",
        &["unto", "disciples", "saith"],
        &["called", "unto", "disciples", "saith", "verily"],
    )?;
    segment.expression(
        44,
        "cast",
        &["abundance", "want", "even"],
        &["cast", "abundance", "want", "even", "living"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        1,
        4,
        "went move_value -> guard -> result -> expression went temple disciples",
        &["went", "temple", "disciples", "saith", "unto"],
    );
    segment.move_value(
        1,
        "went",
        &["temple", "disciples", "saith"],
        &["went", "temple", "disciples", "saith", "unto"],
    )?;
    segment.guard(
        2,
        "jesus",
        &["answering", "unto", "seest"],
        &["jesus", "answering", "unto", "seest", "great"],
    )?;
    segment.result(
        3,
        "peter",
        &["sat", "mount", "olives"],
        &["peter", "sat", "mount", "olives", "over"],
    )?;
    segment.expression(
        4,
        "tell",
        &["shall", "things", "sign"],
        &["tell", "shall", "things", "sign", "fulfilled"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        5,
        8,
        "jesus call -> move_value -> guard -> expression jesus answering began",
        &["jesus", "answering", "began", "say", "take"],
    );
    segment.call(
        5,
        "jesus",
        &["answering", "began", "say"],
        &["jesus", "answering", "began", "say", "take"],
    )?;
    segment.move_value(
        6,
        "christ",
        &["many", "shall", "name"],
        &["christ", "many", "shall", "name", "saying"],
    )?;
    segment.guard(
        7,
        "shall",
        &["hear", "wars", "rumours"],
        &["shall", "hear", "wars", "rumours", "not"],
    )?;
    segment.expression(
        8,
        "nation",
        &["shall", "rise", "against"],
        &["nation", "shall", "rise", "against", "kingdom"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        9,
        12,
        "take move_value -> guard -> drop_value take heed yourselves",
        &["take", "heed", "yourselves", "shall", "deliver"],
    );
    segment.move_value(
        9,
        "take",
        &["heed", "yourselves", "shall"],
        &["take", "heed", "yourselves", "shall", "deliver"],
    )?;
    segment.guard(
        10,
        "nations",
        &["gospel", "must", "first"],
        &["nations", "gospel", "must", "first", "published"],
    )?;
    segment.guard(
        11,
        "shall",
        &["lead", "deliver", "take"],
        &["shall", "lead", "deliver", "take", "thought"],
    )?;
    segment.drop_value(
        12,
        "children",
        &["now", "brother", "shall"],
        &["children", "now", "brother", "shall", "betray"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        13,
        16,
        "men expression -> move_value -> guard men shall hated",
        &["men", "shall", "hated", "name's", "sake"],
    );
    segment.expression(
        13,
        "men",
        &["shall", "hated", "name's"],
        &["men", "shall", "hated", "name's", "sake"],
    )?;
    segment.move_value(
        14,
        "prophet",
        &["shall", "see", "abomination"],
        &["prophet", "shall", "see", "abomination", "desolation"],
    )?;
    segment.move_value(
        15,
        "housetop",
        &["not", "down", "house"],
        &["housetop", "not", "down", "house", "neither"],
    )?;
    segment.guard(
        16,
        "field",
        &["not", "turn", "back"],
        &["field", "not", "turn", "back", "again"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        17,
        20,
        "child panic -> guard -> construct child woe give",
        &["child", "woe", "give", "suck", "days"],
    );
    segment.panic(
        17,
        "child",
        &["woe", "give", "suck"],
        &["child", "woe", "give", "suck", "days"],
    )?;
    segment.guard(
        18,
        "pray",
        &["flight", "not", "winter"],
        &["pray", "flight", "not", "winter"],
    )?;
    segment.construct(
        19,
        "god",
        &["days", "shall", "affliction"],
        &["god", "days", "shall", "affliction", "such"],
    )?;
    segment.guard(
        20,
        "lord",
        &["except", "shortened", "days"],
        &["lord", "except", "shortened", "days", "flesh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        21,
        24,
        "man guard -> expression -> match_value man any shall",
        &["man", "any", "shall", "say", "christ"],
    );
    segment.guard(
        21,
        "man",
        &["any", "shall", "say"],
        &["man", "any", "shall", "say", "christ"],
    )?;
    segment.expression(
        22,
        "false",
        &["christs", "prophets", "shall"],
        &["false", "christs", "prophets", "shall", "rise"],
    )?;
    segment.match_value(
        23,
        "take",
        &["heed", "behold", "foretold"],
        &["take", "heed", "behold", "foretold", "things"],
    )?;
    segment.guard(
        24,
        "days",
        &["tribulation", "sun", "shall"],
        &["days", "tribulation", "sun", "shall", "darkened"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        25,
        28,
        "stars expression -> trait_bound -> move_value stars heaven shall",
        &["stars", "heaven", "shall", "fall", "powers"],
    );
    segment.expression(
        25,
        "stars",
        &["heaven", "shall", "fall"],
        &["stars", "heaven", "shall", "fall", "powers"],
    )?;
    segment.trait_bound(
        26,
        "man",
        &["shall", "see", "son"],
        &["man", "shall", "see", "son", "coming"],
    )?;
    segment.move_value(
        27,
        "shall",
        &["send", "angels", "gather"],
        &["shall", "send", "angels", "gather", "together"],
    )?;
    segment.expression(
        28,
        "now",
        &["learn", "parable", "fig"],
        &["now", "learn", "parable", "fig", "tree"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        29,
        32,
        "like move_value -> guard like manner shall",
        &["like", "manner", "shall", "see", "things"],
    );
    segment.move_value(
        29,
        "like",
        &["manner", "shall", "see"],
        &["like", "manner", "shall", "see", "things"],
    )?;
    segment.guard(
        30,
        "verily",
        &["say", "unto", "generation"],
        &["verily", "say", "unto", "generation", "shall"],
    )?;
    segment.guard(
        31,
        "heaven",
        &["earth", "shall", "pass"],
        &["heaven", "earth", "shall", "pass", "away"],
    )?;
    segment.guard(
        32,
        "man",
        &["day", "hour", "knoweth"],
        &["man", "day", "hour", "knoweth", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_13_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        13,
        33,
        37,
        "take guard -> move_value -> expression -> call take heed watch",
        &["take", "heed", "watch", "pray", "know"],
    );
    segment.guard(
        33,
        "take",
        &["heed", "watch", "pray"],
        &["take", "heed", "watch", "pray", "know"],
    )?;
    segment.move_value(
        34,
        "man",
        &["son", "taking", "far"],
        &["man", "son", "taking", "far", "journey"],
    )?;
    segment.guard(
        35,
        "watch",
        &["therefore", "know", "not"],
        &["watch", "therefore", "know", "not", "master"],
    )?;
    segment.expression(
        36,
        "lest",
        &["coming", "suddenly", "find"],
        &["lest", "coming", "suddenly", "find", "sleeping"],
    )?;
    segment.call(37, "say", &["unto", "watch"], &["say", "unto", "watch"])?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        1,
        4,
        "two drop_value -> guard -> move_value -> construct two days feast",
        &["two", "days", "feast", "passover", "unleavened"],
    );
    segment.drop_value(
        1,
        "two",
        &["days", "feast", "passover"],
        &["two", "days", "feast", "passover", "unleavened"],
    )?;
    segment.guard(
        2,
        "people",
        &["not", "feast", "day"],
        &["people", "not", "feast", "day", "lest"],
    )?;
    segment.move_value(
        3,
        "woman",
        &["bethany", "house", "simon"],
        &["woman", "bethany", "house", "simon", "leper"],
    )?;
    segment.construct(
        4,
        "indignation",
        &["within", "themselves", "why"],
        &["indignation", "within", "themselves", "why", "waste"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        5,
        8,
        "might expression -> call -> guard -> move_value might sold three",
        &["might", "sold", "three", "hundred", "pence"],
    );
    segment.expression(
        5,
        "might",
        &["sold", "three", "hundred"],
        &["might", "sold", "three", "hundred", "pence"],
    )?;
    segment.call(
        6,
        "jesus",
        &["alone", "why", "trouble"],
        &["jesus", "alone", "why", "trouble", "hath"],
    )?;
    segment.guard(
        7,
        "poor",
        &["always", "whensoever", "will"],
        &["poor", "always", "whensoever", "will", "good"],
    )?;
    segment.move_value(
        8,
        "hath",
        &["done", "aforehand", "anoint"],
        &["hath", "done", "aforehand", "anoint", "body"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        9,
        12,
        "verily call -> move_value -> expression verily say unto",
        &["verily", "say", "unto", "wheresoever", "gospel"],
    );
    segment.call(
        9,
        "verily",
        &["say", "unto", "wheresoever"],
        &["verily", "say", "unto", "wheresoever", "gospel"],
    )?;
    segment.move_value(
        10,
        "judas",
        &["iscariot", "twelve", "went"],
        &["judas", "iscariot", "twelve", "went", "unto"],
    )?;
    segment.expression(
        11,
        "heard",
        &["glad", "promised", "give"],
        &["heard", "glad", "promised", "give", "money"],
    )?;
    segment.move_value(
        12,
        "first",
        &["day", "unleavened", "bread"],
        &["first", "day", "unleavened", "bread", "killed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        13,
        16,
        "sendeth move_value -> construct sendeth forth two",
        &["sendeth", "forth", "two", "disciples", "saith"],
    );
    segment.move_value(
        13,
        "sendeth",
        &["forth", "two", "disciples"],
        &["sendeth", "forth", "two", "disciples", "saith"],
    )?;
    segment.move_value(
        14,
        "wheresoever",
        &["shall", "say", "goodman"],
        &["wheresoever", "shall", "say", "goodman", "house"],
    )?;
    segment.construct(
        15,
        "will",
        &["shew", "large", "upper"],
        &["will", "shew", "large", "upper", "room"],
    )?;
    segment.construct(
        16,
        "disciples",
        &["went", "forth", "city"],
        &["disciples", "went", "forth", "city", "found"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        17,
        20,
        "evening expression -> call evening cometh twelve",
        &["evening", "cometh", "twelve", "jesus", "sat"],
    );
    segment.expression(
        17,
        "evening",
        &["cometh", "twelve"],
        &["evening", "cometh", "twelve"],
    )?;
    segment.call(
        18,
        "jesus",
        &["sat", "eat", "verily"],
        &["jesus", "sat", "eat", "verily", "say"],
    )?;
    segment.call(
        19,
        "began",
        &["sorrowful", "say", "unto"],
        &["began", "sorrowful", "say", "unto", "another"],
    )?;
    segment.call(
        20,
        "answered",
        &["unto", "twelve", "dippeth"],
        &["answered", "unto", "twelve", "dippeth", "dish"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        21,
        24,
        "man construct -> call -> expression -> reinitialize man son indeed",
        &["man", "son", "indeed", "goeth", "written"],
    );
    segment.construct(
        21,
        "man",
        &["son", "indeed", "goeth"],
        &["man", "son", "indeed", "goeth", "written"],
    )?;
    segment.call(
        22,
        "jesus",
        &["eat", "took", "bread"],
        &["jesus", "eat", "took", "bread", "blessed"],
    )?;
    segment.expression(
        23,
        "took",
        &["cup", "given", "thanks"],
        &["took", "cup", "given", "thanks", "gave"],
    )?;
    segment.reinitialize(
        24,
        "unto",
        &["blood", "new", "testament"],
        &["unto", "blood", "new", "testament", "shed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        25,
        28,
        "verily reinitialize -> move_value -> expression verily say unto",
        &["verily", "say", "unto", "will", "drink"],
    );
    segment.reinitialize(
        25,
        "verily",
        &["say", "unto", "will"],
        &["verily", "say", "unto", "will", "drink"],
    )?;
    segment.move_value(
        26,
        "sung",
        &["hymn", "went", "mount"],
        &["sung", "hymn", "went", "mount", "olives"],
    )?;
    segment.expression(
        27,
        "jesus",
        &["saith", "unto", "shall"],
        &["jesus", "saith", "unto", "shall", "offended"],
    )?;
    segment.move_value(
        28,
        "risen",
        &["will", "galilee"],
        &["risen", "will", "galilee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        29,
        32,
        "peter guard -> move_value peter unto although",
        &["peter", "unto", "although", "shall", "offended"],
    );
    segment.guard(
        29,
        "peter",
        &["unto", "although", "shall"],
        &["peter", "unto", "although", "shall", "offended"],
    )?;
    segment.guard(
        30,
        "jesus",
        &["saith", "unto", "verily"],
        &["jesus", "saith", "unto", "verily", "say"],
    )?;
    segment.guard(
        31,
        "spake",
        &["vehemently", "die", "thee"],
        &["spake", "vehemently", "die", "thee", "will"],
    )?;
    segment.move_value(
        32,
        "place",
        &["named", "gethsemane", "saith"],
        &["place", "named", "gethsemane", "saith", "disciples"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        33,
        36,
        "peter expression -> drop_value -> move_value -> guard peter taketh james",
        &["peter", "taketh", "james", "john", "began"],
    );
    segment.expression(
        33,
        "peter",
        &["taketh", "james", "john"],
        &["peter", "taketh", "james", "john", "began"],
    )?;
    segment.drop_value(
        34,
        "saith",
        &["unto", "soul", "exceeding"],
        &["saith", "unto", "soul", "exceeding", "sorrowful"],
    )?;
    segment.move_value(
        35,
        "went",
        &["forward", "little", "fell"],
        &["went", "forward", "little", "fell", "ground"],
    )?;
    segment.guard(
        36,
        "abba",
        &["father", "things", "possible"],
        &["abba", "father", "things", "possible", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        37,
        40,
        "peter guard -> result -> move_value peter cometh findeth",
        &["peter", "cometh", "findeth", "sleeping", "saith"],
    );
    segment.guard(
        37,
        "peter",
        &["cometh", "findeth", "sleeping"],
        &["peter", "cometh", "findeth", "sleeping", "saith"],
    )?;
    segment.result(
        38,
        "spirit",
        &["watch", "pray", "lest"],
        &["spirit", "watch", "pray", "lest", "enter"],
    )?;
    segment.move_value(
        39,
        "again",
        &["went", "away", "prayed"],
        &["again", "went", "away", "prayed", "spake"],
    )?;
    segment.guard(
        40,
        "returned",
        &["found", "asleep", "again"],
        &["returned", "found", "asleep", "again", "eyes"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        41,
        44,
        "cometh move_value -> call -> expression cometh third time",
        &["cometh", "third", "time", "saith", "unto"],
    );
    segment.move_value(
        41,
        "cometh",
        &["third", "time", "saith"],
        &["cometh", "third", "time", "saith", "unto"],
    )?;
    segment.move_value(
        42,
        "rise",
        &["betrayeth", "hand"],
        &["rise", "betrayeth", "hand"],
    )?;
    segment.call(
        43,
        "immediately",
        &["while", "yet", "spake"],
        &["immediately", "while", "yet", "spake", "cometh"],
    )?;
    segment.expression(
        44,
        "betrayed",
        &["given", "token", "saying"],
        &["betrayed", "given", "token", "saying", "whomsoever"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        45,
        48,
        "soon move_value -> expression soon goeth straightway",
        &["soon", "goeth", "straightway", "saith", "master"],
    );
    segment.move_value(
        45,
        "soon",
        &["goeth", "straightway", "saith"],
        &["soon", "goeth", "straightway", "saith", "master"],
    )?;
    segment.expression(46, "laid", &["hands", "took"], &["laid", "hands", "took"])?;
    segment.expression(
        47,
        "priest",
        &["stood", "drew", "sword"],
        &["priest", "stood", "drew", "sword", "smote"],
    )?;
    segment.move_value(
        48,
        "jesus",
        &["answered", "unto", "against"],
        &["jesus", "answered", "unto", "against", "thief"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_13(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        49,
        52,
        "daily guard -> move_value -> expression daily temple teaching",
        &["daily", "temple", "teaching", "took", "not"],
    );
    segment.guard(
        49,
        "daily",
        &["temple", "teaching", "took"],
        &["daily", "temple", "teaching", "took", "not"],
    )?;
    segment.move_value(50, "forsook", &["fled"], &["forsook", "fled"])?;
    segment.expression(
        51,
        "man",
        &["followed", "certain", "young"],
        &["man", "followed", "certain", "young", "having"],
    )?;
    segment.move_value(
        52,
        "left",
        &["linen", "cloth", "fled"],
        &["left", "linen", "cloth", "fled", "naked"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_14(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        53,
        56,
        "jesus expression -> drop_value -> guard jesus led away",
        &["jesus", "led", "away", "high", "priest"],
    );
    segment.expression(
        53,
        "jesus",
        &["led", "away", "high"],
        &["jesus", "led", "away", "high", "priest"],
    )?;
    segment.expression(
        54,
        "peter",
        &["followed", "afar", "off"],
        &["peter", "followed", "afar", "off", "even"],
    )?;
    segment.drop_value(
        55,
        "jesus",
        &["chief", "priests", "council"],
        &["jesus", "chief", "priests", "council", "sought"],
    )?;
    segment.guard(
        56,
        "many",
        &["bare", "false", "witness"],
        &["many", "bare", "false", "witness", "against"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_15(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        57,
        60,
        "arose reinitialize -> construct -> guard -> result arose certain bare",
        &["arose", "certain", "bare", "false", "witness"],
    );
    segment.reinitialize(
        57,
        "arose",
        &["certain", "bare", "false"],
        &["arose", "certain", "bare", "false", "witness"],
    )?;
    segment.construct(
        58,
        "heard",
        &["say", "will", "destroy"],
        &["heard", "say", "will", "destroy", "temple"],
    )?;
    segment.guard(
        59,
        "neither",
        &["witness", "agree", "together"],
        &["neither", "witness", "agree", "together"],
    )?;
    segment.result(
        60,
        "priest",
        &["high", "stood", "midst"],
        &["priest", "high", "stood", "midst", "asked"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_16(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        61,
        64,
        "priest result -> call -> expression -> drop_value priest held peace",
        &["priest", "held", "peace", "answered", "nothing"],
    );
    segment.result(
        61,
        "priest",
        &["held", "peace", "answered"],
        &["priest", "held", "peace", "answered", "nothing"],
    )?;
    segment.call(
        62,
        "jesus",
        &["shall", "see", "son"],
        &["jesus", "shall", "see", "son", "man"],
    )?;
    segment.expression(
        63,
        "priest",
        &["high", "rent", "clothes"],
        &["priest", "high", "rent", "clothes", "saith"],
    )?;
    segment.drop_value(
        64,
        "heard",
        &["blasphemy", "think", "condemned"],
        &["heard", "blasphemy", "think", "condemned", "guilty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_17(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        65,
        68,
        "began call -> expression -> match_value -> move_value began spit cover",
        &["began", "spit", "cover", "face", "buffet"],
    );
    segment.call(
        65,
        "began",
        &["spit", "cover", "face"],
        &["began", "spit", "cover", "face", "buffet"],
    )?;
    segment.expression(
        66,
        "peter",
        &["beneath", "palace", "cometh"],
        &["peter", "beneath", "palace", "cometh", "maids"],
    )?;
    segment.match_value(
        67,
        "peter",
        &["saw", "warming", "himself"],
        &["peter", "saw", "warming", "himself", "looked"],
    )?;
    segment.move_value(
        68,
        "denied",
        &["saying", "know", "not"],
        &["denied", "saying", "know", "not", "neither"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_14_group_18(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        14,
        69,
        72,
        "maid match_value -> call -> guard maid saw again",
        &["maid", "saw", "again", "began", "say"],
    );
    segment.match_value(
        69,
        "maid",
        &["saw", "again", "began"],
        &["maid", "saw", "again", "began", "say"],
    )?;
    segment.call(
        70,
        "peter",
        &["denied", "again", "little"],
        &["peter", "denied", "again", "little", "stood"],
    )?;
    segment.guard(
        71,
        "man",
        &["began", "curse", "swear"],
        &["man", "began", "curse", "swear", "saying"],
    )?;
    segment.guard(
        72,
        "peter",
        &["second", "time", "cock"],
        &["peter", "second", "time", "cock", "crew"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        1,
        4,
        "straightway expression -> result -> call straightway morning chief",
        &["straightway", "morning", "chief", "priests", "held"],
    );
    segment.expression(
        1,
        "straightway",
        &["morning", "chief", "priests"],
        &["straightway", "morning", "chief", "priests", "held"],
    )?;
    segment.result(
        2,
        "king",
        &["pilate", "asked", "art"],
        &["king", "pilate", "asked", "art", "jews"],
    )?;
    segment.call(
        3,
        "chief",
        &["priests", "accused", "many"],
        &["chief", "priests", "accused", "many", "things"],
    )?;
    segment.result(
        4,
        "pilate",
        &["asked", "again", "saying"],
        &["pilate", "asked", "again", "saying", "answerest"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        5,
        8,
        "jesus call -> expression -> construct jesus yet answered",
        &["jesus", "yet", "answered", "nothing", "pilate"],
    );
    segment.call(
        5,
        "jesus",
        &["yet", "answered", "nothing"],
        &["jesus", "yet", "answered", "nothing", "pilate"],
    )?;
    segment.expression(
        6,
        "now",
        &["feast", "released", "unto"],
        &["now", "feast", "released", "unto", "prisoner"],
    )?;
    segment.construct(
        7,
        "named",
        &["barabbas", "lay", "bound"],
        &["named", "barabbas", "lay", "bound", "made"],
    )?;
    segment.expression(
        8,
        "multitude",
        &["crying", "aloud", "began"],
        &["multitude", "crying", "aloud", "began", "desire"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        9,
        12,
        "king call -> expression king pilate answered",
        &["king", "pilate", "answered", "saying", "will"],
    );
    segment.call(
        9,
        "king",
        &["pilate", "answered", "saying"],
        &["king", "pilate", "answered", "saying", "will"],
    )?;
    segment.expression(
        10,
        "knew",
        &["chief", "priests", "delivered"],
        &["knew", "chief", "priests", "delivered", "envy"],
    )?;
    segment.expression(
        11,
        "people",
        &["chief", "priests", "moved"],
        &["people", "chief", "priests", "moved", "rather"],
    )?;
    segment.call(
        12,
        "king",
        &["pilate", "answered", "again"],
        &["king", "pilate", "answered", "again", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        13,
        16,
        "cried result -> expression cried again crucify",
        &["cried", "again", "crucify", "pilate", "unto"],
    );
    segment.result(
        13,
        "cried",
        &["again", "crucify"],
        &["cried", "again", "crucify"],
    )?;
    segment.result(
        14,
        "pilate",
        &["unto", "why", "evil"],
        &["pilate", "unto", "why", "evil", "hath"],
    )?;
    segment.expression(
        15,
        "people",
        &["pilate", "willing", "content"],
        &["people", "pilate", "willing", "content", "released"],
    )?;
    segment.expression(
        16,
        "soldiers",
        &["led", "away", "hall"],
        &["soldiers", "led", "away", "hall", "called"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        17,
        20,
        "clothed expression clothed purple platted",
        &["clothed", "purple", "platted", "crown", "thorns"],
    );
    segment.expression(
        17,
        "clothed",
        &["purple", "platted", "crown"],
        &["clothed", "purple", "platted", "crown", "thorns"],
    )?;
    segment.expression(
        18,
        "king",
        &["began", "salute", "hail"],
        &["king", "began", "salute", "hail", "jews"],
    )?;
    segment.expression(
        19,
        "smote",
        &["head", "reed", "spit"],
        &["smote", "head", "reed", "spit", "bowing"],
    )?;
    segment.expression(
        20,
        "mocked",
        &["took", "off", "purple"],
        &["mocked", "took", "off", "purple", "put"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        21,
        24,
        "compel expression -> move_value -> guard -> iterate compel simon cyrenian",
        &["compel", "simon", "cyrenian", "passed", "coming"],
    );
    segment.expression(
        21,
        "compel",
        &["simon", "cyrenian", "passed"],
        &["compel", "simon", "cyrenian", "passed", "coming"],
    )?;
    segment.move_value(
        22,
        "bring",
        &["unto", "place", "golgotha"],
        &["bring", "unto", "place", "golgotha", "interpreted"],
    )?;
    segment.guard(
        23,
        "gave",
        &["drink", "wine", "mingled"],
        &["gave", "drink", "wine", "mingled", "myrrh"],
    )?;
    segment.iterate(
        24,
        "man",
        &["crucified", "parted", "garments"],
        &["man", "crucified", "parted", "garments", "casting"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        25,
        28,
        "third expression -> iterate third hour crucified",
        &["third", "hour", "crucified", "king", "superscription"],
    );
    segment.expression(
        25,
        "third",
        &["hour", "crucified"],
        &["third", "hour", "crucified"],
    )?;
    segment.expression(
        26,
        "king",
        &["superscription", "accusation", "written"],
        &["king", "superscription", "accusation", "written", "over"],
    )?;
    segment.expression(
        27,
        "crucify",
        &["two", "thieves", "right"],
        &["crucify", "two", "thieves", "right", "hand"],
    )?;
    segment.iterate(
        28,
        "scripture",
        &["fulfilled", "saith", "numbered"],
        &[
            "scripture",
            "fulfilled",
            "saith",
            "numbered",
            "transgressors",
        ],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        29,
        32,
        "passed trait_bound -> move_value -> call -> await_value passed railed wagging",
        &["passed", "railed", "wagging", "heads", "saying"],
    );
    segment.trait_bound(
        29,
        "passed",
        &["railed", "wagging", "heads"],
        &["passed", "railed", "wagging", "heads", "saying"],
    )?;
    segment.move_value(
        30,
        "save",
        &["thyself", "down", "cross"],
        &["save", "thyself", "down", "cross"],
    )?;
    segment.call(
        31,
        "likewise",
        &["chief", "priests", "mocking"],
        &["likewise", "chief", "priests", "mocking", "among"],
    )?;
    segment.await_value(
        32,
        "christ",
        &["king", "israel", "descend"],
        &["christ", "king", "israel", "descend", "now"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        33,
        36,
        "sixth move_value -> result -> match_value sixth hour darkness",
        &["sixth", "hour", "darkness", "over", "whole"],
    );
    segment.move_value(
        33,
        "sixth",
        &["hour", "darkness", "over"],
        &["sixth", "hour", "darkness", "over", "whole"],
    )?;
    segment.result(
        34,
        "jesus",
        &["ninth", "hour", "cried"],
        &["jesus", "ninth", "hour", "cried", "loud"],
    )?;
    segment.match_value(
        35,
        "stood",
        &["heard", "behold", "calleth"],
        &["stood", "heard", "behold", "calleth", "elias"],
    )?;
    segment.move_value(
        36,
        "ran",
        &["filled", "spunge", "full"],
        &["ran", "filled", "spunge", "full", "vinegar"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        37,
        40,
        "jesus result -> trait_bound -> expression jesus cried loud",
        &["jesus", "cried", "loud", "voice", "gave"],
    );
    segment.result(
        37,
        "jesus",
        &["cried", "loud", "voice"],
        &["jesus", "cried", "loud", "voice", "gave"],
    )?;
    segment.trait_bound(
        38,
        "veil",
        &["temple", "rent", "twain"],
        &["veil", "temple", "rent", "twain", "top"],
    )?;
    segment.result(
        39,
        "centurion",
        &["stood", "over", "against"],
        &["centurion", "stood", "over", "against", "saw"],
    )?;
    segment.expression(
        40,
        "women",
        &["looking", "afar", "off"],
        &["women", "looking", "afar", "off", "among"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        41,
        44,
        "women move_value -> drop_value women galilee followed",
        &["women", "galilee", "followed", "ministered", "unto"],
    );
    segment.move_value(
        41,
        "women",
        &["galilee", "followed", "ministered"],
        &["women", "galilee", "followed", "ministered", "unto"],
    )?;
    segment.move_value(
        42,
        "now",
        &["even", "preparation", "day"],
        &["now", "even", "preparation", "day", "sabbath"],
    )?;
    segment.move_value(
        43,
        "joseph",
        &["arimathaea", "honourable", "counsellor"],
        &["joseph", "arimathaea", "honourable", "counsellor", "waited"],
    )?;
    segment.drop_value(
        44,
        "pilate",
        &["marvelled", "already", "dead"],
        &["pilate", "marvelled", "already", "dead", "calling"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_15_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        15,
        45,
        47,
        "joseph expression joseph knew centurion",
        &["joseph", "knew", "centurion", "gave", "body"],
    );
    segment.expression(
        45,
        "joseph",
        &["knew", "centurion", "gave"],
        &["joseph", "knew", "centurion", "gave", "body"],
    )?;
    segment.expression(
        46,
        "bought",
        &["fine", "linen", "took"],
        &["bought", "fine", "linen", "took", "down"],
    )?;
    segment.expression(
        47,
        "mary",
        &["magdalene", "mother", "joses"],
        &["mary", "magdalene", "mother", "joses", "beheld"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_16_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        16,
        1,
        4,
        "sabbath move_value -> call -> match_value sabbath past mary",
        &["sabbath", "past", "mary", "magdalene", "mother"],
    );
    segment.move_value(
        1,
        "sabbath",
        &["past", "mary", "magdalene"],
        &["sabbath", "past", "mary", "magdalene", "mother"],
    )?;
    segment.move_value(
        2,
        "very",
        &["early", "morning", "first"],
        &["very", "early", "morning", "first", "day"],
    )?;
    segment.call(
        3,
        "among",
        &["themselves", "shall", "roll"],
        &["among", "themselves", "shall", "roll", "away"],
    )?;
    segment.match_value(
        4,
        "looked",
        &["saw", "stone", "rolled"],
        &["looked", "saw", "stone", "rolled", "away"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_16_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        16,
        5,
        8,
        "man match_value -> reinitialize -> move_value man entering sepulchre",
        &["man", "entering", "sepulchre", "saw", "young"],
    );
    segment.match_value(
        5,
        "man",
        &["entering", "sepulchre", "saw"],
        &["man", "entering", "sepulchre", "saw", "young"],
    )?;
    segment.reinitialize(
        6,
        "jesus",
        &["saith", "unto", "not"],
        &["jesus", "saith", "unto", "not", "affrighted"],
    )?;
    segment.move_value(
        7,
        "peter",
        &["way", "tell", "disciples"],
        &["peter", "way", "tell", "disciples", "goeth"],
    )?;
    segment.move_value(
        8,
        "went",
        &["quickly", "fled", "sepulchre"],
        &["went", "quickly", "fled", "sepulchre", "trembled"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_16_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        16,
        9,
        12,
        "jesus reinitialize -> move_value -> guard jesus now risen",
        &["jesus", "now", "risen", "early", "first"],
    );
    segment.reinitialize(
        9,
        "jesus",
        &["now", "risen", "early"],
        &["jesus", "now", "risen", "early", "first"],
    )?;
    segment.move_value(
        10,
        "went",
        &["told", "mourned", "wept"],
        &["went", "told", "mourned", "wept"],
    )?;
    segment.guard(
        11,
        "heard",
        &["alive", "seen", "believed"],
        &["heard", "alive", "seen", "believed", "not"],
    )?;
    segment.move_value(
        12,
        "appeared",
        &["another", "form", "unto"],
        &["appeared", "another", "form", "unto", "two"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_16_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        16,
        13,
        16,
        "went move_value -> reinitialize -> guard went told unto",
        &["went", "told", "unto", "residue", "neither"],
    );
    segment.move_value(
        13,
        "went",
        &["told", "unto", "residue"],
        &["went", "told", "unto", "residue", "neither"],
    )?;
    segment.reinitialize(
        14,
        "afterward",
        &["appeared", "unto", "eleven"],
        &["afterward", "appeared", "unto", "eleven", "sat"],
    )?;
    segment.move_value(
        15,
        "unto",
        &["world", "preach", "gospel"],
        &["unto", "world", "preach", "gospel", "every"],
    )?;
    segment.guard(
        16,
        "believeth",
        &["baptized", "shall", "saved"],
        &["believeth", "baptized", "shall", "saved", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn mark_16_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Mark,
        16,
        17,
        20,
        "signs reinitialize -> guard -> expression -> move_value signs shall follow",
        &["signs", "shall", "follow", "believe", "name"],
    );
    segment.reinitialize(
        17,
        "signs",
        &["shall", "follow", "believe"],
        &["signs", "shall", "follow", "believe", "name"],
    )?;
    segment.guard(
        18,
        "shall",
        &["take", "serpents", "drink"],
        &["shall", "take", "serpents", "drink", "any"],
    )?;
    segment.expression(
        19,
        "lord",
        &["spoken", "unto", "received"],
        &["lord", "spoken", "unto", "received", "heaven"],
    )?;
    segment.move_value(
        20,
        "lord",
        &["went", "forth", "preached"],
        &["lord", "went", "forth", "preached", "every"],
    )?;
    segment.commit()?;
    Ok(())
}
