// scripts.js

// the 'use strict' directive enforces stricter parsing and error handling on the code at runtime
'use strict';

// add the current year to the page footer
let currentYear = new Date().getFullYear();
let year = document.getElementById('year');
year.textContent = currentYear;

// variable declarations
const q_text = document.getElementById("question_text");
const a_text = document.getElementById("answer_text");
let answer;
const fetch_url = "https://rusty-flash-knowledge.net/v1/flashcards/random";

// eventlistener for the question button
const q_button = document.getElementById("question_button");
q_button.addEventListener("click",  async () => {
  try {
    const response = await fetch(fetch_url);
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    const json = await response.json();
    q_text.innerText = json.content.question;
    answer = json.content.answer;
  } catch (error) {
    console.error(error.message);
  }
});

// eventlistener for the answer button
const a_button = document.getElementById("answer_button");
a_button.addEventListener("click", () => {
  a_text.innerText = answer;
});

// eventlistener for the reset button
const reset_button = document.getElementById("reset_button");
reset_button.addEventListener("click", () => {
  q_text.innerText = "";
  a_text.innerText = "";
});