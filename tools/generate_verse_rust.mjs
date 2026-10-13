import fs from "node:fs";
import path from "node:path";

const root = path.dirname(new URL(import.meta.url).pathname).replace(/\/tools$/, "");
const sourcePath = process.argv[2] || "/tmp/rustament-source/kjv.json";
const canonPath = path.join(root, "canon.json");
const canon = JSON.parse(fs.readFileSync(canonPath, "utf8"));
const source = JSON.parse(fs.readFileSync(sourcePath, "utf8"));

const narrativeBooks = new Set([
  "Genesis",
  "Exodus",
  "Numbers",
  "Joshua",
  "Judges",
  "Ruth",
  "1 Samuel",
  "2 Samuel",
  "1 Kings",
  "2 Kings",
  "1 Chronicles",
  "2 Chronicles",
  "Ezra",
  "Nehemiah",
  "Esther",
]);
const lawBooks = new Set(["Leviticus", "Deuteronomy"]);
const wisdomBooks = new Set(["Job", "Psalms", "Proverbs", "Ecclesiastes", "Song of Solomon"]);
const prophetBooks = new Set([
  "Isaiah",
  "Jeremiah",
  "Lamentations",
  "Ezekiel",
  "Daniel",
  "Hosea",
  "Joel",
  "Amos",
  "Obadiah",
  "Jonah",
  "Micah",
  "Nahum",
  "Habakkuk",
  "Zephaniah",
  "Haggai",
  "Zechariah",
  "Malachi",
]);
const gospelBooks = new Set(["Matthew", "Mark", "Luke", "John"]);
const letterBooks = new Set([
  "Romans",
  "1 Corinthians",
  "2 Corinthians",
  "Galatians",
  "Ephesians",
  "Philippians",
  "Colossians",
  "1 Thessalonians",
  "2 Thessalonians",
  "1 Timothy",
  "2 Timothy",
  "Titus",
  "Philemon",
  "Hebrews",
  "James",
  "1 Peter",
  "2 Peter",
  "1 John",
  "2 John",
  "3 John",
  "Jude",
]);

function functionIdentifier(name) {
  return name
    .toLowerCase()
    .replace(/&/g, " and ")
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "")
    .replace(/^([0-9])/, "_$1");
}

function bookVariant(name) {
  return name
    .replace(/&/g, " and ")
    .split(/[^A-Za-z0-9]+/)
    .filter(Boolean)
    .map((part) => `${part[0].toUpperCase()}${part.slice(1).toLowerCase()}`)
    .join("_")
    .replace(/^([0-9])/, "_$1");
}

function lower(text) {
  return text.toLowerCase().replace(/[’]/g, "'");
}

function has(text, pattern) {
  return pattern.test(text);
}

function genreFor(book) {
  if (book === "Revelation") return "Vision";
  if (gospelBooks.has(book)) return "Gospel";
  if (letterBooks.has(book)) return "Letter";
  if (lawBooks.has(book)) return "Law";
  if (wisdomBooks.has(book)) return "Wisdom";
  if (prophetBooks.has(book)) return "Prophecy";
  if (narrativeBooks.has(book)) return "Narrative";
  return "Narrative";
}

