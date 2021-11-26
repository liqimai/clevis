# Clevis
An __incomplete__ implementation for group project of PolyU COMP 2021 in rust programming language. You may read [project_description.pdf](./project_description.pdf) for more detailed  information. My main goal of doing this project is to practice all kinds of language features of rust. 

# Example Usage
```
> rectangle rect 200 200 100 200
> point p2 100 100
> circle cir1 250 250 50
> square sq1 250 350 100
> delete cir1
> undo
> redo
> foo
"foo" is not a valid command.
> bar foo
"bar" is not a valid command.
```
Press <kbd>CTRL</kbd>+<kbd>D</kbd> to exit.

# Extensibility

The whole program is designed to be easily extensible. On top level, there are two component, a `Commander` and an `App`.
 - `Commander`: responsible to read user input and generate `Command`s for `App`.
 - `App`: designed in MVC architecture. 
    - `Shapes`: a `HashMap` storing all shapes drawn.
    - `Executor`: receive `Command`s and execute them to manipulate `Shapes`.
    - `Renderer`: render the current frame according to `Shapes` in an async asynchronous manner.

## New Shape
Please add new shape types to `crate::shape` by implementing `Shapes` trait.

## New Command
You may add new command types to `crate::command` scope by implementing `Command` trait. Don't forget to add a parse function for your command in `crate::commander::cli_commander`.

## New Commander
Commander is responsible to read user input and generate `Command`s for `App`. There is a default `CliCommander` which fetches command from terminal. You may add a new commander to `crate::commander` by implementing `Commander` trait.

## New Renderer
`Renderer` is responsible to render all shapes into current frame. There is a default `HtmlRenderer` which save all shapes into an webpage and open the webpage by your default browser. You may add a new renderer to `crate::render` by implementing `Renderer` trait.

Have fun! 😄