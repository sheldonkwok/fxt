import { expect, test } from "bun:test";
import { fixMsg } from "./fixer";

const TEST_URL = "https://twitter.com/ProZD/status/1714895861208293467";
const FIXED_URL = "https://fxtwitter.com/ProZD/status/1714895861208293467";

test("pass through normal text", () => {
  const fixed = fixMsg(`this
  is 
  some 
  stuff
  cone 
  would
  say`);

  expect(fixed).toEqual([]);
});

test("fix solo url", () => {
  const fixed = fixMsg(TEST_URL);
  expect(fixed).toEqual([FIXED_URL]);
});

test("fix url in text", () => {
  const content = `this is text
  ${TEST_URL}
  more content`;

  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_URL]);
});

test("fix multiple urls", () => {
  const content = `${TEST_URL}
  ${TEST_URL}`;

  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_URL, FIXED_URL]);
});
