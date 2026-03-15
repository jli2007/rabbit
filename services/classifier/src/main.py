import asyncio
from messaging.client import rabbit

async def handle(body: str):
    print(f"classifying: {body}")
    await rabbit.connect()
    await rabbit.publish("issue.investigate", body)

async def main():
    print("classifier waiting for messages...")
    await rabbit.connect()
    await rabbit.consume("classifier_queue", "issue.classify", handle)

if __name__ == "__main__":
    asyncio.run(main())