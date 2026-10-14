import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const booksRoot = path.join(root, "books");
const sourcePath = process.argv[2] || "/tmp/rustament-source/kjv.json";
const canonPath = path.join(root, "canon.json");
const rustamentPath = path.join(root, "rustament.rs");
const canon = JSON.parse(fs.readFileSync(canonPath, "utf8"));
const source = JSON.parse(fs.readFileSync(sourcePath, "utf8"));
const legacyPartitionFiles = [
  "books/new-testament-general-revelation.rs",
  "books/new-testament-gospels-acts.rs",
  "books/new-testament-pauline.rs",
  "books/old-testament-history.rs",
  "books/old-testament-major-prophets.rs",
  "books/old-testament-minor-prophets.rs",
  "books/old-testament-pentateuch.rs",
  "books/old-testament-wisdom.rs",
];

function functionIdentifier(name) {
  return name
    .toLowerCase()
    .replace(/&/g, " and ")
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "")
    .replace(/^([0-9])/, "_$1");
}

function moduleIdentifier(name) {
  return `book_${functionIdentifier(name).replace(/^_/, "")}`;
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

function rustString(value) {
  return JSON.stringify(value);
}

function rustSlice(values) {
  if (values.length === 0) throw new Error("cannot render an empty Rust slice");
  return `&[${values.map(rustString).join(", ")}]`;
}

function parseMacroCalls(sourceText, marker) {
  const calls = [];
  let cursor = 0;

  while (true) {
    const start = sourceText.indexOf(marker, cursor);
    if (start === -1) break;

    let depth = 1;
    let inString = false;
    let escaped = false;
    let close = start + marker.length;
    for (; close < sourceText.length; close += 1) {
      const character = sourceText[close];
      if (inString) {
        if (escaped) escaped = false;
        else if (character === "\\") escaped = true;
        else if (character === '"') inString = false;
        continue;
      }
      if (character === '"') inString = true;
      else if (character === "(") depth += 1;
      else if (character === ")") {
        depth -= 1;
        if (depth === 0) break;
      }
    }

    if (depth !== 0) throw new Error(`unterminated ${marker} call`);
    calls.push(sourceText.slice(start + marker.length, close));
    cursor = close + 1;
  }

  return calls;
}

function parseChapterProse(file) {
  const fullPath = path.join(root, file);
  if (!fs.existsSync(fullPath)) return [];

  const sourceText = fs.readFileSync(fullPath, "utf8");
  return parseMacroCalls(sourceText, "crate::chapter!(").map((call) => {
    const header = call.match(/^\s*story,\s*"((?:\\.|[^"\\])*)",\s*(\d+),([\s\S]*)$/);
    if (!header) throw new Error(`${file}: malformed chapter! call`);

    const prose = [...header[3].matchAll(/"(?:\\.|[^"\\])*"/g)].map((match) =>
      JSON.parse(match[0]),
    );
    if (prose.length < 2) throw new Error(`${file}: chapter has fewer than two prose lines`);

    return {
      book: JSON.parse(`"${header[1]}"`),
      number: Number(header[2]),
      prose,
    };
  });
}

function proseSourceFiles() {
  const files = new Set([
    ...legacyPartitionFiles,
    ...canon.books.map((book) => book.file),
  ]);
  if (fs.existsSync(booksRoot)) {
    for (const entry of fs.readdirSync(booksRoot).sort()) {
      if (entry.endsWith(".rs")) files.add(path.join("books", entry));
    }
  }
  return [...files].sort();
}

function parseExistingChapters() {
  const chapters = new Map();

  for (const file of proseSourceFiles()) {
    for (const chapter of parseChapterProse(file)) {
      if (!chapters.has(chapter.book)) chapters.set(chapter.book, new Map());
      const bookChapters = chapters.get(chapter.book);
      const previous = bookChapters.get(chapter.number);
      if (previous && JSON.stringify(previous) !== JSON.stringify(chapter.prose)) {
        throw new Error(`${chapter.book} ${chapter.number}: conflicting chapter prose`);
      }
      bookChapters.set(chapter.number, chapter.prose);
    }
  }

  return chapters;
}

