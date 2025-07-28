import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  server: {
    port: 3001
  },
  plugins: [svelte()],
  resolve: {
    alias: {
      '@': '/src'
    }
  }
});