
zfs 2.1/2.2 has a silent file corruption bug, issue [15526](https://github.com/openzfs/zfs/issues/15526) This tool is used to find any zero-byte blocks (4kb * n) dubious file

```
Usage: zfs-issue-15526-check-file [OPTIONS]

Options:
  -p, --path <PATH>            [default: ./**/*.*]
  -t, --threshold <THRESHOLD>  [default: 4]
```

`--path` option uses the path argument of [glob](https://crates.io/crates/glob)

`--threshold` option configures the error threshold and prints the file when multiple 4kb zero bytes appear continuously.