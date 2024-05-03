<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  let isThinking = false;
  let isSpeaking = false;

  let container: HTMLDivElement;

  $: {
    if (container) {
      if (isThinking) {
        Array.from(container.querySelectorAll(".ai-bar")).map((element) => {
          element.classList.add("thinking");
        });
      }
      if (isSpeaking) {
        Array.from(container.querySelectorAll(".ai-bar")).map((element) => {
          element.classList.add("speaking");
        });
      }
    }
  }

  const listened = listen<{ filename: string }>("new-audio", async (event) => {
    console.log("new event:", event);
    const { filename } = event.payload;
    if (!filename) {
      console.error("no filename emitted via event");
      return;
    }

    isThinking = true;

    const result = await invoke("speech_to_text", { filename });

    console.log("AI:", result);

    isThinking = false;
  });
</script>

<div bind:this={container} class="ai-container">
  <div class="ai-bar" style="--i:1.5"></div>
  <div class="ai-bar" style="--i:2"></div>
  <div class="ai-bar" style="--i:2.5"></div>
  <div class="ai-bar" style="--i:2"></div>
  <div class="ai-bar" style="--i:1.5"></div>
</div>

<style>
  .ai-container {
    display: flex;
    gap: 2rem;
    width: 100%;
    justify-content: center;
    align-items: center;
  }
  .ai-bar {
    width: 0.3rem;
    border-radius: 25%;
    height: 1.5rem;
    background-color: #9051e6;
    filter: drop-shadow(0 0 10px #1aaec8) drop-shadow(0 0 6px #9051e6);
  }
</style>
