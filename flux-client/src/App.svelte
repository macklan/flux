<script lang="ts">
  import StreamPlayer from './components/StreamPlayer.svelte';

  let title = 'Flux';

  // Placeholder for user authentication status
  let isAuthenticated = false;

  // Placeholder for stream ID
  let streamId = 'default-stream-id';

  function toggleAuth() {
    isAuthenticated = !isAuthenticated;
  }
</script>

<main>
  <h1>{title}</h1>
    <StreamPlayer {streamId} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 600px;
    margin: 0 auto;
  }

  h1 {
    color: #333;
  }

  button {
    margin: 1em 0;
    padding: 0.5em 1em;
    font-size: 1em;
  }

  p {
    color: #666;
  }
</style>