const stopWords = new Set([
  "a",
  "about",
  "after",
  "all",
  "also",
  "an",
  "and",
  "are",
  "as",
  "at",
  "be",
  "because",
  "been",
  "before",
  "being",
  "but",
  "by",
  "came",
  "can",
  "come",
  "could",
  "did",
  "do",
  "does",
  "for",
  "from",
  "had",
  "has",
  "have",
  "he",
  "her",
  "here",
  "him",
  "his",
  "how",
  "if",
  "in",
  "into",
  "is",
  "it",
  "its",
  "let",
  "may",
  "more",
  "my",
  "of",
  "on",
  "one",
  "only",
  "or",
  "our",
  "out",
  "said",
  "she",
  "should",
  "so",
  "some",
  "than",
  "that",
  "the",
  "their",
  "them",
  "then",
  "there",
  "these",
  "they",
  "this",
  "those",
  "thou",
  "thy",
  "to",
  "up",
  "upon",
  "was",
  "were",
  "what",
  "when",
  "where",
  "which",
  "who",
  "whom",
  "with",
  "ye",
  "you",
  "your",
]);

function sourceTokens(text) {
  return (
    text
      .toLowerCase()
      .replace(/[’]/g, "'")
      .match(/[a-z0-9]+(?:'[a-z0-9]+)?/g) || []
  );
}

function unique(values) {
  return [...new Set(values)];
}

function lexicalTokens(text, limit = 5) {
  const raw = sourceTokens(text);
  const meaningful = unique(raw.filter((token) => token.length >= 3 && !stopWords.has(token)));
  const fallback = unique(raw);
  const tokens = meaningful.length > 0 ? meaningful : fallback;
  if (tokens.length === 0) throw new Error("source verse contains no lexical tokens");
  return tokens.slice(0, limit);
}

const subjectWords = new Set([
  "jesus",
  "christ",
  "messiah",
  "saviour",
  "savior",
  "god",
  "lord",
  "spirit",
  "angel",
  "cherub",
  "seraph",
  "moses",
  "aaron",
  "abraham",
  "isaac",
  "jacob",
  "joseph",
  "david",
  "paul",
  "peter",
  "john",
  "pharaoh",
  "king",
  "queen",
  "priest",
  "levite",
  "prophet",
  "israel",
  "judah",
  "people",
  "nation",
  "nations",
  "man",
  "men",
  "woman",
  "women",
  "child",
  "children",
]);

function subjectFor(text) {
  const lexical = lexicalTokens(text, 8);
  return lexical.find((token) => subjectWords.has(token)) || lexical[0];
}

const operationRules = [
  ["construct", /\b(created|create|made|make|formed|fashioned|built|build|born|begat|begotten)\b/],
  ["move_value", /\b(went|go|gone|came|come|brought|bring|sent|send|depart|journey|flee|fled|crossed)\b/],
  ["drop_value", /\b(died|death|dead|slain|killed|kill|buried|grave|destroyed|destroy)\b/],
  ["reinitialize", /\b(raised|risen|resurrection|arose|ascended|restore|restored|renewed|new)\b/],
  ["guard", /\b(command|commandment|statute|ordinance|law|precept|must|shalt|keep|forbid|forbidden|not|no|nor|neither)\b/],
  ["result", /\b(pray|prayer|cried|cry|wept|weep|supplication|ask|asked|request)\b/],
  ["match_value", /\b(dream|vision|revealed|revelation|saw|behold|whether|interpret)\b/],
  ["call", /\b(said|say|spake|speak|answered|voice|word|declare|teach|taught|preach)\b/],
  ["trait_bound", /\b(altar|sacrifice|offering|offered|tabernacle|temple|worship|praise|sing|song|glory)\b/],
  ["await_value", /\b(faith|believe|believed|trust|hope|wait|waiting|confidence)\b/],
  ["borrow", /\b(love|charity|beloved|kindness|compassion|mercy|merciful|grace)\b/],
  ["return_value", /\b(return|returned|promise|covenant|inheritance|bless|blessed)\b/],
  ["iterate", /\b(genealogy|generations|every|each|numbered|counted|tribes|sons)\b/],
  ["panic", /\b(fear|afraid|woe|wrath|terror|tremble|trembled)\b/],
];

function operationFor(text) {
  const lower = text.toLowerCase();
  return operationRules.find(([, pattern]) => pattern.test(lower))?.[0] || "expression";
}

function verseData(verse) {
  const lexical = lexicalTokens(verse.text, 8);
  const subject = subjectFor(verse.text);
  const objects = lexical.filter((token) => token !== subject).slice(0, 3);
  const safeObjects = objects.length > 0 ? objects : [subject];
  const tokens = unique([subject, ...safeObjects, ...lexical]).slice(0, 5);
  return {
    verse: verse.number,
    operation: operationFor(verse.text),
    subject,
    objects: safeObjects,
    tokens,
  };
}

function groupSummary(steps) {
  const operations = unique(steps.map((step) => step.operation)).join(" -> ");
  const lexical = unique(steps.flatMap((step) => step.tokens)).slice(0, 3);
  return `${steps[0].subject} ${operations} ${lexical.join(" ")}`;
}

function groupContext(steps) {
  return unique(steps.flatMap((step) => step.tokens)).slice(0, 5);
}

function groupRanges(verses) {
  const count = verses.length;
  if (count < 3) return [[0, count]];

  const sizes = [];
  const remainder = count % 4;
  if (remainder === 0) {
    sizes.push(...Array.from({ length: count / 4 }, () => 4));
  } else if (remainder === 1) {
    sizes.push(...Array.from({ length: (count - 5) / 4 }, () => 4), 5);
  } else if (remainder === 2) {
    sizes.push(...Array.from({ length: (count - 6) / 4 }, () => 4), 3, 3);
  } else {
    sizes.push(...Array.from({ length: (count - 3) / 4 }, () => 4), 3);
  }

  const ranges = [];
  let start = 0;
  for (const size of sizes) {
    ranges.push([start, start + size]);
    start += size;
  }
  if (start !== count) throw new Error(`failed to group ${count} verses`);
  return ranges;
}

function renderGroupFunction(book, chapter, groupNumber, verses, start, end) {
  const steps = verses.slice(start, end).map(verseData);
  const firstVerse = steps[0].verse;
  const lastVerse = steps.at(-1).verse;
  const functionName = `${functionIdentifier(book.name)}_${chapter.chapter}_group_${String(groupNumber).padStart(2, "0")}`;
  const lines = [
    `fn ${functionName}(story: &mut Story) -> Result<(), StoryError> {`,
    `    let mut segment = story.segment(Book::${bookVariant(book.name)}, ${chapter.chapter}, ${firstVerse}, ${lastVerse}, ${rustString(groupSummary(steps))}, ${rustSlice(groupContext(steps))});`,
  ];

  for (const step of steps) {
    lines.push(
      `    segment.${step.operation}(${step.verse}, ${rustString(step.subject)}, ${rustSlice(step.objects)}, ${rustSlice(step.tokens)})?;`,
    );
  }

  lines.push("    segment.commit()?;", "    Ok(())", "}", "");
  return { functionName, lines };
}

function renderBook(book, chapters, sourceBook) {
  const bookIdentifier = functionIdentifier(book.name);
  const lines = [
    "use crate::{Book, Story, StoryError};",
    "",
    "pub fn apply(story: &mut Story) -> Result<(), StoryError> {",
    `    ${bookIdentifier}(story)?;`,
    "    Ok(())",
    "}",
    "",
    `fn ${bookIdentifier}(story: &mut Story) -> Result<(), StoryError> {`,
  ];

  for (const chapter of chapters) {
    lines.push(`    ${bookIdentifier}_${chapter.number}(story)?;`);
  }

  lines.push("    Ok(())", "}", "");
  const groupFunctions = [];
  for (const chapter of chapters) {
    const sourceChapter = sourceBook.chapters[chapter.number - 1];
    const chapterFunctionName = `${bookIdentifier}_${chapter.number}`;
    const prose = chapter.prose.map(rustString).join(", ");
    lines.push(
      `fn ${chapterFunctionName}(story: &mut Story) -> Result<(), StoryError> {`,
      `    crate::chapter!(story, ${rustString(book.name)}, ${chapter.number}, ${prose});`,
    );

    const ranges = groupRanges(sourceChapter.verses);
    for (let index = 0; index < ranges.length; index += 1) {
      const [start, end] = ranges[index];
      const group = renderGroupFunction(
        book,
        sourceChapter,
        index + 1,
        sourceChapter.verses,
        start,
        end,
      );
      lines.push(`    ${group.functionName}(story)?;`);
      groupFunctions.push(group);
    }

    lines.push("    Ok(())", "}", "");
  }

  for (const group of groupFunctions) {
    lines.push(...group.lines);
  }
  return `${lines.join("\n")}\n`;
}

function validateSourceBook(book, sourceBook) {
  const expectedFile = `books/${functionIdentifier(book.name)}.rs`;
  if (book.file !== expectedFile) {
    throw new Error(`${book.name}: canon file must be ${expectedFile}, found ${book.file}`);
  }
  if (!sourceBook || !Array.isArray(sourceBook.chapters)) {
    throw new Error(`source is missing ${book.name}`);
  }
  if (sourceBook.chapters.length !== book.chapters) {
    throw new Error(
      `${book.name}: expected ${book.chapters} chapters, found ${sourceBook.chapters.length}`,
    );
  }
  if (!Array.isArray(book.verses) || book.verses.length !== book.chapters) {
    throw new Error(`${book.name}: canon.json has invalid per-chapter verse counts`);
  }

  let verseCount = 0;
  for (let chapterIndex = 0; chapterIndex < sourceBook.chapters.length; chapterIndex += 1) {
    const chapter = sourceBook.chapters[chapterIndex];
    if (chapter.chapter !== chapterIndex + 1) {
      throw new Error(`${book.name}: source chapter order breaks at ${chapter.chapter}`);
    }
    if (!Array.isArray(chapter.verses) || chapter.verses.length === 0) {
      throw new Error(`${book.name} ${chapter.chapter}: source chapter has no verses`);
    }
    if (chapter.verses.length !== book.verses[chapterIndex]) {
      throw new Error(
        `${book.name} ${chapter.chapter}: canon expects ${book.verses[chapterIndex]} verses, found ${chapter.verses.length}`,
      );
    }
    for (let verseIndex = 0; verseIndex < chapter.verses.length; verseIndex += 1) {
      const verse = chapter.verses[verseIndex];
      if (verse.number !== verseIndex + 1) {
        throw new Error(
          `${book.name} ${chapter.chapter}: source verse order breaks at ${verse.number}`,
        );
      }
      if (typeof verse.text !== "string" || verse.text.trim().length === 0) {
        throw new Error(`${book.name} ${chapter.chapter}:${verse.number}: source verse is empty`);
      }
      verseCount += 1;
    }
  }
  if (verseCount !== book.verse_count) {
    throw new Error(`${book.name}: canon expects ${book.verse_count} verses, found ${verseCount}`);
  }
}

function validateProse(book, proseByBook) {
  const proseByChapter = proseByBook.get(book.name);
  if (!proseByChapter || proseByChapter.size !== book.chapters) {
    throw new Error(`${book.name}: expected ${book.chapters} preserved chapter retellings`);
  }

  return Array.from({ length: book.chapters }, (_, index) => {
    const number = index + 1;
    const prose = proseByChapter.get(number);
    if (!prose) throw new Error(`${book.name} ${number}: missing preserved chapter retelling`);
    return { book: book.name, number, prose };
  });
}

const canonBookNames = canon.books.map((book) => book.name);
const sourceByBook = new Map(source.books.map((book) => [book.englishName, book]));
if (canon.books.length !== 66) throw new Error(`expected 66 canonical books, found ${canon.books.length}`);
if (source.books.length !== 66) throw new Error(`expected 66 source books, found ${source.books.length}`);
if (canonBookNames.some((name, index) => source.books[index]?.englishName !== name)) {
  throw new Error("source book order does not match canon.json");
}

const existingChapters = parseExistingChapters();
const renderedBooks = [];
let totalVerses = 0;
let totalSegments = 0;

for (const book of canon.books) {
  const sourceBook = sourceByBook.get(book.name);
  validateSourceBook(book, sourceBook);
  const chapters = validateProse(book, existingChapters);
  const targetFile = `books/${functionIdentifier(book.name)}.rs`;
  const output = renderBook(book, chapters, sourceBook);

  for (const verse of sourceBook.chapters.flatMap((chapter) => chapter.verses)) {
    if (verse.text.length >= 40 && output.includes(verse.text)) {
      throw new Error(`${book.name}: generated output contains a complete source verse`);
    }
  }

  renderedBooks.push({ targetFile, output, sourceBook });
  totalVerses += sourceBook.chapters.reduce((sum, chapter) => sum + chapter.verses.length, 0);
  totalSegments += sourceBook.chapters.reduce(
    (sum, chapter) => sum + groupRanges(chapter.verses).length,
    0,
  );
}

if (totalVerses !== 31_102) throw new Error(`expected 31,102 verses, found ${totalVerses}`);
if (canon.books.reduce((sum, book) => sum + book.chapters, 0) !== 1_189) {
  throw new Error("canon.json does not describe 1,189 chapters");
}
if (canon.books.reduce((sum, book) => sum + book.verse_count, 0) !== totalVerses) {
  throw new Error("canon.json verse totals do not match the source");
}

const targetNames = new Set(renderedBooks.map(({ targetFile }) => path.basename(targetFile)));
fs.mkdirSync(booksRoot, { recursive: true });
for (const entry of fs.readdirSync(booksRoot)) {
  if (entry.endsWith(".rs") && !targetNames.has(entry)) {
    fs.rmSync(path.join(booksRoot, entry), { force: true });
  }
}
fs.rmSync(path.join(root, "verses"), { recursive: true, force: true });

for (const { targetFile, output } of renderedBooks) {
  fs.writeFileSync(path.join(root, targetFile), output);
}

function updateRootModuleWiring() {
  const sourceText = fs.readFileSync(rustamentPath, "utf8");
  const firstModule = sourceText.indexOf("#[path =");
  const verseCounts = sourceText.indexOf("mod verse_counts;", firstModule);
  if (firstModule === -1 || verseCounts === -1) {
    throw new Error("rustament.rs: could not locate the book module wiring block");
  }

  const moduleLines = canon.books
    .map(
      (book) =>
        `#[rustfmt::skip]\n#[path = "${book.file}"]\nmod ${moduleIdentifier(book.name)};`,
    )
    .join("\n");
  const prefix = sourceText.slice(0, firstModule).replace(/(?:#\[rustfmt::skip\]\n)+$/, "");
  let updated = `${prefix}${moduleLines}\n${sourceText.slice(verseCounts)}`;

  const wholeStory = updated.indexOf("pub fn whole_story");
  const firstApply = updated.indexOf("\n    book", wholeStory);
  const applyEnd = updated.indexOf("\n\n    if story.chapters", firstApply);
  if (wholeStory === -1 || firstApply === -1 || applyEnd === -1) {
    throw new Error("rustament.rs: could not locate the whole_story book application block");
  }
  const applyLines = canon.books
    .map((book) => `    ${moduleIdentifier(book.name)}::apply(&mut story)?;`)
    .join("\n");
  updated = `${updated.slice(0, firstApply + 1)}${applyLines}${updated.slice(applyEnd)}`;

  if (updated !== sourceText) fs.writeFileSync(rustamentPath, updated);
}

updateRootModuleWiring();

canon.chapter_count = 1_189;
canon.verse_count = totalVerses;
for (const book of canon.books) {
  const sourceBook = sourceByBook.get(book.name);
  book.file = `books/${functionIdentifier(book.name)}.rs`;
  book.verse_count = sourceBook.chapters.reduce((sum, chapter) => sum + chapter.verses.length, 0);
  book.verses = sourceBook.chapters.map((chapter) => chapter.verses.length);
}
fs.writeFileSync(canonPath, `${JSON.stringify(canon, null, 2)}\n`);

const countsSource = [
  "pub const CANONICAL_VERSE_COUNTS: &[&[u16]] = &[",
  ...canon.books.map((book) => `    &[${book.verses.join(", ")}],`),
  "];",
  "",
  `pub const CANONICAL_VERSE_TOTAL: usize = ${totalVerses};`,
  "",
];
const countsPath = path.join(root, "verse_counts.rs");
fs.writeFileSync(countsPath, countsSource.join("\n"));

const generatedPaths = renderedBooks.map(({ targetFile }) => path.join(root, targetFile));
execFileSync("rustfmt", ["--edition", "2021", rustamentPath, ...generatedPaths, countsPath], {
  cwd: root,
  stdio: "inherit",
});
const largest = generatedPaths.reduce((largestPath, candidate) =>
  fs.statSync(candidate).size > fs.statSync(largestPath).size ? candidate : largestPath,
);
console.log(`generated ${generatedPaths.length} book files`);
console.log(`generated ${totalVerses} verse calls in ${totalSegments} Rust segments`);
console.log(`largest file: ${largest} (${fs.statSync(largest).size} bytes)`);
