use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    ezra(story)?;
    Ok(())
}

fn ezra(story: &mut Story) -> Result<(), StoryError> {
    ezra_1(story)?;
    ezra_2(story)?;
    ezra_3(story)?;
    ezra_4(story)?;
    ezra_5(story)?;
    ezra_6(story)?;
    ezra_7(story)?;
    ezra_8(story)?;
    ezra_9(story)?;
    ezra_10(story)?;
    Ok(())
}

fn ezra_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 1, "Cyrus announces that the Lord has charged him to rebuild the temple, and exiles receive permission, vessels, and travel support.", "Sheshbazzar carries the returned holy inventory, making restoration a physical migration as well as a political decree.");
    ezra_1_group_01(story)?;
    ezra_1_group_02(story)?;
    ezra_1_group_03(story)?;
    Ok(())
}

fn ezra_2(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 2, "A long registry counts returning families, priests, Levites, singers, gatekeepers, and temple servants.", "Some priests cannot prove their ancestry and are paused from the priestly queue until a qualified decision can be made.");
    ezra_2_group_01(story)?;
    ezra_2_group_02(story)?;
    ezra_2_group_03(story)?;
    ezra_2_group_04(story)?;
    ezra_2_group_05(story)?;
    ezra_2_group_06(story)?;
    ezra_2_group_07(story)?;
    ezra_2_group_08(story)?;
    ezra_2_group_09(story)?;
    ezra_2_group_10(story)?;
    ezra_2_group_11(story)?;
    ezra_2_group_12(story)?;
    ezra_2_group_13(story)?;
    ezra_2_group_14(story)?;
    ezra_2_group_15(story)?;
    ezra_2_group_16(story)?;
    ezra_2_group_17(story)?;
    ezra_2_group_18(story)?;
    Ok(())
}

fn ezra_3(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 3, "The people rebuild the altar, keep the feast, and lay the temple foundation while old priests weep and younger people shout.", "The mixed sound records both memory and hope, because restoration rarely produces one clean emotional output.");
    ezra_3_group_01(story)?;
    ezra_3_group_02(story)?;
    ezra_3_group_03(story)?;
    Ok(())
}

fn ezra_4(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 4, "Local opponents offer partnership, but Zerubbabel refuses their unauthorized access and the project faces accusation letters.", "Imperial orders pause the construction, showing how political permissions can interrupt sacred work even after a favorable decree.");
    ezra_4_group_01(story)?;
    ezra_4_group_02(story)?;
    ezra_4_group_03(story)?;
    ezra_4_group_04(story)?;
    ezra_4_group_05(story)?;
    ezra_4_group_06(story)?;
    Ok(())
}

fn ezra_5(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 5, "Haggai and Zechariah prophesy, and Zerubbabel and Jeshua restart the temple despite the governor's investigation.", "Tattenai requests the original authorization from Darius, turning opposition into a search of the imperial archive.");
    ezra_5_group_01(story)?;
    ezra_5_group_02(story)?;
    ezra_5_group_03(story)?;
    ezra_5_group_04(story)?;
    Ok(())
}

fn ezra_6(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 6, "Darius finds Cyrus's decree, orders the opponents to stop, and funds the rebuilding from regional revenue.", "The temple is completed, dedicated, and supplied, and Passover becomes the first major release celebration of the restored community.");
    ezra_6_group_01(story)?;
    ezra_6_group_02(story)?;
    ezra_6_group_03(story)?;
    ezra_6_group_04(story)?;
    ezra_6_group_05(story)?;
    ezra_6_group_06(story)?;
    Ok(())
}

fn ezra_7(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 7, "Ezra, a skilled scribe, receives Artaxerxes' commission to teach the law and govern the community's religious matters.", "He blesses God for turning the king's heart, framing the journey as a providential permission set rather than a human achievement.");
    ezra_7_group_01(story)?;
    ezra_7_group_02(story)?;
    ezra_7_group_03(story)?;
    ezra_7_group_04(story)?;
    ezra_7_group_05(story)?;
    ezra_7_group_06(story)?;
    ezra_7_group_07(story)?;
    Ok(())
}

fn ezra_8(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 8, "Ezra gathers families by the river, notices no Levites among them, and recruits temple servants before departure.", "The travelers fast for protection, refuse a military escort, and arrive safely after entrusting the road to God.");
    ezra_8_group_01(story)?;
    ezra_8_group_02(story)?;
    ezra_8_group_03(story)?;
    ezra_8_group_04(story)?;
    ezra_8_group_05(story)?;
    ezra_8_group_06(story)?;
    ezra_8_group_07(story)?;
    ezra_8_group_08(story)?;
    ezra_8_group_09(story)?;
    Ok(())
}

fn ezra_9(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 9, "Ezra learns that leaders have intermarried with surrounding peoples and tears his robe in public grief.", "His prayer confesses that the returned remnant has repeated the old covenant failure even after receiving a narrow escape.");
    ezra_9_group_01(story)?;
    ezra_9_group_02(story)?;
    ezra_9_group_03(story)?;
    ezra_9_group_04(story)?;
    Ok(())
}

fn ezra_10(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ezra", 10, "Shecaniah proposes a difficult reform, and the assembly gathers in rain to examine the mixed marriages.", "Families are listed as separating from the prohibited unions, closing the book with a painful communal attempt to repair identity.");
    ezra_10_group_01(story)?;
    ezra_10_group_02(story)?;
    ezra_10_group_03(story)?;
    ezra_10_group_04(story)?;
    ezra_10_group_05(story)?;
    ezra_10_group_06(story)?;
    ezra_10_group_07(story)?;
    ezra_10_group_08(story)?;
    ezra_10_group_09(story)?;
    ezra_10_group_10(story)?;
    ezra_10_group_11(story)?;
    Ok(())
}

