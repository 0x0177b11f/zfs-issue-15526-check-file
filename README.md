
zfs 2.1/2.2 has a silent file corruption bug, issue [15526](https://github.com/openzfs/zfs/issues/15526) This tool is used to find any zero-byte blocks (4kb * n) dubious file

```
Usage: zfs-issue-15526-check-file [OPTIONS]

Options:
  -p, --path <PATH>            Scan path, glob format [default: ./**/*.*]
  -t, --threshold <THRESHOLD>  Reporting threshold [default: 16]
  -f                           Check file first 4 Mib
  -h, --help                   Print help
  -V, --version                Print version
```

`--path` option uses the path argument of [glob](https://crates.io/crates/glob)

Example:

* `/XXX/**/*.*` : Any file in the `/XXX` directory (recursively into all subdirectories)
* `/XXX/*.*` : Any file in the `/XXX` directory (no subdirectories)
* `/XXX/**/*.zip` : Any zip file in the `/XXX` directory (recursively into all subdirectories)
* `/XXX/*.zip` : Any zip file in the `/XXX` directory (no subdirectories)

`--threshold` print an alert when zero byte blocks are detected consecutively.

`-f` Quickly detect begin of the file (4 Mib range).
