import * as util from './util.js';

const minRadius = 0.1;
const maxRadius = 0.5;
const minDepth = -5.0;
const maxDepth = -0.3;

export function redraw(scene, fb) {
  scene.render();
  util.ctx.putImageData(fb, 0, 0);
}

function randInRange(min, max) {
  return min + Math.random()*(max - min);
}

function randColor() {
  return {
    r: Math.trunc(randInRange(0, 255)),
    g: Math.trunc(randInRange(0, 255)),
    b: Math.trunc(randInRange(0, 255)),
  }
}

function posToSceneCoord(pos) {
  var x = 4 * (pos.x / util.wx) - 2;
  var y = 4 * (pos.y / util.wy) - 2;
  return { x: x, y: y }
}

export function makeSphere(scene, fb, pos) {
  var coord = posToSceneCoord(pos);
  var rad = randInRange(minRadius, maxRadius);
  var col = randColor();
  scene.make_sphere(coord.x, coord.y, rad, col.r, col.g, col.b);
  redraw(scene, fb);
  return scene.sphere_count() - 1;
}

export function moveSphere(scene, fb, idx, pos) {
  var coord = posToSceneCoord(pos);
  scene.move_sphere(idx, coord.x, coord.y);
  redraw(scene, fb);
}

export function deleteSphere(scene, fb, idx) {
  scene.delete_sphere(idx);
  redraw(scene, fb);
}

export function hitSphere(scene, pos) {
  var maybeIdx = scene.hit_sphere(pos.x, pos.y);
  if (maybeIdx > scene.sphere_count()) {
    return false;
  }
  return maybeIdx;
}
