import { Client, Events, GatewayIntentBits } from "discord.js";

function parseUrl(urlStr: string): URL | undefined {
  try {
    return new URL(urlStr);
  } catch {
    return;
  }
}

function fixTwitter(url: URL): string | undefined {
  const { hostname } = url;
  if (hostname !== "twitter.com" && hostname !== "x.com") return;

  url.hostname = "fxtwitter.com";
  return url.href;
}

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMembers,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
  ],
});

client.on(Events.MessageCreate, async (msg) => {
  const { content } = msg;

  const parsed = parseUrl(content);
  if (!parsed) return;

  const newUrl = fixTwitter(parsed);
  if (!newUrl) return;

  await msg.reply(newUrl);
});

client.once(Events.ClientReady, (c) => console.log(`Ready! Logged in as ${c.user.tag}`));
client.login(process.env.DISCORD_TOKEN);
