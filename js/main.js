//import * as scene from './scene.js';
import * as util from './util.js';
import init, { Scene } from '../pkg/wasm_raytracer.js';

var grabbingSphere = false;
var canvasSelector = "#" + util.canvasName;

async function run() {
  //initialize WebAssembly module
  const wasm = await init();
  const scene = Scene.new();
  scene.render();
  const memory = wasm.memory;
  const ptr = scene.spheres();
  console.log(ptr);
  const spheres = new Uint8Array(memory.buffer, ptr, 100);
  console.log(spheres);
  //$(document).ready(function() {
  //  $(canvasSelector).mousedown(
  //    function(evt) {
  //      var mousePos = util.getMousePos(evt);
  //      grabbingSphere = scene.hitSphere(mousePos);
  //      if (!grabbingSphere) {
  //        grabbingSphere = scene.makeSphere(mousePos);
  //      }
  //      else {
  //        if (evt.which == 2) {
  //          scene.deleteSphere(grabbingSphere);
  //          grabbingSphere = false;
  //        }
  //      }
  //    });
  //  $(canvasSelector).mouseup(
  //    function(evt) {
  //      var mousePos = util.getMousePos(evt);
  //      if (grabbingSphere && evt.which == 1) {
  //        grabbingSphere = false;
  //      }
  //    });
  //  $(canvasSelector).mousemove(
  //    function(evt) {
  //      if (grabbingSphere) {
  //        var mousepos = util.getMousePos(evt);
  //        scene.moveSphere(grabbingSphere, mousepos);
  //      }
  //    });
  //});
}
run();
