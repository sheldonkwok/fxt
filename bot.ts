import { Client, Events, GatewayIntentBits } from "discord.js";

import { fixMsg } from "./fixer";

const { Guilds, GuildMembers, GuildMessages, MessageContent } = GatewayIntentBits;
const client = new Client({ intents: [Guilds, GuildMembers, GuildMessages, MessageContent] });

client.on(Events.MessageCreate, async (msg) => {
  const fixedUrls = fixMsg(msg.content);

  // Could parallelize??
  for (const fixed of fixedUrls) {
    await msg.reply(fixed);
  }
});

client.once(Events.ClientReady, (c) => console.log(`Ready! Logged in as ${c.user.tag}`));
client.login(process.env.DISCORD_TOKEN);
