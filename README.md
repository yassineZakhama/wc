# wc

A Rust implementation of the Unix wc (word count) command-line tool. 

This program counts lines, words, characters, and bytes in text files or standard input.

### Supported Options

| Option | Description                |
|--------|----------------------------|
|   -c   | print the byte counts      |
|   -l   | print the newline counts   |
|   -w   | print the word counts      |
|   -m   | print the character counts |


## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yassineZakhama/wc.git
    cd wc
    ```

2. Build the program

    Cargo is required to build this repository.

    The Following command will execute the build:

    ```sh
    cargo build --release
    ```

## Usage

```sh
./wc.exe wc [OPTION] [FILE]
```

- [OPTION]:

    -l: Count lines.

    -w: Count words.

    -m: Count characters.

    -c: Count bytes.

- [FILE]: Optional. Path to a file. If omitted, the program reads from standard input.

### Examples

<b>Count Lines, Words, and Bytes (Default Behavior)</b>

```sh
./wc.exe wc test.txt
```

Output:

```sh
7145 58164 342190
```

<b>Count Lines Only</b>

```sh
./wc.exe wc -l test.txt
```

Output:

```sh
7145
```

<b>Using standard input (Windows)</b>

```sh
type test.txt | ./wc.exe wc -l
```

Output:

```sh
7145
```

<b>Using standard input (Unix)</b>

```sh
cat test.txt | ./wc wc -l
```

Output:

```sh
7145
```

## Acknowledgments

Inspired by the Unix wc tool.

Implementation of <a href="https://codingchallenges.fyi/challenges/challenge-wc/">John Cricket's Coding Challenge</a>.