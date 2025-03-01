# Zeph

## Directory Structure

- `/` (root): Smart contracts (Hardhat project)
    - `/contracts` keeps the smart contract
- `app-fe/`: Next.js frontend
- `rofl/`: ROFL application

## Setup

1. Install dependencies:

```sh
pnpm install
cd app-fe
pnpm install
```

## Running

1. Start the frontend development server:

```sh
cd app-fe
pnpm dev
```

2. For local ROFL development:

Check the ([`rofl/README.md`](./rofl/README.md)) for instructions on running the ROFL service.

3. Deploy contracts:


```sh
export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

npx hardhat deploy --network sapphire-localnet
```
