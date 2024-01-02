import init, { Point, draw_maze, add_listeners } from "./pkg/pathfinder.js";

async function run() {
  await init();
}

run();

const gridForm = document.getElementById("grid");

if (gridForm) {
  gridForm.addEventListener("submit", (e) => {
    e.preventDefault();
    const data = new FormData(e.target);

    const size = Number(data.get("size")) + 1;
    const maze = draw_maze(size);
    add_listeners(maze);
  });
}
