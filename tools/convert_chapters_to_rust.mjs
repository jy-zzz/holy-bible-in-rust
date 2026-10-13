import fs from "node:fs";
import path from "node:path";

const root = path.dirname(new URL(import.meta.url).pathname).replace(/\/tools$/, "");
const canon = JSON.parse(fs.readFileSync(path.join(root, "canon.json"), "utf8"));

function rustIdentifier(name) {
  return name
    .toLowerCase()
    .replace(/&/g, " and ")
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "")
    .replace(/^([0-9])/, "_$1");
}

function rustString(value) {
  return JSON.stringify(value);
}

function parseChapterCalls(source, file) {
  const marker = "crate::chapter!(";
  const chapters = [];
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

    const body = source.slice(start + marker.length, close);
    const header = body.match(/^\s*story,\s*"((?:\\.|[^"\\])*)",\s*(\d+),([\s\S]*)$/);
    if (!header) {
      throw new Error(`${file}: malformed chapter! call`);
    }

    const book = JSON.parse(`"${header[1]}"`);
    const prose = [...header[3].matchAll(/"(?:\\.|[^"\\])*"/g)].map((match) =>
      JSON.parse(match[0]),
    );
    chapters.push({ book, number: Number(header[2]), prose });
    cursor = close + 1;
  }

  return chapters;
}

function parseChapters(source, file) {
  if (source.includes("crate::chapter!(")) {
    const chapters = parseChapterCalls(source, file);
    for (const chapter of chapters) {
      if (chapter.prose.length < 2) {
        throw new Error(`${file}: ${chapter.book} ${chapter.number} has fewer than two prose lines`);
      }
    }
    return chapters;
  }

  const lines = source.split(/\r?\n/);
  const chapters = [];
  let current = null;

  for (const line of lines) {
    const heading = line.match(/^\/\/ CHAPTER (.+) (\d+)$/);
    if (heading) {
      if (current) chapters.push(current);
      current = {
        book: heading[1],
        number: Number(heading[2]),
        prose: [],
      };
      continue;
    }

    if (!current) continue;
    const prose = line.match(/^\/\/ ?(.*)$/);
    if (prose && prose[1].trim()) {
      current.prose.push(prose[1].trim());
    }
  }

  if (current) chapters.push(current);

  for (const chapter of chapters) {
    if (chapter.prose.length < 2) {
      throw new Error(`${file}: ${chapter.book} ${chapter.number} has fewer than two prose lines`);
    }
  }
  return chapters;
}

function renderBookFunction(chapters) {
  const lines = [
    `fn ${rustIdentifier(chapters[0].book)}(story: &mut Story) -> Result<(), StoryError> {`,
  ];

  for (const chapter of chapters) {
    const prose = chapter.prose.map(rustString).join(", ");
    lines.push(
      `    crate::chapter!(story, ${rustString(chapter.book)}, ${chapter.number}, ${prose});`,
    );
  }

  lines.push("    Ok(())", "}", "");
  return lines;
}

const chaptersByFile = new Map();
for (const book of canon.books) {
  const file = book.file;
  if (!chaptersByFile.has(file)) chaptersByFile.set(file, []);
  chaptersByFile.get(file).push(book);
}

for (const [file, books] of chaptersByFile) {
  const fullPath = path.join(root, file);
  const source = fs.readFileSync(fullPath, "utf8");
  const chapters = parseChapters(source, file);
  const expected = books.flatMap((book) =>
    Array.from({ length: book.chapters }, (_, index) => `${book.name}:${index + 1}`),
  );
  const actual = chapters.map((chapter) => `${chapter.book}:${chapter.number}`);

  if (expected.length !== actual.length || expected.some((value, index) => value !== actual[index])) {
    throw new Error(`${file}: source chapters do not match canon.json in canonical order`);
  }

  const functions = chapters.reduce((groups, chapter) => {
    const key = chapter.book;
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key).push(chapter);
    return groups;
  }, new Map());

  const output = [
    `use crate::{Story, StoryError};`,
    "",
    "pub fn apply(story: &mut Story) -> Result<(), StoryError> {",
    ...books.map((book) => `    ${rustIdentifier(book.name)}(story)?;`),
    "    Ok(())",
    "}",
    "",
    ...[...functions.values()].flatMap(renderBookFunction),
  ];

  fs.writeFileSync(fullPath, `${output.join("\n")}\n`);
  console.log(`converted ${file}: ${chapters.length} chapters`);
}
