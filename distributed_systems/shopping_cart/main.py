import random
import threading
import time
import uuid

USE_TOMBSTONES = True


# ===== SERVER NODE =====
class ServerNode:
    def __init__(self, node_id, all_nodes_ref):
        self.node_id = node_id
        self.all_nodes = all_nodes_ref
        self.cart = {}
        self.processed_requests = set()
        self.is_alive = True
        self.lock = threading.Lock()

        threading.Thread(target=self.anti_entropy_loop, daemon=True).start()

    def receive_request(self, req_id, action, item_id, quantity=0):
        if not self.is_alive:
            return None  # Simulate dead node timeout

        # Simulate Unreliable Network (20% chance to drop packet)
        if random.random() < 0.20:
            return None

        with self.lock:
            # 1. Idempotency Check
            if req_id in self.processed_requests:
                print(
                    f"       -> [IDEMPOTENCY] Node {self.node_id} recognized duplicate ReqID {req_id[:6]}. Ignoring but sending ACK."
                )
                return {"status": "ACK", "msg": "Duplicate"}

            self.processed_requests.add(req_id)
            current_timestamp = time.time()

            # 2. Process Request (LWW)
            if action == "ADD":
                if (
                    item_id not in self.cart
                    or self.cart[item_id]["timestamp"] < current_timestamp
                ):
                    self.cart[item_id] = {
                        "quantity": quantity,
                        "timestamp": current_timestamp,
                        "tombstone": False,
                    }

            elif action == "REMOVE":
                if not USE_TOMBSTONES:
                    # PART 1: Physical Deletion (Creates Zombies)
                    if item_id in self.cart:
                        del self.cart[item_id]
                else:
                    # PART 2: Tombstone Deletion (Fixes Zombies)
                    self.cart[item_id] = {
                        "quantity": 0,
                        "timestamp": current_timestamp,
                        "tombstone": True,
                    }

        return {"status": "ACK"}

    def anti_entropy_loop(self):
        while True:
            time.sleep(5)
            if not self.is_alive:
                continue

            neighbors = [
                n
                for n in self.all_nodes.values()
                if n.node_id != self.node_id and n.is_alive
            ]
            if not neighbors:
                continue

            target_node = random.choice(neighbors)

            with target_node.lock:
                remote_cart = dict(target_node.cart)

            with self.lock:
                for item_id, remote_data in remote_cart.items():
                    local_data = self.cart.get(item_id)

                    if not local_data:
                        self.cart[item_id] = remote_data
                    else:
                        if remote_data["timestamp"] > local_data["timestamp"]:
                            self.cart[item_id] = remote_data


# ===== CLIENT LOGIC =====
def send_client_request(nodes, action, item_id, quantity=0, specific_req_id=None):
    req_id = specific_req_id if specific_req_id else str(uuid.uuid4())
    print(
        f"\n[CLIENT] Trying to {action} {quantity if quantity > 0 else ''} '{item_id}' (ReqID: {req_id[:6]})"
    )

    while True:
        target_node = random.choice(list(nodes.values()))
        print(f"   -> Sending to Node {target_node.node_id}...")

        response = target_node.receive_request(req_id, action, item_id, quantity)

        if response and response.get("status") == "ACK":
            print(f"   -> [SUCCESS] Received ACK from Node {target_node.node_id}.")
            break
        else:
            print("   -> [TIMEOUT] Packet dropped or node dead. Retrying in 500ms...")
            time.sleep(0.5)


# ===== HELPER: PRINT STATE =====
def print_cluster_state(nodes, step_name):
    print(f"\n--- STATE AFTER: {step_name} ---")
    for nid, node in nodes.items():
        status = "ALIVE" if node.is_alive else "DEAD "
        with node.lock:
            visible_cart = {
                k: v["quantity"] for k, v in node.cart.items() if not v["tombstone"]
            }
            raw_state = {
                k: {
                    "qty": v["quantity"],
                    "ts": round(v["timestamp"], 2),
                    "tomb": v["tombstone"],
                }
                for k, v in node.cart.items()
            }

        print(f"Node {nid} [{status}] Visible: {visible_cart}\n\t| Raw: {raw_state}")
    print("--------------------------------------------------\n")


# ===== HELPER: COMPARE NODES =====
def compare_nodes(nodes):
    result = True
    for nid1, node1 in nodes.items():
        for nid2, node2 in nodes.items():
            if nid1 != nid2:
                if node1.cart == node2.cart:
                    print(f"Node {nid1} and Node {nid2} have the same cart state")
                else:
                    print(f"Node {nid1} and Node {nid2} have different cart states")
                    result = False
    return result


# ===== MAIN ORCHESTRATION =====
if __name__ == "__main__":
    print(f"STARTING SIMULATION (USE_TOMBSTONES = {USE_TOMBSTONES})")

    nodes = {1: ServerNode(1, {}), 2: ServerNode(2, {}), 3: ServerNode(3, {})}
    for n in nodes.values():
        n.all_nodes = nodes

    # STEP 1: DEMONSTRATE IDEMPOTENCY (Double Add)
    print("\n[SYSTEM] --- DEMONSTRATING IDEMPOTENCY ---")
    duplicate_req_id = str(uuid.uuid4())

    # First attempt
    send_client_request(nodes, "ADD", "Orange", 1, specific_req_id=duplicate_req_id)
    # Second attempt (simulating a delayed/lost ACK where client retries)
    print("\n[SYSTEM] Simulating lost ACK. Client retries the EXACT SAME request...")
    send_client_request(nodes, "ADD", "Orange", 1, specific_req_id=duplicate_req_id)

    # STEP 2: NORMAL ADDS
    send_client_request(nodes, "ADD", "Apple", 5)
    send_client_request(nodes, "ADD", "Banana", 2)

    print("\n[SYSTEM] Waiting 6 seconds for initial Anti-Entropy sync...")
    time.sleep(6)
    print_cluster_state(nodes, "Initial Setup")

    # STEP 3: THE ZOMBIE SCENARIO
    print("\n[SYSTEM] KILLING NODES 2 AND 3...")
    nodes[2].is_alive = False
    nodes[3].is_alive = False

    print("\n[SYSTEM] Client removing 'Apple'")
    remove_req_id = str(uuid.uuid4())
    send_client_request(nodes, "REMOVE", "Apple", 1, specific_req_id=remove_req_id)

    print("\n[SYSTEM] Waiting 6 seconds for alive nodes to sync the deletion...")
    time.sleep(6)
    print_cluster_state(nodes, "Node 3 Dead, Apple Removed from Alive Nodes")

    print("\n[SYSTEM] REVIVING NODE 2 AND 3...")
    nodes[2].is_alive = True
    nodes[3].is_alive = True

    print("\n[SYSTEM] Waiting 22 seconds for multiple rounds of Anti-Entropy...")
    time.sleep(22)

    print_cluster_state(nodes, "Final Resolution")

    if compare_nodes(nodes):
        if USE_TOMBSTONES:
            print(
                "SUCCESS: Tombstones prevented the Apple from coming back from the dead!"
            )
        else:
            print(
                "ZOMBIE ALERT: Notice how the Apple came back to life because Node 3 re-introduced it!"
            )
    else:
        print("❌ FAILURE: Anti-Entropy failed to resolve the zombie record")
