use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    song_of_solomon(story)?;
    Ok(())
}

fn song_of_solomon(story: &mut Story) -> Result<(), StoryError> {
    song_of_solomon_1(story)?;
    song_of_solomon_2(story)?;
    song_of_solomon_3(story)?;
    song_of_solomon_4(story)?;
    song_of_solomon_5(story)?;
    song_of_solomon_6(story)?;
    song_of_solomon_7(story)?;
    song_of_solomon_8(story)?;
    Ok(())
}

fn song_of_solomon_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 1, "The beloveds begin with desire, praising one another's fragrance, beauty, and nearness rather than discussing abstract architecture.", "The woman's darkened skin and vineyard labor become part of her self-knowledge, while the lovers' private garden language opens the song.");
    song_of_solomon_1_group_01(story)?;
    song_of_solomon_1_group_02(story)?;
    song_of_solomon_1_group_03(story)?;
    song_of_solomon_1_group_04(story)?;
    Ok(())
}

fn song_of_solomon_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 2, "The beloved is compared to a flower among thorns, and the lover to an apple tree offering shade and fruit.", "Spring has arrived with singing birds, but the couple also guards the vineyard from small foxes that can ruin a tender season.");
    song_of_solomon_2_group_01(story)?;
    song_of_solomon_2_group_02(story)?;
    song_of_solomon_2_group_03(story)?;
    song_of_solomon_2_group_04(story)?;
    Ok(())
}

fn song_of_solomon_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 3, "The woman searches the night streets for the one her soul loves, finds him, and refuses to let him go.", "A royal procession appears with incense, guards, and a crowned bridegroom, turning private longing into public celebration.");
    song_of_solomon_3_group_01(story)?;
    song_of_solomon_3_group_02(story)?;
    song_of_solomon_3_group_03(story)?;
    Ok(())
}

fn song_of_solomon_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 4, "The bridegroom praises the beloved from head to heart, calling her a locked garden, a sealed spring, and a source of fragrant delight.", "She invites the north and south winds to move through the garden, and the lovers welcome one another into mutual joy.");
    song_of_solomon_4_group_01(story)?;
    song_of_solomon_4_group_02(story)?;
    song_of_solomon_4_group_03(story)?;
    song_of_solomon_4_group_04(story)?;
    Ok(())
}

fn song_of_solomon_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 5, "The bridegroom knocks, but hesitation and delay turn the night into a search through the city and a beating from watchmen.", "The beloved answers the daughters of Jerusalem by describing her lover's beauty in lavish detail, keeping desire alive through absence.");
    song_of_solomon_5_group_01(story)?;
    song_of_solomon_5_group_02(story)?;
    song_of_solomon_5_group_03(story)?;
    song_of_solomon_5_group_04(story)?;
    Ok(())
}

fn song_of_solomon_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 6, "The search resolves into reunion, and the lovers trade images of gardens, banners, dawn, moon, and sun.", "The beloved is praised as uniquely beautiful, and the surrounding women ask to see the one who moves like a royal procession.");
    song_of_solomon_6_group_01(story)?;
    song_of_solomon_6_group_02(story)?;
    song_of_solomon_6_group_03(story)?;
    Ok(())
}

fn song_of_solomon_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 7, "The bridegroom continues the inventory of the beloved's beauty, from feet and hips to eyes, stature, and breath.", "She answers with confident desire, inviting him into the fields and vineyards where their love can ripen without spectators.");
    song_of_solomon_7_group_01(story)?;
    song_of_solomon_7_group_02(story)?;
    song_of_solomon_7_group_03(story)?;
    Ok(())
}

fn song_of_solomon_8(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Song of Solomon", 8, "The final movement asks for love strong as death, jealous as the grave, and impossible to purchase with all one's wealth.", "The lovers speak of family, vineyards, and a garden, ending with a call to hurry like a gazelle across the fragrant mountains.");
    song_of_solomon_8_group_01(story)?;
    song_of_solomon_8_group_02(story)?;
    song_of_solomon_8_group_03(story)?;
    song_of_solomon_8_group_04(story)?;
    Ok(())
}

