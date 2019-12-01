import * as glue from './glue.js';
import * as util from './util.js';
import init, { Scene } from '../pkg/wasm_raytracer.js';

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
  glue.redraw(scene, framebuffer);

  $(document).ready(function() {
    $(canvasSelector).mousedown(
      function(evt) {
        var mousePos = util.getMousePos(evt);
        grabbingSphere = glue.hitSphere(scene, mousePos);
        if (!grabbingSphere) {
          grabbingSphere = glue.makeSphere(scene, framebuffer, mousePos);
        }
        else {
          if (evt.which == 2) {
            glue.deleteSphere(scene, framebuffer, grabbingSphere);
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
          glue.moveSphere(scene, framebuffer, grabbingSphere, mousepos);
        }
      });
  });
}
run();
