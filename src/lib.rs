use pathfinding::prelude::*;
use rand::seq::SliceRandom;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::{f64, isize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl Point {
    fn to(from: &Point, x: isize, y: isize) -> Point {
        let x = from.x as isize + x;
        let y = from.y as isize + y;

        Point {
            x: usize::try_from(x).expect_throw("x is out of bounds"),
            y: usize::try_from(y).expect_throw("y is out of bounds"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Wall,
    Path,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        Maze {
            width,
            height,
            cells: vec![Cell::Wall; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[self.index(x, y)])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let i = self.index(x, y);
        self.cells[i] = cell;
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let directions = [(2, 0), (-2, 0), (0, 2), (0, -2)];

        for (dx, dy) in directions.iter() {
            if let Ok(nx) = (point.x as isize + *dx).try_into() {
                if let Ok(ny) = (point.y as isize + *dy).try_into() {
                    if nx < self.width && ny < self.height {
                        neighbors.push(Point { x: nx, y: ny });
                    }
                }
            }
        }

        neighbors
    }

    fn generate_maze(&mut self, start: Point) {
        let mut stack = VecDeque::new();
        let mut visited = HashSet::new();

        stack.push_back(start);
        visited.insert(start);

        while let Some(current) = stack.back().cloned() {
            let neighbors = self.neighbors(current);
            let unvisited_neighbors: Vec<Point> = neighbors
                .into_iter()
                .filter(|&neighbor| !visited.contains(&neighbor))
                .collect();

            if !unvisited_neighbors.is_empty() {
                let next = *unvisited_neighbors.choose(&mut rand::thread_rng()).unwrap();
                let wall = Point {
                    x: (current.x + next.x) / 2,
                    y: (current.y + next.y) / 2,
                };

                self.set(wall.x, wall.y, Cell::Path);
                self.set(next.x, next.y, Cell::Path);

                visited.insert(next);
                stack.push_back(next);
            } else {
                stack.pop_back();
            }
        }
    }

    fn successors(&self, start: &Point) -> Vec<Point> {
        vec![
            Point::to(start, -1, 0),
            Point::to(start, 1, 0),
            Point::to(start, 0, -1),
            Point::to(start, 0, 1),
        ]
        .into_iter()
        .filter(|point| {
            let cell = self.get(point.x, point.y);
            return cell.is_some() && cell.unwrap() != Cell::Wall;
        })
        .collect()
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(Cell::Wall) => {
                        write!(f, "#")?;
                    }
                    Some(Cell::Path) => {
                        write!(f, ".")?;
                    }
                    None => {
                        write!(f, " ")?;
                    }
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn get_canvas() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context)
}

#[wasm_bindgen]
pub fn draw_maze(size: usize) -> Maze {
    let (canvas, context) = get_canvas();

    let mut maze = Maze::new(size, size);
    maze.set(1, 1, Cell::Path);

    maze.generate_maze(Point { x: 1, y: 1 });

    let cell_size = f64::from(canvas.width()) / maze.width as f64;

    for y in 0..maze.height {
        for x in 0..maze.width {
            match maze.get(x, y) {
                Some(Cell::Wall) => {
                    context.set_fill_style(&"black".into());
                    context.fill_rect(
                        x as f64 * cell_size,
                        y as f64 * cell_size,
                        cell_size,
                        cell_size,
                    );
                }
                Some(Cell::Path) => {
                    context.set_fill_style(&"white".into());
                    context.fill_rect(
                        x as f64 * cell_size,
                        y as f64 * cell_size,
                        cell_size,
                        cell_size,
                    );
                }
                None => {}
            }
        }
    }

    maze
}

#[wasm_bindgen]
pub fn path_find(maze: Maze, start: Point, goal: Point) {
    let path = bfs(
        &start,
        |n| Maze::successors(&maze, n).into_iter().collect::<Vec<_>>(),
        |n| n == &goal,
    )
    .expect_throw("failed to generate path");

    let (canvas, context) = get_canvas();

    let cell_size = f64::from(canvas.width()) / maze.width as f64;
    let path_size = cell_size * 0.5;

    for point in path {
        context.set_fill_style(&"red".into());
        context.fill_rect(
            point.x as f64 * cell_size + cell_size * 0.25,
            point.y as f64 * cell_size + cell_size * 0.25,
            path_size.into(),
            path_size.into(),
        );
    }

    context.set_fill_style(&"yellow".into());
    context.fill_rect(
        start.x as f64 * cell_size + cell_size * 0.25,
        start.y as f64 * cell_size + cell_size * 0.25,
        path_size.into(),
        path_size.into(),
    );

    context.set_fill_style(&"green".into());
    context.fill_rect(
        goal.x as f64 * cell_size + cell_size * 0.25,
        goal.y as f64 * cell_size + cell_size * 0.25,
        path_size.into(),
        path_size.into(),
    );
}
