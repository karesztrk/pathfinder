import init, { path_find, Point, draw_maze } from "./pkg/pathfinder.js";

async function run() {
  await init();
}

run();

const gridForm = document.getElementById("grid");
const pathFormWrapper = document.getElementById("path_wrapper");
const pathForm = document.getElementById("path");

let maze;

if (gridForm) {
  gridForm.addEventListener("submit", (e) => {
    debugger;
    e.preventDefault();
    const data = new FormData(e.target);

    const size = Number(data.get("size")) + 1;
    maze = draw_maze(size);
    pathFormWrapper.style.display = "block";
  });
}

if (pathForm) {
  pathForm.addEventListener("submit", (e) => {
    if (!maze) {
      throw new Error("Maze uninitialized");
    }
    e.preventDefault();
    const data = new FormData(e.target);

    const [startX, startY] = data.get("start").split(",");
    const [goalX, goalY] = data.get("goal").split(",");
    path_find(maze, new Point(startX, startY), new Point(goalX, goalY));
  });
}
