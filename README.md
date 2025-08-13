`This is task-4 for ackee blockchain security (school-of-solana)`

## Overview

This is a decentralized Twitter-like program that allows users to create tweets, add reactions (likes/dislikes), comment on tweets, and remove their own reactions and comments.


## How It Works

1. **Creating Tweets**: Users create tweets with a topic (up to 32 bytes) and content (up to 500 bytes). The topic serves as part of the PDA seeds, allowing users to create multiple tweets.

2. **Adding Reactions**: Users can like or dislike tweets. Each reaction creates a new PDA account with seeds designed to prevent multiple reactions per user per tweet.

3. **Adding Comments**: Users can comment on tweets with content up to 500 bytes. The comment content hash is used in the PDA seeds for unique identification.

4. **Removing Reactions/Comments**: Users can remove their own reactions and comments, which closes the accounts and returns rent.

## Getting Started

### Prerequisites
For this task you need:
- [Rust installed](https://www.rust-lang.org/tools/install)
    - Make sure to use stable version:
    ```bash
    rustup default stable
    ```
- [Solana installed](https://docs.solana.com/cli/install-solana-cli-tools)
    - Use v2.2.12
    - After you have Solana-CLI installed, you can switch between versions using:
    ```bash
    agave-install init 2.2.12
    ```

- [Anchor installed](https://www.anchor-lang.com/docs/installation)
    - Use v0.31.1
    - After you have Anchor installed, you can switch between versions using:
    ```bash
    avm use 0.31.1
    ```

### Development Commands

**Install dependencies:**
```bash
yarn install
```

**Build the project:**
```bash
anchor build
```

**Test your implementation:**
```bash
anchor test
```

[Anchor Framework Documentation](https://www.anchor-lang.com/)

