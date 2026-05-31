import queue
import random
import sys
import threading
import time

NUM_BANKS = 4
INITIAL_BALANCE = 10000
INITIAL_BANK_BALANCE = INITIAL_BALANCE // NUM_BANKS

# Globalny słownik dla Koordynatora do zbierania wyników Audytu
audit_results = {}
audit_lock = threading.Lock()


# ===== SYMULACJA SIECI =====
def wire_thread(src, dst, q_in, q_out):
    """
    Symuluje kabel sieciowy między dwoma węzłami.
    Zdejmuje wiadomość, czeka (opóźnienie sieci), a potem dostarcza.
    Gwarantuje to FIFO dla pary (src, dst), jednocześnie przetrzymując środki "w locie".
    """
    while True:
        msg = q_in.get()
        time.sleep(random.uniform(0.1, 0.5))  # Pieniądze / Markery w drodze!
        q_out.put((src, msg))


# ===== WĘZEŁ BANKU =====
class BankNode:
    def __init__(self, node_id, all_ids, out_queues, inbox):
        self.node_id = node_id
        self.all_ids = all_ids
        self.out_queues = out_queues  # dst_id -> kolejka wejściowa kabla
        self.inbox = inbox  # Skrzynka odbiorcza banku

        self.balance = INITIAL_BANK_BALANCE
        self.lock = threading.Lock()

        # --- Zmienne dla Chandy'ego-Lamporta ---
        self.local_snapshots = {}  # snap_id -> saldo w momencie zrzutu
        self.channel_states = {}  # snap_id -> {sender_id: suma_kwot_w_drodze}
        self.recording = {}  # snap_id -> zbiór kanałów, które obecnie nagrywamy

        # Start wątków
        threading.Thread(target=self.sender_loop, daemon=True).start()
        threading.Thread(target=self.receiver_loop, daemon=True).start()

    def sender_loop(self):
        """Ciągle wysyła losowe kwoty do innych banków."""
        while True:
            time.sleep(random.uniform(0.1, 0.3))
            with self.lock:
                if self.balance > 0:
                    amount = random.randint(1, min(100, self.balance))
                    dst = random.choice([n for n in self.all_ids if n != self.node_id])

                    self.balance -= amount
                    self.out_queues[dst].put(("TRANSFER", amount))

    def receiver_loop(self):
        """Odbiera transfery i markery z sieci (inbox)."""
        while True:
            src, (msg_type, payload) = self.inbox.get()

            with self.lock:
                if msg_type == "MARKER":
                    snap_id = payload
                    self._handle_marker(src, snap_id)

                elif msg_type == "TRANSFER":
                    amount = payload
                    self.balance += amount

                    # sprawdź czy nagrać te pieniądze
                    for snap_id, recording_channels in self.recording.items():
                        if src in recording_channels:
                            self.channel_states[snap_id][src] += amount

    def _handle_marker(self, src, snap_id):
        """Reguły Chandy'ego-Lamporta przy odbiorze markera."""
        if snap_id not in self.local_snapshots:
            # 1. To pierwszy Marker dla tego Audytu, jaki widzimy.
            self._record_local_state(snap_id)

            # Kanał 'src' oznaczamy jako pusty (nie będziemy na nim nagrywać)
            self.recording[snap_id].remove(src)

            # 2. Wysyłamy własne Markery dalej
            self._broadcast_marker(snap_id)
        else:
            # 1. Widzieliśmy już Markery w tym Audycie. Zamykamy nagrywanie na tym kanale.
            if src in self.recording[snap_id]:
                self.recording[snap_id].remove(src)

        # Jeśli nie ma już żadnych kanałów do nagrywania, kończymy nasz udział w Audycie.
        if len(self.recording[snap_id]) == 0:
            self._finish_snapshot(snap_id)

    def _record_local_state(self, snap_id):
        """Zapisuje lokalne saldo i inicjuje nagrywanie kanałów wejściowych."""
        self.local_snapshots[snap_id] = self.balance
        # Zaczynamy nagrywać WSZYSTKIE kanały wejściowe
        self.recording[snap_id] = set(n for n in self.all_ids if n != self.node_id)
        self.channel_states[snap_id] = {n: 0 for n in self.all_ids if n != self.node_id}

    def _broadcast_marker(self, snap_id):
        """Rozsyła pustą wiadomość MARKER na wszystkie wychodzące krawędzie."""
        for dst in self.all_ids:
            if dst != self.node_id:
                self.out_queues[dst].put(("MARKER", snap_id))

    def _finish_snapshot(self, snap_id):
        """Wysyła zebrany stan do Głównego Koordynatora (konsoli)."""
        local_bal = self.local_snapshots[snap_id]
        chan_states = self.channel_states[snap_id]
        report_to_coordinator(self.node_id, snap_id, local_bal, chan_states)

    def initiate_audit(self, snap_id):
        """Zewnętrzny wyzwalacz Audytu (np. przez administratora)."""
        with self.lock:
            self._record_local_state(snap_id)
            self._broadcast_marker(snap_id)


