# Buddy-Up 🎉🥳  [![Continuous Integration](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml) [![Continuous Deployment](https://github.com/ckoehler/buddy-up/actions/workflows/cd.yaml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/cd.yaml) [![Release](https://github.com/ckoehler/buddy-up/actions/workflows/release.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/release.yml)

Buddy up a changing group of people into unique pairs over time.

## Installation

- Head to the [Releases](https://github.com/ckoehler/buddy-up/releases/latest) and download the binary for your platform.
- Install with Cargo: `cargo install --locked buddy-up`

## How to Use

`buddy --help` has good info. 

You could run it like this:

`buddy pair --input people.csv --output-dir meeting`

Depending on the input, you'll get a table showing the pairings, something like this. 

```
+---------+-------+
| Alice   | Frank |
|---------+-------|
| Bob     | David |
|---------+-------|
| Peter   | John  |
|---------+-------|
| Charlie | Karl  |
|---------+-------|
| Bjorn   | Simon |
+---------+-------+
```

The history of pairs is saved in the `output-dir`

```
❯ ls meeting/
20250213_205644.json
```

Feel free to manually edit the history files, they are just JSON.

## Input

CSV file of the format:

```csv
1,Karl
2,John
3,Simon
4,Frank
5,Peter
6,Alice
7,Bob
8,Charlie
9,David
10,Bjorn
```

Ids need to be unique and positive, and there need to be an even number of people (because pairs, right?).

## Output

The pairs are saved as history into the directory given, as JSON.

## How it Works

### The Problem

Pairing up people is hard. There are algorithms and tools out there to do it, but I hadn't found one that takes into account a 
changing group and (implied) doesn't require all the pairings to be calculated up front.

### The Solution

`Buddy-Up` generates pairs from the given input, then saves the pairings in the history. The next time `Buddy-Up` is run, it reads the history 
and takes it into account in calculating new pairings, which should be unique, at least until everyone has been paired up once already.

Because the problem space is potentially huge, `Buddy-Up` uses a genetic algorithm to come up with the best pairings. I think it works pretty well, but isn't 
perfect. Feel free to open an issue if you have ideas for improving it.

### The Math

TODO
