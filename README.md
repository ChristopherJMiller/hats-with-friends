
## What?

Learning about networking and game design via a very silly idea :)

Basis of the project is the [bevy demo provided by the naia team](https://github.com/naia-lib/naia/tree/main/demos/bevy)

## Setup

```
# One time setup
yay -S binaryen just
just install-global-deps

# In a seperate terminal, start the game server
cargo run -p server

# In a seperate terminal, build and serve the game client
just serve
```
