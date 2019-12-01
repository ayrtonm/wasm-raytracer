import * as util from './util.js';
import { Scene } from '../pkg/wasm_raytracer.js';

var spheres = [];
var minRadius = 0.1;
var maxRadius = 0.5;
var maxDepth = -2;
var bgColor = "deepskyblue";
var fgColor = "red";

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
export function redraw(scene, fb) {
  scene.render();
  var wx = util.wx;
  var wy = util.wy;
  let imageData = new ImageData(fb, wx, wy);
  util.ctx.putImageData(imageData, 0, 0);
}
export function makeSphere(scene, fb, pos) {
  var x = 2 * (pos.x / util.wx) - 1;
  var y = 2 * (pos.y / util.wy) - 1;
  var z = Math.random()*maxDepth;
  var rad = Math.random()*(maxRadius - minRadius) + minRadius;
  console.log(rad);
  scene.make_sphere(x, y, z, rad);
  redraw(scene, fb);
  return scene.sphere_count() - 1;
}
export function moveSphere(scene, fb, idx, pos) {
  scene.move_sphere(idx, pos.x, pos.y);
  redraw(scene, fb);
}
export function deleteSphere(scene, fb, idx) {
  scene.delete_sphere(idx);
  redraw(scene, fb);
}
export function hitSphere(scene, pos) {
  return false;
  //for (var i = 0; i < scene.sphere_count(); i++) {
  //  var s = scene.sphere(i);
  //  var center = s.center();
  //  var radius = s.radius();
  //  if (center.x() <= pos.x + radius && center.x() >= pos.x - radius &&
  //      center.y() <= pos.y + radius && center.y() >= pos.y - radius) {
  //    return i;
  //  }
  //}
  //return false;
}
