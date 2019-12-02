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
  const imageData = new Uint8ClampedArray(memory.buffer, fbPtr, wx*wy*4);
  const frameBuffer= new ImageData(imageData, util.wx, util.wy);
  glue.redraw(scene, frameBuffer);

  $(document).ready(function() {
    $(canvasSelector).mousedown(
      function(evt) {
        var mousePos = util.getMousePos(evt);
        grabbingSphere = glue.hitSphere(scene, mousePos);
        if (grabbingSphere === false) {
          if (evt.which == 1) {
            grabbingSphere = glue.makeSphere(scene, frameBuffer, mousePos);
          }
        }
        else {
          if (evt.which == 2) {
            glue.deleteSphere(scene, frameBuffer, grabbingSphere);
            grabbingSphere = false;
          }
        }
      });
    $(canvasSelector).mouseup(
      function(evt) {
        var mousePos = util.getMousePos(evt);
        if (grabbingSphere !== false && evt.which == 1) {
          grabbingSphere = false;
        }
      });
    $(canvasSelector).mousemove(
      function(evt) {
        if (grabbingSphere !== false) {
          var mousepos = util.getMousePos(evt);
          glue.moveSphere(scene, frameBuffer, grabbingSphere, mousepos);
        }
      });
  });
}
run();
