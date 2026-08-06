// Service worker: makes the app installable and usable offline.
//
// Strategy is network-first with a cache fallback. The previous cache-first
// version served whatever was cached until the cache name changed by hand,
// which meant a deployed fix could sit unseen on a returning user's device.
// Since the WebAssembly bundle has a stable filename (Trunk.toml sets
// filehash = false, because this file names the bundle explicitly), the cache
// cannot tell old from new on its own — so the network decides when it can,
// and the cache only steps in when the device is actually offline.

var cacheName = 'communicator-pwa-v3';
var filesToCache = [
  './',
  './index.html',
  './manifest.json',
  './eframe_template.js',
  './eframe_template_bg.wasm',
];

self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
  // Take over as soon as the new worker is ready rather than waiting for
  // every tab to close.
  self.skipWaiting();
});

self.addEventListener('activate', function (e) {
  e.waitUntil(
    caches.keys().then(function (keys) {
      return Promise.all(
        keys.map(function (key) {
          return key === cacheName ? undefined : caches.delete(key);
        })
      );
    }).then(function () {
      return self.clients.claim();
    })
  );
});

self.addEventListener('fetch', function (e) {
  if (e.request.method !== 'GET') {
    return;
  }
  e.respondWith(
    fetch(e.request)
      .then(function (response) {
        // Keep the cache warm for the next time there is no network.
        if (response && response.ok && response.type === 'basic') {
          var copy = response.clone();
          caches.open(cacheName).then(function (cache) {
            cache.put(e.request, copy);
          });
        }
        return response;
      })
      .catch(function () {
        return caches.match(e.request).then(function (cached) {
          if (cached) {
            return cached;
          }
          // An offline reload of a deep link still has to land somewhere.
          if (e.request.mode === 'navigate') {
            return caches.match('./index.html');
          }
          return Response.error();
        });
      })
  );
});
