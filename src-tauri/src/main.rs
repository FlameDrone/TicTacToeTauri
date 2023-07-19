// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn place(num:usize , turn: usize, mut board: [&str; 9]) -> (String,[&str; 9], String) {
    let symbol;
    let num = num-1;
    let mut won: String = String::new();
    if turn%2 == 0 {
        symbol = "X";
    }
    else{
        symbol = "O"
    }
    board[num] = symbol;
    if check_board(board){
        won = format!("{} won!", symbol);
    }
    else if turn>=8{
        won = format!("Draw!");
    }
    return (format!("{}", symbol),board, won);
}

fn check_board(board: [&str; 9]) -> bool {
    for i in 0..3{
        if board[i*3] == board[i*3+1] && board[i*3+1] == board[i*3+2] && board[i*3+1] != ""{
            return true;
        }
    }
    for i in 0..3{
        if board[i] == board[i+3] && board[i+3] == board[i+6] && board[i+3] != ""{
            return true;
        }
    }
    if board[0] == board[4] && board[4] == board[8] && board[4] != ""{
        return true;
    }
    if board[2] == board[4] && board[4] == board[6] && board[4] != ""{
        return true;
    }
    return false;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![place])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
