import * as util from './util.js';
import { Scene } from '../pkg/wasm_raytracer.js';

var spheres = [];
var minRadius = 5;
var maxRadius = 20;
var maxDepth = -2;
var bgColor = "deepskyblue";

function colorToString(r, g, b) {
  var color = b + (g << 8) + (r << 16);
  var colorString = color.toString(16);
  var slen = colorString.length;
  for (var i = slen; i < 6; i++) {
    colorString = "0" + colorString;
  }
  return "#" + colorString;
}

function randColor() {
  var r = Math.trunc(Math.random() * 256);
  var g = Math.trunc(Math.random() * 256);
  var b = Math.trunc(Math.random() * 256);
  return colorToString(r,g,b);
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
export function redraw(scene) {
  scene.render();
  //var wx = util.canvas.width;
  //var wy = util.canvas.height;
  //util.ctx.fillStyle = bgColor;
  //for (var x = 0; x < wx; x++) {
  //  for (var y = 0; y < wy; y++) {
  //    util.ctx.fillRect(x, y, 1, 1);
  //  }
  //}
}
export function makeSphere(scene, pos) {
  scene.make_sphere(pos.x, pos.y,
                    Math.random()*maxDepth,
                    Math.random()*(maxRadius - minRadius) + minRadius);
  redraw(scene);
  return scene.sphere_count();
}
export function moveSphere(scene, idx, pos) {
  scene.move_sphere(idx, pos.x, pos.y);
  redraw(scene);
}
export function deleteSphere(scene, idx) {
  scene.delete_sphere(idx);
  redraw(scene);
}
export function hitSphere(scene, pos) {
  for (var i = 0; i < scene.sphere_count(); i++) {
    var s = scene.sphere(i);
    var center = s.center();
    var radius = s.radius();
    if (center.x() <= pos.x + radius && center.x() >= pos.x - radius &&
        center.y() <= pos.y + radius && center.y() >= pos.y - radius) {
      return i;
    }
  }
  return false;
}
