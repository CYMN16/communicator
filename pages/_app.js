import '../styles/globals.css';
import Head from 'next/head';
import { useEffect } from 'react';

function MyApp({ Component, pageProps }) {
  useEffect(() => {
    // Register service worker
    if (typeof window !== 'undefined' && 'serviceWorker' in navigator) {
      navigator.serviceWorker.register('sw.js')
        .then((registration) => {
          console.log('Service Worker registered:', registration);
        })
        .catch((error) => {
          console.log('Service Worker registration failed:', error);
        });
    }

    // Check for app updates
    if (typeof window !== 'undefined' && 'serviceWorker' in navigator) {
      navigator.serviceWorker.addEventListener('controllerchange', () => {
        window.location.reload();
      });
    }
  }, []);

  return (
    <>
      <Head>
        {/* Manifest */}
        <link rel="manifest" href="manifest.json" />

        {/* iOS specific meta tags for iOS 12+ */}
        <meta name="apple-mobile-web-app-capable" content="yes" />
        <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
        <meta name="apple-mobile-web-app-title" content="Communicator" />

        {/* Apple icons - iOS uses these for home screen */}
        <link rel="apple-touch-icon" href="apple-touch-icon.png" />
        <link rel="apple-touch-icon" sizes="192x192" href="icon-192.png" />
        <link rel="apple-touch-icon" sizes="512x512" href="icon-512.png" />

        {/* Viewport optimization for mobile and iOS safe areas */}
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover" />
        <meta name="theme-color" content="#000000" />

        {/* Mobile web app meta tags */}
        <meta name="mobile-web-app-capable" content="yes" />
        <meta name="application-name" content="Communicator" />

        {/* Icons for browsers */}
        <link rel="icon" type="image/png" href="icon-192.png" />
        <link rel="shortcut icon" href="icon-192.png" />

        {/* Standalone mode for iOS */}
        <meta name="format-detection" content="telephone=no" />
        <meta name="format-detection" content="email=no" />
      </Head>
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;
