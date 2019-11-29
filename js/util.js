export var canvasname = "scene";
export var canvas = document.getElementById(canvasname);
export var ctx = canvas.getContext("2d");

export function getMousePos(evt) {
  var r = canvas.getBoundingClientRect();
  return {
    x: evt.clientX - r.left,
    y: evt.clientY - r.top
  };
}
