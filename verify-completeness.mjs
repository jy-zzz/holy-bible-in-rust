import fs from "node:fs";
import path from "node:path";

const root = path.dirname(new URL(import.meta.url).pathname);
const canon = JSON.parse(fs.readFileSync(path.join(root, "canon.json"), "utf8"));
const failures = [];
const sourcePath =
  process.argv[2] || process.env.RUSTAMENT_SOURCE || "/tmp/rustament-source/kjv.json";
const allowedOperations = new Set([
  "construct",
  "move_value",
  "drop_value",
  "reinitialize",
  "guard",
  "result",
  "match_value",
  "call",
  "trait_bound",
  "panic",
  "await_value",
  "borrow",
  "return_value",
  "iterate",
  "expression",
]);

function identifier(name) {
  return name
    .toLowerCase()
    .replace(/&/g, " and ")
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "")
    .replace(/^([0-9])/, "_$1");
}

function moduleIdentifier(name) {
  return `book_${identifier(name).replace(/^_/, "")}`;
}

function sourceKey(book, chapter, verse) {
  return `${book}\u0000${chapter}\u0000${verse}`;
}

function sourceTokenSet(text) {
  return new Set(
    text
      .toLowerCase()
      .replace(/[’]/g, "'")
      .match(/[a-z0-9]+(?:'[a-z0-9]+)?/g) || [],
  );
}

let sourceVerseTokens = null;
try {
  const source = JSON.parse(fs.readFileSync(sourcePath, "utf8"));
  sourceVerseTokens = new Map();
  for (const sourceBook of source.books || []) {
    for (const chapter of sourceBook.chapters || []) {
      for (const verse of chapter.verses || []) {
        sourceVerseTokens.set(
          sourceKey(sourceBook.englishName, chapter.chapter, verse.number),
          sourceTokenSet(verse.text),
        );
      }
    }
  }
  if (sourceVerseTokens.size !== 31_102) {
    failures.push(
      `${sourcePath}: expected 31,102 source verses, found ${sourceVerseTokens.size}`,
    );
  }
} catch (error) {
  failures.push(`${sourcePath}: unable to load canonical source (${error.message})`);
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

function parseString(token) {
  try {
    return JSON.parse(token);
  } catch {
    return null;
  }
}

function rawStringEnd(source, start) {
  if (source[start] !== "r") return null;
  let cursor = start + 1;
  while (source[cursor] === "#") cursor += 1;
  if (source[cursor] !== '"') return null;
  const hashes = source.slice(start + 1, cursor);
  const terminator = `"${hashes}`;
  const end = source.indexOf(terminator, cursor + 1);
  return end === -1 ? source.length : end + terminator.length;
}

function normalStringEnd(source, start, quote = '"') {
  let escaped = false;
  for (let cursor = start + 1; cursor < source.length; cursor += 1) {
    const character = source[cursor];
    if (escaped) escaped = false;
    else if (character === "\\") escaped = true;
    else if (character === quote) return cursor + 1;
  }
  return source.length;
}

function maskNonCode(source) {
  const output = source.split("");
  let cursor = 0;

  while (cursor < source.length) {
    if (source.startsWith("//", cursor)) {
      const end = source.indexOf("\n", cursor + 2);
      const stop = end === -1 ? source.length : end;
      for (let index = cursor; index < stop; index += 1) output[index] = " ";
      cursor = stop;
      continue;
    }

    if (source.startsWith("/*", cursor)) {
      let depth = 1;
      let stop = cursor + 2;
      while (stop < source.length && depth > 0) {
        if (source.startsWith("/*", stop)) {
          depth += 1;
          stop += 2;
        } else if (source.startsWith("*/", stop)) {
          depth -= 1;
          stop += 2;
        } else {
          stop += 1;
        }
      }
      for (let index = cursor; index < stop; index += 1) {
        if (source[index] !== "\n" && source[index] !== "\r") output[index] = " ";
      }
      cursor = stop;
      continue;
    }

    const rawEnd = rawStringEnd(source, cursor);
    if (rawEnd !== null) {
      for (let index = cursor; index < rawEnd; index += 1) {
        if (source[index] !== "\n" && source[index] !== "\r") output[index] = " ";
      }
      cursor = rawEnd;
      continue;
    }

    if (source[cursor] === '"') {
      const end = normalStringEnd(source, cursor);
      for (let index = cursor; index < end; index += 1) {
        if (source[index] !== "\n" && source[index] !== "\r") output[index] = " ";
      }
      cursor = end;
      continue;
    }

    // Keep Rust lifetimes such as 'static, but hide ordinary character literals.
    if (
      source[cursor] === "'" &&
      cursor + 2 < source.length &&
      source[cursor + 1] !== "\n" &&
      source[cursor + 1] !== "\r" &&
      source[cursor + 2] === "'"
    ) {
      const end = normalStringEnd(source, cursor, "'");
      for (let index = cursor; index < end; index += 1) output[index] = " ";
      cursor = end;
      continue;
    }

    cursor += 1;
  }

  return output.join("");
}

function matchingDelimiter(maskedSource, openIndex, open = "(", close = ")") {
  let depth = 1;
  for (let cursor = openIndex + 1; cursor < maskedSource.length; cursor += 1) {
    if (maskedSource[cursor] === open) depth += 1;
    else if (maskedSource[cursor] === close) {
      depth -= 1;
      if (depth === 0) return cursor;
    }
  }
  return -1;
}

function extractCalls(source, marker) {
  const masked = maskNonCode(source);
  const calls = [];
  let cursor = 0;
  while (true) {
    const start = masked.indexOf(marker, cursor);
    if (start === -1) break;
    const openIndex = start + marker.length - 1;
    const close = matchingDelimiter(masked, openIndex);
    if (close === -1) {
      failures.push(`unterminated ${marker} call`);
      break;
    }
    calls.push({
      start,
      end: close + 1,
      body: source.slice(openIndex + 1, close),
    });
    cursor = close + 1;
  }
  return calls;
}

function extractMethodCalls(source, objectName) {
  const masked = maskNonCode(source);
  const pattern = new RegExp(
    `\\b${objectName}\\s*\\.\\s*([A-Za-z_][A-Za-z0-9_]*)\\s*\\(`,
    "g",
  );
  const calls = [];

  for (let match = pattern.exec(masked); match; match = pattern.exec(masked)) {
    const openIndex = masked.indexOf("(", match.index);
    const closeIndex = matchingDelimiter(masked, openIndex);
    if (closeIndex === -1) {
      failures.push(`unterminated ${objectName}.${match[1]} call`);
      continue;
    }
    calls.push({
      operation: match[1],
      start: match.index,
      end: closeIndex + 1,
      body: source.slice(openIndex + 1, closeIndex),
    });
  }
  return calls;
}

function splitTopLevel(source, delimiter = ",") {
  const masked = maskNonCode(source);
  const parts = [];
  const stack = [];
  let start = 0;

  for (let cursor = 0; cursor < masked.length; cursor += 1) {
    const character = masked[cursor];
    if (character === "(" || character === "[" || character === "{") {
      stack.push(character);
    } else if (character === ")" || character === "]" || character === "}") {
      stack.pop();
    } else if (character === delimiter && stack.length === 0) {
      parts.push(source.slice(start, cursor).trim());
      start = cursor + 1;
    }
  }
  parts.push(source.slice(start).trim());
  return parts.filter(Boolean);
}

function extractFunctions(source) {
  const masked = maskNonCode(source);
  const functions = [];
  const pattern = /\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(/g;

  for (let match = pattern.exec(masked); match; match = pattern.exec(masked)) {
    const openParen = masked.indexOf("(", match.index);
    const closeParen = matchingDelimiter(masked, openParen);
    if (closeParen === -1) {
      failures.push(`${match[1]}: unterminated function parameter list`);
      continue;
    }
    const bodyOpen = masked.indexOf("{", closeParen + 1);
    if (bodyOpen === -1) {
      failures.push(`${match[1]}: function has no body`);
      continue;
    }
    const bodyClose = matchingDelimiter(masked, bodyOpen, "{", "}");
    if (bodyClose === -1) {
      failures.push(`${match[1]}: unterminated function body`);
      continue;
    }
    functions.push({
      name: match[1],
      start: match.index,
      end: bodyClose + 1,
      header: source.slice(match.index, bodyOpen),
      body: source.slice(bodyOpen + 1, bodyClose),
    });
  }
  return functions;
}

function functionInvocations(source) {
  const masked = maskNonCode(source);
  const calls = [];
  const pattern = /\b([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*story\s*\)\s*\??\s*;/g;
  for (let match = pattern.exec(masked); match; match = pattern.exec(masked)) {
    calls.push(match[1]);
  }
  return calls;
}

function parseStringArray(body) {
  const values = [];
  let cursor = 0;
  while (cursor < body.length) {
    const rawEnd = rawStringEnd(body, cursor);
    if (rawEnd !== null) {
      const value = body.slice(cursor, rawEnd);
      const match = value.match(/^r(#+)"([\s\S]*)"\1$/);
      if (match) values.push(match[2]);
      cursor = rawEnd;
      continue;
    }
    if (body[cursor] === '"') {
      const end = normalStringEnd(body, cursor);
      const value = parseString(body.slice(cursor, end));
      if (value !== null) values.push(value);
      cursor = end;
      continue;
    }
    cursor += 1;
  }
  return values;
}

function parseFunctionsByName(source) {
  return new Map(extractFunctions(source).map((functionInfo) => [functionInfo.name, functionInfo]));
}

function parseChapter(call, file) {
  const header = call.body.match(/^\s*story,\s*"((?:\\.|[^"\\])*)",\s*(\d+),([\s\S]*)$/);
  if (!header) {
    failures.push(`${file}: malformed chapter! call`);
    return null;
  }
  const prose = parseStringArray(header[3]);
  const book = parseString(`"${header[1]}"`);
  if (book === null || prose.length < 2) {
    failures.push(`${file}: chapter ${header[2]} needs at least two prose strings`);
    return null;
  }
  return { book, number: Number(header[2]), prose };
}

function parseSegment(call, file) {
  const args = splitTopLevel(call.body);
  if (args.length !== 6) {
    failures.push(`${file}: malformed story.segment(...) call`);
    return null;
  }
  const book = args[0].match(/^Book::([A-Za-z0-9_]+)$/);
  const chapter = Number(args[1]);
  const start = Number(args[2]);
  const end = Number(args[3]);
  const summary = parseString(args[4]);
  const context = parseStringArray(args[5]);
  if (
    !book ||
    !Number.isInteger(chapter) ||
    !Number.isInteger(start) ||
    !Number.isInteger(end) ||
    summary === null ||
    !/^&\[[\s\S]*\]$/.test(args[5])
  ) {
    failures.push(`${file}: malformed story.segment(...) call`);
    return null;
  }
  if (!context.length) failures.push(`${file}: segment ${chapter}:${start} has no context tokens`);
  if (summary.length === 0 || summary.length > 160 || summary.split(/\s+/).length > 24) {
    failures.push(`${file}: segment ${chapter}:${start}-${end} has an invalid summary`);
  }
  for (const token of context) {
    if (!/^[A-Za-z0-9][A-Za-z0-9'_-]*$/.test(token)) {
      failures.push(`${file}: segment ${chapter}:${start}-${end} has a non-lexical context token`);
    }
  }
  return {
    variant: book[1],
    chapter,
    start,
    end,
    summary,
    context,
  };
}

function parseOperation(call, file) {
  const args = splitTopLevel(call.body);
  if (!allowedOperations.has(call.operation)) {
    failures.push(`${file}: unknown segment operation ${call.operation}`);
    return null;
  }
  if (
    args.length !== 4 ||
    !/^\d+$/.test(args[0]) ||
    !/^&\[[\s\S]*\]$/.test(args[2]) ||
    !/^&\[[\s\S]*\]$/.test(args[3])
  ) {
    failures.push(`${file}: malformed segment.${call.operation}(...) call`);
    return null;
  }
  const subject = parseString(args[1]);
  const objects = parseStringArray(args[2]);
  const tokens = parseStringArray(args[3]);
  if (subject === null || !subject.length || !objects.length || !tokens.length) {
    failures.push(`${file}: segment.${call.operation} ${args[0]} has empty operands or lexical tokens`);
  }
  for (const value of [subject, ...objects, ...tokens]) {
    if (
      value === null ||
      value.length === 0 ||
      value.length > 96 ||
      value.split(/\s+/).length > 8 ||
      !/^[A-Za-z0-9][A-Za-z0-9 _'-]*$/.test(value)
    ) {
      failures.push(`${file}: segment.${call.operation} ${args[0]} has a prose-like lexical operand`);
      break;
    }
  }
  return {
    operation: call.operation,
    verse: Number(args[0]),
    subject,
    objects,
    tokens,
  };
}

function expectedVerses(book) {
  return book.verses.flatMap((count, chapterIndex) =>
    Array.from({ length: count }, (_, verseIndex) => ({
      book: book.name,
      variant: bookVariant(book.name),
      chapter: chapterIndex + 1,
      number: verseIndex + 1,
    })),
  );
}

const expectedFiles = canon.books.map((book) => book.file);
const expectedFileSet = new Set(expectedFiles);
const booksDir = path.join(root, "books");
const actualFiles = fs
  .readdirSync(booksDir)
  .filter((file) => file.endsWith(".rs"))
  .map((file) => `books/${file}`)
  .sort();

if (expectedFiles.length !== 66 || new Set(expectedFiles).size !== 66) {
  failures.push("canon.json: expected exactly 66 unique book files");
}
for (const file of expectedFiles) {
  if (!fs.existsSync(path.join(root, file))) failures.push(`missing ${file}`);
}
for (const file of actualFiles) {
  if (!expectedFileSet.has(file)) failures.push(`unexpected book file ${file}`);
}

const stalePartitions = [
  "new-testament-general-revelation.rs",
  "new-testament-gospels-acts.rs",
  "new-testament-pauline.rs",
  "old-testament-history.rs",
  "old-testament-major-prophets.rs",
  "old-testament-minor-prophets.rs",
  "old-testament-pentateuch.rs",
  "old-testament-wisdom.rs",
];
for (const file of stalePartitions) {
  if (fs.existsSync(path.join(booksDir, file))) failures.push(`stale partition books/${file}`);
}
if (fs.existsSync(path.join(root, "verses"))) failures.push("stale verses/ directory exists");

const rustamentPath = path.join(root, "rustament.rs");
if (!fs.existsSync(rustamentPath)) {
  failures.push("missing rustament.rs");
} else {
  const rustamentSource = fs.readFileSync(rustamentPath, "utf8");
  const wiring = [
    ...rustamentSource.matchAll(
      /#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*mod\s+(book_[A-Za-z0-9_]+)\s*;/g,
    ),
  ].map((match) => ({ file: match[1], module: match[2] }));
  const expectedWiring = canon.books.map((book) => ({
    file: book.file,
    module: moduleIdentifier(book.name),
  }));
  const wiringKeys = wiring.map((entry) => `${entry.file}|${entry.module}`);
  const expectedWiringKeys = expectedWiring.map((entry) => `${entry.file}|${entry.module}`);
  if (
    wiring.length !== expectedWiring.length ||
    new Set(wiringKeys).size !== wiringKeys.length ||
    wiringKeys.some((key, index) => key !== expectedWiringKeys[index])
  ) {
    failures.push("rustament.rs: expected exactly 66 canonical book modules in order");
  }
  if (/verses\/|new_testament_|old_testament_/.test(rustamentSource)) {
    failures.push("rustament.rs: stale verse or eight-way partition wiring remains");
  }

  const applyCalls = [
    ...rustamentSource.matchAll(
      /\bbook_[A-Za-z0-9_]+::apply\(\s*&mut\s+story\s*\)\s*\?\s*;/g,
    ),
  ].map((match) => match[0].match(/\b(book_[A-Za-z0-9_]+)::apply/)[1]);
  const expectedApplyCalls = canon.books.map((book) => moduleIdentifier(book.name));
  if (
    applyCalls.length !== expectedApplyCalls.length ||
    applyCalls.some((name, index) => name !== expectedApplyCalls[index])
  ) {
    failures.push("rustament.rs: whole_story must apply all 66 books in canonical order");
  }
}

let actualChapterCount = 0;
let actualVerseCount = 0;
let actualSegmentCount = 0;
const actualVerses = [];

for (const book of canon.books) {
  const file = book.file;
  const fullPath = path.join(root, file);
  if (!fs.existsSync(fullPath)) continue;
  const source = fs.readFileSync(fullPath, "utf8");
  const expectedId = identifier(book.name);
  const functions = extractFunctions(source);
  const functionsByName = new Map();
  for (const functionInfo of functions) {
    if (functionsByName.has(functionInfo.name)) {
      failures.push(`${file}: duplicate function ${functionInfo.name}`);
    }
    functionsByName.set(functionInfo.name, functionInfo);
  }

  if (!/pub\s+fn\s+apply\s*\(\s*story:\s*&mut\s+Story\s*\)/.test(source)) {
    failures.push(`${file}: missing pub fn apply(story: &mut Story)`);
  }
  const applyFunction = functionsByName.get("apply");
  const bookFunction = functionsByName.get(expectedId);
  if (!bookFunction) {
    failures.push(`${file}: missing book function ${expectedId}`);
  } else if (
    !applyFunction ||
    functionInvocations(applyFunction.body).length !== 1 ||
    functionInvocations(applyFunction.body)[0] !== expectedId
  ) {
    failures.push(`${file}: apply must call ${expectedId}(story) exactly once`);
  }
  if (/crate::verse!|crate::meaning!/.test(source)) {
    failures.push(`${file}: contains the old semantic-only verse ledger`);
  }
  if (/(^|\r?\n)\s*\/\/|\/\*/.test(source)) {
    failures.push(`${file}: generated book source must not use comments as verse substitutes`);
  }
  for (const token of ["TODO", "CHAPTER OMITTED", "SUMMARY OMITTED", "[insert]", "the rest is similar"]) {
    if (source.toLowerCase().includes(token.toLowerCase())) {
      failures.push(`${file}: contains forbidden placeholder ${token}`);
    }
  }

  const expectedChapters = Array.from({ length: book.chapters }, (_, index) => ({
    book: book.name,
    number: index + 1,
  }));
  const groupPattern = new RegExp(`^${expectedId}_(\\d+)_group_(\\d+)$`);
  const groupFunctions = functions.filter((functionInfo) => groupPattern.test(functionInfo.name));
  const chapters = [];
  const expectedChapterFunctions = expectedChapters.map(
    (chapter) => `${expectedId}_${chapter.number}`,
  );
  const actualChapterFunctions = bookFunction
    ? functionInvocations(bookFunction.body)
    : [];
  if (
    actualChapterFunctions.length !== expectedChapterFunctions.length ||
    actualChapterFunctions.some((name, index) => name !== expectedChapterFunctions[index])
  ) {
    failures.push(`${file}: book function must call every chapter function exactly once in order`);
  }

  for (const expectedChapter of expectedChapters) {
    const chapterFunctionName = `${expectedId}_${expectedChapter.number}`;
    const chapterFunction = functionsByName.get(chapterFunctionName);
    if (!chapterFunction) {
      failures.push(`${file}: missing chapter function ${chapterFunctionName}`);
      continue;
    }
    const chapterCalls = extractCalls(chapterFunction.body, "crate::chapter!(");
    if (chapterCalls.length !== 1) {
      failures.push(`${file}:${chapterFunctionName}: expected exactly one chapter! call`);
      continue;
    }
    const chapter = parseChapter(chapterCalls[0], file);
    if (!chapter) continue;
    if (
      chapter.book !== expectedChapter.book ||
      chapter.number !== expectedChapter.number
    ) {
      failures.push(
        `${file}:${chapterFunctionName}: expected ${expectedChapter.book} ${expectedChapter.number}`,
      );
    }
    chapters.push(chapter);

    const expectedGroupFunctions = groupFunctions
      .filter((functionInfo) => functionInfo.name.startsWith(`${expectedId}_${expectedChapter.number}_group_`))
      .map((functionInfo) => functionInfo.name);
    const actualGroupFunctions = functionInvocations(chapterFunction.body).filter((name) =>
      groupPattern.test(name),
    );
    if (
      actualGroupFunctions.length !== expectedGroupFunctions.length ||
      actualGroupFunctions.some((name, index) => name !== expectedGroupFunctions[index])
    ) {
      failures.push(
        `${file}:${chapterFunctionName}: chapter must call every verse-group function exactly once in order`,
      );
    }
  }
  if (extractCalls(source, "crate::chapter!(").length !== chapters.length) {
    failures.push(`${file}: every chapter! call must live inside exactly one chapter function`);
  }

  if (chapters.length !== expectedChapters.length) {
    failures.push(`${file}: expected ${expectedChapters.length} chapters, found ${chapters.length}`);
  }
  actualChapterCount += chapters.length;

  const allSegmentCalls = extractCalls(source, "story.segment(");
  const segments = [];
  for (const groupFunction of groupFunctions) {
    const groupMatch = groupFunction.name.match(groupPattern);
    const groupChapter = Number(groupMatch[1]);
    const groupSource = groupFunction.body;
    const segmentCalls = extractCalls(groupSource, "story.segment(");
    if (segmentCalls.length !== 1) {
      failures.push(`${file}:${groupFunction.name}: expected exactly one story.segment(...) call`);
      continue;
    }

    const segmentCall = segmentCalls[0];
    const segment = parseSegment(segmentCall, file);
    if (!segment) continue;
    if (segment.chapter !== groupChapter) {
      failures.push(
        `${file}:${groupFunction.name}: segment chapter ${segment.chapter} does not match its function`,
      );
    }
    if (segment.variant !== bookVariant(book.name)) {
      failures.push(`${file}:${groupFunction.name}: segment uses the wrong book variant`);
    }

    const segmentMethods = extractMethodCalls(groupSource, "segment");
    const commitCalls = segmentMethods.filter((call) => call.operation === "commit");
    if (commitCalls.length !== 1) {
      failures.push(`${file}:${groupFunction.name}: every group needs exactly one segment.commit()?`);
      continue;
    }
    const commitSource = groupSource.slice(commitCalls[0].start, commitCalls[0].end);
    const commitTail = groupSource.slice(commitCalls[0].end);
    if (!/segment\s*\.\s*commit\s*\(\s*\)/.test(commitSource) || !/^\s*\?\s*;/.test(commitTail)) {
      failures.push(`${file}:${groupFunction.name}: segment.commit()? must propagate its result`);
    }
    const operationCalls = segmentMethods.filter((call) => call.operation !== "commit");
    if (
      operationCalls.length > 0 &&
      commitCalls[0].start < operationCalls[operationCalls.length - 1].start
    ) {
      failures.push(`${file}:${groupFunction.name}: segment.commit()? must follow all verse operations`);
    }
    const operations = operationCalls.map((call) => parseOperation(call, file)).filter(Boolean);
    const expectedSize = segment.end - segment.start + 1;
    const chapterVerseCount = book.verses[segment.chapter - 1];
    if (
      segment.start < 1 ||
      segment.end < segment.start ||
      !Number.isInteger(chapterVerseCount) ||
      segment.end > chapterVerseCount
    ) {
      failures.push(`${file}: invalid segment range ${segment.chapter}:${segment.start}-${segment.end}`);
    }
    if (expectedSize > 5) {
      failures.push(`${file}: segment ${segment.chapter}:${segment.start}-${segment.end} exceeds five verses`);
    }
    const expectedTail = chapterVerseCount - segment.end === 0;
    if (expectedSize < 3 && !expectedTail) {
      failures.push(`${file}: non-final segment ${segment.chapter}:${segment.start}-${segment.end} is too small`);
    }
    if (operations.length !== expectedSize) {
      failures.push(
        `${file}: segment ${segment.chapter}:${segment.start}-${segment.end} expected ${expectedSize} operations, found ${operations.length}`,
      );
    }
    const operationVerses = new Set();
    for (let offset = 0; offset < operations.length; offset += 1) {
      const operation = operations[offset];
      const expectedVerse = segment.start + offset;
      if (operation.verse !== expectedVerse) {
        failures.push(`${file}: segment ${segment.chapter} expected verse ${expectedVerse}, found ${operation.verse}`);
        break;
      }
      const verseKey = `${segment.chapter}:${operation.verse}`;
      if (operationVerses.has(verseKey)) {
        failures.push(`${file}: duplicate verse operation ${verseKey} inside ${groupFunction.name}`);
      }
      operationVerses.add(verseKey);
      if (!operation.subject || !operation.objects.length || !operation.tokens.length) {
        failures.push(`${file}: segment ${segment.chapter}:${operation.verse} lacks direct operands`);
      }
      if (sourceVerseTokens) {
        const verseTokens = sourceVerseTokens.get(
          sourceKey(book.name, segment.chapter, operation.verse),
        );
        if (!verseTokens) {
          failures.push(
            `${file}: missing source wording for ${book.name} ${segment.chapter}:${operation.verse}`,
          );
        } else {
          for (const operand of [
            operation.subject,
            ...operation.objects,
            ...operation.tokens,
          ]) {
            const normalized = operand.toLowerCase().replace(/[’]/g, "'");
            if (!verseTokens.has(normalized)) {
              failures.push(
                `${file}: ${book.name} ${segment.chapter}:${operation.verse} operand ${operand} is not source-derived`,
              );
            }
          }
        }
      }
      actualVerses.push({
        book: book.name,
        variant: segment.variant,
        chapter: segment.chapter,
        number: operation.verse,
      });
    }
    if (sourceVerseTokens) {
      const rangeTokens = new Set();
      for (let verse = segment.start; verse <= segment.end; verse += 1) {
        const verseTokens = sourceVerseTokens.get(sourceKey(book.name, segment.chapter, verse));
        if (verseTokens) {
          for (const token of verseTokens) rangeTokens.add(token);
        }
      }
      for (const token of segment.context) {
        const normalized = token.toLowerCase().replace(/[’]/g, "'");
        if (!rangeTokens.has(normalized)) {
          failures.push(
            `${file}: ${book.name} ${segment.chapter}:${segment.start}-${segment.end} context token ${token} is not source-derived`,
          );
        }
      }
    }
    segments.push({
      ...segment,
      group: groupFunction.name,
      operationVerses: [...operationVerses],
    });
    actualSegmentCount += 1;
  }
  if (allSegmentCalls.length !== segments.length) {
    failures.push(`${file}: every story.segment(...) call must belong to one verse-group function`);
  }

  const expected = expectedVerses(book);
  if (segments.length === 0 && expected.length > 0) failures.push(`${file}: no verse segments`);
  const parsedVerses = segments.flatMap((segment) =>
    segment.operationVerses.map((coordinate) => {
      const [chapter, number] = coordinate.split(":").map(Number);
      return {
        book: book.name,
        variant: segment.variant,
        chapter,
        number,
      };
    }),
  );
  if (parsedVerses.length !== expected.length) {
    failures.push(`${file}: expected ${expected.length} verses, found ${parsedVerses.length}`);
  }
  for (let index = 0; index < expected.length; index += 1) {
    const actual = parsedVerses[index];
    const wanted = expected[index];
    if (!actual) break;
    if (
      actual.variant !== wanted.variant ||
      actual.chapter !== wanted.chapter ||
      actual.number !== wanted.number
    ) {
      failures.push(`${file}: verse ${index + 1} expected ${wanted.chapter}:${wanted.number}`);
      break;
    }
  }
  actualVerseCount += parsedVerses.length;
}

const expectedAllVerses = canon.books.flatMap(expectedVerses);
if (canon.chapter_count !== 1_189) failures.push(`canon.json: expected 1189 chapters, found ${canon.chapter_count}`);
if (canon.verse_count !== 31_102) failures.push(`canon.json: expected 31102 verses, found ${canon.verse_count}`);
if (actualChapterCount !== 1_189) failures.push(`total: expected 1189 chapters, found ${actualChapterCount}`);
if (actualVerseCount !== 31_102) failures.push(`total: expected 31102 verses, found ${actualVerseCount}`);
if (actualVerses.length !== expectedAllVerses.length) {
  failures.push(`total: expected ${expectedAllVerses.length} verse operations, found ${actualVerses.length}`);
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
    failures.push(`total: expected ${wanted.book} ${wanted.chapter}:${wanted.number} at operation ${index + 1}`);
    break;
  }
}

if (failures.length) {
  console.error(failures.join("\n"));
  process.exit(1);
}

console.log(
  `PASS: ${canon.books.length} books, ${actualChapterCount} chapters, ${actualVerseCount} verses, and ${actualSegmentCount} committed Rust segments are represented by direct Rust operations.`,
);
