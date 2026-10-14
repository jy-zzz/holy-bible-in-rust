use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    philemon(story)?;
    Ok(())
}

fn philemon(story: &mut Story) -> Result<(), StoryError> {
    philemon_1(story)?;
    Ok(())
}

fn philemon_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Philemon", 1, "Paul appeals for Onesimus, once an escaped or estranged servant and now a beloved brother, asking Philemon to receive him as he would receive Paul.", "The letter turns a private debt into a test of gospel-shaped fellowship, with Paul offering to repay while trusting love to choose the deeper reconciliation.");
    philemon_1_group_01(story)?;
    philemon_1_group_02(story)?;
    philemon_1_group_03(story)?;
    philemon_1_group_04(story)?;
    philemon_1_group_05(story)?;
    philemon_1_group_06(story)?;
    Ok(())
}

fn philemon_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philemon,
        1,
        1,
        4,
        "paul borrow -> expression paul prisoner jesus",
        &["paul", "prisoner", "jesus", "christ", "timothy"],
    );
    segment.borrow(
        1,
        "paul",
        &["prisoner", "jesus", "christ"],
        &["paul", "prisoner", "jesus", "christ", "timothy"],
    )?;
    segment.borrow(
        2,
        "beloved",
        &["apphia", "archippus", "fellowsoldier"],
        &["beloved", "apphia", "archippus", "fellowsoldier", "church"],
    )?;
    segment.borrow(
        3,
        "god",
        &["grace", "peace", "father"],
        &["god", "grace", "peace", "father", "lord"],
    )?;
    segment.expression(
        4,
        "god",
        &["thank", "making", "mention"],
        &["god", "thank", "making", "mention", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philemon_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philemon,
        1,
        5,
        8,
        "lord await_value -> borrow -> expression lord hearing love",
        &["lord", "hearing", "love", "faith", "hast"],
    );
    segment.await_value(
        5,
        "lord",
        &["hearing", "love", "faith"],
        &["lord", "hearing", "love", "faith", "hast"],
    )?;
    segment.await_value(
        6,
        "communication",
        &["faith", "become", "effectual"],
        &[
            "communication",
            "faith",
            "become",
            "effectual",
            "acknowledging",
        ],
    )?;
    segment.borrow(
        7,
        "great",
        &["joy", "consolation", "love"],
        &["great", "joy", "consolation", "love", "bowels"],
    )?;
    segment.expression(
        8,
        "christ",
        &["wherefore", "though", "might"],
        &["christ", "wherefore", "though", "might", "much"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philemon_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philemon,
        1,
        9,
        12,
        "paul borrow -> construct -> expression -> move_value paul yet love's",
        &["paul", "yet", "love's", "sake", "rather"],
    );
    segment.borrow(
        9,
        "paul",
        &["yet", "love's", "sake"],
        &["paul", "yet", "love's", "sake", "rather"],
    )?;
    segment.construct(
        10,
        "beseech",
        &["thee", "son", "onesimus"],
        &["beseech", "thee", "son", "onesimus", "begotten"],
    )?;
    segment.expression(
        11,
        "time",
        &["past", "thee", "unprofitable"],
        &["time", "past", "thee", "unprofitable", "now"],
    )?;
    segment.move_value(
        12,
        "sent",
        &["again", "therefore", "receive"],
        &["sent", "again", "therefore", "receive", "mine"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philemon_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philemon,
        1,
        13,
        16,
        "would expression -> guard would retained stead",
        &["would", "retained", "stead", "might", "ministered"],
    );
    segment.expression(
        13,
        "would",
        &["retained", "stead", "might"],
        &["would", "retained", "stead", "might", "ministered"],
    )?;
    segment.guard(
        14,
        "without",
        &["mind", "would", "nothing"],
        &["without", "mind", "would", "nothing", "benefit"],
    )?;
    segment.expression(
        15,
        "perhaps",
        &["therefore", "departed", "season"],
        &["perhaps", "therefore", "departed", "season", "shouldest"],
    )?;
    segment.guard(
        16,
        "not",
        &["now", "servant", "above"],
        &["not", "now", "servant", "above", "brother"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philemon_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philemon,
        1,
        17,
        20,
        "count expression -> guard count therefore partner",
        &["count", "therefore", "partner", "receive", "myself"],
    );
    segment.expression(
        17,
        "count",
        &["therefore", "partner", "receive"],
        &["count", "therefore", "partner", "receive", "myself"],
    )?;
    segment.expression(
        18,
        "hath",
        &["wronged", "thee", "oweth"],
        &["hath", "wronged", "thee", "oweth", "ought"],
    )?;
    segment.guard(
        19,
        "paul",
        &["written", "mine", "own"],
        &["paul", "written", "mine", "own", "hand"],
    )?;
    segment.expression(
        20,
        "lord",
        &["yea", "brother", "joy"],
        &["lord", "yea", "brother", "joy", "thee"],
    )?;
    segment.commit()?;
    Ok(())
}

fn philemon_1_group_06(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Philemon,
        1,
        21,
        25,
        "having call -> await_value -> expression -> borrow having confidence obedience",
        &["having", "confidence", "obedience", "wrote", "unto"],
    );
    segment.call(
        21,
        "having",
        &["confidence", "obedience", "wrote"],
        &["having", "confidence", "obedience", "wrote", "unto"],
    )?;
    segment.await_value(
        22,
        "withal",
        &["prepare", "lodging", "trust"],
        &["withal", "prepare", "lodging", "trust", "through"],
    )?;
    segment.expression(
        23,
        "christ",
        &["salute", "thee", "epaphras"],
        &["christ", "salute", "thee", "epaphras", "fellowprisoner"],
    )?;
    segment.expression(
        24,
        "marcus",
        &["aristarchus", "demas", "lucas"],
        &["marcus", "aristarchus", "demas", "lucas", "fellowlabourers"],
    )?;
    segment.borrow(
        25,
        "lord",
        &["grace", "jesus", "christ"],
        &["lord", "grace", "jesus", "christ", "spirit"],
    )?;
    segment.commit()?;
    Ok(())
}
