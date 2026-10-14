import fs from "node:fs";
import path from "node:path";

const root = path.dirname(new URL(import.meta.url).pathname).replace(/\/tools$/, "");
const canon = JSON.parse(fs.readFileSync(path.join(root, "canon.json"), "utf8"));

function extractCalls(source, marker) {
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
    calls.push(source.slice(start + marker.length, close));
    cursor = close + 1;
  }
  return calls;
}

for (const book of canon.books) {
  const file = path.join(root, book.file);
  if (!fs.existsSync(file)) throw new Error(`missing ${book.file}`);
  const source = fs.readFileSync(file, "utf8");
  const chapters = extractCalls(source, "crate::chapter!(").map((call) => {
    const match = call.match(/^\s*story,\s*"((?:\\.|[^"\\])*)",\s*(\d+),/);
    if (!match) throw new Error(`${book.file}: malformed chapter! call`);
    return {
      book: JSON.parse(`"${match[1]}"`),
      number: Number(match[2]),
    };
  });
  const expected = Array.from({ length: book.chapters }, (_, index) => `${book.name}:${index + 1}`);
  const actual = chapters.map((chapter) => `${chapter.book}:${chapter.number}`);
  if (expected.length !== actual.length || expected.some((value, index) => value !== actual[index])) {
    throw new Error(`${book.file}: chapter order does not match canon.json`);
  }
  console.log(`validated ${book.name}: ${chapters.length} chapters`);
}

console.log("Chapter prose is already embedded beside verse segments; no destructive conversion was performed.");
