import { Client, Events, Message, GatewayIntentBits } from "discord.js";

import { fixMsg } from "./fixer.ts";

const { Guilds, GuildMembers, GuildMessages, MessageContent } = GatewayIntentBits;
const client = new Client({ intents: [Guilds, GuildMembers, GuildMessages, MessageContent] });

// Could parallelize??
async function fixUrls(msg: Message, urls: string[]): Promise<void> {
  for (const fixed of urls) {
    await msg.reply(fixed);
  }
}

client.on(Events.MessageCreate, async (msg) => {
  const fixedUrls = fixMsg(msg.content);
  if (fixedUrls.length === 0) return;

  // This doesn't really handle half broken embed messages
  if (msg.embeds.length > 0) return;

  await Promise.all([msg.suppressEmbeds(true), fixUrls(msg, fixedUrls)]);

  // Ensure embed supress
  await Bun.sleep(5000);
  await msg.suppressEmbeds(true);
});

client.once(Events.ClientReady, (c) => console.log(`Ready! Logged in as ${c.user.tag}`));
client.login(process.env.DISCORD_TOKEN);
