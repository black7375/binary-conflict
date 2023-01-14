# Binary Conflict

`binary-conflict` improves git's binary merge behavior. (It's for proof of concept)

Git only indicates that files have coflicted, and should be confirmed through `git checkout --ours -- <path>` or `git checkout --theirs -- <path>`.

![image](https://user-images.githubusercontent.com/25581533/212459170-82753a2a-b643-4389-aec5-777bd0fd5b45.png)

This project makes it easy to select by showing files `base branch` and `their branch`.

![image](https://user-images.githubusercontent.com/25581533/212459178-d0a516f2-8213-4256-b019-c3a340e78f83.png)

## Usage

After installing this, define merge behavior in `.git/config` file.

```
[merge "binary-merge"]
	name = Create base, ours, theirs file for conflict
	driver = ../target/release/binary-conflict %O %A %B %P
```

Then, You can apply it in `.gitattributes` file.

```
[attr]BINARY diff=binary merge=binary-merge -text

*.png BINARY
```

## Sample

```sh
cd example && ./example.sh
```

Grapic source
- [Git Logo](https://commons.wikimedia.org/wiki/File:Git-logo.svg)
- [Rust Logo](https://commons.wikimedia.org/wiki/File:Rust_programming_language_black_logo.svg)
- [Ferris](https://commons.wikimedia.org/wiki/File:Original_Ferris.svg)
