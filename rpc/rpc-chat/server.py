import threading
from concurrent import futures

import grpc

# Import wygenerowanych plików
import chat_pb2
import chat_pb2_grpc


class ChatServerServicer(chat_pb2_grpc.ChatServerServicer):
    def __init__(self):
        self.notes = []
        # Condition pozwala na usypianie wątków klientów, dopóki nie pojawi się nowa wiadomość
        self.condition = threading.Condition()

    def ChatStream(self, request, context):
        """Ten strumień wysyła wiadomości do klienta."""
        last_index = 0

        while context.is_active():
            with self.condition:
                # Czekaj, aż pojawią się nowe wiadomości (lub do 1 sekundy, by sprawdzić is_active)
                while last_index >= len(self.notes) and context.is_active():
                    self.condition.wait(timeout=1.0)

                if not context.is_active():
                    break

                # Wyślij wszystkie nowe wiadomości
                for i in range(last_index, len(self.notes)):
                    yield self.notes[i]

                last_index = len(self.notes)

    def SendNote(self, request, context):
        """Ta funkcja odbiera wiadomość od klienta."""
        with self.condition:
            self.notes.append(request)
            # Powiadom wszystkie nasłuchujące strumienie (klientów) o nowej wiadomości
            self.condition.notify_all()

        print(f"Otrzymano od {request.name}: {request.message}")
        return chat_pb2.Empty()


def serve():
    # Inicjalizacja serwera gRPC
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    chat_pb2_grpc.add_ChatServerServicer_to_server(ChatServerServicer(), server)
    server.add_insecure_port("[::]:50051")
    server.start()
    print("Serwer czatu uruchomiony na porcie 50051...")
    server.wait_for_termination()


if __name__ == "__main__":
    serve()
