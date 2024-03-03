# Self-hosting

## Prerequisites

- [Rust 1.74 or later](https://www.rust-lang.org/) installed and available in your `PATH`
- [Git CLI](https://git-scm.com/) installed and available in your `PATH`
  - Git bundled with applications such as GitHub Desktop will result in the build process failing
- [A Discord application and bot created on the Developer Portal](https://discord.com/developers)

> [!WARNING]  
>
> **Support is provided for hosting on Linux only.** While it should work on Windows or macOS, this may change at any time. That said, you should be able to compile for Linux from either operating system.

## Getting the code

To get Avion's code, just clone the Avion repo:

```bash
git clone https://github.com/SkyfallWasTaken/avion-bot
```

## Setting environment variables

### Development

Open `.env.example` and fill in `DISCORD_TOKEN` and `DISCORD_TESTING_GUILD_ID`.

- **DISCORD_TOKEN**: Your Discord bot token
- **DISCORD_TESTING_GUILD_ID**: Your testing server's server ID

Finally, rename the file to `.env`.

> [!WARNING]  
> **Never** share your Discord bot token.

### Production

Just set the above variables another way.

## Compiling

Finally, run:

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

Avion's binary should be located at `/target/release/avion`.