fn ezra_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        1,
        1,
        4,
        "king construct -> trait_bound king now first",
        &["king", "now", "first", "year", "cyrus"],
    );
    segment.construct(
        1,
        "king",
        &["now", "first", "year"],
        &["king", "now", "first", "year", "cyrus"],
    )?;
    segment.construct(
        2,
        "king",
        &["thus", "saith", "cyrus"],
        &["king", "thus", "saith", "cyrus", "persia"],
    )?;
    segment.construct(
        3,
        "people",
        &["among", "god", "jerusalem"],
        &["people", "among", "god", "jerusalem", "judah"],
    )?;
    segment.trait_bound(
        4,
        "men",
        &["whosoever", "remaineth", "any"],
        &["men", "whosoever", "remaineth", "any", "place"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        1,
        5,
        8,
        "judah construct -> trait_bound -> move_value judah rose chief",
        &["judah", "rose", "chief", "fathers", "benjamin"],
    );
    segment.construct(
        5,
        "judah",
        &["rose", "chief", "fathers"],
        &["judah", "rose", "chief", "fathers", "benjamin"],
    )?;
    segment.trait_bound(
        6,
        "strengthened",
        &["hands", "vessels", "silver"],
        &["strengthened", "hands", "vessels", "silver", "gold"],
    )?;
    segment.move_value(
        7,
        "king",
        &["cyrus", "brought", "forth"],
        &["king", "cyrus", "brought", "forth", "vessels"],
    )?;
    segment.move_value(
        8,
        "king",
        &["even", "cyrus", "persia"],
        &["king", "even", "cyrus", "persia", "bring"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        1,
        9,
        11,
        "number expression -> move_value number thirty chargers",
        &["number", "thirty", "chargers", "gold", "thousand"],
    );
    segment.expression(
        9,
        "number",
        &["thirty", "chargers", "gold"],
        &["number", "thirty", "chargers", "gold", "thousand"],
    )?;
    segment.expression(
        10,
        "thirty",
        &["basons", "gold", "silver"],
        &["thirty", "basons", "gold", "silver", "second"],
    )?;
    segment.move_value(
        11,
        "vessels",
        &["gold", "silver", "five"],
        &["vessels", "gold", "silver", "five", "thousand"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        1,
        4,
        "children move_value -> expression children now province",
        &["children", "now", "province", "went", "captivity"],
    );
    segment.move_value(
        1,
        "children",
        &["now", "province", "went"],
        &["children", "now", "province", "went", "captivity"],
    )?;
    segment.move_value(
        2,
        "zerubbabel",
        &["jeshua", "nehemiah", "seraiah"],
        &["zerubbabel", "jeshua", "nehemiah", "seraiah", "reelaiah"],
    )?;
    segment.expression(
        3,
        "children",
        &["parosh", "two", "thousand"],
        &["children", "parosh", "two", "thousand", "hundred"],
    )?;
    segment.expression(
        4,
        "children",
        &["shephatiah", "three", "hundred"],
        &["children", "shephatiah", "three", "hundred", "seventy"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        5,
        8,
        "children expression children arah seven",
        &["children", "arah", "seven", "hundred", "seventy"],
    );
    segment.expression(
        5,
        "children",
        &["arah", "seven", "hundred"],
        &["children", "arah", "seven", "hundred", "seventy"],
    )?;
    segment.expression(
        6,
        "children",
        &["pahathmoab", "jeshua", "joab"],
        &["children", "pahathmoab", "jeshua", "joab", "two"],
    )?;
    segment.expression(
        7,
        "children",
        &["elam", "thousand", "two"],
        &["children", "elam", "thousand", "two", "hundred"],
    )?;
    segment.expression(
        8,
        "children",
        &["zattu", "nine", "hundred"],
        &["children", "zattu", "nine", "hundred", "forty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        9,
        12,
        "children expression children zaccai seven",
        &["children", "zaccai", "seven", "hundred", "threescore"],
    );
    segment.expression(
        9,
        "children",
        &["zaccai", "seven", "hundred"],
        &["children", "zaccai", "seven", "hundred", "threescore"],
    )?;
    segment.expression(
        10,
        "children",
        &["bani", "six", "hundred"],
        &["children", "bani", "six", "hundred", "forty"],
    )?;
    segment.expression(
        11,
        "children",
        &["bebai", "six", "hundred"],
        &["children", "bebai", "six", "hundred", "twenty"],
    )?;
    segment.expression(
        12,
        "children",
        &["azgad", "thousand", "two"],
        &["children", "azgad", "thousand", "two", "hundred"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        13,
        16,
        "children expression children adonikam six",
        &["children", "adonikam", "six", "hundred", "sixty"],
    );
    segment.expression(
        13,
        "children",
        &["adonikam", "six", "hundred"],
        &["children", "adonikam", "six", "hundred", "sixty"],
    )?;
    segment.expression(
        14,
        "children",
        &["bigvai", "two", "thousand"],
        &["children", "bigvai", "two", "thousand", "fifty"],
    )?;
    segment.expression(
        15,
        "children",
        &["adin", "four", "hundred"],
        &["children", "adin", "four", "hundred", "fifty"],
    )?;
    segment.expression(
        16,
        "children",
        &["ater", "hezekiah", "ninety"],
        &["children", "ater", "hezekiah", "ninety", "eight"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        17,
        20,
        "children expression children bezai three",
        &["children", "bezai", "three", "hundred", "twenty"],
    );
    segment.expression(
        17,
        "children",
        &["bezai", "three", "hundred"],
        &["children", "bezai", "three", "hundred", "twenty"],
    )?;
    segment.expression(
        18,
        "children",
        &["jorah", "hundred", "twelve"],
        &["children", "jorah", "hundred", "twelve"],
    )?;
    segment.expression(
        19,
        "children",
        &["hashum", "two", "hundred"],
        &["children", "hashum", "two", "hundred", "twenty"],
    )?;
    segment.expression(
        20,
        "children",
        &["gibbar", "ninety", "five"],
        &["children", "gibbar", "ninety", "five"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        21,
        24,
        "children expression children bethlehem hundred",
        &["children", "bethlehem", "hundred", "twenty", "three"],
    );
    segment.expression(
        21,
        "children",
        &["bethlehem", "hundred", "twenty"],
        &["children", "bethlehem", "hundred", "twenty", "three"],
    )?;
    segment.expression(
        22,
        "men",
        &["netophah", "fifty", "six"],
        &["men", "netophah", "fifty", "six"],
    )?;
    segment.expression(
        23,
        "men",
        &["anathoth", "hundred", "twenty"],
        &["men", "anathoth", "hundred", "twenty", "eight"],
    )?;
    segment.expression(
        24,
        "children",
        &["azmaveth", "forty", "two"],
        &["children", "azmaveth", "forty", "two"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        25,
        28,
        "children expression children kirjatharim chephirah",
        &["children", "kirjatharim", "chephirah", "beeroth", "seven"],
    );
    segment.expression(
        25,
        "children",
        &["kirjatharim", "chephirah", "beeroth"],
        &["children", "kirjatharim", "chephirah", "beeroth", "seven"],
    )?;
    segment.expression(
        26,
        "children",
        &["ramah", "gaba", "six"],
        &["children", "ramah", "gaba", "six", "hundred"],
    )?;
    segment.expression(
        27,
        "men",
        &["michmas", "hundred", "twenty"],
        &["men", "michmas", "hundred", "twenty", "two"],
    )?;
    segment.expression(
        28,
        "men",
        &["bethel", "two", "hundred"],
        &["men", "bethel", "two", "hundred", "twenty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        29,
        32,
        "children expression children nebo fifty",
        &["children", "nebo", "fifty", "two", "magbish"],
    );
    segment.expression(
        29,
        "children",
        &["nebo", "fifty", "two"],
        &["children", "nebo", "fifty", "two"],
    )?;
    segment.expression(
        30,
        "children",
        &["magbish", "hundred", "fifty"],
        &["children", "magbish", "hundred", "fifty", "six"],
    )?;
    segment.expression(
        31,
        "children",
        &["other", "elam", "thousand"],
        &["children", "other", "elam", "thousand", "two"],
    )?;
    segment.expression(
        32,
        "children",
        &["harim", "three", "hundred"],
        &["children", "harim", "three", "hundred", "twenty"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        33,
        36,
        "children expression children lod hadid",
        &["children", "lod", "hadid", "ono", "seven"],
    );
    segment.expression(
        33,
        "children",
        &["lod", "hadid", "ono"],
        &["children", "lod", "hadid", "ono", "seven"],
    )?;
    segment.expression(
        34,
        "children",
        &["jericho", "three", "hundred"],
        &["children", "jericho", "three", "hundred", "forty"],
    )?;
    segment.expression(
        35,
        "children",
        &["senaah", "three", "thousand"],
        &["children", "senaah", "three", "thousand", "six"],
    )?;
    segment.expression(
        36,
        "children",
        &["priests", "jedaiah", "house"],
        &["children", "priests", "jedaiah", "house", "jeshua"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        37,
        40,
        "children expression children immer thousand",
        &["children", "immer", "thousand", "fifty", "two"],
    );
    segment.expression(
        37,
        "children",
        &["immer", "thousand", "fifty"],
        &["children", "immer", "thousand", "fifty", "two"],
    )?;
    segment.expression(
        38,
        "children",
        &["pashur", "thousand", "two"],
        &["children", "pashur", "thousand", "two", "hundred"],
    )?;
    segment.expression(
        39,
        "children",
        &["harim", "thousand", "seventeen"],
        &["children", "harim", "thousand", "seventeen"],
    )?;
    segment.expression(
        40,
        "children",
        &["levites", "jeshua", "kadmiel"],
        &["children", "levites", "jeshua", "kadmiel", "hodaviah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        41,
        44,
        "children expression children singers asaph",
        &["children", "singers", "asaph", "hundred", "twenty"],
    );
    segment.expression(
        41,
        "children",
        &["singers", "asaph", "hundred"],
        &["children", "singers", "asaph", "hundred", "twenty"],
    )?;
    segment.expression(
        42,
        "children",
        &["porters", "shallum", "ater"],
        &["children", "porters", "shallum", "ater", "talmon"],
    )?;
    segment.expression(
        43,
        "children",
        &["nethinims", "ziha", "hasupha"],
        &["children", "nethinims", "ziha", "hasupha", "tabbaoth"],
    )?;
    segment.expression(
        44,
        "children",
        &["keros", "siaha", "padon"],
        &["children", "keros", "siaha", "padon"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_12(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        45,
        48,
        "children expression children lebanah hagabah",
        &["children", "lebanah", "hagabah", "akkub", "hagab"],
    );
    segment.expression(
        45,
        "children",
        &["lebanah", "hagabah", "akkub"],
        &["children", "lebanah", "hagabah", "akkub"],
    )?;
    segment.expression(
        46,
        "children",
        &["hagab", "shalmai", "hanan"],
        &["children", "hagab", "shalmai", "hanan"],
    )?;
    segment.expression(
        47,
        "children",
        &["giddel", "gahar", "reaiah"],
        &["children", "giddel", "gahar", "reaiah"],
    )?;
    segment.expression(
        48,
        "children",
        &["rezin", "nekoda", "gazzam"],
        &["children", "rezin", "nekoda", "gazzam"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_13(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        49,
        52,
        "children expression children uzza paseah",
        &["children", "uzza", "paseah", "besai", "asnah"],
    );
    segment.expression(
        49,
        "children",
        &["uzza", "paseah", "besai"],
        &["children", "uzza", "paseah", "besai"],
    )?;
    segment.expression(
        50,
        "children",
        &["asnah", "mehunim", "nephusim"],
        &["children", "asnah", "mehunim", "nephusim"],
    )?;
    segment.expression(
        51,
        "children",
        &["bakbuk", "hakupha", "harhur"],
        &["children", "bakbuk", "hakupha", "harhur"],
    )?;
    segment.expression(
        52,
        "children",
        &["bazluth", "mehida", "harsha"],
        &["children", "bazluth", "mehida", "harsha"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_14(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        53,
        56,
        "children expression children barkos sisera",
        &["children", "barkos", "sisera", "thamah", "neziah"],
    );
    segment.expression(
        53,
        "children",
        &["barkos", "sisera", "thamah"],
        &["children", "barkos", "sisera", "thamah"],
    )?;
    segment.expression(
        54,
        "children",
        &["neziah", "hatipha"],
        &["children", "neziah", "hatipha"],
    )?;
    segment.expression(
        55,
        "children",
        &["solomon's", "servants", "sotai"],
        &["children", "solomon's", "servants", "sotai", "sophereth"],
    )?;
    segment.expression(
        56,
        "children",
        &["jaalah", "darkon", "giddel"],
        &["children", "jaalah", "darkon", "giddel"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_15(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        57,
        60,
        "children expression -> move_value children shephatiah hattil",
        &["children", "shephatiah", "hattil", "pochereth", "zebaim"],
    );
    segment.expression(
        57,
        "children",
        &["shephatiah", "hattil", "pochereth"],
        &["children", "shephatiah", "hattil", "pochereth", "zebaim"],
    )?;
    segment.expression(
        58,
        "children",
        &["nethinims", "solomon's", "servants"],
        &["children", "nethinims", "solomon's", "servants", "three"],
    )?;
    segment.move_value(
        59,
        "cherub",
        &["went", "telmelah", "telharsa"],
        &["cherub", "went", "telmelah", "telharsa", "addan"],
    )?;
    segment.expression(
        60,
        "children",
        &["delaiah", "tobiah", "nekoda"],
        &["children", "delaiah", "tobiah", "nekoda", "six"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_16(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        61,
        64,
        "children expression -> guard children priests habaiah",
        &["children", "priests", "habaiah", "koz", "barzillai"],
    );
    segment.expression(
        61,
        "children",
        &["priests", "habaiah", "koz"],
        &["children", "priests", "habaiah", "koz", "barzillai"],
    )?;
    segment.guard(
        62,
        "sought",
        &["register", "among", "reckoned"],
        &["sought", "register", "among", "reckoned", "genealogy"],
    )?;
    segment.guard(
        63,
        "tirshatha",
        &["unto", "not", "eat"],
        &["tirshatha", "unto", "not", "eat", "most"],
    )?;
    segment.expression(
        64,
        "whole",
        &["congregation", "together", "forty"],
        &["whole", "congregation", "together", "forty", "two"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_17(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        65,
        67,
        "beside expression beside servants maids",
        &["beside", "servants", "maids", "seven", "thousand"],
    );
    segment.expression(
        65,
        "beside",
        &["servants", "maids", "seven"],
        &["beside", "servants", "maids", "seven", "thousand"],
    )?;
    segment.expression(
        66,
        "horses",
        &["seven", "hundred", "thirty"],
        &["horses", "seven", "hundred", "thirty", "six"],
    )?;
    segment.expression(
        67,
        "camels",
        &["four", "hundred", "thirty"],
        &["camels", "four", "hundred", "thirty", "five"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_2_group_18(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        2,
        68,
        70,
        "lord move_value -> expression lord chief fathers",
        &["lord", "chief", "fathers", "house", "jerusalem"],
    );
    segment.move_value(
        68,
        "lord",
        &["chief", "fathers", "house"],
        &["lord", "chief", "fathers", "house", "jerusalem"],
    )?;
    segment.expression(
        69,
        "gave",
        &["ability", "unto", "treasure"],
        &["gave", "ability", "unto", "treasure", "work"],
    )?;
    segment.expression(
        70,
        "people",
        &["priests", "levites", "singers"],
        &["people", "priests", "levites", "singers", "porters"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_3_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        3,
        1,
        4,
        "children move_value -> guard -> trait_bound children seventh month",
        &["children", "seventh", "month", "israel", "cities"],
    );
    segment.move_value(
        1,
        "children",
        &["seventh", "month", "israel"],
        &["children", "seventh", "month", "israel", "cities"],
    )?;
    segment.guard(
        2,
        "stood",
        &["jeshua", "son", "jozadak"],
        &["stood", "jeshua", "son", "jozadak", "brethren"],
    )?;
    segment.trait_bound(
        3,
        "people",
        &["set", "altar", "bases"],
        &["people", "set", "altar", "bases", "fear"],
    )?;
    segment.trait_bound(
        4,
        "kept",
        &["feast", "tabernacles", "written"],
        &["kept", "feast", "tabernacles", "written", "offered"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_3_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        3,
        5,
        8,
        "afterward reinitialize -> guard -> move_value afterward offered continual",
        &["afterward", "offered", "continual", "burnt", "offering"],
    );
    segment.reinitialize(
        5,
        "afterward",
        &["offered", "continual", "burnt"],
        &["afterward", "offered", "continual", "burnt", "offering"],
    )?;
    segment.guard(
        6,
        "first",
        &["day", "seventh", "month"],
        &["first", "day", "seventh", "month", "began"],
    )?;
    segment.move_value(
        7,
        "gave",
        &["money", "unto", "masons"],
        &["gave", "money", "unto", "masons", "carpenters"],
    )?;
    segment.move_value(
        8,
        "god",
        &["now", "second", "year"],
        &["god", "now", "second", "year", "coming"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_3_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        3,
        9,
        13,
        "judah iterate -> guard -> borrow -> result judah stood jeshua",
        &["judah", "stood", "jeshua", "sons", "brethren"],
    );
    segment.iterate(
        9,
        "judah",
        &["stood", "jeshua", "sons"],
        &["judah", "stood", "jeshua", "sons", "brethren"],
    )?;
    segment.guard(
        10,
        "lord",
        &["builders", "laid", "foundation"],
        &["lord", "builders", "laid", "foundation", "temple"],
    )?;
    segment.borrow(
        11,
        "lord",
        &["sang", "together", "course"],
        &["lord", "sang", "together", "course", "praising"],
    )?;
    segment.result(
        12,
        "men",
        &["many", "priests", "levites"],
        &["men", "many", "priests", "levites", "chief"],
    )?;
    segment.guard(
        13,
        "people",
        &["not", "discern", "noise"],
        &["people", "not", "discern", "noise", "shout"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_4_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        4,
        1,
        4,
        "judah trait_bound -> construct -> expression judah now adversaries",
        &["judah", "now", "adversaries", "benjamin", "heard"],
    );
    segment.trait_bound(
        1,
        "judah",
        &["now", "adversaries", "benjamin"],
        &["judah", "now", "adversaries", "benjamin", "heard"],
    )?;
    segment.construct(
        2,
        "god",
        &["zerubbabel", "chief", "fathers"],
        &["god", "zerubbabel", "chief", "fathers", "unto"],
    )?;
    segment.construct(
        3,
        "israel",
        &["zerubbabel", "jeshua", "rest"],
        &["israel", "zerubbabel", "jeshua", "rest", "chief"],
    )?;
    segment.expression(
        4,
        "people",
        &["land", "weakened", "hands"],
        &["people", "land", "weakened", "hands", "judah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_4_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        4,
        5,
        8,
        "king expression king hired counsellors",
        &["king", "hired", "counsellors", "against", "frustrate"],
    );
    segment.expression(
        5,
        "king",
        &["hired", "counsellors", "against"],
        &["king", "hired", "counsellors", "against", "frustrate"],
    )?;
    segment.expression(
        6,
        "reign",
        &["ahasuerus", "beginning", "wrote"],
        &["reign", "ahasuerus", "beginning", "wrote", "unto"],
    )?;
    segment.expression(
        7,
        "days",
        &["artaxerxes", "wrote", "bishlam"],
        &["days", "artaxerxes", "wrote", "bishlam", "mithredath"],
    )?;
    segment.expression(
        8,
        "rehum",
        &["chancellor", "shimshai", "scribe"],
        &["rehum", "chancellor", "shimshai", "scribe", "wrote"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_4_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        4,
        9,
        12,
        "wrote expression -> move_value wrote rehum chancellor",
        &["wrote", "rehum", "chancellor", "shimshai", "scribe"],
    );
    segment.expression(
        9,
        "wrote",
        &["rehum", "chancellor", "shimshai"],
        &["wrote", "rehum", "chancellor", "shimshai", "scribe"],
    )?;
    segment.move_value(
        10,
        "nations",
        &["rest", "great", "noble"],
        &["nations", "rest", "great", "noble", "asnapper"],
    )?;
    segment.move_value(
        11,
        "king",
        &["copy", "letter", "sent"],
        &["king", "copy", "letter", "sent", "unto"],
    )?;
    segment.move_value(
        12,
        "king",
        &["known", "unto", "jews"],
        &["king", "known", "unto", "jews", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_4_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        4,
        13,
        16,
        "king guard -> move_value -> construct king known now",
        &["king", "known", "now", "unto", "city"],
    );
    segment.guard(
        13,
        "king",
        &["known", "now", "unto"],
        &["king", "known", "now", "unto", "city"],
    )?;
    segment.move_value(
        14,
        "now",
        &["maintenance", "king's", "palace"],
        &["now", "maintenance", "king's", "palace", "not"],
    )?;
    segment.construct(
        15,
        "search",
        &["made", "book", "records"],
        &["search", "made", "book", "records", "fathers"],
    )?;
    segment.guard(
        16,
        "king",
        &["certify", "city", "builded"],
        &["king", "certify", "city", "builded", "again"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_4_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        4,
        17,
        20,
        "king move_value -> construct -> expression king sent answer",
        &["king", "sent", "answer", "unto", "rehum"],
    );
    segment.move_value(
        17,
        "king",
        &["sent", "answer", "unto"],
        &["king", "sent", "answer", "unto", "rehum"],
    )?;
    segment.move_value(
        18,
        "letter",
        &["sent", "unto", "hath"],
        &["letter", "sent", "unto", "hath", "plainly"],
    )?;
    segment.construct(
        19,
        "commanded",
        &["search", "hath", "made"],
        &["commanded", "search", "hath", "made", "found"],
    )?;
    segment.expression(
        20,
        "mighty",
        &["kings", "over", "jerusalem"],
        &["mighty", "kings", "over", "jerusalem", "ruled"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_4_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        4,
        21,
        24,
        "men guard -> construct -> expression men give now",
        &["men", "give", "now", "commandment", "cause"],
    );
    segment.guard(
        21,
        "men",
        &["give", "now", "commandment"],
        &["men", "give", "now", "commandment", "cause"],
    )?;
    segment.guard(
        22,
        "take",
        &["heed", "now", "fail"],
        &["take", "heed", "now", "fail", "not"],
    )?;
    segment.construct(
        23,
        "king",
        &["now", "copy", "artaxerxes"],
        &["king", "now", "copy", "artaxerxes", "letter"],
    )?;
    segment.expression(
        24,
        "god",
        &["ceased", "work", "house"],
        &["god", "ceased", "work", "house", "jerusalem"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_5_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        5,
        1,
        4,
        "prophet expression -> construct prophet prophets haggai",
        &["prophet", "prophets", "haggai", "zechariah", "son"],
    );
    segment.expression(
        1,
        "prophet",
        &["prophets", "haggai", "zechariah"],
        &["prophet", "prophets", "haggai", "zechariah", "son"],
    )?;
    segment.construct(
        2,
        "rose",
        &["zerubbabel", "son", "shealtiel"],
        &["rose", "zerubbabel", "son", "shealtiel", "jeshua"],
    )?;
    segment.construct(
        3,
        "same",
        &["time", "tatnai", "governor"],
        &["same", "time", "tatnai", "governor", "side"],
    )?;
    segment.construct(
        4,
        "men",
        &["unto", "manner", "names"],
        &["men", "unto", "manner", "names", "make"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_5_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        5,
        5,
        8,
        "god move_value god eye elders",
        &["god", "eye", "elders", "jews", "not"],
    );
    segment.move_value(
        5,
        "god",
        &["eye", "elders", "jews"],
        &["god", "eye", "elders", "jews", "not"],
    )?;
    segment.move_value(
        6,
        "copy",
        &["letter", "tatnai", "governor"],
        &["copy", "letter", "tatnai", "governor", "side"],
    )?;
    segment.move_value(
        7,
        "king",
        &["sent", "letter", "unto"],
        &["king", "sent", "letter", "unto", "wherein"],
    )?;
    segment.move_value(
        8,
        "king",
        &["known", "unto", "went"],
        &["king", "known", "unto", "went", "province"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_5_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        5,
        9,
        12,
        "asked construct -> result -> drop_value asked elders unto",
        &["asked", "elders", "unto", "thus", "commanded"],
    );
    segment.construct(
        9,
        "asked",
        &["elders", "unto", "thus"],
        &["asked", "elders", "unto", "thus", "commanded"],
    )?;
    segment.result(
        10,
        "men",
        &["asked", "names", "certify"],
        &["men", "asked", "names", "certify", "thee"],
    )?;
    segment.construct(
        11,
        "god",
        &["thus", "returned", "answer"],
        &["god", "thus", "returned", "answer", "saying"],
    )?;
    segment.drop_value(
        12,
        "god",
        &["fathers", "provoked", "heaven"],
        &["god", "fathers", "provoked", "heaven", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_5_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        5,
        13,
        17,
        "king construct -> move_value king first year",
        &["king", "first", "year", "cyrus", "babylon"],
    );
    segment.construct(
        13,
        "king",
        &["first", "year", "cyrus"],
        &["king", "first", "year", "cyrus", "babylon"],
    )?;
    segment.construct(
        14,
        "god",
        &["vessels", "gold", "silver"],
        &["god", "vessels", "gold", "silver", "house"],
    )?;
    segment.move_value(
        15,
        "god",
        &["unto", "take", "vessels"],
        &["god", "unto", "take", "vessels", "carry"],
    )?;
    segment.move_value(
        16,
        "god",
        &["same", "sheshbazzar", "laid"],
        &["god", "same", "sheshbazzar", "laid", "foundation"],
    )?;
    segment.construct(
        17,
        "king",
        &["now", "therefore", "seem"],
        &["king", "now", "therefore", "seem", "good"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_6_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        6,
        1,
        4,
        "king construct -> expression -> reinitialize king darius made",
        &["king", "darius", "made", "decree", "search"],
    );
    segment.construct(
        1,
        "king",
        &["darius", "made", "decree"],
        &["king", "darius", "made", "decree", "search"],
    )?;
    segment.expression(
        2,
        "found",
        &["achmetha", "palace", "province"],
        &["found", "achmetha", "palace", "province", "medes"],
    )?;
    segment.construct(
        3,
        "king",
        &["first", "year", "cyrus"],
        &["king", "first", "year", "cyrus", "same"],
    )?;
    segment.reinitialize(
        4,
        "three",
        &["rows", "great", "stones"],
        &["three", "rows", "great", "stones", "row"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_6_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        6,
        5,
        8,
        "god move_value -> expression -> construct god golden silver",
        &["god", "golden", "silver", "vessels", "house"],
    );
    segment.move_value(
        5,
        "god",
        &["golden", "silver", "vessels"],
        &["god", "golden", "silver", "vessels", "house"],
    )?;
    segment.expression(
        6,
        "now",
        &["therefore", "tatnai", "governor"],
        &["now", "therefore", "tatnai", "governor", "beyond"],
    )?;
    segment.construct(
        7,
        "god",
        &["work", "house", "alone"],
        &["god", "work", "house", "alone", "governor"],
    )?;
    segment.construct(
        8,
        "moreover",
        &["make", "decree", "shall"],
        &["moreover", "make", "decree", "shall", "elders"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_6_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        6,
        9,
        12,
        "need expression -> result -> construct need both young",
        &["need", "both", "young", "bullocks", "rams"],
    );
    segment.expression(
        9,
        "need",
        &["both", "young", "bullocks"],
        &["need", "both", "young", "bullocks", "rams"],
    )?;
    segment.result(
        10,
        "god",
        &["offer", "sacrifices", "sweet"],
        &["god", "offer", "sacrifices", "sweet", "savours"],
    )?;
    segment.construct(
        11,
        "made",
        &["decree", "whosoever", "shall"],
        &["made", "decree", "whosoever", "shall", "alter"],
    )?;
    segment.construct(
        12,
        "god",
        &["hath", "caused", "name"],
        &["god", "hath", "caused", "name", "dwell"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_6_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        6,
        13,
        16,
        "tatnai move_value -> guard -> expression tatnai governor side",
        &["tatnai", "governor", "side", "river", "shetharboznai"],
    );
    segment.move_value(
        13,
        "tatnai",
        &["governor", "side", "river"],
        &["tatnai", "governor", "side", "river", "shetharboznai"],
    )?;
    segment.guard(
        14,
        "prophet",
        &["elders", "jews", "builded"],
        &["prophet", "elders", "jews", "builded", "prospered"],
    )?;
    segment.expression(
        15,
        "house",
        &["finished", "third", "day"],
        &["house", "finished", "third", "day", "month"],
    )?;
    segment.expression(
        16,
        "children",
        &["israel", "priests", "levites"],
        &["children", "israel", "priests", "levites", "rest"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_6_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        6,
        17,
        19,
        "god trait_bound -> expression god offered dedication",
        &["god", "offered", "dedication", "house", "hundred"],
    );
    segment.trait_bound(
        17,
        "god",
        &["offered", "dedication", "house"],
        &["god", "offered", "dedication", "house", "hundred"],
    )?;
    segment.expression(
        18,
        "god",
        &["set", "priests", "divisions"],
        &["god", "set", "priests", "divisions", "levites"],
    )?;
    segment.expression(
        19,
        "children",
        &["captivity", "kept", "passover"],
        &["children", "captivity", "kept", "passover", "fourteenth"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_6_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        6,
        20,
        22,
        "children drop_value -> move_value -> construct children priests levites",
        &["children", "priests", "levites", "purified", "together"],
    );
    segment.drop_value(
        20,
        "children",
        &["priests", "levites", "purified"],
        &["children", "priests", "levites", "purified", "together"],
    )?;
    segment.move_value(
        21,
        "children",
        &["israel", "again", "captivity"],
        &["children", "israel", "again", "captivity", "such"],
    )?;
    segment.construct(
        22,
        "lord",
        &["kept", "feast", "unleavened"],
        &["lord", "kept", "feast", "unleavened", "bread"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_7_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        7,
        1,
        4,
        "king expression king now things",
        &["king", "now", "things", "reign", "artaxerxes"],
    );
    segment.expression(
        1,
        "king",
        &["now", "things", "reign"],
        &["king", "now", "things", "reign", "artaxerxes"],
    )?;
    segment.expression(
        2,
        "son",
        &["shallum", "zadok", "ahitub"],
        &["son", "shallum", "zadok", "ahitub"],
    )?;
    segment.expression(
        3,
        "son",
        &["amariah", "azariah", "meraioth"],
        &["son", "amariah", "azariah", "meraioth"],
    )?;
    segment.expression(
        4,
        "son",
        &["zerahiah", "uzzi", "bukki"],
        &["son", "zerahiah", "uzzi", "bukki"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_7_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        7,
        5,
        8,
        "aaron expression -> move_value aaron son abishua",
        &["aaron", "son", "abishua", "phinehas", "eleazar"],
    );
    segment.expression(
        5,
        "aaron",
        &["son", "abishua", "phinehas"],
        &["aaron", "son", "abishua", "phinehas", "eleazar"],
    )?;
    segment.move_value(
        6,
        "moses",
        &["ezra", "went", "babylon"],
        &["moses", "ezra", "went", "babylon", "ready"],
    )?;
    segment.move_value(
        7,
        "children",
        &["went", "israel", "priests"],
        &["children", "went", "israel", "priests", "levites"],
    )?;
    segment.move_value(
        8,
        "king",
        &["jerusalem", "fifth", "month"],
        &["king", "jerusalem", "fifth", "month", "seventh"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_7_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        7,
        9,
        12,
        "first move_value -> guard -> expression first day month",
        &["first", "day", "month", "began", "babylon"],
    );
    segment.move_value(
        9,
        "first",
        &["day", "month", "began"],
        &["first", "day", "month", "began", "babylon"],
    )?;
    segment.guard(
        10,
        "lord",
        &["ezra", "prepared", "heart"],
        &["lord", "ezra", "prepared", "heart", "seek"],
    )?;
    segment.expression(
        11,
        "king",
        &["now", "copy", "letter"],
        &["king", "now", "copy", "letter", "artaxerxes"],
    )?;
    segment.guard(
        12,
        "king",
        &["artaxerxes", "kings", "unto"],
        &["king", "artaxerxes", "kings", "unto", "ezra"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_7_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        7,
        13,
        16,
        "people construct -> move_value -> trait_bound people make decree",
        &["people", "make", "decree", "israel", "priests"],
    );
    segment.construct(
        13,
        "people",
        &["make", "decree", "israel"],
        &["people", "make", "decree", "israel", "priests"],
    )?;
    segment.move_value(
        14,
        "king",
        &["forasmuch", "art", "sent"],
        &["king", "forasmuch", "art", "sent", "seven"],
    )?;
    segment.trait_bound(
        15,
        "king",
        &["carry", "silver", "gold"],
        &["king", "carry", "silver", "gold", "counsellors"],
    )?;
    segment.trait_bound(
        16,
        "silver",
        &["gold", "canst", "find"],
        &["silver", "gold", "canst", "find", "province"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_7_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        7,
        17,
        20,
        "mayest trait_bound -> expression -> guard mayest buy speedily",
        &["mayest", "buy", "speedily", "money", "bullocks"],
    );
    segment.trait_bound(
        17,
        "mayest",
        &["buy", "speedily", "money"],
        &["mayest", "buy", "speedily", "money", "bullocks"],
    )?;
    segment.expression(
        18,
        "whatsoever",
        &["shall", "seem", "good"],
        &["whatsoever", "shall", "seem", "good", "thee"],
    )?;
    segment.expression(
        19,
        "god",
        &["vessels", "given", "thee"],
        &["god", "vessels", "given", "thee", "service"],
    )?;
    segment.guard(
        20,
        "god",
        &["whatsoever", "shall", "needful"],
        &["god", "whatsoever", "shall", "needful", "house"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_7_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        7,
        21,
        24,
        "king construct -> expression -> iterate -> guard king even artaxerxes",
        &["king", "even", "artaxerxes", "make", "decree"],
    );
    segment.construct(
        21,
        "king",
        &["even", "artaxerxes", "make"],
        &["king", "even", "artaxerxes", "make", "decree"],
    )?;
    segment.expression(
        22,
        "unto",
        &["hundred", "talents", "silver"],
        &["unto", "hundred", "talents", "silver", "measures"],
    )?;
    segment.iterate(
        23,
        "god",
        &["whatsoever", "commanded", "heaven"],
        &["god", "whatsoever", "commanded", "heaven", "diligently"],
    )?;
    segment.guard(
        24,
        "certify",
        &["touching", "any", "priests"],
        &["certify", "touching", "any", "priests", "levites"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_7_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        7,
        25,
        28,
        "god guard -> drop_value -> return_value -> move_value god ezra wisdom",
        &["god", "ezra", "wisdom", "thine", "hand"],
    );
    segment.guard(
        25,
        "god",
        &["ezra", "wisdom", "thine"],
        &["god", "ezra", "wisdom", "thine", "hand"],
    )?;
    segment.drop_value(
        26,
        "god",
        &["whosoever", "will", "not"],
        &["god", "whosoever", "will", "not", "law"],
    )?;
    segment.return_value(
        27,
        "lord",
        &["blessed", "god", "fathers"],
        &["lord", "blessed", "god", "fathers", "hath"],
    )?;
    segment.move_value(
        28,
        "king",
        &["hath", "extended", "mercy"],
        &["king", "hath", "extended", "mercy", "unto"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        1,
        4,
        "now move_value -> iterate now chief fathers",
        &["now", "chief", "fathers", "genealogy", "went"],
    );
    segment.move_value(
        1,
        "now",
        &["chief", "fathers", "genealogy"],
        &["now", "chief", "fathers", "genealogy", "went"],
    )?;
    segment.iterate(
        2,
        "david",
        &["sons", "phinehas", "gershom"],
        &["david", "sons", "phinehas", "gershom", "ithamar"],
    )?;
    segment.iterate(
        3,
        "sons",
        &["shechaniah", "pharosh", "zechariah"],
        &["sons", "shechaniah", "pharosh", "zechariah", "reckoned"],
    )?;
    segment.iterate(
        4,
        "sons",
        &["pahathmoab", "elihoenai", "son"],
        &["sons", "pahathmoab", "elihoenai", "son", "zerahiah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        5,
        8,
        "sons iterate sons shechaniah son",
        &["sons", "shechaniah", "son", "jahaziel", "three"],
    );
    segment.iterate(
        5,
        "sons",
        &["shechaniah", "son", "jahaziel"],
        &["sons", "shechaniah", "son", "jahaziel", "three"],
    )?;
    segment.iterate(
        6,
        "sons",
        &["adin", "ebed", "son"],
        &["sons", "adin", "ebed", "son", "jonathan"],
    )?;
    segment.iterate(
        7,
        "sons",
        &["elam", "jeshaiah", "son"],
        &["sons", "elam", "jeshaiah", "son", "athaliah"],
    )?;
    segment.iterate(
        8,
        "sons",
        &["shephatiah", "zebadiah", "son"],
        &["sons", "shephatiah", "zebadiah", "son", "michael"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        9,
        12,
        "sons iterate sons joab obadiah",
        &["sons", "joab", "obadiah", "son", "jehiel"],
    );
    segment.iterate(
        9,
        "sons",
        &["joab", "obadiah", "son"],
        &["sons", "joab", "obadiah", "son", "jehiel"],
    )?;
    segment.iterate(
        10,
        "sons",
        &["shelomith", "son", "josiphiah"],
        &["sons", "shelomith", "son", "josiphiah", "hundred"],
    )?;
    segment.iterate(
        11,
        "sons",
        &["bebai", "zechariah", "son"],
        &["sons", "bebai", "zechariah", "son", "twenty"],
    )?;
    segment.iterate(
        12,
        "sons",
        &["azgad", "johanan", "son"],
        &["sons", "azgad", "johanan", "son", "hakkatan"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        13,
        16,
        "last iterate -> move_value last sons adonikam",
        &["last", "sons", "adonikam", "whose", "names"],
    );
    segment.iterate(
        13,
        "last",
        &["sons", "adonikam", "whose"],
        &["last", "sons", "adonikam", "whose", "names"],
    )?;
    segment.iterate(
        14,
        "sons",
        &["bigvai", "uthai", "zabbud"],
        &["sons", "bigvai", "uthai", "zabbud", "seventy"],
    )?;
    segment.iterate(
        15,
        "gathered",
        &["together", "river", "runneth"],
        &["gathered", "together", "river", "runneth", "ahava"],
    )?;
    segment.move_value(
        16,
        "sent",
        &["eliezer", "ariel", "shemaiah"],
        &["sent", "eliezer", "ariel", "shemaiah", "elnathan"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        17,
        20,
        "sent move_value -> iterate -> expression sent commandment unto",
        &["sent", "commandment", "unto", "iddo", "chief"],
    );
    segment.move_value(
        17,
        "sent",
        &["commandment", "unto", "iddo"],
        &["sent", "commandment", "unto", "iddo", "chief"],
    )?;
    segment.move_value(
        18,
        "god",
        &["good", "hand", "brought"],
        &["god", "good", "hand", "brought", "man"],
    )?;
    segment.iterate(
        19,
        "hashabiah",
        &["jeshaiah", "sons", "merari"],
        &["hashabiah", "jeshaiah", "sons", "merari", "brethren"],
    )?;
    segment.expression(
        20,
        "david",
        &["nethinims", "princes", "appointed"],
        &["david", "nethinims", "princes", "appointed", "service"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        21,
        24,
        "god expression -> panic god proclaimed fast",
        &["god", "proclaimed", "fast", "river", "ahava"],
    );
    segment.expression(
        21,
        "god",
        &["proclaimed", "fast", "river"],
        &["god", "proclaimed", "fast", "river", "ahava"],
    )?;
    segment.panic(
        22,
        "king",
        &["ashamed", "require", "band"],
        &["king", "ashamed", "require", "band", "soldiers"],
    )?;
    segment.expression(
        23,
        "god",
        &["fasted", "besought", "intreated"],
        &["god", "fasted", "besought", "intreated"],
    )?;
    segment.expression(
        24,
        "separated",
        &["twelve", "chief", "priests"],
        &["separated", "twelve", "chief", "priests", "sherebiah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        25,
        28,
        "weighed trait_bound -> expression -> call weighed unto silver",
        &["weighed", "unto", "silver", "gold", "vessels"],
    );
    segment.trait_bound(
        25,
        "weighed",
        &["unto", "silver", "gold"],
        &["weighed", "unto", "silver", "gold", "vessels"],
    )?;
    segment.expression(
        26,
        "even",
        &["weighed", "unto", "hand"],
        &["even", "weighed", "unto", "hand", "six"],
    )?;
    segment.expression(
        27,
        "twenty",
        &["basons", "gold", "thousand"],
        &["twenty", "basons", "gold", "thousand", "drams"],
    )?;
    segment.call(
        28,
        "lord",
        &["unto", "holy", "vessels"],
        &["lord", "unto", "holy", "vessels", "silver"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        29,
        32,
        "watch guard -> move_value watch keep until",
        &["watch", "keep", "until", "weigh", "chief"],
    );
    segment.guard(
        29,
        "watch",
        &["keep", "until", "weigh"],
        &["watch", "keep", "until", "weigh", "chief"],
    )?;
    segment.move_value(
        30,
        "took",
        &["priests", "levites", "weight"],
        &["took", "priests", "levites", "weight", "silver"],
    )?;
    segment.move_value(
        31,
        "departed",
        &["river", "ahava", "twelfth"],
        &["departed", "river", "ahava", "twelfth", "day"],
    )?;
    segment.move_value(
        32,
        "jerusalem",
        &["abode", "three", "days"],
        &["jerusalem", "abode", "three", "days"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_8_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        8,
        33,
        36,
        "now expression -> iterate -> move_value now fourth day",
        &["now", "fourth", "day", "silver", "gold"],
    );
    segment.expression(
        33,
        "now",
        &["fourth", "day", "silver"],
        &["now", "fourth", "day", "silver", "gold"],
    )?;
    segment.iterate(
        34,
        "number",
        &["weight", "every", "written"],
        &["number", "weight", "every", "written", "time"],
    )?;
    segment.move_value(
        35,
        "children",
        &["carried", "away", "captivity"],
        &["children", "carried", "away", "captivity", "offered"],
    )?;
    segment.expression(
        36,
        "delivered",
        &["king's", "commissions", "unto"],
        &["delivered", "king's", "commissions", "unto", "lieutenants"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_9_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        9,
        1,
        4,
        "people move_value -> iterate -> expression -> trait_bound people now things",
        &["people", "now", "things", "done", "princes"],
    );
    segment.move_value(
        1,
        "people",
        &["now", "things", "done"],
        &["people", "now", "things", "done", "princes"],
    )?;
    segment.iterate(
        2,
        "people",
        &["taken", "daughters", "themselves"],
        &["people", "taken", "daughters", "themselves", "sons"],
    )?;
    segment.expression(
        3,
        "heard",
        &["thing", "rent", "garment"],
        &["heard", "thing", "rent", "garment", "mantle"],
    )?;
    segment.trait_bound(
        4,
        "god",
        &["assembled", "unto", "every"],
        &["god", "assembled", "unto", "every", "trembled"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_9_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        9,
        5,
        8,
        "evening reinitialize -> call -> expression -> borrow evening sacrifice arose",
        &["evening", "sacrifice", "arose", "heaviness", "having"],
    );
    segment.reinitialize(
        5,
        "evening",
        &["sacrifice", "arose", "heaviness"],
        &["evening", "sacrifice", "arose", "heaviness", "having"],
    )?;
    segment.call(
        6,
        "god",
        &["ashamed", "blush", "lift"],
        &["god", "ashamed", "blush", "lift", "face"],
    )?;
    segment.expression(
        7,
        "since",
        &["days", "fathers", "great"],
        &["since", "days", "fathers", "great", "trespass"],
    )?;
    segment.borrow(
        8,
        "lord",
        &["now", "little", "space"],
        &["lord", "now", "little", "space", "grace"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_9_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        9,
        9,
        12,
        "god guard -> call -> move_value god bondmen yet",
        &["god", "bondmen", "yet", "hath", "not"],
    );
    segment.guard(
        9,
        "god",
        &["bondmen", "yet", "hath"],
        &["god", "bondmen", "yet", "hath", "not"],
    )?;
    segment.call(
        10,
        "god",
        &["now", "shall", "say"],
        &["god", "now", "shall", "say", "forsaken"],
    )?;
    segment.move_value(
        11,
        "hast",
        &["commanded", "servants", "prophets"],
        &["hast", "commanded", "servants", "prophets", "saying"],
    )?;
    segment.guard(
        12,
        "now",
        &["therefore", "give", "not"],
        &["now", "therefore", "give", "not", "daughters"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_9_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        9,
        13,
        15,
        "god move_value -> guard -> match_value god evil deeds",
        &["god", "evil", "deeds", "great", "trespass"],
    );
    segment.move_value(
        13,
        "god",
        &["evil", "deeds", "great"],
        &["god", "evil", "deeds", "great", "trespass"],
    )?;
    segment.guard(
        14,
        "people",
        &["again", "break", "commandments"],
        &["people", "again", "break", "commandments", "join"],
    )?;
    segment.match_value(
        15,
        "lord",
        &["god", "israel", "art"],
        &["lord", "god", "israel", "art", "righteous"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        1,
        4,
        "now result -> call -> construct -> expression now ezra prayed",
        &["now", "ezra", "prayed", "confessed", "weeping"],
    );
    segment.result(
        1,
        "now",
        &["ezra", "prayed", "confessed"],
        &["now", "ezra", "prayed", "confessed", "weeping"],
    )?;
    segment.call(
        2,
        "shechaniah",
        &["son", "jehiel", "sons"],
        &["shechaniah", "son", "jehiel", "sons", "elam"],
    )?;
    segment.construct(
        3,
        "god",
        &["now", "therefore", "make"],
        &["god", "now", "therefore", "make", "covenant"],
    )?;
    segment.expression(
        4,
        "arise",
        &["matter", "belongeth", "unto"],
        &["arise", "matter", "belongeth", "unto", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        5,
        8,
        "israel construct -> move_value israel arose ezra",
        &["israel", "arose", "ezra", "made", "chief"],
    );
    segment.construct(
        5,
        "israel",
        &["arose", "ezra", "made"],
        &["israel", "arose", "ezra", "made", "chief"],
    )?;
    segment.move_value(
        6,
        "god",
        &["ezra", "rose", "house"],
        &["god", "ezra", "rose", "house", "went"],
    )?;
    segment.construct(
        7,
        "judah",
        &["made", "proclamation", "throughout"],
        &["judah", "made", "proclamation", "throughout", "jerusalem"],
    )?;
    segment.move_value(
        8,
        "whosoever",
        &["would", "not", "within"],
        &["whosoever", "would", "not", "within", "three"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        9,
        12,
        "men expression -> call -> construct -> guard men judah benjamin",
        &["men", "judah", "benjamin", "gathered", "themselves"],
    );
    segment.expression(
        9,
        "men",
        &["judah", "benjamin", "gathered"],
        &["men", "judah", "benjamin", "gathered", "themselves"],
    )?;
    segment.call(
        10,
        "priest",
        &["ezra", "stood", "unto"],
        &["priest", "ezra", "stood", "unto", "transgressed"],
    )?;
    segment.construct(
        11,
        "lord",
        &["now", "therefore", "make"],
        &["lord", "now", "therefore", "make", "confession"],
    )?;
    segment.guard(
        12,
        "congregation",
        &["answered", "loud", "voice"],
        &["congregation", "answered", "loud", "voice", "hast"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        13,
        16,
        "people guard -> move_value -> expression people many time",
        &["people", "many", "time", "much", "rain"],
    );
    segment.guard(
        13,
        "people",
        &["many", "time", "much"],
        &["people", "many", "time", "much", "rain"],
    )?;
    segment.move_value(
        14,
        "now",
        &["rulers", "congregation", "stand"],
        &["now", "rulers", "congregation", "stand", "taken"],
    )?;
    segment.expression(
        15,
        "jonathan",
        &["son", "asahel", "jahaziah"],
        &["jonathan", "son", "asahel", "jahaziah", "tikvah"],
    )?;
    segment.expression(
        16,
        "children",
        &["captivity", "ezra", "priest"],
        &["children", "captivity", "ezra", "priest", "certain"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        17,
        20,
        "men construct -> iterate -> trait_bound men made end",
        &["men", "made", "end", "taken", "strange"],
    );
    segment.construct(
        17,
        "men",
        &["made", "end", "taken"],
        &["men", "made", "end", "taken", "strange"],
    )?;
    segment.iterate(
        18,
        "among",
        &["sons", "priests", "found"],
        &["among", "sons", "priests", "found", "taken"],
    )?;
    segment.trait_bound(
        19,
        "gave",
        &["hands", "would", "put"],
        &["gave", "hands", "would", "put", "away"],
    )?;
    segment.iterate(
        20,
        "sons",
        &["immer", "hanani", "zebadiah"],
        &["sons", "immer", "hanani", "zebadiah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        21,
        24,
        "sons iterate -> expression sons harim maaseiah",
        &["sons", "harim", "maaseiah", "elijah", "shemaiah"],
    );
    segment.iterate(
        21,
        "sons",
        &["harim", "maaseiah", "elijah"],
        &["sons", "harim", "maaseiah", "elijah", "shemaiah"],
    )?;
    segment.iterate(
        22,
        "sons",
        &["pashur", "elioenai", "maaseiah"],
        &["sons", "pashur", "elioenai", "maaseiah", "ishmael"],
    )?;
    segment.expression(
        23,
        "judah",
        &["levites", "jozabad", "shimei"],
        &["judah", "levites", "jozabad", "shimei", "kelaiah"],
    )?;
    segment.expression(
        24,
        "singers",
        &["eliashib", "porters", "shallum"],
        &["singers", "eliashib", "porters", "shallum", "telem"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_07(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        25,
        28,
        "israel iterate israel moreover sons",
        &["israel", "moreover", "sons", "parosh", "ramiah"],
    );
    segment.iterate(
        25,
        "israel",
        &["moreover", "sons", "parosh"],
        &["israel", "moreover", "sons", "parosh", "ramiah"],
    )?;
    segment.iterate(
        26,
        "sons",
        &["elam", "mattaniah", "zechariah"],
        &["sons", "elam", "mattaniah", "zechariah", "jehiel"],
    )?;
    segment.iterate(
        27,
        "sons",
        &["zattu", "elioenai", "eliashib"],
        &["sons", "zattu", "elioenai", "eliashib", "mattaniah"],
    )?;
    segment.iterate(
        28,
        "sons",
        &["bebai", "jehohanan", "hananiah"],
        &["sons", "bebai", "jehohanan", "hananiah", "zabbai"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_08(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        29,
        32,
        "sons iterate -> expression sons bani meshullam",
        &["sons", "bani", "meshullam", "malluch", "adaiah"],
    );
    segment.iterate(
        29,
        "sons",
        &["bani", "meshullam", "malluch"],
        &["sons", "bani", "meshullam", "malluch", "adaiah"],
    )?;
    segment.iterate(
        30,
        "sons",
        &["pahathmoab", "adna", "chelal"],
        &["sons", "pahathmoab", "adna", "chelal", "benaiah"],
    )?;
    segment.iterate(
        31,
        "sons",
        &["harim", "eliezer", "ishijah"],
        &["sons", "harim", "eliezer", "ishijah", "malchiah"],
    )?;
    segment.expression(
        32,
        "benjamin",
        &["malluch", "shemariah"],
        &["benjamin", "malluch", "shemariah"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_09(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        33,
        36,
        "sons iterate -> expression sons hashum mattenai",
        &["sons", "hashum", "mattenai", "mattathah", "zabad"],
    );
    segment.iterate(
        33,
        "sons",
        &["hashum", "mattenai", "mattathah"],
        &["sons", "hashum", "mattenai", "mattathah", "zabad"],
    )?;
    segment.iterate(
        34,
        "sons",
        &["bani", "maadai", "amram"],
        &["sons", "bani", "maadai", "amram", "uel"],
    )?;
    segment.expression(
        35,
        "benaiah",
        &["bedeiah", "chelluh"],
        &["benaiah", "bedeiah", "chelluh"],
    )?;
    segment.expression(
        36,
        "vaniah",
        &["meremoth", "eliashib"],
        &["vaniah", "meremoth", "eliashib"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_10(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        37,
        40,
        "mattaniah expression mattaniah mattenai jaasau",
        &["mattaniah", "mattenai", "jaasau", "bani", "binnui"],
    );
    segment.expression(
        37,
        "mattaniah",
        &["mattenai", "jaasau"],
        &["mattaniah", "mattenai", "jaasau"],
    )?;
    segment.expression(
        38,
        "bani",
        &["binnui", "shimei"],
        &["bani", "binnui", "shimei"],
    )?;
    segment.expression(
        39,
        "shelemiah",
        &["nathan", "adaiah"],
        &["shelemiah", "nathan", "adaiah"],
    )?;
    segment.expression(
        40,
        "machnadebai",
        &["shashai", "sharai"],
        &["machnadebai", "shashai", "sharai"],
    )?;
    segment.commit()?;
    Ok(())
}

fn ezra_10_group_11(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Ezra,
        10,
        41,
        44,
        "azareel expression -> iterate azareel shelemiah shemariah",
        &["azareel", "shelemiah", "shemariah", "joseph", "shallum"],
    );
    segment.expression(
        41,
        "azareel",
        &["shelemiah", "shemariah"],
        &["azareel", "shelemiah", "shemariah"],
    )?;
    segment.expression(
        42,
        "joseph",
        &["shallum", "amariah"],
        &["joseph", "shallum", "amariah"],
    )?;
    segment.iterate(
        43,
        "sons",
        &["nebo", "jeiel", "mattithiah"],
        &["sons", "nebo", "jeiel", "mattithiah", "zabad"],
    )?;
    segment.expression(
        44,
        "children",
        &["taken", "strange", "wives"],
        &["children", "taken", "strange", "wives"],
    )?;
    segment.commit()?;
    Ok(())
}