const actionRules = [
  ["Create", /\b(created|create|made|formed|fashioned|built|build)\b/],
  ["Generate", /\b(begat|begotten|born|birth|conceived|offspring|seed)\b/],
  ["Die", /\b(died|death|dead|slain|buried|grave|mourn)\b/],
  ["Rise", /\b(raised|risen|resurrection|arose|ascended|ascension)\b/],
  ["Heal", /\b(heal|healed|healing|blind|lame|leper|sick|disease)\b/],
  ["Suffer", /\b(crucif|cross|suffer|suffering|persecut|tribulation)\b/],
  ["Deliver", /\b(deliver|delivered|save|saved|salvation|redeem|redeemed|rescue)\b/],
  ["Judge", /\b(judge|judgment|judgement|wrath|woe|punish|vengeance)\b/],
  ["Restore", /\b(forgive|forgiven|mercy|merciful|grace|restore|restored|return)\b/],
  ["Command", /\b(command|commandment|statute|ordinance|law|precept|must|shalt)\b/],
  ["Worship", /\b(altar|sacrifice|offering|offered|tabernacle|temple|worship)\b/],
  ["Praise", /\b(praise|sing|song|bless|blessed|hallelujah|glory)\b/],
  ["Pray", /\b(pray|prayer|cried|cry|wept|weep|supplication|ask)\b/],
  ["Reveal", /\b(dream|vision|revealed|revelation|saw|behold|angel)\b/],
  ["Teach", /\b(teach|taught|wisdom|knowledge|understand|parable|doctrine)\b/],
  ["Speak", /\b(said|say|spake|speak|answered|voice|word|declare)\b/],
  ["Move", /\b(went|go|came|come|brought|bring|sent|send|depart|journey)\b/],
  ["Provide", /\b(food|bread|water|wine|rain|harvest|flock|herd|manna)\b/],
  ["Rule", /\b(king|kingdom|reign|throne|ruler|prince|authority)\b/],
  ["Love", /\b(love|charity|beloved|kindness|compassion)\b/],
  ["Trust", /\b(faith|believe|believed|trust|hope|confidence)\b/],
  ["Fail", /\b(sin|sinned|iniquity|transgress|trespass|evil|wicked)\b/],
];

function actionFor(text) {
  for (const [action, pattern] of actionRules) {
    if (has(text, pattern)) return action;
  }
  return "Witness";
}

function actorFor(text) {
  if (has(text, /\b(jesus|christ|messiah|son of man|saviour|savior)\b/)) return "Messiah";
  if (has(text, /\b(lord|god|almighty|most high|jehovah)\b/)) return "God";
  if (has(text, /\b(angel|cherub|seraph)\b/)) return "Messenger";
  if (has(text, /\b(spirit|holy ghost)\b/)) return "Spirit";
  if (has(text, /\b(priest|levite|high priest)\b/)) return "Priest";
  if (has(text, /\b(king|queen|ruler|prince|pharaoh|governor)\b/)) return "King";
  if (has(text, /\b(moses|aaron|abraham|isaac|jacob|joseph|david|paul|peter|john)\b/)) {
    return "Witness";
  }
  if (has(text, /\b(israel|judah|people|nation|nations|church|disciples)\b/)) return "Community";
  return "Humanity";
}

function focusFor(text, book) {
  if (has(text, /\b(heaven|earth|sun|moon|star|light|darkness|sea|waters)\b/)) return "Cosmos";
  if (has(text, /\b(covenant|promise|oath|swear|inheritance)\b/)) return "Covenant";
  if (has(text, /\b(law|commandment|statute|ordinance|precept)\b/)) return "Law";
  if (has(text, /\b(temple|tabernacle|altar|sanctuary|house of god)\b/)) return "Temple";
  if (has(text, /\b(king|kingdom|throne|reign|ruler)\b/)) return "Kingdom";
  if (has(text, /\b(son|daughter|father|mother|brother|sister|wife|husband|seed)\b/)) {
    return "Household";
  }
  if (has(text, /\b(just|justice|judgment|poor|widow|fatherless|oppress)\b/)) return "Justice";
  if (has(text, /\b(wisdom|wise|knowledge|understanding|instruction)\b/)) return "Wisdom";
  if (has(text, /\b(spirit|ghost|power|anoint)\b/)) return "Spirit";
  if (has(text, /\b(satan|devil|serpent|dragon|enemy|adversary)\b/)) return "Adversary";
  if (has(text, /\b(christ|jesus|messiah|lamb|cross|resurrection)\b/)) return "Messiah";
  if (book === "Revelation") return "Renewal";
  if (has(text, /\b(israel|judah|jerusalem|zion)\b/)) return "Israel";
  if (has(text, /\b(nation|nations|gentile|people)\b/)) return "Nations";
  return "Community";
}

