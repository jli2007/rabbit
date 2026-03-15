import asyncio
from aio_pika import connect_robust, ExchangeType

async def main():
    connection = await connect_robust("amqp://guest:guest@localhost/")
    
    queue_name = "test_queue"
    exchange_name = "test_event"
    routing_key = "test_key"
    
    channel = await connection.channel()
    exchange = await channel.declare_exchange(exchange_name, ExchangeType.TOPIC, durable=True)
    queue = await channel.declare_queue(queue_name, durable=True)
    
    await queue.bind(exchange, routing_key=routing_key)
    
    print("waiting for messages...")
    
    async with queue.iterator() as messages:
          async for message in messages:
              async with message.process():
                  print(f"received: {message.body.decode()}")

if __name__ == "__main__":
    asyncio.run(main())
