import * as glue from './glue.js';
import * as util from './util.js';
import init, { Scene } from '../pkg/wasm_raytracer.js';

var grabbingSphere = false;
var showingHelp = false
var canvasSelector = "#" + util.canvasName;

function showHelp(pos) {
  const winx = 110;
  const winy = 40;
  const cornerRadius = 10;
  pos.x -= winx;
  pos.y -= winy;
  var textpx = pos.x + 5;
  var textpy = pos.y + 3;
  util.ctx.fillStyle = "#000000";
  util.ctx.globalAlpha = 0.7;
  util.ctx.beginPath();
  util.ctx.moveTo(pos.x + cornerRadius, pos.y);
  util.ctx.lineTo(pos.x + winx - cornerRadius, pos.y);
  util.ctx.arc(pos.x + winx - cornerRadius, pos.y + cornerRadius, cornerRadius, -Math.PI / 2.0, 0);
  util.ctx.lineTo(pos.x + winx, pos.y + winy - cornerRadius);
  util.ctx.arc(pos.x + winx - cornerRadius, pos.y + winy - cornerRadius, cornerRadius, 0, Math.PI / 2.0);
  util.ctx.lineTo(pos.x + cornerRadius, pos.y + winy);
  util.ctx.arc(pos.x + cornerRadius, pos.y + winy - cornerRadius, cornerRadius, Math.PI / 2.0, Math.PI);
  util.ctx.lineTo(pos.x, pos.y + cornerRadius);
  util.ctx.arc(pos.x + cornerRadius, pos.y + cornerRadius, cornerRadius, Math.PI, 1.5 * Math.PI);
  util.ctx.stroke();
  util.ctx.fill();
  util.ctx.globalAlpha = 1;
  util.ctx.fillStyle = "#ffffff";
  util.ctx.fillText("Left click to make and", textpx, textpy + 10);
  util.ctx.fillText("move spheres, middle", textpx, textpy + 20);
  util.ctx.fillText("click to delete", textpx, textpy + 30);
}

async function run() {
  //initialize WebAssembly modules and memory
  const wasm = await init();
  const memory = wasm.memory;

  //initialize and draw the scene
  const scene = Scene.new(util.wx, util.wy);
  const fbPtr = scene.framebuffer();
  const bytesPerPixel = 4;
  const imageData = new Uint8ClampedArray(memory.buffer, fbPtr, util.wx*util.wy*bytesPerPixel);
  const frameBuffer= new ImageData(imageData, util.wx, util.wy);
  glue.redraw(scene, frameBuffer);

  $(document).ready(function() {
    $(canvasSelector).mouseenter(
      function(evt) {
        showingHelp = true;
      });
    $(canvasSelector).mouseleave(
      function(evt) {
        glue.redisplay(frameBuffer);
        showingHelp = false;
        grabbingSphere = false;
      });
    $(canvasSelector).mousedown(
      function(evt) {
        showingHelp = false;
        glue.redisplay(frameBuffer);
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
        if (showingHelp) {
          glue.redisplay(frameBuffer);
          var mousepos = util.getMousePos(evt);
          showHelp(mousepos);
        }
        if (grabbingSphere !== false) {
          var mousepos = util.getMousePos(evt);
          glue.moveSphere(scene, frameBuffer, grabbingSphere, mousepos);
        }
      });
  });
}
run();
