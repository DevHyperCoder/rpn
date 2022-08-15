# rpn - Reverse Polish Notation calculator

`rpn` is a command line [reverse polish notation][1] calculator with some extra
features. It is somewhat similar to [forth][2], a stack based programming
language.

# Installation

> You need to have the rust toolchain installed.

```sh
git clone https://github.com/DevHyperCoder/rpn
cd rpn/
cargo run
```

# Usage

> It is recommended that you learn the basics of RPN.

> More documentation will be added in the form of a wiki soon.

## Basic operations

```
Algebraic: 1 + 2
RPN: 1 2 +

Algebraic: 2 * 3.14 * 5 * 5
RPN: 2 3.14 5 5 * * *

Algebraic: 2 * (1 + 2)
RPN: 2 1 2 + *
```

## Advanced operations

### 1. Variables:

Define a variable: `3.14 pi =`

Now use it anywhere: `pi 2 *`

### 2. Procedures:

Syntax: `<name> def <...> end`

Define a proc: `add1 def 1 + end`

Call it: `2 add1` (output: 3)

# Contributions

PRs and Issues are welcome.

# LICENSE

`rpn` is licensed under the GNU General Public License 3. Our copy of
GPL-3 can be found [here](./LICENSE)

[1]: <https://en.wikipedia.org/wiki/Reverse_Polish_notation>
[2]: <https://en.wikipedia.org/wiki/Forth_(programming_language)>
