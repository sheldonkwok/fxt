FROM oven/bun:latest

WORKDIR /opt/app

COPY package.json bun.lockb ./
RUN bun install

COPY tsconfig.json *.ts ./

CMD ["bun", "bot.ts"]