# ===== KOORDYNATOR (Agregacja i Wydruk Wyników) =====
def report_to_coordinator(node_id, snap_id, balance, channels):
    with audit_lock:
        if snap_id not in audit_results:
            audit_results[snap_id] = {}

        audit_results[snap_id][node_id] = {"balance": balance, "channels": channels}

        # odebrano raporty od wszystkich węzłów -> Podsumowujemy!
        if len(audit_results[snap_id]) == NUM_BANKS:
            print_audit_results(snap_id)


def print_audit_results(snap_id):
    results = audit_results[snap_id]
    total_local = 0
    total_in_transit = 0

    print("\n" + "=" * 50)
    print(f"ZAKOŃCZONO AUDYT CHANDY'EGO-LAMPORTA (ID: {snap_id})")
    print("=" * 50)

    for nid, data in results.items():
        print(
            f"[Bank {nid}] Saldo lok: {data['balance']} PLN | Kasa w drodze do banku: {sum(data['channels'].values())} PLN"
        )
        total_local += data["balance"]
        total_in_transit += sum(data["channels"].values())

        for sender, amt in data["channels"].items():
            if amt > 0:
                print(f"   -> W locie: {amt} PLN (Z banku {sender} -> do {nid})")

    grand_total = total_local + total_in_transit
    print("-" * 50)
    print(f"Suma Saldo Lokalne : {total_local} PLN")
    print(f"Suma w Kanałach    : {total_in_transit} PLN")
    print(f"SUMA GLOBALNA      : {grand_total} PLN")
    print("=" * 50 + "\n")

    if grand_total == INITIAL_BALANCE:
        print("SUKCES: Prawa fizyki zachowane, stan księgowy się zgadza!\n")
    else:
        print("BŁĄD: Zgubiono pieniądze!\n")


# ===== MAIN - ORKIESTRACJA SIECI =====
if __name__ == "__main__":
    print(f" Uruchamianie systemu bankowego. Całkowity budżet: {INITIAL_BALANCE} PLN")
    bank_ids = list(range(NUM_BANKS))

    # 1. Konfiguracja "Kabli" (Network Wires) i Skrzynek Odbiorczych
    inboxes = {i: queue.Queue() for i in bank_ids}
    network_queues = {
        i: {j: queue.Queue() for j in bank_ids if i != j} for i in bank_ids
    }

    for i in bank_ids:
        for j in bank_ids:
            if i != j:
                threading.Thread(
                    target=wire_thread,
                    args=(i, j, network_queues[i][j], inboxes[j]),
                    daemon=True,
                ).start()

    # 2. Inicjalizacja Banków
    banks = {}
    for i in bank_ids:
        banks[i] = BankNode(i, bank_ids, network_queues[i], inboxes[i])

    time.sleep(1)

    # 3. Pętla Audytora
    audit_counter = 1
    while True:
        input(
            ">>> Wciśnij ENTER, aby przeprowadzić AUDYT (Naiwny + Chandy-Lamport) <<<\n"
        )

        # --- A. NAIWNY AUDYT ---
        naive_sum = 0
        print("--- NAIWNY AUDYT ---")
        for i, bank in banks.items():
            with bank.lock:
                naive_sum += bank.balance
        print(
            f"❌ Naiwny odczyt sald wykazał: {naive_sum} PLN (zgubiono {INITIAL_BALANCE - naive_sum} PLN w locie!)"
        )
        print(
            "Rozpoczynamy skanowanie siecią Markera (Chandy-Lamport). Proszę czekać...\n"
        )

        # --- B. CHANDY-LAMPORT AUDYT ---
        # dowolony bank jako inicjatora
        banks[random.choice(bank_ids)].initiate_audit(f"SNAP-{audit_counter}")
        audit_counter += 1
        time.sleep(2)
