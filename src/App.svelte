<script lang="ts">
  import Microphone from "./lib/Microphone.svelte";
</script>

<main class="container">
  <div class="row">
    <div class="col center">
      <h1 class="title">Personal GPT</h1>
    </div>
  </div>

  <div class="row">
    <div class="col p-2">
      <Microphone />
    </div>
  </div>
</main>

<style>
  .title {
    color: rgb(240, 252, 255);
  }
</style>
