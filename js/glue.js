import * as util from './util.js';

export function redraw(scene, fb) {
  scene.render();
  redisplay(fb);
}

export function redisplay(fb) {
  util.ctx.putImageData(fb, 0, 0);
}

function posToSceneCoord(pos) {
  var x = (pos.x / util.wx);
  var y = (pos.y / util.wy);
  return { x: x, y: y }
}

export function makeSphere(scene, fb, pos) {
  var coord = posToSceneCoord(pos);
  scene.make_sphere(coord.x, coord.y);
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
  var coord = posToSceneCoord(pos);
  var maybeIdx = scene.hit_sphere(coord.x, coord.y);
  if (maybeIdx >= scene.sphere_count()) {
    return false;
  }
  return maybeIdx;
}
