FROM oven/bun:latest

COPY package.json bun.lockb./
RUN bun install

COPY tsconfig.json bot.ts ./

RUN ["bun", "bot.ts"]