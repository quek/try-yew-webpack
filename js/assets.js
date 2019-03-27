"use strict";

const ASSETS = {
  '1.png': require('../assets/1.png')
};

window.assets = function(name) {
  return ASSETS[name];
};
