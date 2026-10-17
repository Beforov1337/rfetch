# rfetch

A minimal, blazing fast system fetch tool written in Rust with **zero external dependencies**.

```text
      .--.
     |o_o |     os        artix
     |:_/ |     kernel    7.2.9
    //   \ \    uptime    1h 33m
   (|     | )   packages  617
  /'\_   _/`\
  \___)=(___/
```

## Features

- **Blazing fast**: reads directly from `/proc` and `/etc/os-release`, prints in `< 1ms`.
- **Zero dependencies**: standard library only, single static binary.
- **Customizable ASCII art**: comes with a cute multi-colored Tux. Put your own art into `ascii.txt` and it will automatically align and render!
- **Fallback support**: includes the ASCII art inside the binary via `include_str!`, so you can move the executable anywhere.

## Installation

### From Source

Make sure you have Rust and Cargo installed:

```bash
git clone https://github.com/Beforov1337/rfetch.git
cd rfetch
cargo build --release
```

Move the compiled binary to your `$PATH`:

```bash
install -Dm755 target/release/rfetch ~/.local/bin/rfetch
```

## Custom Art

Drop any ASCII art into `ascii.txt` next to the binary or in your working directory:

```bash
echo "(>'-')>" > ascii.txt
rfetch
```

## License

MIT
