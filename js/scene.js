import * as util from './util.js';
import { Scene } from '../pkg/wasm_raytracer.js';

var spheres = [];
var minRadius = 5;
var maxRadius = 20;
var maxDepth = -2;
var bgColor = "deepskyblue";

function randColor() {
  var r = Math.trunc(Math.random() * 256);
  var g = Math.trunc(Math.random() * 256);
  var b = Math.trunc(Math.random() * 256);
  var color = b + (g << 8) + (r << 16);
  var colorString = color.toString(16);
  var slen = colorString.length;
  for (var i = slen; i < 6; i++) {
    colorString = "0" + colorString;
  }
  return "#" + colorString;
}
export function drawSphere(sphere) {
  util.ctx.beginPath();
  util.ctx.arc(sphere.x, sphere.y, sphere.radius, 0, 2 * Math.PI);
  util.ctx.fillStyle = sphere.color;
  util.ctx.fill();
  util.ctx.lineWidth = 1;
  util.ctx.strokeStyle = sphere.color;
  util.ctx.stroke();
}
export function redraw() {
  const x = Scene.new();
  x.render();
  //util.ctx.fillStyle = bgColor;
  //util.ctx.fillRect(0, 0, util.canvas.width, util.canvas.height);
  for (var i = 0; i < spheres.length; i++) {
    drawSphere(spheres[i]);
  }
}
export function makeSphere(pos) {
  spheres.push({"x": pos.x,
                "y": pos.y,
                "z": Math.random()*maxDepth,
                "radius": Math.random()*(maxRadius - minRadius) + minRadius,
                "color": randColor()});
  redraw();
  return spheres.length - 1;
}
export function moveSphere(idx, pos) {
  spheres[idx].x = pos.x;
  spheres[idx].y = pos.y;
  redraw();
}
export function deleteSphere(idx) {
  spheres.splice(idx, 1);
  redraw();
}
export function hitSphere(pos) {
  for (var i = 0; i < spheres.length; i++) {
    if (spheres[i].x <= pos.x + spheres[i].radius && spheres[i].x >= pos.x - spheres[i].radius &&
        spheres[i].y <= pos.y + spheres[i].radius && spheres[i].y >= pos.y - spheres[i].radius) {
      return i;
    }
  }
  return false;
}
