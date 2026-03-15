import asyncio
from aio_pika import connect_robust, ExchangeType

async def main():
    connection = await connect_robust("amqp://guest:guest@localhost/")
    channel = await connection.channel()
    
    print("waiting for messages...")
    channel.start_consuming()

if __name__ == "__main__":
    asyncio.run(main())
