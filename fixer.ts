export function fixMsg(content: string): string[] {
  const matches = Array.from(content.matchAll(/https:\/\/(twitter|x).com\/\w+\/status\/\d+/g));
  const fixes: string[] = [];

  if (matches.length === 0) return fixes;

  const spoilers = getSpoilers(content);

  for (const match of matches) {
    const index = match.index;
    if (index === undefined) continue;

    let fix = match[0].replace(/(twitter|x).com/, getFixer());

    for (const [start, end] of spoilers) {
      if (index > start && index < end) fix = `||${fix}||`;
    }

    fixes.push(fix);
  }

  return fixes;
}

const SPOILER = "||";

function getSpoilers(content: string): number[][] {
  const indexes: number[] = [];
  let start = 0;

  while (start < content.length) {
    const index = content.indexOf(SPOILER, start);
    if (index === -1) break;

    indexes.push(index);
    start = index + 2;
  }

  const pairs: number[][] = [];
  if (indexes.length < 2) return pairs;

  for (let i = 0; i < indexes.length; i += 2) {
    const start = indexes[i];
    const end = indexes[i + 1];

    pairs.push([start, end]);
  }

  return pairs;
}

const DEFAULT_FIX = "fxtwitter.com";
const SECRET_FIXERS = (process.env.SECRET_FIXERS || "").split(",");

function getFixer(): string {
  if (SECRET_FIXERS.length === 0) return DEFAULT_FIX;

  const randomIndex = Math.floor(Math.random() * SECRET_FIXERS.length);
  return SECRET_FIXERS[randomIndex];
}
