use rand::seq::SliceRandom;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Wall,
    Path,
}

struct Maze {
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

    fn generate_maze_recursive_backtracker(&mut self, start: Point) {
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

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(Cell::Wall) => print!("#"),
                    Some(Cell::Path) => print!(" "),
                    None => print!(" "),
                }
            }
            println!("");
        }
    }
}

fn main() {
    let width = 4;
    let height = 4;
    let mut maze = Maze::new(width, height);

    // Start with a path at the top-left corner
    let start = Point { x: 0, y: 0 };
    maze.generate_maze_recursive_backtracker(start);

    maze.print();
}
