# Buddy-Up  [![Continuous Integration](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml) [![Continuous Deployment](https://github.com/ckoehler/buddy-up/actions/workflows/cd.yaml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/cd.yaml) [![Release](https://github.com/ckoehler/buddy-up/actions/workflows/release.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/release.yml)

Pair up a group of people without (much) repetition.

## How to Use

`buddy --help` has good info. You could run it like this:

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
 20250213_205644.json
```

Feel free to manually edit the history files, they are just JSON.

## Input

CSV file of the format:

```csv
1, John
2, Alice
3, Bob
```

Ids need to be unique and positive.

## Output

The pairs are saved as history into the directory given.
