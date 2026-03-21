import sys
import threading

import grpc

# Import wygenerowanych plików
import chat_pb2
import chat_pb2_grpc


def receive_messages(stub):
    """Nasłuchuje wiadomości z serwera w osobnym wątku."""
    try:
        # Rozpocznij nasłuchiwanie strumienia
        responses = stub.ChatStream(chat_pb2.Empty())
        for note in responses:
            # Nadpisz obecną linię w konsoli, aby ładnie wyświetlić nową wiadomość
            print(f"\r[{note.name}] {note.message}\n> ", end="")
    except grpc.RpcError:
        print("\nRozłączono z serwerem.")


def run():
    # Połącz z serwerem gRPC
    channel = grpc.insecure_channel("malinka.tailb2454f.ts.net:50051")
    stub = chat_pb2_grpc.ChatServerStub(channel)

    name = input("Podaj swój nick: ")

    # Uruchom wątek odbierający wiadomości w tle (daemon=True pozwala na zamknięcie programu)
    threading.Thread(target=receive_messages, args=(stub,), daemon=True).start()

    print("Możesz zacząć pisać wiadomości. Wpisz 'quit', aby wyjść.")

    while True:
        try:
            msg = input("> ")
            if msg.lower() == "quit":
                break
            if msg.strip() == "":
                continue

            # Clear the previous input line
            sys.stdout.write("\033[F")  # move cursor up
            sys.stdout.write("\033[K")  # clear line

            # Stwórz obiekt Note i wyślij na serwer
            note = chat_pb2.Note(name=name, message=msg)
            stub.SendNote(note)
        except KeyboardInterrupt:
            break


if __name__ == "__main__":
    run()
