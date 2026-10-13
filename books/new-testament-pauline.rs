use crate::{Story, StoryError};

pub fn apply(story: &mut Story) -> Result<(), StoryError> {
    romans(story)?;
    _1_corinthians(story)?;
    _2_corinthians(story)?;
    galatians(story)?;
    ephesians(story)?;
    philippians(story)?;
    colossians(story)?;
    _1_thessalonians(story)?;
    _2_thessalonians(story)?;
    _1_timothy(story)?;
    _2_timothy(story)?;
    titus(story)?;
    philemon(story)?;
    Ok(())
}

fn romans(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Romans", 1, "Paul introduces himself as a commissioned messenger and says the good news reveals a rightness that runs on trust rather than status.", "He watches humanity exchange the Creator's glory for carved dependencies, and the resulting codebase spirals into pride, envy, violence, and approval of every bad patch.");
    crate::chapter!(story, "Romans", 2, "The reviewer who condemns everyone else discovers the same warnings in their own build log, because God's audit examines hidden motives as well as public commits.", "Possessing a law, a badge, or a repository label cannot substitute for a changed heart; the true mark is an inward work of the Spirit.");
    crate::chapter!(story, "Romans", 3, "Every faction has failed its tests, yet God's faithfulness is not canceled by human unfaithfulness or by the loudest list of exceptions.", "The new verdict comes through trust in Jesus rather than boasting in rule-keeping, leaving the whole team equally dependent on grace.");
    crate::chapter!(story, "Romans", 4, "Abraham is presented as the ancient case study: counted right before circumcision, before the badge, and before any performance metric could be claimed.", "His trust treats God's promise as a reliable return value, bringing life from a body that looked past its normal operating window.");
    crate::chapter!(story, "Romans", 5, "Being made right opens peace with God, and suffering is reframed as a chain that can compile perseverance, character, and hope.", "Adam's trespass spreads like a corrupted base commit, but the gift through Christ overflows farther, bringing life where the old failure brought death.");
    crate::chapter!(story, "Romans", 6, "Grace is not permission to keep shipping the same destructive release; those joined to Christ are pictured as having died to the old ownership model.", "The former master paid in death, while the new master gives a life that is not earned as wages but received as a gift.");
    crate::chapter!(story, "Romans", 7, "The law can expose the bug without possessing the power to fix it, leaving a conflicted developer wanting the good while finding the old process running again.", "The cry for rescue is answered by Jesus Christ, because diagnosis alone cannot free a person from the body of death.");
    crate::chapter!(story, "Romans", 8, "There is no condemnation for those in Christ, whose new runtime is governed by the Spirit rather than by the old sin process.", "Creation groans, believers wait, and the Spirit intercedes; through every crash and unknown state, God's love refuses to release its hold.");
    crate::chapter!(story, "Romans", 9, "Paul grieves over Israel and distinguishes ancestry from the deeper freedom of God's calling, using a potter and clay image to discuss mercy and patience.", "Gentiles who trusted found the promised righteousness, while Israel's pursuit through self-made credentials stumbled over the stone God provided.");
    crate::chapter!(story, "Romans", 10, "Zeal without knowledge is an overconfident build, and righteousness cannot be uploaded by climbing into heaven or digging into the abyss.", "The message is near enough to confess and trust, while the good news travels outward through messengers whose feet carry the deployment.");
    crate::chapter!(story, "Romans", 11, "Israel's present stumbling is not a final deletion: Gentiles are grafted in like branches warned against boasting over the root.", "Paul traces a mystery of mercy, ends in astonished praise, and reminds every branch that the whole tree depends on God's generosity.");
    crate::chapter!(story, "Romans", 12, "The community is urged to offer its whole life as a living service and to stop copying the world's default configuration.", "Many members receive different gifts, but sincere love, patient hope, practical hospitality, and blessing enemies are the shared interface.");
    crate::chapter!(story, "Romans", 13, "Paul addresses public authority, taxes, and civic obligations, then compresses the law's neighbor-facing work into love.", "The night is far spent, so believers are told to put on the Lord's character and stop provisioning the appetites that belong to darkness.");
    crate::chapter!(story, "Romans", 14, "Disputed diets and special days become a lesson in not turning personal settings into a universal gate, since each servant answers to the Lord.", "Stronger members must avoid making a sibling stumble; the kingdom's core values are righteousness, peace, and joy rather than menu arguments.");
    crate::chapter!(story, "Romans", 15, "The strong are told to carry the weak, and Christ is offered as the pattern for welcoming people across old boundary conditions.", "Paul describes his mission to unreached regions, asks for prayer about danger in Jerusalem, and hopes to visit Rome on the way toward Spain.");
    crate::chapter!(story, "Romans", 16, "Paul greets a long registry of coworkers, naming women, household churches, prisoners, hosts, and laborers whose names keep the mission from becoming a solo project.", "He warns against smooth teachers who divide communities, then closes with confidence that God's wisdom will outlast every deceptive fork.");
    Ok(())
}

