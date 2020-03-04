var cacheName = 'nem-pwa-%%CACHE_IDENT%%';
var appShellFiles = [
  '/assets/pwa/index.html',
];

self.addEventListener('install', function(event) {
  console.log('nem pwa sw: installing pwa');
  event.waitUntil(
    caches.open(cacheName).then(function(cache) {
      console.log('nem pwa sw: caching app shell and content');
      return cache.addAll(appShellFiles);
    })
  );
});

self.addEventListener('fetch', function(event) {
  event.respondWith(
    caches.match(event.request, {cacheName}).then(function(req) {
      if (req) {
        console.log(`nem pwa sw: using cache ${cacheName} for resource: ${event.request.url}`);
        return req;
      }

      console.log(`nem pwa sw: cache ${cacheName} missing resource, fetching: ${event.request.url}`);
      return fetch(event.request).then(function(response) {
        return caches.open(cacheName).then(function(cache) {
          console.log('nem pwa sw: caching new resource: ' + event.request.url);
          cache.put(event.request, response.clone());
          return response;
        });
      });
    })
  );
});

self.addEventListener('activate', function(event) {
  console.log('nem pwa sw: activating pwa');
  event.waitUntil(
    caches.keys().then(function(keyList) {
      return Promise.all(keyList.map(function(key) {
        if (cacheName.indexOf(key) === -1) {
          console.log('nem pwa sw: deleting cache: ' + key);
          return caches.delete(key);
        }
      }));
    })
  );
});

