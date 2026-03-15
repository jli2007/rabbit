from aio_pika import connect_robust, Message, ExchangeType
from aio_pika.abc import AbstractChannel, AbstractExchange, AbstractRobustConnection
import os
from dotenv import load_dotenv

load_dotenv(".env.local")

EXCHANGE_NAME = os.getenv("RABBITMQ_EXCHANGE", "agent.swarm")
RABBITMQ_URL = os.getenv("RABBITMQ_URL", "amqp://guest:guest@localhost/")

class RabbitMQ:
    _connection: AbstractRobustConnection
    _channel: AbstractChannel
    _exchange: AbstractExchange

    async def connect(self) -> None:
        self._connection = await connect_robust(RABBITMQ_URL)
        self._channel = await self._connection.channel()
        self._exchange = await self._channel.declare_exchange(
            EXCHANGE_NAME, ExchangeType.TOPIC, durable=True
        )

    async def publish(self, routing_key: str, body: str) -> None:
        await self._exchange.publish(
            Message(body.encode(), content_type="application/json"),
            routing_key=routing_key,
        )

    async def consume(self, queue_name: str, routing_key: str, handler) -> None:
        queue = await self._channel.declare_queue(queue_name, durable=True)
        await queue.bind(self._exchange, routing_key=routing_key)

        async with queue.iterator() as messages:
            async for message in messages:
                async with message.process():
                    await handler(message.body.decode())


rabbit = RabbitMQ()