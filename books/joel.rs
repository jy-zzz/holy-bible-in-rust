use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    joel(story)?;
    Ok(())
}

fn joel(story: &mut Story) -> Result<(), StoryError> {
    joel_1(story)?;
    joel_2(story)?;
    joel_3(story)?;
    Ok(())
}

fn joel_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Joel",
        1,
        "A plague of locusts consumed the land as though an army of tiny automated",
        "jobs had deleted every green asset from the production environment.",
        "Joel summoned elders, farmers, priests, and everyone else to a fast and",
        "a communal incident review, because the disaster was both ecological",
        "grief and a warning that the day of the Maintainer was approaching."
    );
    joel_1_group_01(story)?;
    joel_1_group_02(story)?;
    joel_1_group_03(story)?;
    joel_1_group_04(story)?;
    joel_1_group_05(story)?;
    Ok(())
}

fn joel_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Joel",
        2,
        "The advancing swarm sounded like a disciplined release train crossing",
        "hills, walls, and windows, leaving no shortcut through the outage.",
        "Yet the call was to return with the whole heart, not theatrical patches;",
        "afterward the Maintainer would restore grain, pour out the Spirit on",
        "every kind of contributor, and make prophecy a shared interface."
    );
    joel_2_group_01(story)?;
    joel_2_group_02(story)?;
    joel_2_group_03(story)?;
    joel_2_group_04(story)?;
    joel_2_group_05(story)?;
    joel_2_group_06(story)?;
    joel_2_group_07(story)?;
    joel_2_group_08(story)?;
    Ok(())
}

fn joel_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Joel",
        3,
        "The nations were summoned to a valley of decision where injustice would",
        "be reviewed in public and no empire could hide behind its old ticket queue.",
        "Judah would become a refuge, while the surrounding powers faced a final",
        "harvest of violence; meanwhile the Maintainer's presence would make the",
        "restored city a living source rather than an abandoned artifact."
    );
    joel_3_group_01(story)?;
    joel_3_group_02(story)?;
    joel_3_group_03(story)?;
    joel_3_group_04(story)?;
    joel_3_group_05(story)?;
    Ok(())
}

