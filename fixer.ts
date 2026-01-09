export function fixMsg(content: string): string[] {
  const fixes: string[] = [];
  const spoilers = getSpoilers(content);

  // Fix Twitter/X URLs
  const twitterMatches = Array.from(content.matchAll(/https:\/\/(twitter|x)\.com\/\w+\/status\/\d+/g));
  for (const match of twitterMatches) {
    const index = match.index;
    if (index === undefined) continue;

    let fix = match[0].replace(/(twitter|x)\.com/, getTwitterFixer());

    for (const [start, end] of spoilers) {
      if (index > start && index < end) fix = `||${fix}||`;
    }

    fixes.push(fix);
  }

  // Fix TikTok URLs
  const tiktokMatches = Array.from(content.matchAll(/https:\/\/(?:www\.|vm\.|vt\.)?tiktok\.com\/[^\s|]*/g));
  for (const match of tiktokMatches) {
    const index = match.index;
    if (index === undefined) continue;

    let fix = match[0].replace(/tiktok\.com/, getTikTokFixer());

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

const DEFAULT_TWITTER_FIX = "fxtwitter.com";
const SECRET_FIXERS = (process.env.SECRET_FIXERS || "").split(",").filter(s => s.length > 0);

const DEFAULT_TIKTOK_FIX = "tnktok.com";

function getTwitterFixer(): string {
  if (SECRET_FIXERS.length === 0) return DEFAULT_TWITTER_FIX;

  const randomIndex = Math.floor(Math.random() * SECRET_FIXERS.length);
  return SECRET_FIXERS[randomIndex];
}

function getTikTokFixer(): string {
  return DEFAULT_TIKTOK_FIX;
}
