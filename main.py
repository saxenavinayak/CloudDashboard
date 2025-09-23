import discord
import os

# Create an Intents object so we can listen to messages
intents = discord.Intents.default()
intents.message_content = True  # Required to read message text
token = os.environ["DISCORD_API_TOKEN"]
# Create client instance
client = discord.Client(intents=intents)

@client.event
async def on_ready():
    print(f'Bot logged in as {client.user}')

@client.event
async def on_message(message):
    # Avoid responding to yourself
    if message.author == client.user:
        return
    
    # Print every message to console
    print(f"{message.author}: {message.content}")
    
    # Example: simple reply
    if message.content.lower() == "!ping":
        await message.channel.send("pong!")

# Replace with your bot token
client.run(token)
