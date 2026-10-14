use crate::{Book, Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    obadiah(story)?;
    Ok(())
}

fn obadiah(story: &mut Story) -> Result<(), StoryError> {
    obadiah_1(story)?;
    Ok(())
}

fn obadiah_1(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(
        story,
        "Obadiah",
        1,
        "Edom celebrated Jerusalem's disaster, entered the broken city like a",
        "looter with admin access, and handed survivors over to their enemies.",
        "The oracle reversed the boast: the mountain of pride would be pulled down,",
        "the stolen inheritance returned, and the day of the Maintainer would make",
        "every nation answer for how it treated a neighbor in crisis."
    );
    obadiah_1_group_01(story)?;
    obadiah_1_group_02(story)?;
    obadiah_1_group_03(story)?;
    obadiah_1_group_04(story)?;
    obadiah_1_group_05(story)?;
    Ok(())
}

fn obadiah_1_group_01(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Obadiah,
        1,
        1,
        4,
        "lord move_value -> construct lord vision obadiah",
        &["lord", "vision", "obadiah", "thus", "saith"],
    );
    segment.move_value(
        1,
        "lord",
        &["vision", "obadiah", "thus"],
        &["lord", "vision", "obadiah", "thus", "saith"],
    )?;
    segment.construct(
        2,
        "behold",
        &["made", "thee", "small"],
        &["behold", "made", "thee", "small", "among"],
    )?;
    segment.move_value(
        3,
        "pride",
        &["thine", "heart", "hath"],
        &["pride", "thine", "heart", "hath", "deceived"],
    )?;
    segment.move_value(
        4,
        "though",
        &["exalt", "thyself", "eagle"],
        &["though", "exalt", "thyself", "eagle", "set"],
    )?;
    segment.commit()?;
    Ok(())
}

fn obadiah_1_group_02(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Obadiah,
        1,
        5,
        8,
        "thieves move_value -> expression -> drop_value thieves thee robbers",
        &["thieves", "thee", "robbers", "night", "art"],
    );
    segment.move_value(
        5,
        "thieves",
        &["thee", "robbers", "night"],
        &["thieves", "thee", "robbers", "night", "art"],
    )?;
    segment.expression(
        6,
        "things",
        &["esau", "searched", "hidden"],
        &["things", "esau", "searched", "hidden", "sought"],
    )?;
    segment.move_value(
        7,
        "men",
        &["confederacy", "brought", "thee"],
        &["men", "confederacy", "brought", "thee", "even"],
    )?;
    segment.drop_value(
        8,
        "lord",
        &["shall", "not", "day"],
        &["lord", "shall", "not", "day", "saith"],
    )?;
    segment.commit()?;
    Ok(())
}

fn obadiah_1_group_03(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Obadiah,
        1,
        9,
        12,
        "men iterate -> guard -> expression men mighty teman",
        &["men", "mighty", "teman", "shall", "dismayed"],
    );
    segment.iterate(
        9,
        "men",
        &["mighty", "teman", "shall"],
        &["men", "mighty", "teman", "shall", "dismayed"],
    )?;
    segment.guard(
        10,
        "jacob",
        &["violence", "against", "brother"],
        &["jacob", "violence", "against", "brother", "shame"],
    )?;
    segment.expression(
        11,
        "day",
        &["stoodest", "other", "side"],
        &["day", "stoodest", "other", "side", "strangers"],
    )?;
    segment.guard(
        12,
        "shouldest",
        &["not", "looked", "day"],
        &["shouldest", "not", "looked", "day", "brother"],
    )?;
    segment.commit()?;
    Ok(())
}

fn obadiah_1_group_04(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Obadiah,
        1,
        13,
        16,
        "people guard -> return_value people shouldest not",
        &["people", "shouldest", "not", "entered", "gate"],
    );
    segment.guard(
        13,
        "people",
        &["shouldest", "not", "entered"],
        &["people", "shouldest", "not", "entered", "gate"],
    )?;
    segment.guard(
        14,
        "neither",
        &["shouldest", "stood", "crossway"],
        &["neither", "shouldest", "stood", "crossway", "cut"],
    )?;
    segment.return_value(
        15,
        "lord",
        &["day", "near", "heathen"],
        &["lord", "day", "near", "heathen", "hast"],
    )?;
    segment.guard(
        16,
        "drunk",
        &["holy", "mountain", "shall"],
        &["drunk", "holy", "mountain", "shall", "heathen"],
    )?;
    segment.commit()?;
    Ok(())
}

fn obadiah_1_group_05(story: &mut Story) -> Result<(), StoryError> {
    let mut segment = story.segment(
        Book::Obadiah,
        1,
        17,
        21,
        "jacob expression -> guard -> move_value jacob mount zion",
        &["jacob", "mount", "zion", "shall", "deliverance"],
    );
    segment.expression(
        17,
        "jacob",
        &["mount", "zion", "shall"],
        &["jacob", "mount", "zion", "shall", "deliverance"],
    )?;
    segment.guard(
        18,
        "jacob",
        &["house", "shall", "fire"],
        &["jacob", "house", "shall", "fire", "joseph"],
    )?;
    segment.expression(
        19,
        "south",
        &["shall", "possess", "mount"],
        &["south", "shall", "possess", "mount", "esau"],
    )?;
    segment.expression(
        20,
        "children",
        &["captivity", "host", "israel"],
        &["children", "captivity", "host", "israel", "shall"],
    )?;
    segment.move_value(
        21,
        "saviours",
        &["shall", "mount", "zion"],
        &["saviours", "shall", "mount", "zion", "judge"],
    )?;
    segment.commit()?;
    Ok(())
}