function registerFor(text, book) {
  if (has(text, /\b(begat|genealogy|generations|son of|daughter of)\b/)) return "Genealogy";
  if (has(text, /\b(woe|judgment|judgement|wrath|day of the lord|punish)\b/)) return "Warning";
  if (has(text, /\b(command|commandment|statute|ordinance|precept|shalt|must)\b/)) {
    return "Instruction";
  }
  if (has(text, /\b(covenant|promise|oath|swear|shall be|will be)\b/)) return "Promise";
  if (has(text, /\b(pray|prayer|cried|cry|supplication|ask)\b/)) return "Petition";
  if (has(text, /\b(vision|dream|revealed|behold|saw)\b/)) return "Vision";
  if (has(text, /\b(praise|sing|song|hallelujah|glory)\b/)) return "Praise";
  if (has(text, /\b(weep|wept|lament|sorrow|grief)\b/)) return "Lament";
  if (book === "Psalms" || book === "Proverbs" || book === "Ecclesiastes") return "Wisdom";
  if (book === "Revelation") return "Vision";
  return "Narration";
}

const signalRules = [
  ["Beginning", /\b(beginning|first|created|create|made|formed|light)\b/],
  ["Separation", /\b(separate|divided|divide|apart|distinguish)\b/],
  ["Naming", /\b(call(ed)?|name(d)?|called)\b/],
  ["Genealogy", /\b(begat|genealogy|generations|son of|daughter of)\b/],
  ["Covenant", /\b(covenant|promise|oath|swear|inheritance)\b/],
  ["Exodus", /\b(egypt|pharaoh|plague|passover|flee|deliver)\b/],
  ["Wilderness", /\b(wilderness|desert|manna|camp|tent)\b/],
  ["Land", /\b(land|ground|mountain|city|field|vineyard)\b/],
  ["Kingship", /\b(king|kingdom|throne|reign|ruler|prince)\b/],
  ["Exile", /\b(exile|captiv|scatter|foreign|babylon|assyria)\b/],
  ["Return", /\b(return|restore|restored|rebuild|come again)\b/],
  ["Temple", /\b(temple|tabernacle|altar|sanctuary|house of god)\b/],
  ["Sacrifice", /\b(sacrifice|offering|blood|altar|atonement)\b/],
  ["Justice", /\b(justice|righteous|poor|widow|fatherless|oppress)\b/],
  ["Mercy", /\b(mercy|merciful|grace|compassion|forgive)\b/],
  ["Wisdom", /\b(wisdom|wise|knowledge|understanding|instruction)\b/],
  ["Praise", /\b(praise|sing|song|bless|hallelujah|glory)\b/],
  ["Lament", /\b(weep|wept|lament|sorrow|grief|mourning)\b/],
  ["Prophecy", /\b(prophet|vision|dream|oracle|behold|day of the lord)\b/],
  ["Messenger", /\b(angel|messenger|sent)\b/],
  ["Healing", /\b(heal|healed|blind|lame|leper|sick|disease)\b/],
  ["Hospitality", /\b(feed|food|bread|water|guest|stranger|house)\b/],
  ["Conflict", /\b(war|battle|fight|enemy|foe|persecut)\b/],
  ["Persecution", /\b(prison|suffer|tribulation|persecut|affliction)\b/],
  ["Resurrection", /\b(raised|risen|resurrection|arose|ascended)\b/],
  ["Mission", /\b(preach|teach|disciple|witness|sent|nations)\b/],
  ["Church", /\b(church|assembly|brethren|body of christ)\b/],
  ["Judgment", /\b(judge|judgment|judgement|wrath|woe|punish)\b/],
  ["Renewal", /\b(new heaven|new earth|life|renew|restore|river of life)\b/],
  ["Provision", /\b(food|bread|water|rain|harvest|flock|herd|manna)\b/],
  ["Prayer", /\b(pray|prayer|cried|cry|supplication)\b/],
  ["Marriage", /\b(marriage|married|wife|husband|bride|bridegroom)\b/],
  ["Birth", /\b(born|birth|conceived|womb|travail)\b/],
  ["Death", /\b(died|death|dead|slain|buried|grave)\b/],
  ["Famine", /\b(famine|hunger|hungry|drought)\b/],
  ["Harvest", /\b(harvest|fruit|vine|wheat|reap|grain)\b/],
  ["Law", /\b(law|commandment|statute|ordinance|precept)\b/],
  ["Spirit", /\b(spirit|ghost|power|anoint)\b/],
  ["Baptism", /\b(baptiz|water|jordan)\b/],
  ["Faith", /\b(faith|believe|trust|hope|confidence)\b/],
  ["Love", /\b(love|charity|beloved|kindness|compassion)\b/],
  ["Neighbor", /\b(neighbor|brother|sister|one another)\b/],
  ["Adversary", /\b(satan|devil|serpent|dragon|enemy|adversary)\b/],
  ["Vision", /\b(vision|dream|revealed|behold|saw)\b/],
  ["Witness", /\b(witness|testimony|record|remember)\b/],
];

