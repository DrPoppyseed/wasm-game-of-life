import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE_PX = 16;
const GRID_COLOR_HEX = "#cccccc";
const DEAD_COLOR_HEX = "#ffffff";
const ALIVE_COLOR_HEX = "#000000";

const canvas = document.getElementById("canvas");
const playPauseButton = document.getElementById("play-pause");
const stepNextButton = document.getElementById("step-next");
const iterationSpan = document.getElementById("iteration-span");

let iteration = 0;

const canvasWidth = Math.floor(canvas.clientWidth / CELL_SIZE_PX);
const canvasHeight = Math.floor(canvas.clientHeight / CELL_SIZE_PX);
const universe = Universe.new(canvasWidth, canvasHeight, 0.3);
const width = universe.width();
const height = universe.height();

canvas.height = (CELL_SIZE_PX + 1) * height + 1;
canvas.width = (CELL_SIZE_PX + 1) * width + 1;

const ctx = canvas.getContext("2d");

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR_HEX;

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE_PX + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE_PX + 1) + 1, (CELL_SIZE_PX + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE_PX + 1) + 1);
    ctx.lineTo((CELL_SIZE_PX + 1) * width + 1, j * (CELL_SIZE_PX + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, col) => row * width + col;

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      ctx.fillStyle = bitIsSet(idx, cells) ? DEAD_COLOR_HEX : ALIVE_COLOR_HEX;

      ctx.fillRect(
        col * (CELL_SIZE_PX + 1) + 1,
        row * (CELL_SIZE_PX + 1) + 1,
        CELL_SIZE_PX,
        CELL_SIZE_PX,
      );
    }
  }

  ctx.stroke();
};

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

const isPaused = () => animationId === null;

playPauseButton.addEventListener("click", () => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

stepNextButton.addEventListener("click", () => {
  if (!isPaused()) {
    pause();
  }

  drawGrid();
  drawCells();
  universe.tick();
  iteration += 1;
  iterationSpan.textContent = `${iteration}`;
});

let animationId = null;
const renderLoop = () => {
  drawGrid();
  drawCells();

  universe.tick();
  iteration += 1;
  iterationSpan.textContent = `${iteration}`;

  animationId = requestAnimationFrame(renderLoop);
};

drawGrid();
drawCells();
play();
