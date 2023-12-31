use pathfinding::prelude::*;
use rand::seq::SliceRandom;
use std::cell::Cell;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::rc::Rc;
use std::{f64, isize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::{CanvasRenderingContext2d, HtmlInputElement};

const PATH_SIZE_RATIO: f64 = 0.5;
const PATH_POSITION_OFFSET: f64 = 0.25;
const PATH_COLOR: &str = "white";
const WALL_COLOR: &str = "black";
const START_COLOR: &str = "cornflowerblue";
const TRAIL_COLOR: &str = "firebrick";
const GOAL_COLOR: &str = "forestgreen";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

trait Drawable {
    fn draw(&self);
    fn clear(&self);
    fn redraw(&self) {
        self.clear();
        self.draw();
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridCell {
    Wall,
    Path,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<GridCell>,
    cell_size: f64,
}

#[wasm_bindgen]
pub enum Algorithm {
    Dijkstra,
    Bfs,
    Dfs,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn to(from: &Point, x: isize, y: isize) -> Point {
        let x = from.x as isize + x;
        let y = from.y as isize + y;

        Point {
            x: usize::try_from(x).expect_throw("x is out of bounds"),
            y: usize::try_from(y).expect_throw("y is out of bounds"),
        }
    }
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![GridCell::Wall; width * height],
            cell_size: Maze::calc_cell_size(width),
        }
    }

    fn calc_cell_size(width: usize) -> f64 {
        let (canvas, _context) = get_canvas();
        f64::from(canvas.width()) / width as f64
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, x: usize, y: usize) -> Option<GridCell> {
        if x < self.width && y < self.height {
            Some(self.cells[self.index(x, y)])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: GridCell) {
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

                self.set(wall.x, wall.y, GridCell::Path);
                self.set(next.x, next.y, GridCell::Path);

                visited.insert(next);
                stack.push_back(next);
            } else {
                stack.pop_back();
            }
        }
    }

    fn successors(&self, start: &Point) -> Vec<Point> {
        [
            Point::to(start, -1, 0),
            Point::to(start, 1, 0),
            Point::to(start, 0, -1),
            Point::to(start, 0, 1),
        ]
        .into_iter()
        .filter(|point| {
            self.get(point.x, point.y)
                .is_some_and(|c| c != GridCell::Wall)
        })
        .collect()
    }
}

impl Drawable for Maze {
    fn draw(&self) {
        let (_canvas, context) = get_canvas();

        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(GridCell::Wall) => {
                        context.set_fill_style(&WALL_COLOR.into());
                        context.fill_rect(
                            x as f64 * self.cell_size,
                            y as f64 * self.cell_size,
                            self.cell_size,
                            self.cell_size,
                        );
                    }
                    Some(GridCell::Path) => {
                        context.set_fill_style(&PATH_COLOR.into());
                        context.fill_rect(
                            x as f64 * self.cell_size,
                            y as f64 * self.cell_size,
                            self.cell_size,
                            self.cell_size,
                        );
                    }
                    None => {}
                }
            }
        }
    }

    fn clear(&self) {
        let (canvas, context) = get_canvas();
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(GridCell::Wall) => {
                        write!(f, "#")?;
                    }
                    Some(GridCell::Path) => {
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)?;

        Ok(())
    }
}

#[wasm_bindgen]
pub struct Path {
    cell_size: f64,
    steps: Vec<Point>,
}

impl Path {
    fn new(maze: &Maze, start: Point, goal: Point, algorithm: Algorithm) -> Self {
        Self {
            cell_size: Path::calc_cell_size(maze.width),
            steps: Path::calc(maze, &start, &goal, &algorithm),
        }
    }

    fn calc(maze: &Maze, start: &Point, goal: &Point, algorithm: &Algorithm) -> Vec<Point> {
        let path = match algorithm {
            Algorithm::Bfs => bfs(
                start,
                |n| Maze::successors(&maze, n).into_iter().collect::<Vec<_>>(),
                |n| n == goal,
            ),

            Algorithm::Dfs => dfs(
                *start,
                |n| Maze::successors(&maze, n).into_iter(),
                |n| n == goal,
            ),

            Algorithm::Dijkstra => dijkstra(
                start,
                |n| Maze::successors(&maze, n).into_iter().map(|n| (n, 1)),
                |n| n == goal,
            )
            .map(|n| n.0),
        };
        path.expect("failed to generate path")
    }

