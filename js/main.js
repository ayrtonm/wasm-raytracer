import * as scene from './scene.js';
import * as util from './util.js';

var grabbingSphere = false;
var canvasselector = "#" + util.canvasname;

$(document).ready(function() {
  $(canvasselector).mousedown(
    function(evt) {
      var mousePos = util.getMousePos(evt);
      grabbingSphere = scene.hitSphere(mousePos);
      if (!grabbingSphere) {
        grabbingSphere = scene.makeSphere(mousePos);
      }
      else {
        if (evt.which == 2) {
          scene.deleteSphere(grabbingSphere);
          grabbingSphere = false;
        }
      }
    });
  $(canvasselector).mouseup(
    function(evt) {
      var mousePos = util.getMousePos(evt);
      if (grabbingSphere && evt.which == 1) {
        grabbingSphere = false;
      }
    });
  $(canvasselector).mousemove(
    function(evt) {
      if (grabbingSphere) {
        var mousepos = util.getMousePos(evt);
        scene.moveSphere(grabbingSphere, mousepos);
      }
    });
});

scene.redraw();
