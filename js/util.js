export var canvasName = "scene";
export var canvas = document.getElementById(canvasName);
export var ctx = canvas.getContext("2d");

export function getMousePos(evt) {
  var r = canvas.getBoundingClientRect();
  return {
    x: evt.clientX - r.left,
    y: evt.clientY - r.top
  };
}
