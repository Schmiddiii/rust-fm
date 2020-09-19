# rust-fm
rust-fm is a cli-file manager written in Rust. It has Vim-like keybindings, but focuses on just typing where to go.

## What it can
* Navigate in the folder structure of the system
* Typing shows only the elements you are interested in using fuzzy finding
* Preview of folders

## What it cannot yet do
* Basic file and folder actions like copy-paste, editing a file, deleting, ...
* Preview of file
* Open in folder provided in the arguments

## How to use it
* Clone the repository
* Build the binary using cargo build (rust and cargo have to be installed, see [installation guide](https://www.rust-lang.org/tools/install).
* Binary should be target/debug/rust-fm
