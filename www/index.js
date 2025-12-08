import init, { AnswerGrid, Answer, Square } from '../pkg/connections.js';

const wasm = await init();

const WIDTH = 4;
const HEIGHT = 4;

const ANSWERS = [
    {
        group: "yellow",
        description: "ANAGRAMS",
        answers: [
            "NAMED",
            "AMEND",
            "ADMEN",
            "MANED"
        ]
    },
    {
        group: "green",
        description: "BLOCK UP",
        answers: [
            "DAM",
            "CHOKE",
            "CONGEST",
            "OCCLUDE"
        ]
    },
    {
        group: "blue",
        description: "ENDING IN WORDS FOR COUNTRY",
        answers: [
            "DAMNATION",
            "GESTATE",
            "ISOPOLITY",
            "HIGHLAND"
        ]
    },
    {
        group: "purple",
        description: "CONTAINING SYNONYMS FOR SOAK",
        answers: [
            "ISOPODA",
            "STEEPLE",
            "DIPED",
            "RETURN"
        ]
    },
];

let answers = ANSWERS.map((a) => { Answer.new(a.group, a.description, a.answers) });
const answerGrid = AnswerGrid.new(WIDTH, HEIGHT, answers);
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