# Buddy-Up 🎉🥳  [![Continuous Integration](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/ci.yml) [![Build Apps](https://github.com/ckoehler/buddy-up/actions/workflows/build-apps.yml/badge.svg)](https://github.com/ckoehler/buddy-up/actions/workflows/build-apps.yml)

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

IDs need to be unique and positive, and there need to be an even number of people (because pairs, right?).

## Output

In addition to the actual table of pairs shown above, the pairs are saved as history into the given output directory, as JSON.

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

For $n$ persons there are $n!$ ways to arrange them in a row so we can pair them up two-by-two, like `(1 2)(3 4)` etc. But we don't care about 
how the pairs are ordered relative to each other, i.e. `(1 2)(3 4)` is equivalent (for us) to `(3 4)(1 2)`. There are $(\frac{n}{2})!$ ways to arrange the pairs,
so we'll divide by that. We also don't care how the pairs are ordered within, i.e. `(1 2)` is equivalent (for us) to `(2 1)`. So we only keep half of all the pairs,
that is $2^\frac{n}{2}$. Putting it all together, there are $k$ ways to form pairs from $n$ people, such that:

$$k = \frac{n!}{(\frac{n}{2})! * 2^\frac{n}{2}}$$

That gets big really fast. For $n=10$, $k \approx 1000$; for $n=20$, $k \approx 650,000,000 $.

Then I needed a way to save pairs as history. Fortunately, there are a lot fewer unique pairs. Arranging pairs in a $n x n$ matrix, we can throw out the
diagonal (because no one will be paired up with just themselves), and half of the rest, because it's symmetric along the diagonal (because pairs don't
have an internal order), so that we end up with $p$ unique pairs, where $p$ is:

$$p = \frac{n^2 - n}{2}$$

That's manageable. For $n=10$ that's $p = 45$ pairs, for $n=20$ that's $p = 190$. Easy peasy.

Finally, we note that these pairs start repeating after $n-1$ meetings (proving this is left as an exercise for the reader, but it makes sense intuitively, too).
This is important to set expectations: at best, you will get repeating pairs after $n-1$ meetings. Ideally only 1 per meeting until everyone has met with
everyone else again (another $n-1$ meetings), but I don't know if the algorithm can provide that. Even so, it does a pretty good job!

### A word on the fitness function

The history saves how many times each pair has met. Fewer is better. The algorithm looks up the score for every pair in each potential pairing and adds them up. This sum
is the score for that pairing, and the algorithm will try and minimize that score to find the ideal pairing.
