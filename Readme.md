# 🎄 My Solutions to Advent of Code! 🌟

This repository contains my personal solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges, written in Rust, and soon adding in C.

## 🚀 Getting Started

### Prerequisites

What things you need to install the software:

- Rust (See [official installation guide](https://www.rust-lang.org/tools/install))
- Shell (Bash or any POSIX-compliant shell)

### 📁 Structure

The project is organized as follows:

```
AdventOfCode/
├── 2023/
│   ├── day00/
│   │   ├── solutions
│   ├── day01/
│   │   ├── solutions
│   ├── ... and so on
├── add.sh - Script to add a new day's directory and initialize solutions
├── run.sh - Script to compile and run solutions
```

## 🛠️ Usage

### Adding a New Day

To add a new day's directory and initialize solutions:

```bash
chmod +x add.sh

./add.sh <year> <day>
```

### Running Solutions

To compile and run a solution for a specific day:

```bash
chmod +x run.sh

./run.sh <year> <day>
```