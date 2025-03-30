# Solana Wallet CLI

![Solana](https://img.shields.io/badge/Solana-Blockchain-green)
![Rust](https://img.shields.io/badge/Rust-Programming_Language-orange)

A simple **Rust-based CLI** tool to interact with the Solana blockchain. This tool allows users to **check their SOL balance** and **request an airdrop** on the Solana **Devnet**.

## ⚡ Features
- ✅ **Check Solana Wallet Balance**
- ✅ **Request Airdrop (Devnet only)**
- ✅ **Handles invalid inputs gracefully**

---

## 🛠 Installation

### **1️⃣ Clone the Repository**
```bash
git clone https://github.com/cdpandora/TELEREQUESTER.git
cd TELEREQUESTER
```

### **2️⃣ Install Dependencies**
Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
cargo build
```

### **3️⃣ Run the Program**
```bash
cargo run
```

---

## 🚀 Usage Guide

### **Checking Solana Wallet Balance**
1. Run the program: `cargo run`
2. Enter your **Solana wallet address**
3. Select **Option 1** to check the balance

### **Requesting an Airdrop (Devnet Only)**
1. Run the program: `cargo run`
2. Enter your **Solana wallet address**
3. Select **Option 2**
4. Enter the amount of **SOL** you wish to request
5. Get a **transaction link** to verify the airdrop

---

## 📜 Code Breakdown

### **🔹 Fetch Wallet Balance**
The function `fetch_balance()` fetches the balance in **SOL** from the Solana RPC.

### **🔹 Request Airdrop**
The function `request_airdrop()` allows users to receive free **SOL** on Devnet for testing purposes.

---

## 🔗 Explorer & RPC Details
- **Solana Devnet RPC:** `https://api.devnet.solana.com`
- **Solana Explorer:** [Solana Explorer](https://explorer.solana.com/?cluster=devnet)

---

## 🛠 Dependencies
This project uses:
- [`solana-client`](https://docs.rs/solana-client/latest/solana_client/)
- [`solana-sdk`](https://docs.rs/solana-sdk/latest/solana_sdk/)
- [`tokio`](https://tokio.rs/) *(Optional for async operations)*


---

### 🌟 **Like the project? Give it a star! ⭐**