# Termulator
Hobby project to create a 3d renderer in a terminal environment.
![termulator 0 1](https://github.com/user-attachments/assets/e65320d9-1d04-4e26-8798-d5522e2a87ad)

I've always liked old-school / retro visuals in entertainment media. Espescially videogames.
I thought it would be a fun and challenging project to try to create my own 3d rendering engine, without any graphics API's, in a terminal.

## Features
For now it only renders a cube and allows free movement in all directions using WASD to move and arrow keys to look around.

The cube is rendered by first defining the vertices and indexes.
The renderer then loops over the vertices and draws lines in-between the vertices by using the indices to know which vertex connects to which.

This is mostly the same approach as how a GPU works. 
The biggest differences being that this draws line by line, instead of triangles, and that everything is run on the CPU instead of the GPU.

## Limitations
Currently this can only draw a predefined set of vertices. It would be cool to be able to load external models.

The renderer does not properly draw vertices that are out-of-bounds. 
Lines that have 1 vertex outside of the viewport space, are not drawn at all.

There is no depth shading or any other kind of shading / lighting implemented. This would be very cool and make it alot more flexible for all kinds of applications (games, simulations...)

## Future Goals
- Load external models
- Add proper out-of-bounds checking to draw shapes that are partly outside of the viewport
- Add depth shading
- Add lighting

## Building and dependencies
This is made using the rust programming language, with the following dependencies:
- crossterm (to get user input)
- ratatui (to easily format the terminal output and draw the lines)
- color-eyre (for error handling)

In order to build and run this yourself:
```
git clone https://github.com/Jensert/termulator
cd termulator
cargo run
```
