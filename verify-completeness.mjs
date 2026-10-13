import fs from "node:fs";
import path from "node:path";

const root = path.dirname(new URL(import.meta.url).pathname);
const canon = JSON.parse(fs.readFileSync(path.join(root, "canon.json"), "utf8"));
const failures = [];
let actualChapterCount = 0;
let actualVerseCount = 0;
const actualVerses = [];

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

function parseRustString(token) {
  try {
    return JSON.parse(token);
  } catch {
    return null;
  }
}

function extractMacroCalls(source, marker) {
  const calls = [];
  let cursor = 0;

  while (true) {
    const start = source.indexOf(marker, cursor);
    if (start === -1) break;

    let depth = 1;
    let inString = false;
    let escaped = false;
    let close = start + marker.length;

    for (; close < source.length; close += 1) {
      const character = source[close];
      if (inString) {
        if (escaped) {
          escaped = false;
        } else if (character === "\\") {
          escaped = true;
        } else if (character === '"') {
          inString = false;
        }
        continue;
      }
      if (character === '"') {
        inString = true;
      } else if (character === "(") {
        depth += 1;
      } else if (character === ")") {
        depth -= 1;
        if (depth === 0) break;
      }
    }

    calls.push(source.slice(start + marker.length, close));
    cursor = close + 1;
  }

  return calls;
}

function extractChapterCalls(source) {
  return extractMacroCalls(source, "crate::chapter!(");
}

function extractVerseCalls(source) {
  return extractMacroCalls(source, "crate::verse!(");
}

const booksByFile = canon.books.reduce((groups, book) => {
  if (!groups.has(book.file)) groups.set(book.file, []);
  groups.get(book.file).push(book);
  return groups;
}, new Map());

for (const [file, books] of booksByFile) {
  const fullPath = path.join(root, file);
  if (!fs.existsSync(fullPath)) {
    failures.push(`missing file ${file}`);
    continue;
  }

  const source = fs.readFileSync(fullPath, "utf8");
  if (!/pub\s+fn\s+apply\s*\(\s*story:\s*&mut\s+Story\s*\)/.test(source)) {
    failures.push(`${file}: missing executable pub fn apply(story: &mut Story)`);
  }

  for (const book of books) {
    const functionPattern = new RegExp(`\\bfn\\s+${functionIdentifier(book.name)}\\s*\\(`);
    if (!functionPattern.test(source)) {
      failures.push(`${book.name}: missing executable book function in ${file}`);
    }
  }

  const chapters = [];
  for (const call of extractChapterCalls(source)) {
    const match = call.match(/^\s*story,\s*"((?:\\.|[^"\\])*)",\s*(\d+),([\s\S]*)$/);
    if (!match) {
      failures.push(`${file}: malformed chapter! call`);
      continue;
    }
    const bookName = parseRustString(`"${match[1]}"`);
    const number = Number(match[2]);
    const prose = [...match[3].matchAll(/"(?:\\.|[^"\\])*"/g)]
      .map((literal) => parseRustString(literal[0]))
      .filter((line) => line !== null);

    if (bookName === null || prose.length < 2) {
      failures.push(`${file}: malformed chapter! call near chapter ${number}`);
      continue;
    }
    chapters.push({ book: bookName, number, prose });
  }

  const expected = books.flatMap((book) =>
    Array.from({ length: book.chapters }, (_, index) => ({
      book: book.name,
      number: index + 1,
    })),
  );

  if (chapters.length !== expected.length) {
    failures.push(`${file}: expected ${expected.length} chapter! calls, found ${chapters.length}`);
  }

  for (let index = 0; index < expected.length; index += 1) {
    const actual = chapters[index];
    const wanted = expected[index];
    if (!actual) break;
    if (actual.book !== wanted.book || actual.number !== wanted.number) {
      failures.push(
        `${file}: expected ${wanted.book} ${wanted.number} at call ${index + 1}, found ${actual.book} ${actual.number}`,
      );
      break;
    }
    if (actual.prose.length < 2) {
      failures.push(`${actual.book} ${actual.number}: needs at least two prose strings`);
    }
  }

  actualChapterCount += chapters.length;
}

