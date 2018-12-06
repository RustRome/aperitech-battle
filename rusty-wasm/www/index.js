import { Game } from "rusty-wasm";

const game = Game.new("canvas");

let key = 37;

window.onkeydown = event => (key = event.keyCode);

setInterval(() => {
  game.tick(key);
}, 100);