fn _1_corinthians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Corinthians", 1, "Paul reports factional quarrels around favorite leaders and asks whether the cross has been split into competing ownership groups.", "God's wisdom overturns status metrics: the apparently foolish message and the unimpressive community become the place where boasting is disabled.");
    crate::chapter!(story, "1 Corinthians", 2, "Paul says he arrived without theatrical credentials, centering his proclamation on Christ crucified and trusting the Spirit's demonstration.", "The Spirit searches the deep things of God, while a merely natural evaluator treats spiritual wisdom like an unreadable binary artifact.");
    crate::chapter!(story, "1 Corinthians", 3, "The Corinthians are still on starter food because jealousy and rivalry show an immature codebase, even though they boast about being advanced.", "Leaders are only workers on God's field and builders on one foundation; each person's work will pass through a revealing fire test.");
    crate::chapter!(story, "1 Corinthians", 4, "Apostles are stewards under review by the Lord, not celebrities whose approval can be settled by applause or internet scoreboards.", "Paul contrasts the Corinthians' imagined reign with the apostles' hardships, then offers a fatherly correction backed by love rather than swagger.");
    crate::chapter!(story, "1 Corinthians", 5, "A scandalous relationship is tolerated inside the assembly, so Paul orders a decisive removal of the corrupt process for the sake of restoration and communal health.", "He distinguishes judging insiders from outsiders and tells the church not to treat persistent, public harm as a harmless configuration choice.");
    crate::chapter!(story, "1 Corinthians", 6, "Believers are taking one another to hostile courts, exposing the community's failure to settle ordinary disputes with wisdom among its own members.", "Bodies belong to the Lord, so sexual union is not disposable data; the body is a temple purchased at great cost.");
    crate::chapter!(story, "1 Corinthians", 7, "Paul answers questions about marriage, singleness, separation, and mixed-belief households, giving counsel that values fidelity while admitting differing callings.", "Each person should remain faithful in the station where they were called, remembering that ordinary circumstances can still serve the Lord's work.");
    crate::chapter!(story, "1 Corinthians", 8, "Food associated with idols may be technically harmless to one conscience, but knowledge without love can become a sharp tool that wounds a less certain sibling.", "Paul would rather surrender a permitted privilege than make another believer crash, choosing relational care over a proud claim of rights.");
    crate::chapter!(story, "1 Corinthians", 9, "Paul defends an apostle's right to support while explaining why he often declines that right, adapting himself to many kinds of people to win some.", "The mission is run like disciplined training: athletes practice for a fading prize, but the gospel's prize calls for endurance that keeps the body in check.");
    crate::chapter!(story, "1 Corinthians", 10, "Israel's wilderness history becomes a warning against idolatry, entitlement, and testing God after receiving remarkable provisions.", "The table of the Lord cannot be merged with the table of demons; whatever freedom remains must be filtered through God's glory and another person's good.");
    crate::chapter!(story, "1 Corinthians", 11, "Paul discusses worship gatherings, head coverings, and the dignity of men and women before turning to a meal where the wealthy leave the poor waiting.", "The shared bread and cup require honest self-examination, because a feast that humiliates the needy has become a broken integration test.");
    crate::chapter!(story, "1 Corinthians", 12, "The Spirit distributes many gifts through one body, so no member can declare another useless or claim to be the whole application alone.", "Wisdom, healing, languages, service, and discernment differ in operation, yet all are energized by the same God for the common build.");
    crate::chapter!(story, "1 Corinthians", 13, "Impressive speech, prophecy, knowledge, faith, generosity, and sacrifice all return empty results when love is absent from the function.", "Love is patient, durable, and truthful; gifts have an expiration horizon, but love remains when partial knowledge has finally completed.");
    crate::chapter!(story, "1 Corinthians", 14, "Paul prioritizes intelligible prophecy over unshared tongues in public worship, because the gathered church needs understandable encouragement and correction.", "Order matters: speakers take turns, discernment has room to work, and every contribution should build the body instead of turning worship into concurrent logs.");
    crate::chapter!(story, "1 Corinthians", 15, "Paul recites the central resurrection witness, then treats Christ's rising as the first successful return from death's locked process.", "The dead will be raised with transformed bodies, so labor is not wasted; the final victory makes steadfast work worth the cost.");
    crate::chapter!(story, "1 Corinthians", 16, "Paul gives practical instructions for a collection, travel plans, and possible visits, showing that theology still needs calendars, budgets, and trustworthy couriers.", "He commends coworkers, asks the church to stand firm in love, and ends with a warning against refusing the Lord's arrival.");
    Ok(())
}

