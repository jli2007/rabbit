import pika

def main():
    connection = pika.BlockingConnection(
        pika.ConnectionParameters(host="localhost")
    )
    channel = connection.channel()
    print("waiting for messages...")
    channel.start_consuming()

if __name__ == "__main__":
    main()