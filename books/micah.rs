use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    micah(story)?;
    Ok(())
}

fn micah(story: &mut Story) -> Result<(), StoryError> {
    micah_1(story)?;
    micah_2(story)?;
    micah_3(story)?;
    micah_4(story)?;
    micah_5(story)?;
    micah_6(story)?;
    micah_7(story)?;
    Ok(())
}

fn micah_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Micah",
        1,
        "Micah announced that the Maintainer was leaving the heavenly courtroom",
        "to inspect Samaria and Jerusalem, whose corruption had become visible",
        "from every mountain.",
        "The cities were named through a chain of puns and grief, as if each place",
        "name carried its own failing test; the prophet mourned from town to town",
        "while invasion advanced across the map."
    );
    micah_1_group_01(story)?;
    micah_1_group_02(story)?;
    micah_1_group_03(story)?;
    micah_1_group_04(story)?;
    Ok(())
}

fn micah_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Micah",
        2,
        "Powerful people designed evil in their beds, woke up, and executed it",
        "against fields, houses, and families before sunrise.",
        "They wanted prophets who promised pleasant output, but Micah gave a",
        "different migration notice: the land would be redistributed, and a future",
        "shepherd-king would gather the remnant through the broken gate."
    );
    micah_2_group_01(story)?;
    micah_2_group_02(story)?;
    micah_2_group_03(story)?;
    Ok(())
}

fn micah_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Micah",
        3,
        "The rulers processed their neighbors as if people were raw material for",
        "a profitable feature, then expected the Maintainer to answer their calls.",
        "Priests taught for payment, prophets sold predictions, and leaders built",
        "on blood; Micah said the whole city would become a field while justice",
        "was treated as an optional plugin."
    );
    micah_3_group_01(story)?;
    micah_3_group_02(story)?;
    micah_3_group_03(story)?;
    Ok(())
}

fn micah_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Micah",
        4,
        "In a later vision, the mountain of the Maintainer's house rose above the",
        "competing dashboards, and nations came to learn a better way of peace.",
        "Swords became farming tools, exiles were gathered, and the community that",
        "had limped through failure was promised a strong future; the present siege",
        "was real, but it was not the final version."
    );
    micah_4_group_01(story)?;
    micah_4_group_02(story)?;
    micah_4_group_03(story)?;
    Ok(())
}

fn micah_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Micah",
        5,
        "From a small and easily overlooked town would come a ruler whose origin",
        "reached deeper than the current release cycle.",
        "This shepherd would stand firm, protect the flock, and delete the idols,",
        "defenses, and enchanted shortcuts that had made the people trust every",
        "system except the One who gave them life."
    );
    micah_5_group_01(story)?;
    micah_5_group_02(story)?;
    micah_5_group_03(story)?;
    micah_5_group_04(story)?;
    Ok(())
}

fn micah_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Micah",
        6,
        "The Maintainer opened a covenant lawsuit and asked what evidence of",
        "gratitude the people thought would settle the case.",
        "Micah answered for them: not mountains of gifts or extravagant rituals,",
        "but a life that practices justice, cherishes mercy, and walks humbly",
        "rather than pretending a ceremony can hide an unethical codebase."
    );
    micah_6_group_01(story)?;
    micah_6_group_02(story)?;
    micah_6_group_03(story)?;
    micah_6_group_04(story)?;
    Ok(())
}

fn micah_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Micah",
        7,
        "Micah searched the city for a faithful person and found a social network",
        "tangled with betrayal, bribery, and family-level merge conflicts.",
        "Still, the prophet chose to wait for the Maintainer, confident that the",
        "light would return; the book ended by celebrating a God who casts sins",
        "into deep water and keeps ancient promises alive."
    );
    micah_7_group_01(story)?;
    micah_7_group_02(story)?;
    micah_7_group_03(story)?;
    micah_7_group_04(story)?;
    micah_7_group_05(story)?;
    Ok(())
}

