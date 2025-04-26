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

> [!NOTE]
> [The Note content inside MarkDown document  FROM HERE](https://stackoverflow.com/questions/25654845/how-can-i-create-a-text-box-for-a-note-in-markdown)
<!-- -->
> [!NOTE]
> [Write two separate block quotes in sequence using markdown](https://stackoverflow.com/questions/12979577/how-can-i-write-two-separate-blockquotes-in-sequence-using-markdown)
> <!-- -->
> [!IMPORTANT]
> The TWS Client should run local

## Note

> __Note__: Notice here

## checked

:warning: Warning: File paths may change based on releases (especially __major__ version bumps)

Reference the __release__ tag or branch and _not_ the ~~master~~ branch because paths are subject to change for each release

* For example:
  * :white_check_mark: Test check Mark
  * :white_check_mark: Test check Mark
  * :x: Instead of: Test cross

<!--
* For example:
  * :white_check_mark: Use: <code>https\://github.com/ryanoasis/nerd-fonts/tree/<b>v3.0.0</b>/patched-fonts/Hermit/Regular/HurmitNerdFont-Regular.otf</code>
  * :white_check_mark: Use: <code>https\://github.com/ryanoasis/nerd-fonts/blob/<b>0.9.0</b>/patched-fonts/Hermit/Medium/complete/Hurmit%20Medium%20Nerd%20Font%20Complete.otf</code>
  * :x: Instead of: <code>https\://github.com/ryanoasis/nerd-fonts/blob/<del>master</del>/patched-fonts/Hermit/Medium/complete/Hurmit%20Medium%20Nerd%20Font%20Complete.otf</code>
-->

## Test Hello World World - create by cargo init

```bash
cargo run
```

## github icon

<!--
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
-->

## Test Link Logo

[![Foo](http://www.google.com.au/images/nav_logo7.png)](http://google.com.au/)

## Test github logo

[![Foo](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://meta.stackexchange.com/questions/38915/creating-an-image-link-in-markdown-format)

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
