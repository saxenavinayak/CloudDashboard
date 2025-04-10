import discord
import asyncio

TOKEN = "nice_try"

intents = discord.Intents.default()
intents.messages = True
intents.guilds = True

client = discord.Client(intents=intents)

# @client.event
# async def on_ready():
#     print(f'Logged in as {client.user}')

# @client.event
# async def on_message(message):
#     if message.channel.name == "general":
#         print(f"{message.author}: {message.content}")  # Send this to Kafka or other pipelines

client.run(TOKEN)
