# init the file structure

## create new directory

```bash
m̀kdir rust_ib_async && cd $_

```

## create a new rust project

```bash
# point stands for  current directory
cargo init ,
```

## add cartes to project

```bash
cargo add ib_async
```

## output Cargo.toml

```txt
# cat Cargo.toml 
[package]
name = "rust_ib_async"
version = "0.1.0"
edition = "2024"

[dependencies]
ib_async = "0.1.1"
```

> [!NOTE] [The Note content inside MarkDown document  FROM HERE](https://stackoverflow.com/questions/25654845/how-can-i-create-a-text-box-for-a-note-in-markdown)
<!-- -->
> [!NOTE] [Write two separate block quotes in sequence using markdown](https://stackoverflow.com/questions/12979577/how-can-i-write-two-separate-blockquotes-in-sequence-using-markdown)
> <!-- -->
> [!IMPORTANT] The TWS Client should run local

## Test Hello World World - create by cargo init

```bash
cargo run
```

## github icon

//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github

## Test Link Logo

[![Foo](http://www.google.com.au/images/nav_logo7.png)](http://google.com.au/)

## GITHUB Marker

> [!NOTE]
> Useful information that users should know, even when skimming content.
<!-- -->
> [!TIP]
> Helpful advice for doing things better or more easily.
<!-- -->
> [!IMPORTANT]
> Key information users need to know to achieve their goal.
<!-- -->
> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.
<!-- -->
> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.
