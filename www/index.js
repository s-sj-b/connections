import init, { AnswerGrid, Square } from '../pkg/connections.js';

const wasm = await init();

const WIDTH = 4;
const HEIGHT = 4;
const CELL_WIDTH = 100;
const CELL_HEIGHT = 100;

// once wasm has loaded, do stuff ...
const answerGrid = AnswerGrid.new(WIDTH, HEIGHT);
answerGrid.setup();


const buttons = document.getElementsByClassName("button");
Array.from(buttons).forEach(button => {
    button.addEventListener("click", event => {
        // grab the co-ordinates from the element id
        let x = Number(button.id[1]);
        let y = Number(button.id[0]);
        answerGrid.toggle_square(x, y);
    });
});


const submitButton = document.getElementById("submit-button");
submitButton.addEventListener("click", event => {
    let selectedSquares = answerGrid.submit_selection();
    console.log(selectedSquares);
});