fn joel_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        1,
        1,
        4,
        "lord move_value -> expression lord word joel",
        &["lord", "word", "joel", "son", "pethuel"],
    );
    segment.move_value(
        1,
        "lord",
        &["word", "joel", "son"],
        &["lord", "word", "joel", "son", "pethuel"],
    )?;
    segment.expression(
        2,
        "men",
        &["hear", "old", "give"],
        &["men", "hear", "old", "give", "ear"],
    )?;
    segment.expression(
        3,
        "children",
        &["tell", "another", "generation"],
        &["children", "tell", "another", "generation"],
    )?;
    segment.expression(
        4,
        "palmerworm",
        &["hath", "left", "locust"],
        &["palmerworm", "hath", "left", "locust", "eaten"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        1,
        5,
        8,
        "awake reinitialize -> move_value -> construct -> expression awake drunkards weep",
        &["awake", "drunkards", "weep", "howl", "drinkers"],
    );
    segment.reinitialize(
        5,
        "awake",
        &["drunkards", "weep", "howl"],
        &["awake", "drunkards", "weep", "howl", "drinkers"],
    )?;
    segment.move_value(
        6,
        "nation",
        &["land", "strong", "without"],
        &["nation", "land", "strong", "without", "number"],
    )?;
    segment.construct(
        7,
        "hath",
        &["laid", "vine", "waste"],
        &["hath", "laid", "vine", "waste", "barked"],
    )?;
    segment.expression(
        8,
        "lament",
        &["like", "virgin", "girded"],
        &["lament", "like", "virgin", "girded", "sackcloth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        1,
        9,
        12,
        "lord trait_bound -> reinitialize -> expression -> iterate lord meat offering",
        &["lord", "meat", "offering", "drink", "cut"],
    );
    segment.trait_bound(
        9,
        "lord",
        &["meat", "offering", "drink"],
        &["lord", "meat", "offering", "drink", "cut"],
    )?;
    segment.reinitialize(
        10,
        "field",
        &["wasted", "land", "mourneth"],
        &["field", "wasted", "land", "mourneth", "corn"],
    )?;
    segment.expression(
        11,
        "ashamed",
        &["husbandmen", "howl", "vinedressers"],
        &["ashamed", "husbandmen", "howl", "vinedressers", "wheat"],
    )?;
    segment.iterate(
        12,
        "vine",
        &["dried", "fig", "tree"],
        &["vine", "dried", "fig", "tree", "languisheth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        1,
        13,
        16,
        "gird move_value -> result -> guard gird yourselves lament",
        &["gird", "yourselves", "lament", "priests", "howl"],
    );
    segment.move_value(
        13,
        "gird",
        &["yourselves", "lament", "priests"],
        &["gird", "yourselves", "lament", "priests", "howl"],
    )?;
    segment.result(
        14,
        "sanctify",
        &["fast", "call", "solemn"],
        &["sanctify", "fast", "call", "solemn", "assembly"],
    )?;
    segment.move_value(
        15,
        "lord",
        &["alas", "day", "hand"],
        &["lord", "alas", "day", "hand", "destruction"],
    )?;
    segment.guard(
        16,
        "not",
        &["meat", "cut", "off"],
        &["not", "meat", "cut", "off", "eyes"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        1,
        17,
        20,
        "seed expression -> construct -> result seed rotten under",
        &["seed", "rotten", "under", "clods", "garners"],
    );
    segment.expression(
        17,
        "seed",
        &["rotten", "under", "clods"],
        &["seed", "rotten", "under", "clods", "garners"],
    )?;
    segment.construct(
        18,
        "beasts",
        &["groan", "herds", "cattle"],
        &["beasts", "groan", "herds", "cattle", "perplexed"],
    )?;
    segment.result(
        19,
        "lord",
        &["thee", "will", "cry"],
        &["lord", "thee", "will", "cry", "fire"],
    )?;
    segment.result(
        20,
        "beasts",
        &["field", "cry", "unto"],
        &["beasts", "field", "cry", "unto", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        1,
        4,
        "blow panic -> guard -> expression blow trumpet zion",
        &["blow", "trumpet", "zion", "sound", "alarm"],
    );
    segment.panic(
        1,
        "blow",
        &["trumpet", "zion", "sound"],
        &["blow", "trumpet", "zion", "sound", "alarm"],
    )?;
    segment.guard(
        2,
        "day",
        &["darkness", "gloominess", "clouds"],
        &["day", "darkness", "gloominess", "clouds", "thick"],
    )?;
    segment.expression(
        3,
        "fire",
        &["devoureth", "behind", "flame"],
        &["fire", "devoureth", "behind", "flame", "burneth"],
    )?;
    segment.expression(
        4,
        "appearance",
        &["horses", "horsemen", "shall"],
        &["appearance", "horses", "horsemen", "shall", "run"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        5,
        8,
        "like expression -> guard like noise chariots",
        &["like", "noise", "chariots", "tops", "mountains"],
    );
    segment.expression(
        5,
        "like",
        &["noise", "chariots", "tops"],
        &["like", "noise", "chariots", "tops", "mountains"],
    )?;
    segment.expression(
        6,
        "people",
        &["face", "shall", "much"],
        &["people", "face", "shall", "much", "pained"],
    )?;
    segment.guard(
        7,
        "men",
        &["shall", "run", "like"],
        &["men", "shall", "run", "like", "mighty"],
    )?;
    segment.guard(
        8,
        "neither",
        &["shall", "thrust", "another"],
        &["neither", "shall", "thrust", "another", "walk"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        9,
        12,
        "shall expression -> panic -> call shall run fro",
        &["shall", "run", "fro", "city", "wall"],
    );
    segment.expression(
        9,
        "shall",
        &["run", "fro", "city"],
        &["shall", "run", "fro", "city", "wall"],
    )?;
    segment.panic(
        10,
        "earth",
        &["shall", "quake", "heavens"],
        &["earth", "shall", "quake", "heavens", "tremble"],
    )?;
    segment.call(
        11,
        "lord",
        &["shall", "utter", "voice"],
        &["lord", "shall", "utter", "voice", "army"],
    )?;
    segment.expression(
        12,
        "lord",
        &["therefore", "now", "saith"],
        &["lord", "therefore", "now", "saith", "turn"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        13,
        16,
        "lord guard -> trait_bound -> expression -> move_value lord rend heart",
        &["lord", "rend", "heart", "not", "garments"],
    );
    segment.guard(
        13,
        "lord",
        &["rend", "heart", "not"],
        &["lord", "rend", "heart", "not", "garments"],
    )?;
    segment.trait_bound(
        14,
        "knoweth",
        &["will", "return", "repent"],
        &["knoweth", "will", "return", "repent", "leave"],
    )?;
    segment.expression(
        15,
        "blow",
        &["trumpet", "zion", "sanctify"],
        &["blow", "trumpet", "zion", "sanctify", "fast"],
    )?;
    segment.move_value(
        16,
        "people",
        &["gather", "sanctify", "congregation"],
        &["people", "gather", "sanctify", "congregation", "assemble"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        17,
        20,
        "lord guard -> expression -> construct -> move_value lord priests ministers",
        &["lord", "priests", "ministers", "weep", "between"],
    );
    segment.guard(
        17,
        "lord",
        &["priests", "ministers", "weep"],
        &["lord", "priests", "ministers", "weep", "between"],
    )?;
    segment.expression(
        18,
        "lord",
        &["will", "jealous", "land"],
        &["lord", "will", "jealous", "land", "pity"],
    )?;
    segment.construct(
        19,
        "lord",
        &["yea", "will", "answer"],
        &["lord", "yea", "will", "answer", "say"],
    )?;
    segment.move_value(
        20,
        "will",
        &["remove", "far", "off"],
        &["will", "remove", "far", "off", "northern"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        21,
        24,
        "lord guard -> move_value -> expression lord fear not",
        &["lord", "fear", "not", "land", "glad"],
    );
    segment.guard(
        21,
        "lord",
        &["fear", "not", "land"],
        &["lord", "fear", "not", "land", "glad"],
    )?;
    segment.guard(
        22,
        "not",
        &["afraid", "beasts", "field"],
        &["not", "afraid", "beasts", "field", "pastures"],
    )?;
    segment.move_value(
        23,
        "children",
        &["glad", "zion", "rejoice"],
        &["children", "glad", "zion", "rejoice", "lord"],
    )?;
    segment.expression(
        24,
        "floors",
        &["shall", "full", "wheat"],
        &["floors", "shall", "full", "wheat", "vats"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        25,
        28,
        "will move_value -> trait_bound -> expression will restore years",
        &["will", "restore", "years", "locust", "hath"],
    );
    segment.move_value(
        25,
        "will",
        &["restore", "years", "locust"],
        &["will", "restore", "years", "locust", "hath"],
    )?;
    segment.trait_bound(
        26,
        "lord",
        &["shall", "eat", "plenty"],
        &["lord", "shall", "eat", "plenty", "satisfied"],
    )?;
    segment.expression(
        27,
        "israel",
        &["shall", "know", "midst"],
        &["israel", "shall", "know", "midst", "lord"],
    )?;
    segment.move_value(
        28,
        "spirit",
        &["shall", "pass", "afterward"],
        &["spirit", "shall", "pass", "afterward", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_2_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        2,
        29,
        32,
        "spirit expression -> move_value spirit servants handmaids",
        &["spirit", "servants", "handmaids", "days", "will"],
    );
    segment.expression(
        29,
        "spirit",
        &["servants", "handmaids", "days"],
        &["spirit", "servants", "handmaids", "days", "will"],
    )?;
    segment.expression(
        30,
        "will",
        &["shew", "wonders", "heavens"],
        &["will", "shew", "wonders", "heavens", "earth"],
    )?;
    segment.move_value(
        31,
        "sun",
        &["shall", "turned", "darkness"],
        &["sun", "shall", "turned", "darkness", "moon"],
    )?;
    segment.move_value(
        32,
        "lord",
        &["shall", "pass", "whosoever"],
        &["lord", "shall", "pass", "whosoever", "call"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        3,
        1,
        4,
        "judah move_value -> expression -> return_value judah behold days",
        &["judah", "behold", "days", "time", "shall"],
    );
    segment.move_value(
        1,
        "judah",
        &["behold", "days", "time"],
        &["judah", "behold", "days", "time", "shall"],
    )?;
    segment.move_value(
        2,
        "nations",
        &["will", "gather", "bring"],
        &["nations", "will", "gather", "bring", "down"],
    )?;
    segment.expression(
        3,
        "people",
        &["cast", "lots", "given"],
        &["people", "cast", "lots", "given", "boy"],
    )?;
    segment.return_value(
        4,
        "yea",
        &["tyre", "zidon", "coasts"],
        &["yea", "tyre", "zidon", "coasts", "palestine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        3,
        5,
        8,
        "taken expression -> match_value -> iterate taken silver gold",
        &["taken", "silver", "gold", "carried", "temples"],
    );
    segment.expression(
        5,
        "taken",
        &["silver", "gold", "carried"],
        &["taken", "silver", "gold", "carried", "temples"],
    )?;
    segment.expression(
        6,
        "children",
        &["judah", "jerusalem", "sold"],
        &["children", "judah", "jerusalem", "sold", "unto"],
    )?;
    segment.match_value(
        7,
        "behold",
        &["will", "raise", "place"],
        &["behold", "will", "raise", "place", "whither"],
    )?;
    segment.iterate(
        8,
        "children",
        &["will", "sell", "sons"],
        &["children", "will", "sell", "sons", "daughters"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        3,
        9,
        12,
        "men move_value -> call men proclaim among",
        &["men", "proclaim", "among", "gentiles", "prepare"],
    );
    segment.move_value(
        9,
        "men",
        &["proclaim", "among", "gentiles"],
        &["men", "proclaim", "among", "gentiles", "prepare"],
    )?;
    segment.call(
        10,
        "beat",
        &["plowshares", "swords", "pruninghooks"],
        &["beat", "plowshares", "swords", "pruninghooks", "spears"],
    )?;
    segment.move_value(
        11,
        "assemble",
        &["yourselves", "heathen", "gather"],
        &["assemble", "yourselves", "heathen", "gather", "together"],
    )?;
    segment.move_value(
        12,
        "heathen",
        &["wakened", "valley", "jehoshaphat"],
        &["heathen", "wakened", "valley", "jehoshaphat", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_3_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        3,
        13,
        16,
        "put move_value -> expression -> call put sickle harvest",
        &["put", "sickle", "harvest", "ripe", "get"],
    );
    segment.move_value(
        13,
        "put",
        &["sickle", "harvest", "ripe"],
        &["put", "sickle", "harvest", "ripe", "get"],
    )?;
    segment.expression(
        14,
        "lord",
        &["multitudes", "valley", "decision"],
        &["lord", "multitudes", "valley", "decision", "day"],
    )?;
    segment.expression(
        15,
        "sun",
        &["moon", "shall", "darkened"],
        &["sun", "moon", "shall", "darkened", "stars"],
    )?;
    segment.call(
        16,
        "lord",
        &["shall", "roar", "zion"],
        &["lord", "shall", "roar", "zion", "utter"],
    )?;
    segment.commit()?;
    Ok(())
}

fn joel_3_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Joel,
        3,
        17,
        21,
        "lord guard -> move_value -> expression lord shall know",
        &["lord", "shall", "know", "god", "dwelling"],
    );
    segment.guard(
        17,
        "lord",
        &["shall", "know", "god"],
        &["lord", "shall", "know", "god", "dwelling"],
    )?;
    segment.move_value(
        18,
        "shall",
        &["pass", "day", "mountains"],
        &["shall", "pass", "day", "mountains", "drop"],
    )?;
    segment.expression(
        19,
        "egypt",
        &["shall", "desolation", "edom"],
        &["egypt", "shall", "desolation", "edom", "desolate"],
    )?;
    segment.expression(
        20,
        "judah",
        &["shall", "dwell", "ever"],
        &["judah", "shall", "dwell", "ever", "jerusalem"],
    )?;
    segment.guard(
        21,
        "lord",
        &["will", "cleanse", "blood"],
        &["lord", "will", "cleanse", "blood", "not"],
    )?;
    segment.commit()?;
    Ok(())
}
