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
  const wx = util.canvas.width;
  const wy = util.canvas.height;
  const scene = Scene.new(wx, wy);
  const fbPtr = scene.framebuffer();
  const framebuffer = new Uint8ClampedArray(memory.buffer, fbPtr, wx*wy*4);
  redraw(scene, framebuffer);
  //setTimeout(function() {redraw(scene, framebuffer)}, 1000);
  //setTimeout(function() {
  //  scene.make_sphere(1,0,-1,0.5);
  //  redraw(scene, framebuffer);
  //  setTimeout(function() {
  //    scene.move_sphere(3,-1,0);
  //    //scene.delete_sphere(3);
  //    redraw(scene, framebuffer);
  //  }, 1000)
  //}, 1000);


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
