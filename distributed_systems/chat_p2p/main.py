import random
import threading
import time

USE_CAUSAL_DELIVERY = True
NUM_NODES = 3


class ChatNode:
    def __init__(self, node_id, nodes_ref):
        self.node_id = node_id
        self.nodes = nodes_ref
        self.vector_clock = [0] * NUM_NODES
        self.delivery_buffer = []
        self.chat_history = []
        self.lock = threading.Lock()

    def broadcast(self, text, forced_delays=None):
        """Wysyła wiadomość do wszystkich węzłów w sieci."""
        with self.lock:
            # 1. Inkrementuj własny licznik przed wysłaniem
            self.vector_clock[self.node_id] += 1
            vc_copy = list(self.vector_clock)

            msg = {"sender": self.node_id, "text": text, "vc": vc_copy}

            # 2. Wyświetl na swoim własnym ekranie od razu
            self.chat_history.append(f"[V:{vc_copy}] Node {self.node_id}: {text}")

        print(
            f"\n[WYSYŁANIE] Node {self.node_id} wysyła: '{text}' z wektorem {vc_copy}"
        )

        # 3. Multicast - wyślij do wszystkich innych w tle (symulacja sieci)
        forced_delays = forced_delays or {}
        for target_id, target_node in self.nodes.items():
            if target_id != self.node_id:
                # Użyj wymuszonego opóźnienia (dla testów) lub losowego
                delay = forced_delays.get(target_id, random.uniform(0.1, 1.5))
                threading.Thread(
                    target=self._network_send, args=(target_node, msg, delay)
                ).start()

    def _network_send(self, target_node, msg, delay):
        """Symuluje sieć poprzez uśpienie wątku na określony czas."""
        time.sleep(delay)
        target_node.receive_message(msg)

    def receive_message(self, msg):
        """Odbiera wiadomość z sieci."""
        with self.lock:
            if not USE_CAUSAL_DELIVERY:
                # ZEPSUTY CZAT: Brak buforowania, wyświetl od razu
                self._deliver_to_screen(msg)
                # Aktualizacja zegara wg wzoru z zadania
                for i in range(NUM_NODES):
                    self.vector_clock[i] = max(self.vector_clock[i], msg["vc"][i])
            else:
                # NAPRAWIONY CZAT: Zegary wektorowe i buforowanie
                self.delivery_buffer.append(msg)
                print(
                    f"   -> [BUFOR] Node {self.node_id} otrzymał z sieci i zbuforował '{msg['text']}' {msg['vc']}"
                )
                self._check_buffer()

    def _check_buffer(self):
        """Sprawdza, czy można przenieść wiadomości z bufora na ekran."""
        delivered_any = True
        # Bez pętli, dostarczenie jednej wiadomości może odblokować kolejną czekającą w buforze!
        while delivered_any:
            delivered_any = False
            for msg in list(self.delivery_buffer):
                if self._can_deliver(msg):
                    self.delivery_buffer.remove(msg)
                    self._deliver_to_screen(msg)

                    # Przy dostarczaniu na ekran aktualizujemy nasz lokalny zegar
                    for i in range(NUM_NODES):
                        self.vector_clock[i] = max(self.vector_clock[i], msg["vc"][i])

                    delivered_any = True

    def _can_deliver(self, msg):
        """Sprawdza 2 warunki Causal Delivery na podstawie Zegara Wektorowego."""
        sender = msg["sender"]
        msg_vc = msg["vc"]

        # Warunek 1: To musi być kolejna w sekwencji wiadomość od tego nadawcy
        if msg_vc[sender] != self.vector_clock[sender] + 1:
            return False

        # Warunek 2: Czy widzieliśmy już wszystko, co widział nadawca?
        for k in range(NUM_NODES):
            if k != sender and msg_vc[k] > self.vector_clock[k]:
                return False

        return True

    def _deliver_to_screen(self, msg):
        self.chat_history.append(f"[V:{msg['vc']}] Node {msg['sender']}: {msg['text']}")
        print(f"   => [EKRAN] Node {self.node_id} wyświetla: '{msg['text']}'")


# GŁÓWNA SYMULACJA
if __name__ == "__main__":
    print(f"START SYMULACJI (USE_CAUSAL_DELIVERY = {USE_CAUSAL_DELIVERY})\n")

    # 1. Inicjalizacja węzłów
    nodes = {}
    for i in range(NUM_NODES):
        nodes[i] = ChatNode(i, nodes)

    # 2. Scenariusz łamiący przyczynowość
    # Node 0 zadaje pytanie.
    # Do Node 1 pakiet leci szybko (0.1s), ale do Node 2 utknął w sieci na 3 sekundy!
    nodes[0].broadcast("Czy Ziemia jest okrągła?", forced_delays={1: 0.1, 2: 3.0})

    # Dajemy chwilę Node 1 na odbiór pytania...
    time.sleep(0.5)

    # Node 1 odpowiada. Odpowiedź leci do wszystkich błyskawicznie (0.1s).
    nodes[1].broadcast("Tak, to prawda!", forced_delays={0: 0.1, 2: 0.1})

    # Czekamy na dostarczenie opóźnionego pakietu (tego 3-sekundowego)
    time.sleep(3.5)

    # 3. Wyświetlenie końcowej historii czatu na każdym węźle
    print("\n" + "=" * 50)
    print("HISTORIA CZATU NA POSZCZEGÓLNYCH WĘZŁACH:")
    print("=" * 50)

    for i in range(NUM_NODES):
        print(f"\n--- Ekran Węzła {i} ---")
        for line in nodes[i].chat_history:
            print(line)
