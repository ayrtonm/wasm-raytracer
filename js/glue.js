import * as util from './util.js';

const minRadius = 0.1;
const maxRadius = 0.3;
const minDepth = -5.0;
const maxDepth = -0.3;

export function redraw(scene, fb) {
  scene.render();
  let imageData = new ImageData(fb, util.wx, util.wy);
  util.ctx.putImageData(imageData, 0, 0);
}

function randInRange(min, max) {
  return min + Math.random()*(max - min);
}

function posToSceneCoord(pos) {
  var x = 2 * (pos.x / util.wx) - 1;
  var y = 2 * (pos.y / util.wy) - 1;
  return { x: x, y: y }
}

//this function doesn't always create spheres where we click
export function makeSphere(scene, fb, pos) {
  var coord = posToSceneCoord(pos);
  var z = randInRange(minDepth, maxDepth);
  var rad = randInRange(minRadius, maxRadius);
  scene.make_sphere(coord.x, coord.y, z, rad);
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
