import { invoke } from "@tauri-apps/api/tauri";

let buttons: HTMLCollection | null;
let turn: number = 0;
let board: string[] = ["","",""  ,"","",""  ,"","",""];
let won: HTMLElement | null;

async function place(num: number){
  if(buttons){
    num = num+1;
    let b = document.getElementById("b"+num);
    if(b && won){
      [b.innerText,board, won.textContent] = await invoke("place", {
      num: num,
      turn: turn,
      board: board,
    })  
      b.setAttribute("disabled", "disabled");
      turn += 1;
      if(won.textContent!=""){
        for (let i = 0; i < buttons.length; i++) {
          buttons[i].setAttribute("disabled", "disabled");
        }
      }
    }
  }
}

window.addEventListener("DOMContentLoaded", () => {

  buttons = document.getElementsByClassName("button");
  won = document.getElementById("won");

  for (let i = 0; i < buttons.length; i++) {
    buttons[i]?.addEventListener("click", (e) => {
      e.preventDefault();
      place(i);
    });
  }
});