fn song_of_solomon_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        1,
        1,
        4,
        "song trait_bound -> borrow -> move_value song songs solomon's",
        &["song", "songs", "solomon's", "kiss", "kisses"],
    );
    segment.trait_bound(
        1,
        "song",
        &["songs", "solomon's"],
        &["song", "songs", "solomon's"],
    )?;
    segment.borrow(
        2,
        "kiss",
        &["kisses", "mouth", "love"],
        &["kiss", "kisses", "mouth", "love", "better"],
    )?;
    segment.borrow(
        3,
        "savour",
        &["good", "ointments", "name"],
        &["savour", "good", "ointments", "name", "ointment"],
    )?;
    segment.move_value(
        4,
        "king",
        &["draw", "will", "run"],
        &["king", "draw", "will", "run", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        1,
        5,
        8,
        "black expression -> construct -> move_value black comely daughters",
        &["black", "comely", "daughters", "jerusalem", "tents"],
    );
    segment.expression(
        5,
        "black",
        &["comely", "daughters", "jerusalem"],
        &["black", "comely", "daughters", "jerusalem", "tents"],
    )?;
    segment.construct(
        6,
        "children",
        &["look", "not", "black"],
        &["children", "look", "not", "black", "sun"],
    )?;
    segment.expression(
        7,
        "tell",
        &["soul", "loveth", "feedest"],
        &["tell", "soul", "loveth", "feedest", "makest"],
    )?;
    segment.move_value(
        8,
        "women",
        &["know", "not", "fairest"],
        &["women", "know", "not", "fairest", "among"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        1,
        9,
        12,
        "compared borrow -> expression -> construct compared thee love",
        &["compared", "thee", "love", "company", "horses"],
    );
    segment.borrow(
        9,
        "compared",
        &["thee", "love", "company"],
        &["compared", "thee", "love", "company", "horses"],
    )?;
    segment.expression(
        10,
        "cheeks",
        &["comely", "rows", "jewels"],
        &["cheeks", "comely", "rows", "jewels", "neck"],
    )?;
    segment.construct(
        11,
        "will",
        &["make", "thee", "borders"],
        &["will", "make", "thee", "borders", "gold"],
    )?;
    segment.expression(
        12,
        "king",
        &["while", "sitteth", "table"],
        &["king", "while", "sitteth", "table", "spikenard"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        1,
        13,
        17,
        "bundle borrow -> match_value -> expression bundle myrrh well",
        &["bundle", "myrrh", "well", "beloved", "unto"],
    );
    segment.borrow(
        13,
        "bundle",
        &["myrrh", "well", "beloved"],
        &["bundle", "myrrh", "well", "beloved", "unto"],
    )?;
    segment.borrow(
        14,
        "beloved",
        &["unto", "cluster", "camphire"],
        &["beloved", "unto", "cluster", "camphire", "vineyards"],
    )?;
    segment.match_value(
        15,
        "behold",
        &["art", "fair", "love"],
        &["behold", "art", "fair", "love", "hast"],
    )?;
    segment.match_value(
        16,
        "behold",
        &["art", "fair", "beloved"],
        &["behold", "art", "fair", "beloved", "yea"],
    )?;
    segment.expression(
        17,
        "beams",
        &["house", "cedar", "rafters"],
        &["beams", "house", "cedar", "rafters", "fir"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        2,
        1,
        4,
        "rose expression -> borrow -> move_value rose sharon lily",
        &["rose", "sharon", "lily", "valleys", "among"],
    );
    segment.expression(
        1,
        "rose",
        &["sharon", "lily", "valleys"],
        &["rose", "sharon", "lily", "valleys"],
    )?;
    segment.borrow(
        2,
        "lily",
        &["among", "thorns", "love"],
        &["lily", "among", "thorns", "love", "daughters"],
    )?;
    segment.borrow(
        3,
        "apple",
        &["tree", "among", "trees"],
        &["apple", "tree", "among", "trees", "wood"],
    )?;
    segment.move_value(
        4,
        "brought",
        &["banqueting", "house", "banner"],
        &["brought", "banqueting", "house", "banner", "over"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        2,
        5,
        8,
        "stay borrow -> expression -> guard -> match_value stay flagons comfort",
        &["stay", "flagons", "comfort", "apples", "sick"],
    );
    segment.borrow(
        5,
        "stay",
        &["flagons", "comfort", "apples"],
        &["stay", "flagons", "comfort", "apples", "sick"],
    )?;
    segment.expression(
        6,
        "left",
        &["hand", "under", "head"],
        &["left", "hand", "under", "head", "right"],
    )?;
    segment.guard(
        7,
        "charge",
        &["daughters", "jerusalem", "roes"],
        &["charge", "daughters", "jerusalem", "roes", "hinds"],
    )?;
    segment.match_value(
        8,
        "voice",
        &["beloved", "behold", "cometh"],
        &["voice", "beloved", "behold", "cometh", "leaping"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        2,
        9,
        12,
        "beloved match_value -> move_value beloved like roe",
        &["beloved", "like", "roe", "young", "hart"],
    );
    segment.match_value(
        9,
        "beloved",
        &["like", "roe", "young"],
        &["beloved", "like", "roe", "young", "hart"],
    )?;
    segment.move_value(
        10,
        "beloved",
        &["spake", "unto", "rise"],
        &["beloved", "spake", "unto", "rise", "love"],
    )?;
    segment.move_value(
        11,
        "winter",
        &["past", "rain", "over"],
        &["winter", "past", "rain", "over", "gone"],
    )?;
    segment.move_value(
        12,
        "flowers",
        &["appear", "earth", "time"],
        &["flowers", "appear", "earth", "time", "singing"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        2,
        13,
        17,
        "fig move_value -> call -> expression -> borrow fig tree putteth",
        &["fig", "tree", "putteth", "forth", "green"],
    );
    segment.move_value(
        13,
        "fig",
        &["tree", "putteth", "forth"],
        &["fig", "tree", "putteth", "forth", "green"],
    )?;
    segment.call(
        14,
        "dove",
        &["art", "clefts", "rock"],
        &["dove", "art", "clefts", "rock", "secret"],
    )?;
    segment.expression(
        15,
        "take",
        &["foxes", "little", "spoil"],
        &["take", "foxes", "little", "spoil", "vines"],
    )?;
    segment.borrow(
        16,
        "beloved",
        &["mine", "feedeth", "among"],
        &["beloved", "mine", "feedeth", "among", "lilies"],
    )?;
    segment.move_value(
        17,
        "until",
        &["day", "break", "shadows"],
        &["until", "day", "break", "shadows", "flee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        3,
        1,
        4,
        "night guard -> move_value night bed sought",
        &["night", "bed", "sought", "soul", "loveth"],
    );
    segment.guard(
        1,
        "night",
        &["bed", "sought", "soul"],
        &["night", "bed", "sought", "soul", "loveth"],
    )?;
    segment.move_value(
        2,
        "will",
        &["rise", "now", "city"],
        &["will", "rise", "now", "city", "streets"],
    )?;
    segment.move_value(
        3,
        "watchmen",
        &["city", "found", "saw"],
        &["watchmen", "city", "found", "saw", "soul"],
    )?;
    segment.move_value(
        4,
        "little",
        &["passed", "found", "soul"],
        &["little", "passed", "found", "soul", "loveth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        3,
        5,
        8,
        "charge guard -> expression -> match_value -> iterate charge daughters jerusalem",
        &["charge", "daughters", "jerusalem", "roes", "hinds"],
    );
    segment.guard(
        5,
        "charge",
        &["daughters", "jerusalem", "roes"],
        &["charge", "daughters", "jerusalem", "roes", "hinds"],
    )?;
    segment.expression(
        6,
        "cometh",
        &["wilderness", "like", "pillars"],
        &["cometh", "wilderness", "like", "pillars", "smoke"],
    )?;
    segment.match_value(
        7,
        "men",
        &["behold", "bed", "solomon's"],
        &["men", "behold", "bed", "solomon's", "threescore"],
    )?;
    segment.iterate(
        8,
        "man",
        &["hold", "swords", "expert"],
        &["man", "hold", "swords", "expert", "war"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        3,
        9,
        11,
        "king construct -> move_value king solomon made",
        &["king", "solomon", "made", "himself", "chariot"],
    );
    segment.construct(
        9,
        "king",
        &["solomon", "made", "himself"],
        &["king", "solomon", "made", "himself", "chariot"],
    )?;
    segment.construct(
        10,
        "made",
        &["pillars", "thereof", "silver"],
        &["made", "pillars", "thereof", "silver", "bottom"],
    )?;
    segment.move_value(
        11,
        "king",
        &["forth", "daughters", "zion"],
        &["king", "forth", "daughters", "zion", "behold"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        4,
        1,
        4,
        "behold match_value -> move_value -> expression behold art fair",
        &["behold", "art", "fair", "love", "hast"],
    );
    segment.match_value(
        1,
        "behold",
        &["art", "fair", "love"],
        &["behold", "art", "fair", "love", "hast"],
    )?;
    segment.move_value(
        2,
        "teeth",
        &["like", "flock", "sheep"],
        &["teeth", "like", "flock", "sheep", "even"],
    )?;
    segment.expression(
        3,
        "lips",
        &["like", "thread", "scarlet"],
        &["lips", "like", "thread", "scarlet", "speech"],
    )?;
    segment.expression(
        4,
        "david",
        &["neck", "like", "tower"],
        &["david", "neck", "like", "tower", "builded"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        4,
        5,
        8,
        "two expression -> move_value -> guard two breasts like",
        &["two", "breasts", "like", "young", "roes"],
    );
    segment.expression(
        5,
        "two",
        &["breasts", "like", "young"],
        &["two", "breasts", "like", "young", "roes"],
    )?;
    segment.move_value(
        6,
        "until",
        &["day", "break", "shadows"],
        &["until", "day", "break", "shadows", "flee"],
    )?;
    segment.guard(
        7,
        "art",
        &["fair", "love", "spot"],
        &["art", "fair", "love", "spot", "thee"],
    )?;
    segment.move_value(
        8,
        "lebanon",
        &["spouse", "look", "top"],
        &["lebanon", "spouse", "look", "top", "amana"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        4,
        9,
        12,
        "hast expression -> borrow hast ravished heart",
        &["hast", "ravished", "heart", "sister", "spouse"],
    );
    segment.expression(
        9,
        "hast",
        &["ravished", "heart", "sister"],
        &["hast", "ravished", "heart", "sister", "spouse"],
    )?;
    segment.borrow(
        10,
        "fair",
        &["love", "sister", "spouse"],
        &["fair", "love", "sister", "spouse", "much"],
    )?;
    segment.expression(
        11,
        "lips",
        &["spouse", "drop", "honeycomb"],
        &["lips", "spouse", "drop", "honeycomb", "honey"],
    )?;
    segment.expression(
        12,
        "garden",
        &["inclosed", "sister", "spouse"],
        &["garden", "inclosed", "sister", "spouse", "spring"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        4,
        13,
        16,
        "plants expression -> move_value plants orchard pomegranates",
        &["plants", "orchard", "pomegranates", "pleasant", "fruits"],
    );
    segment.expression(
        13,
        "plants",
        &["orchard", "pomegranates", "pleasant"],
        &["plants", "orchard", "pomegranates", "pleasant", "fruits"],
    )?;
    segment.expression(
        14,
        "spikenard",
        &["saffron", "calamus", "cinnamon"],
        &["spikenard", "saffron", "calamus", "cinnamon", "trees"],
    )?;
    segment.expression(
        15,
        "fountain",
        &["gardens", "well", "living"],
        &["fountain", "gardens", "well", "living", "waters"],
    )?;
    segment.move_value(
        16,
        "awake",
        &["north", "wind", "south"],
        &["awake", "north", "wind", "south", "blow"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        5,
        1,
        4,
        "garden move_value -> call -> expression -> borrow garden sister spouse",
        &["garden", "sister", "spouse", "gathered", "myrrh"],
    );
    segment.move_value(
        1,
        "garden",
        &["sister", "spouse", "gathered"],
        &["garden", "sister", "spouse", "gathered", "myrrh"],
    )?;
    segment.call(
        2,
        "sleep",
        &["heart", "waketh", "voice"],
        &["sleep", "heart", "waketh", "voice", "beloved"],
    )?;
    segment.expression(
        3,
        "put",
        &["off", "coat", "shall"],
        &["put", "off", "coat", "shall", "washed"],
    )?;
    segment.borrow(
        4,
        "beloved",
        &["put", "hand", "hole"],
        &["beloved", "put", "hand", "hole", "door"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        5,
        5,
        8,
        "rose borrow -> move_value rose open beloved",
        &["rose", "open", "beloved", "hands", "dropped"],
    );
    segment.borrow(
        5,
        "rose",
        &["open", "beloved", "hands"],
        &["rose", "open", "beloved", "hands", "dropped"],
    )?;
    segment.move_value(
        6,
        "opened",
        &["beloved", "withdrawn", "himself"],
        &["opened", "beloved", "withdrawn", "himself", "gone"],
    )?;
    segment.move_value(
        7,
        "watchmen",
        &["went", "city", "found"],
        &["watchmen", "went", "city", "found", "smote"],
    )?;
    segment.borrow(
        8,
        "charge",
        &["daughters", "jerusalem", "find"],
        &["charge", "daughters", "jerusalem", "find", "beloved"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        5,
        9,
        12,
        "women borrow -> expression women beloved another",
        &["women", "beloved", "another", "fairest", "among"],
    );
    segment.borrow(
        9,
        "women",
        &["beloved", "another", "fairest"],
        &["women", "beloved", "another", "fairest", "among"],
    )?;
    segment.borrow(
        10,
        "beloved",
        &["white", "ruddy", "chiefest"],
        &["beloved", "white", "ruddy", "chiefest", "among"],
    )?;
    segment.expression(
        11,
        "head",
        &["most", "fine", "gold"],
        &["head", "most", "fine", "gold", "locks"],
    )?;
    segment.expression(
        12,
        "eyes",
        &["doves", "rivers", "waters"],
        &["eyes", "doves", "rivers", "waters", "washed"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        5,
        13,
        16,
        "cheeks expression -> borrow cheeks bed spices",
        &["cheeks", "bed", "spices", "sweet", "flowers"],
    );
    segment.expression(
        13,
        "cheeks",
        &["bed", "spices", "sweet"],
        &["cheeks", "bed", "spices", "sweet", "flowers"],
    )?;
    segment.expression(
        14,
        "hands",
        &["gold", "rings", "set"],
        &["hands", "gold", "rings", "set", "beryl"],
    )?;
    segment.expression(
        15,
        "legs",
        &["pillars", "marble", "set"],
        &["legs", "pillars", "marble", "set", "sockets"],
    )?;
    segment.borrow(
        16,
        "mouth",
        &["most", "sweet", "yea"],
        &["mouth", "most", "sweet", "yea", "altogether"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        6,
        1,
        4,
        "women move_value -> borrow women whither beloved",
        &["women", "whither", "beloved", "gone", "fairest"],
    );
    segment.move_value(
        1,
        "women",
        &["whither", "beloved", "gone"],
        &["women", "whither", "beloved", "gone", "fairest"],
    )?;
    segment.move_value(
        2,
        "beloved",
        &["gone", "down", "garden"],
        &["beloved", "gone", "down", "garden", "beds"],
    )?;
    segment.borrow(
        3,
        "beloved's",
        &["beloved", "mine", "feedeth"],
        &["beloved's", "beloved", "mine", "feedeth", "among"],
    )?;
    segment.borrow(
        4,
        "art",
        &["beautiful", "love", "tirzah"],
        &["art", "beautiful", "love", "tirzah", "comely"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        6,
        5,
        8,
        "turn expression -> move_value turn away thine",
        &["turn", "away", "thine", "eyes", "overcome"],
    );
    segment.expression(
        5,
        "turn",
        &["away", "thine", "eyes"],
        &["turn", "away", "thine", "eyes", "overcome"],
    )?;
    segment.move_value(
        6,
        "teeth",
        &["flock", "sheep", "washing"],
        &["teeth", "flock", "sheep", "washing", "whereof"],
    )?;
    segment.expression(
        7,
        "piece",
        &["pomegranate", "temples", "within"],
        &["piece", "pomegranate", "temples", "within", "locks"],
    )?;
    segment.expression(
        8,
        "threescore",
        &["queens", "fourscore", "concubines"],
        &["threescore", "queens", "fourscore", "concubines", "virgins"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(Book::Song_Of_Solomon, 6, 9, 13, "dove match_value -> expression -> move_value -> construct -> return_value dove undefiled mother", &["dove", "undefiled", "mother", "choice", "bare"]);
    segment.match_value(
        9,
        "dove",
        &["undefiled", "mother", "choice"],
        &["dove", "undefiled", "mother", "choice", "bare"],
    )?;
    segment.expression(
        10,
        "looketh",
        &["forth", "morning", "fair"],
        &["looketh", "forth", "morning", "fair", "moon"],
    )?;
    segment.move_value(
        11,
        "went",
        &["down", "garden", "nuts"],
        &["went", "down", "garden", "nuts", "see"],
    )?;
    segment.construct(
        12,
        "ever",
        &["aware", "soul", "made"],
        &["ever", "aware", "soul", "made", "like"],
    )?;
    segment.return_value(
        13,
        "return",
        &["shulamite", "look", "thee"],
        &["return", "shulamite", "look", "thee", "will"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        7,
        1,
        4,
        "beautiful expression -> guard beautiful feet shoes",
        &["beautiful", "feet", "shoes", "prince's", "daughter"],
    );
    segment.expression(
        1,
        "beautiful",
        &["feet", "shoes", "prince's"],
        &["beautiful", "feet", "shoes", "prince's", "daughter"],
    )?;
    segment.guard(
        2,
        "navel",
        &["like", "round", "goblet"],
        &["navel", "like", "round", "goblet", "wanteth"],
    )?;
    segment.expression(
        3,
        "two",
        &["breasts", "like", "young"],
        &["two", "breasts", "like", "young", "roes"],
    )?;
    segment.expression(
        4,
        "neck",
        &["tower", "ivory", "thine"],
        &["neck", "tower", "ivory", "thine", "eyes"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        7,
        5,
        8,
        "king expression -> borrow -> move_value king thine head",
        &["king", "thine", "head", "thee", "like"],
    );
    segment.expression(
        5,
        "king",
        &["thine", "head", "thee"],
        &["king", "thine", "head", "thee", "like"],
    )?;
    segment.borrow(
        6,
        "fair",
        &["pleasant", "art", "love"],
        &["fair", "pleasant", "art", "love", "delights"],
    )?;
    segment.expression(
        7,
        "stature",
        &["like", "palm", "tree"],
        &["stature", "like", "palm", "tree", "breasts"],
    )?;
    segment.move_value(
        8,
        "will",
        &["palm", "tree", "take"],
        &["will", "palm", "tree", "take", "hold"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        7,
        9,
        13,
        "roof call -> borrow -> move_value -> match_value -> reinitialize roof mouth like",
        &["roof", "mouth", "like", "best", "wine"],
    );
    segment.call(
        9,
        "roof",
        &["mouth", "like", "best"],
        &["roof", "mouth", "like", "best", "wine"],
    )?;
    segment.borrow(
        10,
        "beloved's",
        &["desire", "toward"],
        &["beloved's", "desire", "toward"],
    )?;
    segment.move_value(
        11,
        "beloved",
        &["forth", "field", "lodge"],
        &["beloved", "forth", "field", "lodge", "villages"],
    )?;
    segment.match_value(
        12,
        "get",
        &["early", "vineyards", "see"],
        &["get", "early", "vineyards", "see", "vine"],
    )?;
    segment.reinitialize(
        13,
        "mandrakes",
        &["give", "smell", "gates"],
        &["mandrakes", "give", "smell", "gates", "manner"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_8_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        8,
        1,
        4,
        "wert guard -> move_value -> expression wert brother sucked",
        &["wert", "brother", "sucked", "breasts", "mother"],
    );
    segment.guard(
        1,
        "wert",
        &["brother", "sucked", "breasts"],
        &["wert", "brother", "sucked", "breasts", "mother"],
    )?;
    segment.move_value(
        2,
        "would",
        &["lead", "thee", "bring"],
        &["would", "lead", "thee", "bring", "mother's"],
    )?;
    segment.expression(
        3,
        "left",
        &["hand", "under", "head"],
        &["left", "hand", "under", "head", "right"],
    )?;
    segment.guard(
        4,
        "charge",
        &["daughters", "jerusalem", "stir"],
        &["charge", "daughters", "jerusalem", "stir", "not"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_8_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        8,
        5,
        8,
        "cometh move_value -> drop_value -> guard cometh wilderness leaning",
        &["cometh", "wilderness", "leaning", "beloved", "raised"],
    );
    segment.move_value(
        5,
        "cometh",
        &["wilderness", "leaning", "beloved"],
        &["cometh", "wilderness", "leaning", "beloved", "raised"],
    )?;
    segment.drop_value(
        6,
        "set",
        &["seal", "thine", "heart"],
        &["set", "seal", "thine", "heart", "arm"],
    )?;
    segment.guard(
        7,
        "many",
        &["waters", "cannot", "quench"],
        &["many", "waters", "cannot", "quench", "love"],
    )?;
    segment.guard(
        8,
        "little",
        &["sister", "hath", "breasts"],
        &["little", "sister", "hath", "breasts", "shall"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_8_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        8,
        9,
        11,
        "wall construct -> expression -> move_value wall will build",
        &["wall", "will", "build", "palace", "silver"],
    );
    segment.construct(
        9,
        "wall",
        &["will", "build", "palace"],
        &["wall", "will", "build", "palace", "silver"],
    )?;
    segment.expression(
        10,
        "wall",
        &["breasts", "like", "towers"],
        &["wall", "breasts", "like", "towers", "eyes"],
    )?;
    segment.move_value(
        11,
        "solomon",
        &["vineyard", "baalhamon", "unto"],
        &["solomon", "vineyard", "baalhamon", "unto", "keepers"],
    )?;
    segment.commit()?;
    Ok(())
}

fn song_of_solomon_8_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Song_Of_Solomon,
        8,
        12,
        14,
        "vineyard guard -> call -> construct vineyard mine solomon",
        &["vineyard", "mine", "solomon", "must", "thousand"],
    );
    segment.guard(
        12,
        "vineyard",
        &["mine", "solomon", "must"],
        &["vineyard", "mine", "solomon", "must", "thousand"],
    )?;
    segment.call(
        13,
        "dwellest",
        &["gardens", "companions", "hearken"],
        &["dwellest", "gardens", "companions", "hearken", "voice"],
    )?;
    segment.construct(
        14,
        "make",
        &["haste", "beloved", "like"],
        &["make", "haste", "beloved", "like", "roe"],
    )?;
    segment.commit()?;
    Ok(())
}
