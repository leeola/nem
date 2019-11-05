var cacheName = 'foo';
var appShellFiles = [
  '/assets/pwa/index.html',
];

self.addEventListener('install', function(event) {
  console.log('nem: installing pwa');
  event.waitUntil(
    caches.open(cacheName).then(function(cache) {
      console.log('nem: caching app shell and content');
      return cache.addAll(appShellFiles);
    })
  );
});

self.addEventListener('fetch', function(event) {
  console.log('nem: installing pwa');
  event.respondWith(
    caches.match(event.request).then(function(req) {
      console.log('nem: fetching resource: ' + event.request.url);
      return req || fetch(event.request).then(function(response) {
        return caches.open(cacheName).then(function(cache) {
          console.log('nem: caching new resource: ' + event.request.url);
          cache.put(event.request, response.clone());
          return response;
        });
      });
    })
  );
});

self.addEventListener('activate', function(event) {
  console.log('nem: activating pwa');
  event.waitUntil(
    caches.keys().then(function(keyList) {
      return Promise.all(keyList.map(function(key) {
        if (cacheName.indexOf(key) === -1) {
          console.log('nem: deleting cache: ' + key);
          return caches.delete(key);
        }
      }));
    })
  );
});

