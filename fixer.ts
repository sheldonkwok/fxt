export function fixMsg(content: string): string[] {
  const matches = content.matchAll(/https:\/\/(twitter|x).com\/\w+\/status\/\d+/g);
  const fixes: string[] = [];

  for (const match of matches) {
    fixes.push(match[0].replace(/(twitter|x).com/, "fxtwitter.com"));
  }

  return fixes;
}
