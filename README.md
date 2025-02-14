# Buddy-Up  [![Continuous Integration](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml) [![Continuous Deployment](https://github.com/ckoehler/buddy-up/actions/workflows/cd.yaml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/cd.yaml) [![Release](https://github.com/ckoehler/buddy-up/actions/workflows/release.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/release.yml)

Pair up a group of people without (much) repetition.

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