function signalsFor(text) {
  const signals = [];
  for (const [signal, pattern] of signalRules) {
    if (has(text, pattern)) signals.push(signal);
    if (signals.length === 5) break;
  }
  return signals;
}

function fallbackSignalFor(action) {
  const signals = {
    Create: "Beginning",
    Generate: "Genealogy",
    Die: "Death",
    Rise: "Resurrection",
    Heal: "Healing",
    Suffer: "Persecution",
    Deliver: "Mercy",
    Judge: "Judgment",
    Restore: "Renewal",
    Command: "Law",
    Worship: "Temple",
    Praise: "Praise",
    Pray: "Prayer",
    Reveal: "Vision",
    Teach: "Wisdom",
    Speak: "Witness",
    Move: "Land",
    Provide: "Provision",
    Rule: "Kingship",
    Love: "Love",
    Trust: "Faith",
    Fail: "Adversary",
    Witness: "Witness",
  };
  return signals[action] || "Witness";
}

function rustFormFor({ action, genre, register }) {
  if (register === "Genealogy") return "Iterate";
  if (register === "Instruction" || register === "Warning") return "Guard";
  if (register === "Promise") return "Return";
  if (register === "Petition") return "Result";
  if (register === "Vision") return "Match";

  switch (action) {
    case "Create":
    case "Generate":
      return "Construct";
    case "Move":
      return "Move";
    case "Die":
      return "Drop";
    case "Rise":
    case "Restore":
      return "Reinitialize";
    case "Judge":
    case "Fail":
      return "Guard";
    case "Reveal":
      return "Match";
    case "Worship":
    case "Praise":
      return "Trait";
    case "Pray":
    case "Deliver":
      return "Result";
    case "Teach":
    case "Speak":
      return "Call";
    case "Provide":
    case "Love":
      return "Borrow";
    case "Trust":
      return "Await";
    default:
      return genre === "Letter" ? "Call" : "Expression";
  }
}

function meaningFor(book, verseText) {
  const text = lower(verseText);
  const genre = genreFor(book);
  const actor = actorFor(text);
  const action = actionFor(text);
  const focus = focusFor(text, book);
  const register = registerFor(text, book);
  const signals = signalsFor(text);
  return {
    genre,
    actor,
    action,
    focus,
    register,
    rustForm: rustFormFor({ action, genre, register }),
    signals: signals.length > 0 ? signals : [fallbackSignalFor(action)],
  };
}

function renderMeaning(meaning) {
  const signals = meaning.signals.map((signal) => `Signal::${signal}`).join(", ");
  return `crate::meaning!(Genre::${meaning.genre}, Actor::${meaning.actor}, Action::${meaning.action}, Focus::${meaning.focus}, Register::${meaning.register}, RustForm::${meaning.rustForm}, ${signals})`;
}

