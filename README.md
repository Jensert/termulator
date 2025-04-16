# Termulator
My hobby project to create a 3d renderer in a terminal environment.

I've always liked old-school / retro visuals in entertainment media. Espescially videogames.
I thought it would be a fun and challenging project to try to create my own 3d rendering engine in a terminal.

## Features
For now it only renders a square and allows free movement in all directions using WASD to move and arrow keys to look around:

## Goals
Future goal is to be able to load and render external models.
Afterwards I want to create a simple 3d game using this renderer

## Building and dependencies
This is made using the rust programming language, with the following dependencies:
- crossterm
- ratatui
- color-eyre

In order to build and run this yourself:
```
git clone https://github.com/Jensert/termulator
cd termulator
cargo run
```
