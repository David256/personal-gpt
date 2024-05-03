<script lang="ts">
  let microphoneWave: HTMLDivElement;
  let microphoneSubwave: HTMLDivElement;

  export let onStartRecording: () => void = () => {};
  export let onStopRecording: () => void = () => {};

  const onMouseDown = () => {
    // recording
    microphoneWave.classList.add("recording");
    microphoneSubwave.classList.add("subrecording");
    onStartRecording();
  };

  const onMouseUp = () => {
    microphoneWave.classList.remove("recording");
    microphoneSubwave.classList.remove("subrecording");
    onStopRecording();
  };
</script>

<div class="microphone-container">
  <div class="microphone-wave" bind:this={microphoneWave}></div>
  <div class="microphone-subwave" bind:this={microphoneSubwave}></div>
  <div class="icon-container">
    <div
      class="microphone-icon"
      role="button"
      tabindex="0"
      on:mousedown={onMouseDown}
      on:mouseup={onMouseUp}
      on:blur={onMouseUp}
    >
      <img src="/microphone.svg" alt="()" />
    </div>
  </div>
</div>

<style>
  @keyframes idle {
    0% {
      filter: drop-shadow(0 0 1px #1aaec8) drop-shadow(0 0 6px #9051e6);
    }
    100% {
      filter: drop-shadow(0 0 20px #1aaec8) drop-shadow(0 0 15px #9051e6);
    }
  }
  .microphone-subwave {
    position: absolute;
    background-color: transparent;
    border: 5px solid hsl(189, 77%, 24%);
    width: 40vw;
    border-radius: 100%;
    transition: 1.5s all ease;
    transform: scale(1);
    aspect-ratio: 1;
  }

  .microphone-wave {
    position: absolute;
    background-color: hsl(189, 77%, 34%);
    /* hsl(304, 100%, 25%); */
    width: 40vw;
    border-radius: 100%;
    transition: 1.5s all ease;
    transform: scale(1);
    aspect-ratio: 1;
  }

  .microphone-container {
    position: relative;
    aspect-ratio: 1;
    border-radius: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .icon-container {
    background: linear-gradient(-45deg, #9051e6, #1aaec8);
    /* filter: drop-shadow(0 0 12px #1aaec8) drop-shadow(0 0 1px blue); */
    filter: drop-shadow(0 0 1px #1aaec8) drop-shadow(0 0 6px #9051e6);
    padding: 2px;
  }
  .icon-container:hover {
    /* filter: drop-shadow(0 0 14px #1aaec8) drop-shadow(0 0 6px #9051e6); */
    box-shadow: 0px 0px 10px 1px hsl(265, 75%, 61%);
    cursor: pointer;
    /* Animation */
    animation-name: idle;
    animation-duration: 0.1s;
    animation-timing-function: ease-out;
    /* animation-delay: 0s; */
    animation-iteration-count: 1;
    animation-direction: forwards;
    animation-fill-mode: backwards;
  }
  .icon-container,
  .microphone-icon {
    aspect-ratio: 1;
    width: 25vh;
    border-radius: 100%;
    display: flex;
    justify-content: center;
  }
  .microphone-icon {
    background-color: black;
  }
  .microphone-icon:hover {
    background: radial-gradient(
      farthest-corner at 40px 40px,
      hsl(189, 77%, 44%),
      hsl(265, 75%, 21%) /* 61% */
    );
  }
  .microphone-icon:active {
    background: radial-gradient(
      farthest-corner at 50px 50px,
      hsl(265, 75%, 21%) 0%,
      hsl(265, 75%, 11%) 32%,
      hsl(265, 75%, 11%) 62%,
      /* 61% */ hsl(189, 77%, 44%)
    );
  }
  .microphone-icon img {
    width: 15vw;
  }
</style>