const sourceByBook = new Map(source.books.map((book) => [book.englishName, book]));
const canonBookNames = canon.books.map((book) => book.name);
const missingBooks = canonBookNames.filter((book) => !sourceByBook.has(book));
if (missingBooks.length > 0) {
  throw new Error(`source is missing canonical books: ${missingBooks.join(", ")}`);
}

const verseCounts = [];
let totalVerses = 0;
const generatedByFile = new Map();

for (const canonBook of canon.books) {
  const sourceBook = sourceByBook.get(canonBook.name);
  if (sourceBook.chapters.length !== canonBook.chapters) {
    throw new Error(
      `${canonBook.name}: expected ${canonBook.chapters} chapters, found ${sourceBook.chapters.length}`,
    );
  }

  const counts = [];
  const chapters = [];
  for (const sourceChapter of sourceBook.chapters) {
    counts.push(sourceChapter.verses.length);
    totalVerses += sourceChapter.verses.length;
    chapters.push({
      number: sourceChapter.chapter,
      verses: sourceChapter.verses.map((verse) => ({
        number: verse.number,
        meaning: meaningFor(canonBook.name, verse.text),
      })),
    });
  }
  verseCounts.push(counts);
  generatedByFile.set(canonBook.file, [
    ...(generatedByFile.get(canonBook.file) || []),
    { name: canonBook.name, chapters },
  ]);
}

if (totalVerses !== 31_102) {
  throw new Error(`expected the standard Protestant 31,102 verses, found ${totalVerses}`);
}

canon.verse_count = totalVerses;
for (const book of canon.books) {
  const sourceBook = sourceByBook.get(book.name);
  book.verse_count = sourceBook.chapters.reduce((count, chapter) => count + chapter.verses.length, 0);
  book.verses = sourceBook.chapters.map((chapter) => chapter.verses.length);
}
fs.writeFileSync(canonPath, `${JSON.stringify(canon, null, 2)}\n`);

const countsSource = [
  "pub const CANONICAL_VERSE_COUNTS: &[&[u16]] = &[",
  ...verseCounts.map((counts) => `    &[${counts.join(", ")}],`),
  "];",
  "",
  `pub const CANONICAL_VERSE_TOTAL: usize = ${totalVerses};`,
  "",
];
fs.writeFileSync(path.join(root, "verse_counts.rs"), countsSource.join("\n"));

const versesRoot = path.join(root, "verses");
fs.rmSync(versesRoot, { recursive: true, force: true });
fs.mkdirSync(versesRoot, { recursive: true });

for (const [file, books] of generatedByFile) {
  const output = [
    "use crate::{Action, Actor, Book, Focus, Genre, Register, RustForm, Signal, Story, StoryError};",
    "",
    "pub fn apply(story: &mut Story) -> Result<(), StoryError> {",
    ...books.map((book) => `    ${functionIdentifier(book.name)}(story)?;`),
    "    Ok(())",
    "}",
    "",
  ];

  for (const book of books) {
    output.push(
      `fn ${functionIdentifier(book.name)}(story: &mut Story) -> Result<(), StoryError> {`,
    );
    for (const chapter of book.chapters) {
      for (const verse of chapter.verses) {
        output.push(
          `    crate::verse!(story, Book::${bookVariant(book.name)}, ${chapter.number}, ${verse.number}, ${renderMeaning(verse.meaning)});`,
        );
      }
    }
    output.push("    Ok(())", "}", "");
  }

  const target = path.join(root, file.replace(/^books\//, "verses/"));
  fs.writeFileSync(target, output.join("\n"));
  console.log(`generated ${target}: ${books.reduce((total, book) => total + book.chapters.reduce((n, chapter) => n + chapter.verses.length, 0), 0)} verses`);
}

console.log(`generated verse ledger: ${totalVerses} verses`);
