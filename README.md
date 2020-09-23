# rust-fm
rust-fm is a cli-file manager written in Rust. It has Vim-like keybindings, but focuses on just typing where to go.

## What it can
* Navigate in the folder structure of the system
* Typing shows only the elements you are interested in using fuzzy finding
* Preview of folders and (text)files
* Opening files in $EDITOR or using xdg-open
* Copy-Paste

## What it cannot yet do
* Basic file and folder actions like renaming, deleting, ...
* Create a file or folder

## How to install it
* Clone the repository
* Build the binary using cargo build (rust and cargo have to be installed, see [installation guide](https://www.rust-lang.org/tools/install))
* Binary should be target/debug/rust-fm

## Usage
All vim-like keys have been bound to the uppercase variants.
Therefor J and K move around the elements in the directory,
H and L moves between directories.
Furthermore Q quits the application.

Despite having these keys, the main feature is just typing where you want to go.
When starting to type the selection of elements in the directory will reduce. 
To view all possibilities again just hit the escape key.

## Additional Features

### Copy-Paste
To copy and paste filesor folders you first have to highlight the files you want. To highlight files press the space bar when selecting the item. You can highlight many items at once. Now you have to yank your files or folders using 'Y'. Now move to the folder you want those items to be and press 'P' to paste.
 
