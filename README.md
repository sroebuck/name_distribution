# Name distribution tool

This simple Rust tool takes lists of the frequency of occurance of surnames in births, deaths and marriages in Scotland over years from 1975 to 2015 and uses this information to generate equally sized buckets of surname letter ranges.

So, for example, if you wanted to divide the Scottish population into 5 groups you could ask the tool to do that and it would generate five buckets:

| Letter range | Percentage of population |
|:------------:|:------------------------:|
|    A - C     |           19%            |
|    D - H     |           21%            |
|   I - MCH    |           19%            |
|   MCI - RO   |           21%            |
|    RP - Z    |           19%            |

The choice of bucket boundary is made to try to keep things simple whilst keeping the population roughly evenly spread across the buckets.  By default the tool assumes that you don't want to deviate more than 2% from an exact allocation to each bucket.  In the example of 5 buckets you need to increase the deviation a fair bit before you can reduce the number of letters in a range to two characters across the board:

| Letter range | Percentage of population |
|:------------:|:------------------------:|
|    A - C     |           19%            |
|    D - H     |           21%            |
|    I - MC    |           24%            |
|    MD - R    |           17%            |
|    S - Z     |           19%            |

## Installation

Currently the tool isn't distributed as a binary.  If you have rust installed you can install with:

    cargo install --git=https://github.com/sroebuck/name_distribution.git

This should happily build on every platform that rust supports including Windows, Mac and Linux.

## Use

The tool is designed to be used from the command line.  Just enter:

    name_distribution -n 10

This will output a distribution of surname letter ranges for 10 buckets of equally distributed people across the Scottish population.  The output is in CSV format so:

    name_distribution -n 10 > results.csv

will pipe the result into a file called `results.csv`.

Use the `-h` option to display the help message and any other options.

---

The original names lists came from:

* [Common Scottish Surnames in Birth, Marriage and Death registers](https://www.nrscotland.gov.uk/statistics-and-data/statistics/statistics-by-theme/vital-events/names/most-common-surnames)

## Release notes

### Version 0.1.0

First release
