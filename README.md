# 🏗️ Fee Distribution Smart Contract

## 📌 Overview
This repository contains the **Fee Distribution Smart Contract** for the **CryptoSeeds MicroVerse** on the **Solana Blockchain**. This contract ensures the proper distribution of fees collected from network transactions into three main categories:

- **33% to Holders** → Users who hold RYP tokens manually **harvest** their rewards from the MicroVerse Dapp.
- **34% to Stakers** → Stakers **harvest** their rewards based on their staking tier within the **farm dashboard**.
- **33% to Treasury** → Funds are stored in the **CryptoSeeds Treasury Wallet** for ecosystem development.

## 📂 Repository Structure
```
Fee-Distribution/
│── programs/
│   ├── fee_distribution/
│   │   ├── src/
│   │   │   ├── lib.rs      # Main Rust Smart Contract Code
│   │   ├── Cargo.toml      # Rust dependencies
│   │   ├── Anchor.toml     # Anchor configuration
│── tests/
│   ├── fee_distribution.ts # JavaScript/TypeScript test scripts
│── migrations/
│── anchor.toml             # Anchor Project Config
│── Cargo.toml              # Rust Project Dependencies
│── README.md               # Documentation
```

## 🚀 Deployment Instructions
### **1️⃣ Clone the Repository**
```bash
git clone https://github.com/CryptoSeeds-RYP/Fee-Distribution.git
cd Fee-Distribution
```

### **2️⃣ Install Dependencies**
Ensure you have the required tools installed:
```bash
solana --version  # Check Solana CLI installation
anchor --version  # Check Anchor Framework installation
```
If not installed:
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### **3️⃣ Set Up Solana Devnet**
```bash
solana config set --url https://api.devnet.solana.com
solana airdrop 2 # Fund your wallet with Devnet SOL
```

### **4️⃣ Build and Deploy**
```bash
cd programs/fee_distribution
anchor build
anchor deploy
```
Once deployed, you’ll get a **Program ID**.

### **5️⃣ Update `Anchor.toml`**
```toml
[programs.devnet]
fee_distribution = "Your_New_Program_ID"
```

### **6️⃣ Test the Smart Contract**
```bash
anchor test
```

## 🔐 Treasury Wallet Address
All **treasury allocations (33%)** are stored in:
📌 **8goFS74mVUbq7cESLQSRyeUD5pTR5FTaBdRXHYmwwqYf**

## ❓ Need Help?
If you run into issues, feel free to open an **issue** on GitHub or contact the **CryptoSeeds-RYP team**.

🚀 **Built for a Sustainable & Decentralized Future!** 🌱
