import sys
import threading

import grpc

import chat_pb2
import chat_pb2_grpc


def receive_messages(stub):
    """Nasłuchuje wiadomości z serwera w osobnym wątku."""
    try:
        responses = stub.ChatStream(chat_pb2.Empty())
        for note in responses:
            print(f"\r[{note.name}] {note.message}\n> ", end="")
    except grpc.RpcError:
        print("\nRozłączono z serwerem.")


def run():
    channel = grpc.insecure_channel("malinka.tailb2454f.ts.net:50051")
    stub = chat_pb2_grpc.ChatServerStub(channel)

    name = input("Podaj swój nick: ")

    threading.Thread(target=receive_messages, args=(stub,), daemon=True).start()

    print("Możesz zacząć pisać wiadomości. Wpisz 'quit', aby wyjść.")

    while True:
        try:
            msg = input("> ")
            if msg.lower() == "quit":
                break
            if msg.strip() == "":
                continue

            sys.stdout.write("\033[F")  # move cursor up
            sys.stdout.write("\033[K")  # clear line

            note = chat_pb2.Note(name=name, message=msg)
            stub.SendNote(note)
        except KeyboardInterrupt:
            break


if __name__ == "__main__":
    run()
