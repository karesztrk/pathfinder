import init, {
  draw_maze,
  draw_path,
  Algorithm,
  clean,
  add_listeners,
  Point,
} from "./pkg/pathfinder.js";

async function run() {
  await init();
}

run();

const gridForm = document.getElementById("grid");
const pathWrapper = document.getElementById("path_wrapper");
const pathForm = document.getElementById("path");
const info = document.getElementById("info");
const cleanMaze = document.getElementById("clean");

if (gridForm) {
  gridForm.addEventListener("submit", (e) => {
    e.preventDefault();
    const data = new FormData(e.target);

    const size = Number(data.get("size")) + 1;
    const maze = draw_maze(size);
    add_listeners(maze);

    reveal();

    if (pathForm) {
      pathForm.addEventListener("submit", (e) => {
        e.preventDefault();
        const data = new FormData(e.target);
        const alg = data.get("algorithm");
        const [start_x, start_y] = data.get("start").split(",");
        const [goal_x, goal_y] = data.get("goal").split(",");
        const start = new Point(start_x, start_y);
        const goal = new Point(goal_x, goal_y);
        draw_path(maze, start, goal, Algorithm[alg]);
      });
    }
    if (cleanMaze) {
      cleanMaze.addEventListener("click", () => {
        clean(maze);
      });
    }
  });
}

function reveal() {
  if (pathWrapper) {
    pathWrapper.style.display = "block";
  }
  if (cleanMaze) {
    cleanMaze.style.display = "block";
  }
}
