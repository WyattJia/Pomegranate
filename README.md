![](./image/pomegranate.jpg)

# Pomegranate()

Pomegranate is a tiny and fast log-structured merge-tree written in Rust.

**Pomegranate is still under development.**

## Why Rust?

Because Rust is a memory-safe, concurrency-friendly, high-performance programming language, and it also has rich and modern language features.
Based on its feature, I think it is very suitable for the development of storage application.

## Design

#### Memory Buffer

The memory buffer consists of *R* runs. In the slsm, one run corresponds to one skiplsit.
Only one *run* is activate at any time.

#### Skiplist

Skiplists are probabilistic data structures that provide for fast search within an orderedsequence of values. ey are composed of decreasingly sparse sorted runs of values thatare set in parallel, so that a search consists of searching a run until a key is found that isgreater than the desired one, then repeating the same process on the next-densest run,until the correct key is found.

#### Memory Buffer Index

Bloom lters nd important use in the sLSM when pairedone-to-one with runs in memory and on disk.

#### Disk Storage 

The disk-backed store is for more permanent storage and is only touched by merges fromthe memory buffer. ere areLdisk levels, each withDruns on each level. A run isan immutable sequence of sorted key-value pairs, and is captured within one memory-mapped le on disk.

## How to run

*Note: before you run, you should install Rust and Cargo first.*

```bash
git clone https://github.com/wellls/Pomegranate.git

cd /path/to/Pomegranate

cargo build
```

## Reference

* [Log-structured merge-tree](https://en.wikipedia.org/wiki/Log-structured_merge-tree)
* [The Skiplist-Based LSM Tree](https://arxiv.org/pdf/1809.03261.pdf)
* [Rust-lang](https://www.rust-lang.org/)

