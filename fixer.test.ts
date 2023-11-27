import { expect, test } from "bun:test";
import { fixMsg } from "./fixer";

const TEST_URL = "https://twitter.com/ProZD/status/1714895861208293467";
const FIXED_URL = "https://fxtwitter.com/ProZD/status/1714895861208293467";
const FIXED_SPOILER = `||${FIXED_URL}||`;
const TEST_CH = "This message is considered harmful";
const FIXED_CH = "\"Considered harmful\" phrase considered harmful";

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

test("fix nothing", () => {
  const content = `this is text that should be unchanged`;
  const fixed = fixMsg(content);
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

test("handle spoilers", () => {
  const content = `shhhh ||${TEST_URL}||`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_SPOILER]);
});

test("handle multiple spoilers", () => {
  const content = `shhhh ||${TEST_URL}||
  shh||${TEST_URL}|| `;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_SPOILER, FIXED_SPOILER]);
});

test("handle broken spoilers", () => {
  const content = `shhhh ||secret|| ${TEST_URL}||`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_URL]);
});

test("respond to nerds", () => {
  const content = TEST_CH;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_CH]);
});