fn _2_corinthians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Corinthians", 1, "Paul explains his changed travel plan and introduces God as the source of comfort who equips sufferers to comfort others.", "His ministry is not a sequence of contradictory yes-and-no commits; God's promises find their Yes in Christ, with the Spirit as the deposit.");
    crate::chapter!(story, "2 Corinthians", 2, "A severe offender is now to be forgiven and welcomed, lest excessive sorrow hand the person over to despair and Satan's advantage.", "Paul describes the gospel as a fragrance whose effect differs by recipient, and thanks God for leading the ministry through triumph and honest witness.");
    crate::chapter!(story, "2 Corinthians", 3, "The Corinthians themselves are living letters, written by the Spirit on human hearts rather than engraved on stone or carried as a credential packet.", "The old covenant's glory was real but temporary; in Christ the veil is removed, and the Spirit brings freedom with an ever-deepening transformation.");
    crate::chapter!(story, "2 Corinthians", 4, "Paul carries treasure in fragile clay containers so the surpassing power is clearly God's and not the courier's.", "Affliction presses but does not own him, because present trouble is producing an unseen weight of glory while faith keeps speaking.");
    crate::chapter!(story, "2 Corinthians", 5, "The earthly tent is temporary, but God prepares a lasting dwelling and gives the Spirit as advance assurance while believers walk by faith.", "Christ's love creates a new-creation ministry of reconciliation, with ambassadors urging enemies to receive peace from God.");
    crate::chapter!(story, "2 Corinthians", 6, "Paul lists hardships, patience, purity, truthful speech, and the paradox of appearing poor while making many rich in what matters.", "He asks the Corinthians not to bind themselves to incompatible worship systems and opens his heart for a restored, spacious relationship.");
    crate::chapter!(story, "2 Corinthians", 7, "Titus brings good news of the church's repentance, and Paul celebrates sorrow that turned toward life instead of merely producing regret.", "The community's earnestness, concern, and readiness to repair the breach show that disciplined correction has yielded healthy fruit.");
    crate::chapter!(story, "2 Corinthians", 8, "Macedonian assemblies give generously despite severe hardship, making their abundance of willingness a model for the Corinthians' promised collection.", "Christ's self-emptying becomes the pattern for balanced sharing, so one community's surplus can meet another's lack without coercion.");
    crate::chapter!(story, "2 Corinthians", 9, "Paul wants the gift ready before he arrives, not extracted by last-minute pressure, because generosity should be chosen with a willing heart.", "Sowing generously produces thanksgiving to God, and the collection becomes a ministry whose output is praise, prayer, and practical provision.");
    crate::chapter!(story, "2 Corinthians", 10, "Paul answers accusations that his letters are bold but his presence weak, insisting that the weapons of his service demolish proud arguments rather than bodies.", "He refuses to measure himself by rival boasting and will commend only what the Lord has genuinely built through the mission.");
    crate::chapter!(story, "2 Corinthians", 11, "False apostles market a counterfeit gospel, so Paul exposes their credentials while warning the church not to be seduced by polished presentation.", "His own catalog is a ledger of imprisonments, beatings, danger, sleeplessness, and daily anxiety for the churches, not a résumé of comfort.");
    crate::chapter!(story, "2 Corinthians", 12, "Paul mentions extraordinary visions but refuses to build a throne from them, instead accepting a persistent weakness that keeps him dependent on grace.", "Power reaches its mature expression in weakness, and Paul worries that his next visit may find unresolved jealousy, quarrels, and unrepentant disorder.");
    crate::chapter!(story, "2 Corinthians", 13, "Paul announces a careful examination on his coming visit and urges the Corinthians to test themselves for life in Christ.", "He closes with a call to restoration, shared peace, and the fellowship of Father, Son, and Spirit rather than another round of factional debugging.");
    Ok(())
}