    fn calc_cell_size(width: usize) -> f64 {
        let (canvas, _ctx) = get_canvas();
        canvas.width() as f64 / width as f64
    }

    pub fn calc_path_size(cell_size: f64) -> f64 {
        cell_size * PATH_SIZE_RATIO
    }

    pub fn get_path_size(&self) -> f64 {
        Path::calc_path_size(self.cell_size)
    }

    fn get_cell_position(&self, point: &Point) -> (f64, f64) {
        Path::calc_cell_position(self.cell_size, point)
    }

    pub fn calc_cell_position(cell_size: f64, point: &Point) -> (f64, f64) {
        let x = point.x as f64 * cell_size + cell_size * PATH_POSITION_OFFSET;
        let y = point.y as f64 * cell_size + cell_size * PATH_POSITION_OFFSET;
        (x, y)
    }
}

impl Drawable for Path {
    fn draw(&self) {
        let start = self.steps.first().expect("path steps are empty");
        let goal = self.steps.last().expect("path steps are empty");
        let (_canvas, context) = get_canvas();

        let path_size = self.get_path_size();

        // Trail
        for point in self.steps.iter() {
            context.set_fill_style(&TRAIL_COLOR.into());
            let (x, y) = self.get_cell_position(&point);
            context.fill_rect(x, y, path_size.into(), path_size.into());
        }

        // Start
        context.set_fill_style(&START_COLOR.into());
        let (start_x, start_y) = self.get_cell_position(start);
        context.fill_rect(start_x, start_y, path_size.into(), path_size.into());

        // Goal
        context.set_fill_style(&GOAL_COLOR.into());
        let (goal_x, goal_y) = self.get_cell_position(&goal);
        context.fill_rect(goal_x, goal_y, path_size.into(), path_size.into());
    }

    fn clear(&self) {
        let (_canvas, context) = get_canvas();
        let path_size = self.get_path_size();
        for point in self.steps.iter() {
            context.set_fill_style(&TRAIL_COLOR.into());
            let (x, y) = self.get_cell_position(&point);
            context.clear_rect(x, y, path_size.into(), path_size.into());
        }
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

fn set_input_value(id: &str, value: &str) {
    let document = web_sys::window().unwrap().document().unwrap();

    document
        .get_element_by_id(id)
        .expect(&format!("should have #{} on the page", id))
        .dyn_ref::<HtmlInputElement>()
        .expect(&format!("#{} should be an `HtmlInputElement`", id))
        .set_value(value);
}

#[wasm_bindgen]
pub fn draw_maze(size: usize) -> Maze {
    let mut maze = Maze::new(size, size);
    maze.set(1, 1, GridCell::Path);

    maze.generate_maze(Point { x: 1, y: 1 });
    maze.clear();
    maze.draw();
    maze
}

#[wasm_bindgen]
pub fn add_listeners(maze: &Maze) {
    let (canvas, context) = get_canvas();
    let start = Rc::new(Cell::new(None));
    let goal = Rc::new(Cell::new(None));
    {
        let cell_size = Maze::calc_cell_size(maze.width);
        let path_size = Path::calc_path_size(cell_size);
        let maze = maze.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let x = (event.offset_x() as f64 / cell_size) as usize;
            let y = (event.offset_y() as f64 / cell_size) as usize;

            if maze.get(x, y).is_some_and(|c| c == GridCell::Path) {
                if start.get().is_none() {
                    let point = Point { x, y };
                    start.set(Some(point));
                    context.set_fill_style(&START_COLOR.into());
                    let (point_x, point_y) = Path::calc_cell_position(cell_size, &point);
                    context.fill_rect(point_x, point_y, path_size.into(), path_size.into());

                    set_input_value("start", &point.to_string());
                } else if goal.get().is_none() {
                    let point = Point { x, y };
                    goal.set(Some(point));
                    context.set_fill_style(&GOAL_COLOR.into());
                    let (point_x, point_y) = Path::calc_cell_position(cell_size, &point);
                    context.fill_rect(point_x, point_y, path_size.into(), path_size.into());

                    set_input_value("goal", &point.to_string());
                }
            }
        });
        canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}

#[wasm_bindgen]
pub fn draw_path(maze: &Maze, start: Point, goal: Point, algorithm: Algorithm) -> Path {
    maze.redraw();
    let path = Path::new(&maze, start, goal, algorithm);
    path.draw();
    path
}

#[wasm_bindgen]
pub fn clean(maze: &Maze) {
    maze.redraw();
}
