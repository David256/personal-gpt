<script lang="ts">
  export let title: string;
</script>

<div class="modal-container">
  <div class="modal-background">
    <div class="modal-border">
      <div class="modal-body">
        <div class="modal-header">
          <div class="modal-header-title">
            {title}
          </div>
          <span role="button" class="modal-close">&times;</span>
        </div>
        <div class="modal-body-content">
          <slot />
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
  }
  .modal-background {
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
  }
  .modal-body {
    width: 75vw;
    padding: 0.5rem;
    box-sizing: border-box;
    min-height: 30vh;
    background-color: black;
  }
  .modal-border {
    /* This 3 lines are duplicated, refactorize */
    background: linear-gradient(-45deg, #9051e6, #1aaec8);
    filter: drop-shadow(0 0 1px #1aaec8) drop-shadow(0 0 6px #9051e6);
    padding: 2px;
  }
  .modal-close:hover {
    filter: brightness(2);
    background-color: black;
    rotate: 90deg;
    transition: all 1s;
  }
  .modal-close {
    transition: all 1s;
    /* background-color: rgb(95, 0, 0); */
    background-color: transparent;
    border: 1px solid rgb(95, 0, 0);
    aspect-ratio: 1;
    width: 1.5rem;
    border-radius: 100%;
    text-align: center;
    cursor: pointer;
  }
  .modal-header {
    padding-bottom: 0.5rem;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid purple;
    filter: drop-shadow(0 0 1px #1aaec8) drop-shadow(0 0 6px #9051e6);
  }
  .modal-body-content {
    padding-top: 0.5rem;
  }
</style>
