<script lang="ts">
  import { onMount } from "svelte";
  let videoElement: HTMLVideoElement | null = null;

  onMount(async () => {
    videoElement = document.getElementById("video-player") as HTMLVideoElement;
    let stream = null;
    try {
      stream = await navigator.mediaDevices.getUserMedia({
        video: true,
      });
      videoElement.srcObject = stream;
      videoElement.play();
      console.log("GOT STREAM", stream);
    } catch (error) {
      console.error("SHIT BROKE", error);
    }
  });
</script>

<div class="stream-player">
  <video id="video-player">
    <track
      kind="captions"
      label="English captions"
      src=""
      srclang="en"
      default
    />
  </video>
</div>

<style>
  .stream-player {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  video {
    width: 100%;
    max-width: 600px;
  }

  button {
    margin-top: 10px;
  }
</style>
