FROM oven/bun:latest

WORKDIR /opt/app

COPY package.json bun.lockb ./
COPY packages/bot/package.json ./packages/bot/
RUN bun install

COPY tsconfig.json ./
COPY packages/bot/ ./packages/bot/

CMD ["bun", "packages/bot/bot.ts"]
