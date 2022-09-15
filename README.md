# Clevis
An __incomplete__ implementation for group project of HK PolyU COMP 2021 in rust programming language. You may read [project_description.pdf](./project_description.pdf) for more detailed  information. My main goal of doing this project is to practice all kinds of language features of rust. 

# Example Usage
Run the program by `cargo run`:
```
$ cargo run
... (some help information)
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

The whole program is designed to be highly extensible. On top level, there are two component, a `Commander` and an `App`.
 - `Commander`: responsible to read user input and generate `Command`s for `App`.
 - `App`: designed in MVC architecture. 
    - `Shapes`: a `HashMap` storing all shapes.
    - `Executor`: receive `Command`s and execute them to manipulate `Shapes`.
    - `Renderer`: render the current frame according to `Shapes` in an async asynchronous manner.

## New Shape
To add a new shape, you need to
1. Add a new struct in `src/shapes.rs`, and implementing `Shape` trait for it.
2. When you implement `draw_on()` function, which is required by `Shape` trait, if you need help from renderer side, feel free add a new member function like `draw_your_shpae()` to `Renderer` trait in `src/renderer.rs`. If you want existing renderers to support your new shape, please implement `draw_your_shpae()` for all existing renderers.
3. Add a new command to `src/command/draw_shape.rs` which allows users to draw your shape.

## New Command
You may add new command types to `crate::command` scope by implementing `Command` trait. Don't forget to add a parse function for your command in `crate::commander::cli_commander`.

## New Commander
Commander is responsible to read user input and generate `Command`s for `App`. There is a default `CliCommander` which fetches command from terminal. You may add a new commander to `crate::commander` by implementing `Commander` trait.

## New Renderer
`Renderer` is responsible to render all shapes into current frame. There is a default `HtmlRenderer` which save all shapes into an webpage and open the webpage by your default browser. You may add a new renderer to `crate::render` by implementing `Renderer` trait.

Have fun! 😄