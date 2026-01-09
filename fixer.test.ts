import { expect, test } from "bun:test";
import { fixMsg } from "./fixer";

const TEST_URL = "https://twitter.com/ProZD/status/1714895861208293467";
const FIXED_URL = "https://fxtwitter.com/ProZD/status/1714895861208293467";
const FIXED_SPOILER = `||${FIXED_URL}||`;

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

// TikTok tests
const TIKTOK_TEST_URL = "https://www.tiktok.com/@username/video/1234567890";
const TIKTOK_FIXED_URL = "https://www.tnktok.com/@username/video/1234567890";
const TIKTOK_FIXED_SPOILER = `||${TIKTOK_FIXED_URL}||`;

const TIKTOK_SHORT_URL = "https://vm.tiktok.com/ZMjKL1234/";
const TIKTOK_SHORT_FIXED = "https://vm.tnktok.com/ZMjKL1234/";

test("fix tiktok url", () => {
  const fixed = fixMsg(TIKTOK_TEST_URL);
  expect(fixed).toEqual([TIKTOK_FIXED_URL]);
});

test("fix tiktok short url", () => {
  const fixed = fixMsg(TIKTOK_SHORT_URL);
  expect(fixed).toEqual([TIKTOK_SHORT_FIXED]);
});

test("fix tiktok url in text", () => {
  const content = `check this out ${TIKTOK_TEST_URL} amazing video`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([TIKTOK_FIXED_URL]);
});

test("fix multiple tiktok urls", () => {
  const content = `${TIKTOK_TEST_URL} and ${TIKTOK_SHORT_URL}`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([TIKTOK_FIXED_URL, TIKTOK_SHORT_FIXED]);
});

test("handle tiktok spoilers", () => {
  const content = `secret video ||${TIKTOK_TEST_URL}||`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([TIKTOK_FIXED_SPOILER]);
});

test("fix both twitter and tiktok urls", () => {
  const content = `${TEST_URL} and ${TIKTOK_TEST_URL}`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_URL, TIKTOK_FIXED_URL]);
});

// Instagram tests
const INSTAGRAM_TEST_URL = "https://www.instagram.com/p/ABC123/";
const INSTAGRAM_FIXED_URL = "https://www.ddinstagram.com/p/ABC123/";
const INSTAGRAM_FIXED_SPOILER = `||${INSTAGRAM_FIXED_URL}||`;

const INSTAGRAM_REEL_URL = "https://www.instagram.com/reel/XYZ789/";
const INSTAGRAM_REEL_FIXED = "https://www.ddinstagram.com/reel/XYZ789/";

test("fix instagram url", () => {
  const fixed = fixMsg(INSTAGRAM_TEST_URL);
  expect(fixed).toEqual([INSTAGRAM_FIXED_URL]);
});

test("fix instagram reel url", () => {
  const fixed = fixMsg(INSTAGRAM_REEL_URL);
  expect(fixed).toEqual([INSTAGRAM_REEL_FIXED]);
});

test("fix instagram url in text", () => {
  const content = `check this out ${INSTAGRAM_TEST_URL} cool post`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([INSTAGRAM_FIXED_URL]);
});

test("fix multiple instagram urls", () => {
  const content = `${INSTAGRAM_TEST_URL} and ${INSTAGRAM_REEL_URL}`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([INSTAGRAM_FIXED_URL, INSTAGRAM_REEL_FIXED]);
});

test("handle instagram spoilers", () => {
  const content = `secret post ||${INSTAGRAM_TEST_URL}||`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([INSTAGRAM_FIXED_SPOILER]);
});

test("fix twitter, tiktok, and instagram urls", () => {
  const content = `${TEST_URL} and ${TIKTOK_TEST_URL} and ${INSTAGRAM_TEST_URL}`;
  const fixed = fixMsg(content);
  expect(fixed).toEqual([FIXED_URL, TIKTOK_FIXED_URL, INSTAGRAM_FIXED_URL]);
});