fn galatians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Galatians", 1, "Paul insists that his commission and gospel did not come from a human approval queue, recounting his former zeal and the sudden rerouting of his life.", "Even an angelic-looking patch must be rejected if it replaces the good news already received, because there is no second gospel with better branding.");
    crate::chapter!(story, "Galatians", 2, "Paul recounts the Jerusalem agreement and confronts Peter for withdrawing from Gentile table fellowship when certain visitors arrive.", "A person is made right through Christ's faithfulness and trust, not by reinstalling the law as a badge; Paul says he now lives from Christ's life.");
    crate::chapter!(story, "Galatians", 3, "The Galatians began by the Spirit but are tempted to finish by fleshly performance, as though a successful migration required reverting to the old schema.", "Abraham's promise precedes the law, which served as a temporary guardian until faith's arrival made all who trust children and heirs.");
    crate::chapter!(story, "Galatians", 4, "Heirs under guardians resemble servants until the appointed time, but God sends the Son and the Spirit so adoption becomes an intimate, lived reality.", "Paul fears the Galatians are returning to calendar-based bondage and compares his appeal to labor pains until Christ's shape appears in them.");
    crate::chapter!(story, "Galatians", 5, "Christ's freedom is not a license for self-indulgence or a reason to make others carry a yoke; love fulfills the law by serving a neighbor.", "The flesh and Spirit pull in opposite directions, while the Spirit's fruit grows as a coherent cluster that outshines rivalry, envy, and rage.");
    crate::chapter!(story, "Galatians", 6, "The community should restore a caught offender gently, carry burdens, and still take responsibility for its own load instead of competitive comparison.", "Sowing to the flesh harvests decay, but patient goodness reaches a due season; Paul signs off with the cross as his only boast.");
    Ok(())
}

fn ephesians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Ephesians", 1, "Paul blesses God for choosing and adopting believers in Christ, where redemption, forgiveness, and a vast inheritance are disclosed.", "He prays that the church will know the hope, wealth, and resurrection power already directed toward Christ and shared with his body.");
    crate::chapter!(story, "Ephesians", 2, "Those once dead in trespasses are made alive by grace, not by a résumé of good works, and are remade for a life of prepared goodness.", "Christ dismantles the wall between Jew and Gentile, creating one new humanity and one household with access to the Father through the Spirit.");
    crate::chapter!(story, "Ephesians", 3, "Paul explains the revealed mystery that Gentiles share the promise, then describes his imprisonment as service for this wide-open inclusion.", "He prays for inner strength and for Christ's love to exceed every measurement, trusting God to do more than the church can request or imagine.");
    crate::chapter!(story, "Ephesians", 4, "The church is urged to walk worthy of its calling through humility, patience, unity, and diverse gifts that mature the whole body.", "Believers must discard the old deceitful runtime, speak truth, work honestly, forgive quickly, and avoid grieving the Spirit who sealed them.");
    crate::chapter!(story, "Ephesians", 5, "Paul calls the community to imitate God's self-giving love, reject hidden works of darkness, and walk wisely while the days are difficult.", "Marriage becomes a metaphor of sacrificial union, with mutual honor replacing domination and the church pictured as cherished rather than exploited.");
    crate::chapter!(story, "Ephesians", 6, "Household relationships are addressed with responsibility on every side, while children are guided and parents warned against provoking them into resentment.", "The final deployment equips believers with truth, righteousness, peace, faith, salvation, and the Spirit's word for a struggle against unseen powers.");
    Ok(())
}