fn micah_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        1,
        1,
        4,
        "lord move_value -> trait_bound -> expression lord word micah",
        &["lord", "word", "micah", "morasthite", "days"],
    );
    segment.move_value(
        1,
        "lord",
        &["word", "micah", "morasthite"],
        &["lord", "word", "micah", "morasthite", "days"],
    )?;
    segment.trait_bound(
        2,
        "people",
        &["hear", "hearken", "earth"],
        &["people", "hear", "hearken", "earth", "therein"],
    )?;
    segment.move_value(
        3,
        "lord",
        &["behold", "cometh", "forth"],
        &["lord", "behold", "cometh", "forth", "place"],
    )?;
    segment.expression(
        4,
        "mountains",
        &["shall", "molten", "under"],
        &["mountains", "shall", "molten", "under", "valleys"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        1,
        5,
        8,
        "jacob guard -> construct -> return_value jacob transgression sins",
        &["jacob", "transgression", "sins", "house", "israel"],
    );
    segment.guard(
        5,
        "jacob",
        &["transgression", "sins", "house"],
        &["jacob", "transgression", "sins", "house", "israel"],
    )?;
    segment.construct(
        6,
        "therefore",
        &["will", "make", "samaria"],
        &["therefore", "will", "make", "samaria", "heap"],
    )?;
    segment.return_value(
        7,
        "graven",
        &["images", "thereof", "shall"],
        &["graven", "images", "thereof", "shall", "beaten"],
    )?;
    segment.construct(
        8,
        "therefore",
        &["will", "wail", "howl"],
        &["therefore", "will", "wail", "howl", "stripped"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        1,
        9,
        12,
        "judah move_value -> guard judah wound incurable",
        &["judah", "wound", "incurable", "unto", "gate"],
    );
    segment.move_value(
        9,
        "judah",
        &["wound", "incurable", "unto"],
        &["judah", "wound", "incurable", "unto", "gate"],
    )?;
    segment.guard(
        10,
        "declare",
        &["not", "gath", "weep"],
        &["declare", "not", "gath", "weep", "house"],
    )?;
    segment.move_value(
        11,
        "pass",
        &["away", "inhabitant", "saphir"],
        &["pass", "away", "inhabitant", "saphir", "having"],
    )?;
    segment.move_value(
        12,
        "lord",
        &["inhabitant", "maroth", "waited"],
        &["lord", "inhabitant", "maroth", "waited", "carefully"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        1,
        13,
        16,
        "inhabitant expression -> guard -> move_value -> construct inhabitant lachish bind",
        &["inhabitant", "lachish", "bind", "chariot", "swift"],
    );
    segment.expression(
        13,
        "inhabitant",
        &["lachish", "bind", "chariot"],
        &["inhabitant", "lachish", "bind", "chariot", "swift"],
    )?;
    segment.guard(
        14,
        "therefore",
        &["shalt", "give", "presents"],
        &["therefore", "shalt", "give", "presents", "moreshethgath"],
    )?;
    segment.move_value(
        15,
        "yet",
        &["will", "bring", "heir"],
        &["yet", "will", "bring", "heir", "unto"],
    )?;
    segment.construct(
        16,
        "children",
        &["make", "thee", "bald"],
        &["children", "make", "thee", "bald", "poll"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        2,
        1,
        4,
        "woe panic -> expression -> move_value -> call woe devise iniquity",
        &["woe", "devise", "iniquity", "work", "evil"],
    );
    segment.panic(
        1,
        "woe",
        &["devise", "iniquity", "work"],
        &["woe", "devise", "iniquity", "work", "evil"],
    )?;
    segment.expression(
        2,
        "man",
        &["covet", "fields", "take"],
        &["man", "covet", "fields", "take", "violence"],
    )?;
    segment.move_value(
        3,
        "lord",
        &["therefore", "thus", "saith"],
        &["lord", "therefore", "thus", "saith", "behold"],
    )?;
    segment.call(
        4,
        "day",
        &["shall", "take", "parable"],
        &["day", "shall", "take", "parable", "against"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        2,
        5,
        8,
        "therefore guard -> reinitialize therefore shalt none",
        &["therefore", "shalt", "none", "shall", "cast"],
    );
    segment.guard(
        5,
        "therefore",
        &["shalt", "none", "shall"],
        &["therefore", "shalt", "none", "shall", "cast"],
    )?;
    segment.guard(
        6,
        "prophesy",
        &["not", "say", "shall"],
        &["prophesy", "not", "say", "shall", "take"],
    )?;
    segment.guard(
        7,
        "jacob",
        &["art", "named", "house"],
        &["jacob", "art", "named", "house", "spirit"],
    )?;
    segment.reinitialize(
        8,
        "people",
        &["even", "late", "risen"],
        &["people", "even", "late", "risen", "enemy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        2,
        9,
        13,
        "women trait_bound -> move_value -> expression -> construct women people cast",
        &["women", "people", "cast", "pleasant", "houses"],
    );
    segment.trait_bound(
        9,
        "women",
        &["people", "cast", "pleasant"],
        &["women", "people", "cast", "pleasant", "houses"],
    )?;
    segment.move_value(
        10,
        "arise",
        &["depart", "not", "rest"],
        &["arise", "depart", "not", "rest", "polluted"],
    )?;
    segment.expression(
        11,
        "man",
        &["walking", "spirit", "falsehood"],
        &["man", "walking", "spirit", "falsehood", "lie"],
    )?;
    segment.construct(
        12,
        "jacob",
        &["will", "surely", "assemble"],
        &["jacob", "will", "surely", "assemble", "thee"],
    )?;
    segment.move_value(
        13,
        "king",
        &["breaker", "broken", "passed"],
        &["king", "breaker", "broken", "passed", "through"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        3,
        1,
        4,
        "jacob guard -> borrow -> expression jacob hear pray",
        &["jacob", "hear", "pray", "heads", "princes"],
    );
    segment.guard(
        1,
        "jacob",
        &["hear", "pray", "heads"],
        &["jacob", "hear", "pray", "heads", "princes"],
    )?;
    segment.borrow(
        2,
        "hate",
        &["good", "love", "evil"],
        &["hate", "good", "love", "evil", "pluck"],
    )?;
    segment.expression(
        3,
        "people",
        &["eat", "flesh", "flay"],
        &["people", "eat", "flesh", "flay", "skin"],
    )?;
    segment.guard(
        4,
        "lord",
        &["shall", "cry", "unto"],
        &["lord", "shall", "cry", "unto", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        3,
        5,
        8,
        "lord construct -> move_value -> guard -> call lord thus saith",
        &["lord", "thus", "saith", "concerning", "prophets"],
    );
    segment.construct(
        5,
        "lord",
        &["thus", "saith", "concerning"],
        &["lord", "thus", "saith", "concerning", "prophets"],
    )?;
    segment.move_value(
        6,
        "therefore",
        &["night", "shall", "unto"],
        &["therefore", "night", "shall", "unto", "not"],
    )?;
    segment.guard(
        7,
        "shall",
        &["seers", "ashamed", "diviners"],
        &["shall", "seers", "ashamed", "diviners", "confounded"],
    )?;
    segment.call(
        8,
        "spirit",
        &["truly", "full", "power"],
        &["spirit", "truly", "full", "power", "lord"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        3,
        9,
        12,
        "jacob result -> construct -> move_value -> expression jacob hear pray",
        &["jacob", "hear", "pray", "heads", "house"],
    );
    segment.result(
        9,
        "jacob",
        &["hear", "pray", "heads"],
        &["jacob", "hear", "pray", "heads", "house"],
    )?;
    segment.construct(
        10,
        "build",
        &["zion", "blood", "jerusalem"],
        &["build", "zion", "blood", "jerusalem", "iniquity"],
    )?;
    segment.move_value(
        11,
        "heads",
        &["thereof", "judge", "reward"],
        &["heads", "thereof", "judge", "reward", "priests"],
    )?;
    segment.expression(
        12,
        "therefore",
        &["shall", "zion", "sake"],
        &["therefore", "shall", "zion", "sake", "plowed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        4,
        1,
        4,
        "lord move_value -> guard -> construct lord last days",
        &["lord", "last", "days", "shall", "pass"],
    );
    segment.move_value(
        1,
        "lord",
        &["last", "days", "shall"],
        &["lord", "last", "days", "shall", "pass"],
    )?;
    segment.move_value(
        2,
        "nations",
        &["many", "shall", "say"],
        &["nations", "many", "shall", "say", "mountain"],
    )?;
    segment.guard(
        3,
        "people",
        &["shall", "judge", "among"],
        &["people", "shall", "judge", "among", "many"],
    )?;
    segment.construct(
        4,
        "man",
        &["shall", "sit", "every"],
        &["man", "shall", "sit", "every", "under"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        4,
        5,
        8,
        "people iterate -> expression -> construct -> move_value people will walk",
        &["people", "will", "walk", "every", "name"],
    );
    segment.iterate(
        5,
        "people",
        &["will", "walk", "every"],
        &["people", "will", "walk", "every", "name"],
    )?;
    segment.expression(
        6,
        "lord",
        &["day", "saith", "will"],
        &["lord", "day", "saith", "will", "assemble"],
    )?;
    segment.construct(
        7,
        "will",
        &["make", "halted", "remnant"],
        &["will", "make", "halted", "remnant", "cast"],
    )?;
    segment.move_value(
        8,
        "tower",
        &["flock", "strong", "hold"],
        &["tower", "flock", "strong", "hold", "daughter"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        4,
        9,
        13,
        "king guard -> move_value -> call -> construct king now why",
        &["king", "now", "why", "dost", "cry"],
    );
    segment.guard(
        9,
        "king",
        &["now", "why", "dost"],
        &["king", "now", "why", "dost", "cry"],
    )?;
    segment.move_value(
        10,
        "woman",
        &["pain", "labour", "bring"],
        &["woman", "pain", "labour", "bring", "forth"],
    )?;
    segment.call(
        11,
        "nations",
        &["now", "many", "gathered"],
        &["nations", "now", "many", "gathered", "against"],
    )?;
    segment.guard(
        12,
        "lord",
        &["know", "not", "thoughts"],
        &["lord", "know", "not", "thoughts", "neither"],
    )?;
    segment.construct(
        13,
        "arise",
        &["thresh", "daughter", "zion"],
        &["arise", "thresh", "daughter", "zion", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        5,
        1,
        4,
        "now expression -> move_value now gather thyself",
        &["now", "gather", "thyself", "troops", "daughter"],
    );
    segment.expression(
        1,
        "now",
        &["gather", "thyself", "troops"],
        &["now", "gather", "thyself", "troops", "daughter"],
    )?;
    segment.move_value(
        2,
        "judah",
        &["bethlehem", "ephratah", "though"],
        &["judah", "bethlehem", "ephratah", "though", "little"],
    )?;
    segment.move_value(
        3,
        "therefore",
        &["will", "give", "until"],
        &["therefore", "will", "give", "until", "time"],
    )?;
    segment.expression(
        4,
        "lord",
        &["shall", "stand", "feed"],
        &["lord", "shall", "stand", "feed", "strength"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        5,
        5,
        8,
        "man move_value -> expression -> guard man shall peace",
        &["man", "shall", "peace", "assyrian", "land"],
    );
    segment.move_value(
        5,
        "man",
        &["shall", "peace", "assyrian"],
        &["man", "shall", "peace", "assyrian", "land"],
    )?;
    segment.expression(
        6,
        "shall",
        &["waste", "land", "assyria"],
        &["shall", "waste", "land", "assyria", "sword"],
    )?;
    segment.guard(
        7,
        "jacob",
        &["remnant", "shall", "midst"],
        &["jacob", "remnant", "shall", "midst", "many"],
    )?;
    segment.move_value(
        8,
        "jacob",
        &["remnant", "shall", "among"],
        &["jacob", "remnant", "shall", "among", "gentiles"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        5,
        9,
        12,
        "thine expression -> move_value -> guard thine hand shall",
        &["thine", "hand", "shall", "lifted", "adversaries"],
    );
    segment.expression(
        9,
        "thine",
        &["hand", "shall", "lifted"],
        &["thine", "hand", "shall", "lifted", "adversaries"],
    )?;
    segment.move_value(
        10,
        "lord",
        &["shall", "pass", "day"],
        &["lord", "shall", "pass", "day", "saith"],
    )?;
    segment.expression(
        11,
        "will",
        &["cut", "off", "cities"],
        &["will", "cut", "off", "cities", "land"],
    )?;
    segment.guard(
        12,
        "will",
        &["cut", "off", "witchcrafts"],
        &["will", "cut", "off", "witchcrafts", "thine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        5,
        13,
        15,
        "graven guard -> drop_value graven images will",
        &["graven", "images", "will", "cut", "off"],
    );
    segment.guard(
        13,
        "graven",
        &["images", "will", "cut"],
        &["graven", "images", "will", "cut", "off"],
    )?;
    segment.drop_value(
        14,
        "will",
        &["pluck", "groves", "midst"],
        &["will", "pluck", "groves", "midst", "thee"],
    )?;
    segment.guard(
        15,
        "will",
        &["execute", "vengeance", "anger"],
        &["will", "execute", "vengeance", "anger", "fury"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        6,
        1,
        4,
        "lord call -> expression -> move_value lord hear now",
        &["lord", "hear", "now", "saith", "arise"],
    );
    segment.call(
        1,
        "lord",
        &["hear", "now", "saith"],
        &["lord", "hear", "now", "saith", "arise"],
    )?;
    segment.expression(
        2,
        "lord",
        &["hear", "mountains", "lord's"],
        &["lord", "hear", "mountains", "lord's", "controversy"],
    )?;
    segment.expression(
        3,
        "people",
        &["done", "unto", "thee"],
        &["people", "done", "unto", "thee", "wherein"],
    )?;
    segment.move_value(
        4,
        "brought",
        &["thee", "land", "egypt"],
        &["brought", "thee", "land", "egypt", "redeemed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        6,
        5,
        8,
        "people call -> move_value -> expression -> borrow people remember now",
        &["people", "remember", "now", "balak", "king"],
    );
    segment.call(
        5,
        "people",
        &["remember", "now", "balak"],
        &["people", "remember", "now", "balak", "king"],
    )?;
    segment.move_value(
        6,
        "lord",
        &["wherewith", "shall", "bow"],
        &["lord", "wherewith", "shall", "bow", "myself"],
    )?;
    segment.expression(
        7,
        "lord",
        &["will", "pleased", "thousands"],
        &["lord", "will", "pleased", "thousands", "rams"],
    )?;
    segment.borrow(
        8,
        "man",
        &["hath", "shewed", "thee"],
        &["man", "hath", "shewed", "thee", "good"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        6,
        9,
        12,
        "man call -> expression man lord's voice",
        &["man", "lord's", "voice", "crieth", "unto"],
    );
    segment.call(
        9,
        "man",
        &["lord's", "voice", "crieth"],
        &["man", "lord's", "voice", "crieth", "unto"],
    )?;
    segment.expression(
        10,
        "yet",
        &["treasures", "wickedness", "house"],
        &["yet", "treasures", "wickedness", "house", "wicked"],
    )?;
    segment.expression(
        11,
        "shall",
        &["count", "pure", "wicked"],
        &["shall", "count", "pure", "wicked", "balances"],
    )?;
    segment.expression(
        12,
        "men",
        &["rich", "thereof", "full"],
        &["men", "rich", "thereof", "full", "violence"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        6,
        13,
        16,
        "therefore construct -> guard therefore will make",
        &["therefore", "will", "make", "thee", "sick"],
    );
    segment.construct(
        13,
        "therefore",
        &["will", "make", "thee"],
        &["therefore", "will", "make", "thee", "sick"],
    )?;
    segment.guard(
        14,
        "shalt",
        &["eat", "not", "satisfied"],
        &["shalt", "eat", "not", "satisfied", "casting"],
    )?;
    segment.guard(
        15,
        "shalt",
        &["sow", "not", "reap"],
        &["shalt", "sow", "not", "reap", "tread"],
    )?;
    segment.construct(
        16,
        "statutes",
        &["omri", "kept", "works"],
        &["statutes", "omri", "kept", "works", "house"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        7,
        1,
        4,
        "woe guard -> await_value -> expression woe gathered summer",
        &["woe", "gathered", "summer", "fruits", "grapegleanings"],
    );
    segment.guard(
        1,
        "woe",
        &["gathered", "summer", "fruits"],
        &["woe", "gathered", "summer", "fruits", "grapegleanings"],
    )?;
    segment.await_value(
        2,
        "man",
        &["good", "perished", "earth"],
        &["man", "good", "perished", "earth", "none"],
    )?;
    segment.expression(
        3,
        "evil",
        &["both", "hands", "earnestly"],
        &["evil", "both", "hands", "earnestly", "prince"],
    )?;
    segment.expression(
        4,
        "best",
        &["brier", "most", "upright"],
        &["best", "brier", "most", "upright", "sharper"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        7,
        5,
        8,
        "trust guard -> await_value trust not friend",
        &["trust", "not", "friend", "put", "confidence"],
    );
    segment.guard(
        5,
        "trust",
        &["not", "friend", "put"],
        &["trust", "not", "friend", "put", "confidence"],
    )?;
    segment.guard(
        6,
        "son",
        &["dishonoureth", "father", "daughter"],
        &["son", "dishonoureth", "father", "daughter", "riseth"],
    )?;
    segment.await_value(
        7,
        "lord",
        &["therefore", "will", "look"],
        &["lord", "therefore", "will", "look", "unto"],
    )?;
    segment.guard(
        8,
        "rejoice",
        &["not", "against", "mine"],
        &["rejoice", "not", "against", "mine", "enemy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        7,
        9,
        12,
        "lord move_value -> match_value -> construct lord will bear",
        &["lord", "will", "bear", "indignation", "sinned"],
    );
    segment.move_value(
        9,
        "lord",
        &["will", "bear", "indignation"],
        &["lord", "will", "bear", "indignation", "sinned"],
    )?;
    segment.match_value(
        10,
        "lord",
        &["mine", "enemy", "shall"],
        &["lord", "mine", "enemy", "shall", "see"],
    )?;
    segment.construct(
        11,
        "day",
        &["walls", "built", "shall"],
        &["day", "walls", "built", "shall", "decree"],
    )?;
    segment.move_value(
        12,
        "day",
        &["shall", "even", "thee"],
        &["day", "shall", "even", "thee", "assyria"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_7_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        7,
        13,
        16,
        "notwithstanding expression notwithstanding land shall",
        &["notwithstanding", "land", "shall", "desolate", "dwell"],
    );
    segment.expression(
        13,
        "notwithstanding",
        &["land", "shall", "desolate"],
        &["notwithstanding", "land", "shall", "desolate", "dwell"],
    )?;
    segment.expression(
        14,
        "people",
        &["feed", "rod", "flock"],
        &["people", "feed", "rod", "flock", "thine"],
    )?;
    segment.expression(
        15,
        "according",
        &["days", "coming", "land"],
        &["according", "days", "coming", "land", "egypt"],
    )?;
    segment.expression(
        16,
        "nations",
        &["shall", "see", "confounded"],
        &["nations", "shall", "see", "confounded", "might"],
    )?;
    segment.commit()?;
    Ok(())
}

fn micah_7_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Micah,
        7,
        17,
        20,
        "shall panic -> guard -> borrow shall lick dust",
        &["shall", "lick", "dust", "like", "serpent"],
    );
    segment.panic(
        17,
        "shall",
        &["lick", "dust", "like"],
        &["shall", "lick", "dust", "like", "serpent"],
    )?;
    segment.guard(
        18,
        "god",
        &["like", "unto", "thee"],
        &["god", "like", "unto", "thee", "pardoneth"],
    )?;
    segment.borrow(
        19,
        "will",
        &["turn", "again", "compassion"],
        &["will", "turn", "again", "compassion", "subdue"],
    )?;
    segment.borrow(
        20,
        "jacob",
        &["wilt", "perform", "truth"],
        &["jacob", "wilt", "perform", "truth", "mercy"],
    )?;
    segment.commit()?;
    Ok(())
}
