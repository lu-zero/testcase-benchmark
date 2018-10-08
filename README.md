# testcase for [cargo#6156](https://github.com/rust-lang/cargo/issues/6156)

Minimal testcase

the fake bencher supports:
```
--help
```
```
--special-option
```

Testing it:

```
cargo bench --help
```
```
cargo bench --special-option
```

