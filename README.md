# Yarchi
### **Y**et **A**nother **R**ust **C**HIP-8 **I**nterpreter!

Yarchi is a simple yet functional CHIP-8 interpreter (emulator).

## Features
- Runs CHIP-8 programs and games (obviously!)
- Adjustable screen size
- Adjustable emulation speed
- Adjustable fore- and background color
- Possibility to run in CHIP-48 mode (required for some games to function properly)
- Debug mode

## Prerequisites
The interpreter depends on SDL2 library in order to handle video, audio and inputs. It must be therefore installed on your system.

On GNU/Linux operating systems this can be as simple as installing (example from Ubuntu 20.04):
```
apt install libsdl2-2.0-0
```

or on a MacOS (with `brew`):
```
brew install sdl2
```
For more information please see https://formulae.brew.sh/formula/sdl2.

## Running the interpreter
Simply run from your command line, while specifying the ROM path and passing arguments / flags.

For example, to run a program in debug mode with large screen:

```
./yarchi -d --screen-size large path/to/your/program.rom
```

## Building the interpreter
Yarchi depends only on a handful of dependencies and can be built very simply with the standard Rust toolchain by running from within the root directory:

```
cargo build
```

## Controls
Controls use the 'typical' mapping which is the following:

*(left: mapped on keyboard, right: original controls)*
| | | | | | | | | |
|-|-|-|-|-|-|-|-|-|
|1|2|3|4| |1|2|3|C|
|Q|W|E|R| |4|5|6|D|
|A|S|D|F| |7|8|9|E|
|Z|X|C|V| |A|B|C|D|

*the mapping is independent of your keyboard layout (i.e. bottom left key, whether `Z` or `Y`, is always `A`)

## Debug Mode
The interpreter also contains a debug mode, which can be 'activated' by passing `-d` or `--debug`. This mode makes it possible to inspect CHIP-8's state (program counter value, register values etc.), to pause / resume emulation and to execute emulation cycles one by one (when paused).

The described actions are available upon pressing:
- `P` - prints current state of CHIP-8
- `End` - pause/resume emulation
- `PgDown` - executes next cycle (4 instructions; possible only if emulation is paused)

## Contributions and collaboration
Something's not working right? Do you want to add a feature to the interpreter? Are you building one yourself and have questions?

Pull requests as well as opening an issue is more than welcome!