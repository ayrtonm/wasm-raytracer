import { redraw, makeSphere, deleteSphere, moveSphere, hitSphere } from './scene.js';
import * as util from './util.js';
import init, { Scene, Sphere } from '../pkg/wasm_raytracer.js';

var grabbingSphere = false;
var canvasSelector = "#" + util.canvasName;

async function run() {
  //initialize WebAssembly modules and memory
  const wasm = await init();
  const memory = wasm.memory;

  //initialize and draw the scene
  const scene = Scene.new(util.canvas.width, util.canvas.height);
  const fbPtr = scene.framebuffer();
  const framebuffer = new Uint8ClampedArray(memory.buffer, fbPtr, 320*240*4);
  redraw(scene, framebuffer);


  $(document).ready(function() {
    $(canvasSelector).mousedown(
      function(evt) {
        var mousePos = util.getMousePos(evt);
        grabbingSphere = hitSphere(scene, mousePos);
        if (!grabbingSphere) {
          grabbingSphere = makeSphere(scene, framebuffer, mousePos);
        }
        else {
          if (evt.which == 2) {
            deleteSphere(scene, framebuffer, grabbingSphere);
            grabbingSphere = false;
          }
        }
      });
    $(canvasSelector).mouseup(
      function(evt) {
        var mousePos = util.getMousePos(evt);
        if (grabbingSphere && evt.which == 1) {
          grabbingSphere = false;
        }
      });
    $(canvasSelector).mousemove(
      function(evt) {
        if (grabbingSphere) {
          var mousepos = util.getMousePos(evt);
          moveSphere(scene, framebuffer, grabbingSphere, mousepos);
        }
      });
  });
}
run();
