"use strict";

import firebase from 'firebase/app';
import 'firebase/auth';
import 'firebase/firestore';
import('./assets.js');


fetch('/__/firebase/init.json')
  .then(response => {
    return response.json();
  })
  .then(config => {
    firebase.initializeApp(config);

    firebase.auth().onAuthStateChanged(function(user) {
      if (user) {
        import('../target/wasm32-unknown-unknown/debug/minimal.js');
      } else {
        const provider = new firebase.auth.GoogleAuthProvider();
        firebase.auth().signInWithRedirect(provider);
      }
    });
  });