fn philippians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Philippians", 1, "Paul thanks the Philippians for partnership from the first commit and says his imprisonment has advanced the gospel through guards, coworkers, and public courage.", "Whether by life or death, Christ remains the central value; the church is urged to live as one team, standing firm without being frightened by opposition.");
    crate::chapter!(story, "Philippians", 2, "The community is called to the mind of Christ, who descends from status into obedient service and is then exalted above every name.", "Believers should work out their salvation with reverent seriousness, shining as lights while Timothy and Epaphroditus model dependable care.");
    crate::chapter!(story, "Philippians", 3, "Paul dumps his old credential stack, counting ancestry, law observance, and prestige as loss beside knowing Christ and sharing his resurrection life.", "He presses toward the prize, warns against appetites that make idols of the belly, and points to a citizenship whose future includes transformed bodies.");
    crate::chapter!(story, "Philippians", 4, "Paul asks Euodia and Syntyche to reconcile, then teaches joy, gentleness, prayer, and disciplined attention in an anxious world.", "The Philippians' gift is a fragrant act of partnership, and Paul has learned contentment in both scarcity and abundance through Christ's strength.");
    Ok(())
}

fn colossians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Colossians", 1, "Paul celebrates the Colossians' faith and presents Christ as the visible image of the invisible God, the one through whom creation holds together.", "The gospel reaches outsiders, and Paul's labor is to present everyone mature, empowered by the same Christ who lives among his people as hope.");
    crate::chapter!(story, "Colossians", 2, "The church is warned against persuasive philosophy, secret knowledge, food calendars, and angelic intermediaries that make shadows more important than Christ's body.", "In Christ the fullness of deity dwells, the old record is canceled, and believers share a new life rather than submitting again to humanly invented restrictions.");
    crate::chapter!(story, "Colossians", 3, "Since believers have been raised with Christ, their attention should move upward while they terminate rage, slander, lust, and deceit in the old process.", "The new self crosses old divisions, and household instructions are framed by compassion, patience, forgiveness, love, and work done for the Lord.");
    crate::chapter!(story, "Colossians", 4, "Paul asks for alert prayer and an open door for the message, while urging gracious, seasoned speech that answers each person wisely.", "A closing roster names coworkers, messengers, and household churches, and Archippus receives a direct reminder to complete the ministry he received.");
    Ok(())
}

fn _1_thessalonians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Thessalonians", 1, "Paul remembers how the Thessalonians received the message with joy amid pressure and became an example whose faith report traveled widely.", "Their conversion meant abandoning idols to serve the living God and waiting for Jesus, the rescuer from the coming judgment.");
    crate::chapter!(story, "1 Thessalonians", 2, "Paul describes the mission as free from flattery, greed, and status games, combining a mother's tenderness with a father's steady encouragement.", "The Thessalonians accepted the word as God's work, endured local hostility, and became Paul's joy even while separated by repeated travel barriers.");
    crate::chapter!(story, "1 Thessalonians", 3, "Timothy is sent to strengthen the church under affliction, and his good report revives Paul with news of their faith and love.", "Paul prays that their love overflow toward everyone and that their hearts become blameless and steady before the Lord's arrival.");
    crate::chapter!(story, "1 Thessalonians", 4, "Paul urges holiness, honorable relationships, quiet work, and practical care, because love should not become an excuse for disorderly dependence.", "He comforts mourners with the promise that the dead in Christ are not lost from the coming gathering, which will reunite the people with the Lord.");
    crate::chapter!(story, "1 Thessalonians", 5, "The day of the Lord arrives unexpectedly, so believers should stay awake, sober, and dressed in faith, love, and hope rather than panic.", "Paul gives a dense community checklist: respect workers, help the weak, rejoice, pray, test prophecies, hold good, reject evil, and pursue peace.");
    Ok(())
}

fn _2_thessalonians(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Thessalonians", 1, "Paul praises the church's growing faith and endurance under persecution, describing righteous judgment and relief when Jesus is revealed.", "He prays that God will make them worthy of the calling and empower every good purpose, so Christ's name is honored in them.");
    crate::chapter!(story, "2 Thessalonians", 2, "Paul corrects rumors that the day of the Lord has already arrived, pointing to a rebellion and a lawless figure before the final unveiling.", "The church must stand firm in the received teaching while God strengthens its hope, comfort, and every good work.");
    crate::chapter!(story, "2 Thessalonians", 3, "Paul asks for prayer that the message will run freely and that the church will be guarded from hostile and unreliable people.", "He confronts idleness directly: those unwilling to work should not consume the community's shared table, though correction must remain brotherly rather than hateful.");
    Ok(())
}