const verseRoot = path.join(root, "verses");
const expectedVerseFiles = new Set(
  [...booksByFile.keys()].map((file) => file.replace(/^books\//, "verses/")),
);

function expectedVerses(books) {
  return books.flatMap((book) => {
    if (!Array.isArray(book.verses) || book.verses.length !== book.chapters) {
      failures.push(
        `${book.name}: canon.json needs one verse-count entry for each of its ${book.chapters} chapters`,
      );
      return [];
    }

    const countedVerses = book.verses.reduce((total, count) => total + count, 0);
    if (countedVerses !== book.verse_count) {
      failures.push(
        `${book.name}: canon.json verse_count is ${book.verse_count}, but chapter counts add to ${countedVerses}`,
      );
    }

    return book.verses.flatMap((count, chapterIndex) =>
      Array.from({ length: count }, (_, verseIndex) => ({
        book: book.name,
        variant: bookVariant(book.name),
        chapter: chapterIndex + 1,
        number: verseIndex + 1,
      })),
    );
  });
}

for (const file of expectedVerseFiles) {
  const fullPath = path.join(root, file);
  if (!fs.existsSync(fullPath)) {
    failures.push(`missing file ${file}`);
    continue;
  }

  const source = fs.readFileSync(fullPath, "utf8");
  if (!/pub\s+fn\s+apply\s*\(\s*story:\s*&mut\s+Story\s*\)/.test(source)) {
    failures.push(`${file}: missing executable pub fn apply(story: &mut Story)`);
  }
  if (/\/\/|\/\*/.test(source)) {
    failures.push(`${file}: verse ledger must not contain comments`);
  }
  if (/["']|r#*"/.test(source)) {
    failures.push(`${file}: verse ledger must not contain string or character literals`);
  }

  const books = booksByFile.get(file.replace(/^verses\//, "books/")) || [];
  for (const book of books) {
    const functionPattern = new RegExp(`\\bfn\\s+${functionIdentifier(book.name)}\\s*\\(`);
    if (!functionPattern.test(source)) {
      failures.push(`${book.name}: missing executable verse function in ${file}`);
    }
  }

  const calls = extractVerseCalls(source);
  const parsed = [];
  for (const call of calls) {
    const match = call.match(
      /^\s*story,\s*Book::([A-Za-z0-9_]+),\s*(\d+),\s*(\d+),\s*([\s\S]*)$/,
    );
    if (!match) {
      failures.push(`${file}: malformed verse! call`);
      continue;
    }

    const meaning = match[4].trim();
    if (
      !/^crate::meaning!\([\s\S]*\)$/.test(meaning) ||
      !/RustForm::[A-Za-z0-9_]+/.test(meaning) ||
      !/Signal::[A-Za-z0-9_]+/.test(meaning)
    ) {
      failures.push(
        `${file}: verse ${match[1]} ${match[2]}:${match[3]} is missing a complete crate::meaning! payload`,
      );
    }
    if (/Signal::Unclassified/.test(meaning)) {
      failures.push(
        `${file}: verse ${match[1]} ${match[2]}:${match[3]} uses the generic Unclassified signal`,
      );
    }

    parsed.push({
      variant: match[1],
      chapter: Number(match[2]),
      number: Number(match[3]),
    });
  }

  const expected = expectedVerses(books);
  if (calls.length !== expected.length) {
    failures.push(`${file}: expected ${expected.length} verse! calls, found ${calls.length}`);
  }

  for (let index = 0; index < expected.length; index += 1) {
    const actual = parsed[index];
    const wanted = expected[index];
    if (!actual) break;
    if (
      actual.variant !== wanted.variant ||
      actual.chapter !== wanted.chapter ||
      actual.number !== wanted.number
    ) {
      failures.push(
        `${file}: expected ${wanted.book} ${wanted.chapter}:${wanted.number} at verse call ${
          index + 1
        }, found ${actual.variant} ${actual.chapter}:${actual.number}`,
      );
      break;
    }
  }

  actualVerseCount += parsed.length;
  actualVerses.push(
    ...parsed.map((verse) => ({
      ...verse,
      file,
    })),
  );
}

if (fs.existsSync(verseRoot)) {
  for (const file of fs.readdirSync(verseRoot)) {
    if (!file.endsWith(".rs")) continue;
    const relativePath = `verses/${file}`;
    if (!expectedVerseFiles.has(relativePath)) {
      failures.push(`unexpected verse ledger file ${relativePath}`);
    }
  }
}

const forbidden = ["TODO", "CHAPTER OMITTED", "SUMMARY OMITTED", "[insert"];
for (const file of fs.readdirSync(path.join(root, "books"))) {
  const fullPath = path.join(root, "books", file);
  if (!file.endsWith(".rs")) continue;
  const source = fs.readFileSync(fullPath, "utf8");
  for (const token of forbidden) {
    if (source.toUpperCase().includes(token.toUpperCase())) {
      failures.push(`${file}: contains forbidden placeholder ${token}`);
    }
  }
}

if (!/\#\[path\s*=\s*"books\/old-testament-pentateuch\.rs"\]/.test(
  fs.readFileSync(path.join(root, "rustament.rs"), "utf8"),
)) {
  failures.push("rustament.rs: missing module wiring");
}

if (actualChapterCount !== canon.chapter_count) {
  failures.push(`total: expected ${canon.chapter_count} chapters, found ${actualChapterCount}`);
}

const expectedAllVerses = expectedVerses(canon.books);
if (canon.verse_count !== 31_102) {
  failures.push(`canon.json: expected verse_count 31102, found ${canon.verse_count}`);
}
if (actualVerseCount !== expectedAllVerses.length) {
  failures.push(
    `total: expected ${expectedAllVerses.length} verse! calls, found ${actualVerseCount}`,
  );
}
if (actualVerseCount !== 31_102) {
  failures.push(`total: expected 31102 verses, found ${actualVerseCount}`);
}

for (let index = 0; index < expectedAllVerses.length; index += 1) {
  const actual = actualVerses[index];
  const wanted = expectedAllVerses[index];
  if (!actual) break;
  if (
    actual.variant !== wanted.variant ||
    actual.chapter !== wanted.chapter ||
    actual.number !== wanted.number
  ) {
    failures.push(
      `total: expected ${wanted.book} ${wanted.chapter}:${wanted.number} at verse call ${
        index + 1
      }, found ${actual.variant} ${actual.chapter}:${actual.number}`,
    );
    break;
  }
}

const firstVerse = actualVerses[0];
if (
  !firstVerse ||
  firstVerse.variant !== bookVariant("Genesis") ||
  firstVerse.chapter !== 1 ||
  firstVerse.number !== 1
) {
  failures.push("total: first verse must be Genesis 1:1");
}

const lastVerse = actualVerses.at(-1);
if (
  !lastVerse ||
  lastVerse.variant !== bookVariant("Revelation") ||
  lastVerse.chapter !== 22 ||
  lastVerse.number !== 21
) {
  failures.push("total: last verse must be Revelation 22:21");
}

if (failures.length > 0) {
  console.error(failures.join("\n"));
  process.exit(1);
}

console.log(
  `PASS: ${canon.books.length} books, ${actualChapterCount} executable chapters, and ${actualVerseCount} executable verses are present.`,
);