fn _1_timothy(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "1 Timothy", 1, "Paul charges Timothy to stop teachers from drifting into speculative myths and endless genealogical debates that generate noise instead of faithful stewardship.", "The law exposes wrongdoing, while mercy appoints a former persecutor as a living demonstration that Christ came to save sinners.");
    crate::chapter!(story, "1 Timothy", 2, "The church is urged to pray for rulers and all people, seeking a peaceful life in which the saving purpose of God can be made known.", "Paul discusses conduct in gathered worship, emphasizing modesty, teachability, and the ordering of instruction within the community.");
    crate::chapter!(story, "1 Timothy", 3, "Paul lists the character required of overseers and servants: steady households, good reputations, self-control, hospitality, and tested faith.", "The church is called God's household and the support of truth, with Christ's revealed mystery as its central confession.");
    crate::chapter!(story, "1 Timothy", 4, "Later distortions will forbid good gifts and promote counterfeit holiness, so Timothy must reject myths and train for godliness.", "His public reading, teaching, example, and gift must keep advancing, letting progress become visible rather than hidden in private intention.");
    crate::chapter!(story, "1 Timothy", 5, "Timothy is taught how to treat older and younger believers, widows, elders, and accusations, balancing family responsibility with careful communal support.", "Leaders deserve honor and impartial judgment, while hurried appointments and selective favoritism are warned against as dangerous shortcuts.");
    crate::chapter!(story, "1 Timothy", 6, "Paul addresses servants, contentment, and the trap of turning religion into a revenue strategy that pierces hearts with many griefs.", "The rich are told to be generous and hope in God, while Timothy is charged to guard the deposit and avoid arguments mislabeled as knowledge.");
    Ok(())
}

fn _2_timothy(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "2 Timothy", 1, "Paul remembers Timothy's sincere faith and urges him to rekindle the gift, because the Spirit supplies courage, love, and disciplined judgment rather than fear.", "He is not to be ashamed of the testimony or of Paul's chains, but to guard the good deposit through the Spirit's help.");
    crate::chapter!(story, "2 Timothy", 2, "Timothy must pass the teaching to reliable people, endure hardship like a soldier, compete lawfully like an athlete, and labor patiently like a farmer.", "He should avoid quarrels, cleanse himself for honorable use, and correct opponents gently in hope that God grants them a change of mind.");
    crate::chapter!(story, "2 Timothy", 3, "The last days will feature self-absorbed, money-loving, boastful people whose religious interface masks a denial of its power.", "Timothy must continue in the Scriptures that trained him for salvation and equipped him for every good work, remembering Paul's example and endurance.");
    crate::chapter!(story, "2 Timothy", 4, "Paul gives his final charge to preach steadily through favorable and difficult seasons, because many will prefer teachers who echo their appetites.", "He looks toward the completed race, asks Timothy to bring the cloak and scrolls, names deserters and helpers, and trusts the Lord who stood by him.");
    Ok(())
}

fn titus(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Titus", 1, "Titus is left in Crete to appoint elders whose households, character, and teaching can withstand disorder and empty talk.", "Rebellious teachers exploit households for dishonest gain, so their mouths must be stopped and their claims tested against the hard local reputation.");
    crate::chapter!(story, "Titus", 2, "Different generations and roles are given concrete patterns of self-control, dignity, faithfulness, and good teaching suited to a healthy household.", "Grace trains believers to reject worldly cravings while waiting for Christ's appearing and devoting themselves to a people eager for good work.");
    crate::chapter!(story, "Titus", 3, "Christians should respect authorities, avoid slander, show gentleness, and remember that salvation came from mercy and renewal rather than personal achievements.", "After warning against divisive speculation, Paul gives practical travel notes, support instructions, and a final greeting grounded in grace.");
    Ok(())
}

fn philemon(story: &mut Story) -> Result<(), StoryError> {
    crate::chapter!(story, "Philemon", 1, "Paul appeals for Onesimus, once an escaped or estranged servant and now a beloved brother, asking Philemon to receive him as he would receive Paul.", "The letter turns a private debt into a test of gospel-shaped fellowship, with Paul offering to repay while trusting love to choose the deeper reconciliation.");
    Ok(())
}
